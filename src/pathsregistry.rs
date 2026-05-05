use crate::qp44::{CoinType, Heritage};
use quantom_value::QuantPerm;
use crate::protocolvalue::{Qtm, Value};

pub struct PathInfo {
    pub path: &'static str,
    pub coordinate: [u8; 32],
}

pub struct PathsRegistry;

impl PathsRegistry {

    // 🔹 Full derivation (path + coordinate)
    pub fn derive(
        coin: CoinType,
        manifold: &QuantPerm,
    ) -> PathInfo {

        PathInfo {
            path: Self::get_path(coin),
            coordinate: Qtm::derive_coordinate(manifold),
        }
    }

    // 🔹 Path only (static)
    pub fn get_path(coin: CoinType) -> &'static str {
        match coin {
            CoinType::Bitcoin  => "m/44'/0'/0'/0/0",
            CoinType::Ethereum => "m/44'/60'/0'/0/0",
            CoinType::Solana   => "m/44'/501'/0'/0'",
            CoinType::Tron     => "m/44'/195'/0'/0/0",
        }
    }

    // 🔹 Coordinate only (identity invariant)
    pub fn derive_coordinate(manifold: &QuantPerm) -> [u8; 32] {
        Qtm::derive_coordinate(manifold)
    }


// 🔹 Economic resolution (NOT registry concern, but acceptable helper)
pub fn resolve_value(
    heritage: &Heritage,
) -> Value {
    Qtm::economy(heritage)
}
}