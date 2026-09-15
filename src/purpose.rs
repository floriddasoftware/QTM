use quantom_value::{Perm, QuantPerm};
use blake3;
use bip39::{Language, Mnemonic};
use sha2::{Digest, Sha256};
use thiserror::Error;

// ─────────────────────────────────────────────
// 🔹 Errors
// ─────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum BIP39Error {
    #[error("Invalid mnemonic phrase: {0}")]
    InvalidMnemonic(String),

    #[error("Invalid mnemonic language")]
    InvalidLanguage,

    #[error("Invalid entropy length: expected {expected}, got {actual}")]
    InvalidEntropyLength {
        expected: usize,
        actual: usize,
    },

    #[error("Unsupported word count: {0}")]
    UnsupportedWordCount(usize),
}

// ─────────────────────────────────────────────
// 🔹 Languages
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BIP39Language {
    English,
    ChineseSimplified,
    ChineseTraditional,
    French,
    Italian,
    Japanese,
    Korean,
    Spanish,
    Czech,
    Portuguese,
}

impl BIP39Language {
    pub fn to_bip39_language(self) -> Language {
        // The selected bip39 crate/version currently exposes
        // English through this API.
        //
        // Keep this mapping explicit rather than pretending
        // the other language variants are currently implemented.
        Language::English
    }
}

// ─────────────────────────────────────────────
// 🔹 BIP39 Handler
// ─────────────────────────────────────────────

pub struct BIP39Handler;

impl BIP39Handler {
    /// Strict language detection.
    pub fn detect_language(
        phrase: &str,
    ) -> Result<BIP39Language, BIP39Error> {
        for lang in Self::all_languages() {
            if Mnemonic::parse_in(
                lang.to_bip39_language(),
                phrase,
            )
            .is_ok()
            {
                return Ok(lang);
            }
        }

        Err(BIP39Error::InvalidMnemonic(
            "Could not detect language".into(),
        ))
    }

    /// Lightweight detection.
    pub fn detect_language_opt(
        phrase: &str,
    ) -> Option<BIP39Language> {
        Self::detect_language(phrase).ok()
    }

    /// Return deterministic 32-byte conditioning entropy
    /// derived from the BIP39 seed.
    ///
    /// IMPORTANT:
    /// This is NOT the original BIP39 entropy.
    /// It is a 32-byte PERM conditioning value derived from
    /// the BIP39 512-bit seed.
    pub fn mnemonic_entropy(
        phrase: &str,
        language: BIP39Language,
        passphrase: Option<&str>,
    ) -> Result<[u8; 32], BIP39Error> {
        let mnemonic = Mnemonic::parse_in(
            language.to_bip39_language(),
            phrase,
        )
        .map_err(|e| BIP39Error::InvalidMnemonic(e.to_string()))?;

        // BIP39 PBKDF2-derived 512-bit seed.
        let seed = mnemonic.to_seed(
            passphrase.unwrap_or(""),
        );

        // PERM receives a fixed 256-bit conditioning value.
        let hash = blake3::hash(&seed);

        Ok(*hash.as_bytes())
    }

