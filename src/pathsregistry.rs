use quantom_value::{Heritage, QuantPerm};

use crate::protocolvalue::Qtm;

// ─────────────────────────────────────────────
// 🔹 Path Constants
// ─────────────────────────────────────────────

pub const PURPOSE_44: u128 = 44;
pub const HARDENED_OFFSET: u128 = 0x8000_0000;

// ─────────────────────────────────────────────
// 🔹 Coin Types
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoinType {
    Bitcoin = 0,
    Ethereum = 60,
    Tron = 195,
    Solana = 501,
}

impl CoinType {
    pub fn retained_mass(self) -> u128 {
        self as u128
    }

    pub fn name(self) -> &'static str {
        match self {
            CoinType::Bitcoin => "Bitcoin",
            CoinType::Ethereum => "Ethereum",
            CoinType::Tron => "Tron",
            CoinType::Solana => "Solana",
        }
    }

    pub fn all() -> &'static [CoinType] {
        &[
            CoinType::Bitcoin,
            CoinType::Ethereum,
            CoinType::Solana,
            CoinType::Tron,
        ]
    }
}

// ─────────────────────────────────────────────
// 🔹 Purpose
// ─────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub enum Purpose {
    BIP44,
    BIP32,
    Custom(u32),
}

impl Purpose {
    pub fn value(self) -> u128 {
        match self {
            Purpose::BIP44 => 44,
            Purpose::BIP32 => 32,
            Purpose::Custom(v) => v as u128,
        }
    }
}

// ─────────────────────────────────────────────
// 🔹 Wallet Output
// ─────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct WalletOutput {
    pub coin: CoinType,
    pub coordinate: [u8; 32],
    pub commitment: [u8; 32],
}

// ─────────────────────────────────────────────
// 🔹 Path Resolution
// ─────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct PathInfo {
    pub path: &'static str,
    pub coordinate: [u8; 32],
}

pub struct PathsRegistry;

impl PathsRegistry {
    /// Full derivation:
    /// static BIP path + invariant coordinate
    pub fn derive(
        coin: CoinType,
        manifold: &QuantPerm,
    ) -> PathInfo {
        PathInfo {
            path: Self::get_path(coin),
            coordinate: Self::derive_coordinate(manifold),
        }
    }

    /// Static registry path
    pub fn get_path(coin: CoinType,
    ) -> &'static str {
        match coin {
            CoinType::Bitcoin =>
                "m/44'/0'/0'/0/0",
            CoinType::Ethereum =>
                "m/44'/60'/0'/0/0",
            CoinType::Solana =>
                "m/44'/501'/0'/0'",
            CoinType::Tron =>
                "m/44'/195'/0'/0/0",
        }
    }

    /// Identity invariant.
    /// Never changes through send/receive.
    pub fn derive_coordinate(
        manifold: &QuantPerm,
    ) -> [u8; 32] {
        Qtm::derive_coordinate(manifold)
    }

    /// Economic interpretation of a heritage.
    pub fn resolve_value(
        heritage: &Heritage,
    ) -> Qtm {
        Qtm::economy(heritage)
    }
}