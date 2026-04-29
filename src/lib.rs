pub mod qp44;
pub mod purpose;
pub mod protocolvalue;
pub mod protocol_id;
pub mod pathsregistry;
pub mod config;
pub mod economic_gate;

// Re-exports (SDK surface)
pub use qp44::{QP44, WalletRequest, CoinType};
pub use purpose::{Purpose, QuantPermBuilder};
pub use protocol_id::QuantumId;
pub use protocolvalue::Qtm;
pub use pathsregistry::PathsRegistry;