    /// Extract the EXACT BIP39 12-position coordinate.
    ///
    /// BIP39:
    ///
    ///     word index ∈ [0, 2047]
    ///
    /// PERM:
    ///
    ///     domain index ∈ [0, 2047]
    ///
    /// Therefore:
    ///
    ///     BIP39 index N == PERM index N
    ///
    /// No re-randomization or re-indexing occurs here.
    pub fn mnemonic_indices(
        phrase: &str,
        language: BIP39Language,
    ) -> Result<[u16; 12], BIP39Error> {
        let mnemonic = Mnemonic::parse_in(
            language.to_bip39_language(),
            phrase,
        )
        .map_err(|e| BIP39Error::InvalidMnemonic(e.to_string()))?;

        // A 12-word BIP39 mnemonic contains exactly
        // 128 bits of entropy.
        let entropy = mnemonic.to_entropy();

        if entropy.len() != 16 {
            let word_count = match entropy.len() {
                16 => 12,
                20 => 15,
                24 => 18,
                28 => 21,
                32 => 24,
                _ => 0,
            };

            if word_count != 12 {
                return Err(BIP39Error::UnsupportedWordCount(word_count));
            }

            return Err(BIP39Error::InvalidEntropyLength {
                expected: 16,
                actual: entropy.len(),
            });
        }

        // Reconstruct the exact 132-bit BIP39 payload:
        //
        //     ENT = 128 bits
        //     CS  =   4 bits
        //     ----------------
        //     total = 132 bits
        //
        // The 132 bits are then divided into twelve 11-bit
        // word indices.
        let checksum_hash = Sha256::digest(&entropy);

        let checksum_bit =
            (checksum_hash[0] >> 7) & 1;

        let mut bits = [0u8; 17];

        // Copy the 128 entropy bits.
        bits[..16].copy_from_slice(&entropy);

        // Store the four checksum bits in the final nibble.
        bits[16] = checksum_bit << 7;

        // BIP39 12-word mnemonics require four checksum bits.
        for i in 1..4 {
            let bit =
                (checksum_hash[0] >> (7 - i)) & 1;

            bits[16] |= bit << (7 - i);
        }

        let mut indices = [0u16; 12];

        // Read twelve consecutive 11-bit groups.
        //
        // Each resulting value is exactly the BIP39
        // word-list index in [0, 2047].
        for position in 0..12 {
            let start_bit = position * 11;
            let mut value = 0u16;

            for bit_offset in 0..11 {
                let absolute_bit =
                    start_bit + bit_offset;

                let byte_index =
                    absolute_bit / 8;

                let bit_index =
                    7 - (absolute_bit % 8);

                let bit =
                    (bits[byte_index] >> bit_index) & 1;

                value =
                    (value << 1) | bit as u16;
            }

            if value >= 2048 {
                return Err(BIP39Error::InvalidMnemonic(
                    "BIP39 index outside 2048-word domain".into(),
                ));
            }

            indices[position] = value;
        }

        Ok(indices)
    }

    /// Helper: all declared languages.
    fn all_languages() -> [BIP39Language; 10] {
        [
            BIP39Language::English,
            BIP39Language::ChineseSimplified,
            BIP39Language::ChineseTraditional,
            BIP39Language::French,
            BIP39Language::Italian,
            BIP39Language::Japanese,
            BIP39Language::Korean,
            BIP39Language::Spanish,
            BIP39Language::Czech,
            BIP39Language::Portuguese,
        ]
    }
}

// ─────────────────────────────────────────────
// 🔹 Seed Source
// ─────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum SeedSource {
    BIP39 {
        phrase: String,
        language: BIP39Language,
        passphrase: Option<String>,
    },

    Raw(Vec<u8>),

    Custom(Vec<u8>),
}

// ─────────────────────────────────────────────
// 🔹 Purpose
// ─────────────────────────────────────────────
//
// Identity compiler:
//
//     BIP39 word indices
//             │
//             ▼
//     PERM coordinate environment
//             │
//             ▼
//       QuantPerm geometry
//
// The BIP39 index domain is preserved exactly:
//
//     BIP39 word 0    → PERM index 0
//     BIP39 word 1    → PERM index 1
//     ...
//     BIP39 word 2047 → PERM index 2047
//
// No secondary index derivation occurs.
//

pub struct Purpose;

impl Purpose {
    pub fn quantperm_seed(
        input: SeedSource,
    ) -> Result<QuantPerm, String> {
        match input {
            SeedSource::BIP39 {
                phrase,
                language,
                passphrase,
            } => {
                // ─────────────────────────────────────
                // 1. Preserve the native BIP39 coordinate.
                // ─────────────────────────────────────

                let indices =
                    BIP39Handler::mnemonic_indices(
                        &phrase,
                        language,
                    )
                    .map_err(|e| e.to_string())?;

                // ─────────────────────────────────────
                // 2. Derive independent 256-bit
                //    conditioning material.
                // ─────────────────────────────────────

                let entropy =
                    BIP39Handler::mnemonic_entropy(
                        &phrase,
                        language,
                        passphrase.as_deref(),
                    )
                    .map_err(|e| e.to_string())?;

                // ─────────────────────────────────────
                // 3. Build PERM directly from the
                //    original BIP39 coordinate.
                // ─────────────────────────────────────

                let perm =
                    Self::perm_from_indices(
                        &indices,
                        &entropy,
                    );

                Ok(QuantPerm::new(perm))
            }

            SeedSource::Raw(data) => {
                let entropy =
                    Self::validate_entropy(&data)?;

                let indices =
                    Self::derive_raw_indices(&entropy);

                let perm =
                    Self::perm_from_indices(
                        &indices,
                        &entropy,
                    );

                Ok(QuantPerm::new(perm))
            }

            SeedSource::Custom(data) => {
                let entropy =
                    Self::hash_to_entropy(&data);

                let indices =
                    Self::derive_raw_indices(&entropy);

                let perm =
                    Self::perm_from_indices(
                        &indices,
                        &entropy,
                    );

                Ok(QuantPerm::new(perm))
            }
        }
    }

