use crate::protocolvalue;
use crate::qp44::{CoinType, Heritage};
use crate::protocol_id::QuantumId;
use quantom_value::DimensionObservation;

//
// 🔹 Secure Balance Proof Gate (Sealed)
//
mod sealed {
    pub trait Sealed {}
}

pub trait BalanceProof: sealed::Sealed {}

pub struct VerifiedBalanceProof(());

impl sealed::Sealed for VerifiedBalanceProof {}
impl BalanceProof for VerifiedBalanceProof {}

impl VerifiedBalanceProof {
    fn new() -> Self {
        VerifiedBalanceProof(())
    }
}

//
// 🔹 Canonical Gate (NO DUPLICATION)
//
pub fn verify_balance(
    balance: u128,
    required: u128,
) -> Option<VerifiedBalanceProof> {
    (balance >= required).then(|| VerifiedBalanceProof::new())
}

//
// 🔹 Canonical Cost Verification (ONLY SOURCE OF TRUTH)
//
pub fn verify_cost(
    heritage: &Heritage,
) -> Option<VerifiedBalanceProof> {

    // 1. Observation (ONLY from Heritage)
    let obs = DimensionObservation {
        dimension: heritage.state.dimension(),
        structural_value: heritage.state.structural_value(),
        activations: heritage.state.activations(),
        seed: heritage.transition.origin,
    };

    // 2. Protocol (stateless law)
    let protocol = QuantumId::new();
    // 🔥 Use NETWORK (tau) as instantaneous field
    let нетшорк = heritage.transition.net_work;
    
    let eco =
        protocolvalue::Qtm::from_observation(
            &obs,
            &protocol,
        ); 
        let density =
        eco.density.unwrap_or(1);

    let gross_work =
        heritage.transition.gross_work;

    
    let required = protocol
        ._debt(&obs, heritage.transition.tau)
        .unwrap_or(0);

    let credit =
        нетшорк.saturating_div(density);

    let debt =
        gross_work.saturating_div(required);




    // 3. Gate
    verify_balance(credit, debt)
}

//
// 🔹 Structure (Projection only — no external truth)
//
#[derive(Clone)]
pub struct Structure {
    pub coin: CoinType,
    pub commitment: [u8; 32], // 🔥 canonical identity
    pub density: u128,        // 🔥 derived, not fetched
}

impl Structure {

    pub fn from_heritage(
        coin: CoinType,
        heritage: &Heritage,
        commitment: [u8; 32],
    ) -> Self {

        let protocol = QuantumId::new();

        let obs = DimensionObservation {
            dimension: heritage.state.dimension(),
            structural_value: heritage.state.structural_value(),
            activations: heritage.state.activations(),
            seed: heritage.transition.origin,
        };

        let density = protocol.density(&obs).unwrap_or(0);

        Structure {
            coin,
            commitment,
            density,
        }
    }

    pub fn prove(
        &self,
        required: u128,
    ) -> Option<VerifiedBalanceProof> {
        verify_balance(self.density, required)
    }
}

//
// 🔹 Economy (PURE PROJECTION — NO NETWORK)
//
pub struct Economy {
    pub gravity: u128,
    pub structures: Vec<Structure>,
}

impl Economy {

    pub fn total_density(&self) -> u128 {
        self.structures.iter().map(|s| s.density).sum()
    }

    pub fn from_transition(
        heritage: &Heritage,
        commitment: [u8; 32],
    ) -> Self {

        let mut structures = Vec::new();

        for &coin in CoinType::all() {
            structures.push(
                Structure::from_heritage(
                    coin,
                    heritage,
                    commitment,
                )
            );
        }

        Economy {
            gravity: heritage.transition.tau,
            structures,
        }
    }
}

//
// 🔹 Global Ledger (Optional Aggregation Layer)
//
pub struct EconomyLedger {
    pub states: Vec<Economy>,
}

impl EconomyLedger {

    pub fn total_density(&self) -> u128 {
        self.states
            .iter()
            .flat_map(|e| &e.structures)
            .map(|s| s.density)
            .sum()
    }
}