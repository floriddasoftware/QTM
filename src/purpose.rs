use quantom_value::{Perm, QuantPerm};
use blake3;
use bip39::{Mnemonic, Language};
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
    InvalidEntropyLength { expected: usize, actual: usize },

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
        // ⚠️ Current bip39 crate only supports English
        Language::English
    }
}


// ─────────────────────────────────────────────
// 🔹 BIP39 Handler
// ─────────────────────────────────────────────

pub struct BIP39Handler;

impl BIP39Handler {
    /// Strict detection (returns error)
    pub fn detect_language(
        phrase: &str,
    ) -> Result<BIP39Language, BIP39Error> {
        for lang in Self::all_languages() {
            if Mnemonic::parse_in(lang.to_bip39_language(), phrase).is_ok() {
                return Ok(lang);
            }
        }

        Err(BIP39Error::InvalidMnemonic(
            "Could not detect language".into(),
        ))
    }

    /// Lightweight detection (SDK-friendly)
    pub fn detect_language_opt(
        phrase: &str,
    ) -> Option<BIP39Language> {
        Self::detect_language(phrase).ok()
    }

    /// Always return deterministic 32-byte entropy
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

        // Modern bip39 → to_seed
        let seed = mnemonic.to_seed(passphrase.unwrap_or(""));

        // Reduce to 32 bytes deterministically
        let hash = blake3::hash(&seed);

        Ok(*hash.as_bytes())
    }

    /// Helper: all supported languages
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
// 🔹 Purpose (Identity Compiler)
// ─────────────────────────────────────────────

pub struct Purpose;

impl Purpose {
    pub fn quantperm_seed(
        input: SeedSource,
    ) -> Result<QuantPerm, String> {
        let entropy = match input {
            SeedSource::BIP39 {
                phrase,
                language,
                passphrase,
            } => BIP39Handler::mnemonic_entropy(
                &phrase,
                language,
                passphrase.as_deref(),
            )
            .map_err(|e| e.to_string())?,

            SeedSource::Raw(data) => Self::validate_entropy(&data)?,

            SeedSource::Custom(data) => Self::hash_to_entropy(&data),
        };

        let perm = Self::perm_from_entropy(&entropy);

        Ok(QuantPerm::new(perm))
    }

    // ─────────────────────────────────────────────
    // 🔹 Entropy Handling
    // ─────────────────────────────────────────────

    fn validate_entropy(data: &[u8]) -> Result<[u8; 32], String> {
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

    fn hash_to_entropy(data: &[u8]) -> [u8; 32] {
        *blake3::hash(data).as_bytes()
    }

    // ─────────────────────────────────────────────
    // 🔹 Perm Construction (CRITICAL)
    // ─────────────────────────────────────────────

    fn perm_from_entropy(entropy: &[u8; 32]) -> Perm {
        let indices = Self::derive_indices(entropy);

        Perm::genesis_construct(&indices, entropy)
            .expect("Valid deterministic perm construction")
    }

    fn derive_indices(entropy: &[u8; 32]) -> [u16; 12] {
        let mut indices = [0u16; 12];

        let mut hasher = blake3::Hasher::new();
        hasher.update(b"PERM::INDICES::V1");
        hasher.update(entropy);

        let hash = hasher.finalize();
        let bytes = hash.as_bytes();

        for i in 0..12 {
            let val =
                ((bytes[i * 2] as u16) << 8)
                | (bytes[i * 2 + 1] as u16);

            indices[i] = val % 2048;
        }

        indices
    }
}

// ─────────────────────────────────────────────
// 🔹 Builder (Developer API)
// ─────────────────────────────────────────────

pub struct QuantPermBuilder {
    source: Option<SeedSource>,
}

impl QuantPermBuilder {
    pub fn new() -> Self {
        Self { source: None }
    }

    /// Auto-detect language (safe default)
    pub fn from_mnemonic(phrase: impl Into<String>) -> Self {
        let phrase_str = phrase.into();

        let language =
            BIP39Handler::detect_language_opt(&phrase_str)
                .unwrap_or(BIP39Language::English);

        Self {
            source: Some(SeedSource::BIP39 {
                phrase: phrase_str,
                language,
                passphrase: None,
            }),
        }
    }

    pub fn from_entropy(entropy: [u8; 32]) -> Self {
        Self {
            source: Some(SeedSource::Raw(entropy.to_vec())),
        }
    }

    pub fn from_custom(data: Vec<u8>) -> Self {
        Self {
            source: Some(SeedSource::Custom(data)),
        }
    }

    pub fn build(self) -> Result<QuantPerm, String> {
        let source = self.source.ok_or("Missing seed")?;
        Purpose::quantperm_seed(source)
    }
}