    // ─────────────────────────────────────────────
    // 🔹 Entropy Handling
    // ─────────────────────────────────────────────

    fn validate_entropy(
        data: &[u8],
    ) -> Result<[u8; 32], String> {
        if data.len() != 32 {
            return Err(format!(
                "Entropy must be 32 bytes (got {})",
                data.len()
            ));
        }

        let mut out = [0u8; 32];

        out.copy_from_slice(data);

        Ok(out)
    }

    fn hash_to_entropy(
        data: &[u8],
    ) -> [u8; 32] {
        *blake3::hash(data).as_bytes()
    }

    // ─────────────────────────────────────────────
    // 🔹 PERM Construction
    // ─────────────────────────────────────────────
    //
    // CRITICAL:
    //
    // BIP39 path:
    //
    //     mnemonic
    //        │
    //        ▼
    //     [u16;12]
    //        │
    //        ▼
    //      PERM
    //
    // There is NO:
    //
    //     entropy → random indices
    //
    // transformation for BIP39.
    //
    // The indices entering PERM are the exact BIP39
    // word-list indices.
    //

    fn perm_from_indices(
        indices: &[u16; 12],
        entropy: &[u8; 32],
    ) -> Perm {
        Perm::genesis_construct(
            indices,
            entropy,
        )
        .expect(
            "Valid deterministic PERM construction",
        )
    }

    // ─────────────────────────────────────────────
    // 🔹 Raw/Custom coordinate derivation
    // ─────────────────────────────────────────────
    //
    // Raw and Custom sources do not have native BIP39
    // word indices, so they require a deterministic
    // coordinate derivation.
    //
    // This path is intentionally separate from BIP39.
    //

    fn derive_raw_indices(
        entropy: &[u8; 32],
    ) -> [u16; 12] {
        let mut indices = [0u16; 12];

        let mut hasher =
            blake3::Hasher::new();

        hasher.update(
            b"PERM::RAW_INDICES::V1",
        );

        hasher.update(entropy);

        let hash =
            hasher.finalize();

        let bytes =
            hash.as_bytes();

        for i in 0..12 {
            let val =
                ((bytes[i * 2] as u16) << 8)
                | (bytes[i * 2 + 1] as u16);

            indices[i] =
                val % 2048;
        }

        indices
    }
}

// ─────────────────────────────────────────────
// 🔹 Builder
// ─────────────────────────────────────────────

pub struct QuantPermBuilder {
    source: Option<SeedSource>,
}

impl QuantPermBuilder {
    pub fn new() -> Self {
        Self {
            source: None,
        }
    }

    /// Construct directly from a BIP39 mnemonic.
    ///
    /// The BIP39 word indices are preserved exactly.
    pub fn from_mnemonic(
        phrase: impl Into<String>,
    ) -> Self {
        let phrase_str =
            phrase.into();

        let language =
            BIP39Handler::detect_language_opt(
                &phrase_str,
            )
            .unwrap_or(
                BIP39Language::English,
            );

        Self {
            source: Some(
                SeedSource::BIP39 {
                    phrase: phrase_str,
                    language,
                    passphrase: None,
                },
            ),
        }
    }

    pub fn from_entropy(
        entropy: [u8; 32],
    ) -> Self {
        Self {
            source: Some(
                SeedSource::Raw(
                    entropy.to_vec(),
                ),
            ),
        }
    }

    pub fn from_custom(
        data: Vec<u8>,
    ) -> Self {
        Self {
            source: Some(
                SeedSource::Custom(data),
            ),
        }
    }

    pub fn quantperm_from_perm(
        perm: &Perm,
    ) -> Result<QuantPerm, String> {
        Ok(QuantPerm::new(*perm))
    }

    pub fn build(
        self,
    ) -> Result<QuantPerm, String> {
        let source =
            self.source
                .ok_or("Missing seed")?;

        Purpose::quantperm_seed(source)
    }
}
