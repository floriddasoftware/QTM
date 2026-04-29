use std::collections::HashMap;
use futures::future::join_all;

use crate::pathsregistry::PathsRegistry;
use crate::qp44::{CoinType, Heritage};        
use crate::protocol_id::QuantumId;       
use quantom_value::{DimensionObservation};

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


// 🔹 Canonical Gate

pub fn verify_balance(balance: u128, required: u128) -> Option<VerifiedBalanceProof> {
    if balance >= required {
        Some(VerifiedBalanceProof::new())
    } else {
        None
    }
}


pub fn verify_cost(
    heritage: Heritage,
) -> Option<VerifiedBalanceProof> {

    // ─────────────────────────────
    // 1. Observation is DERIVED ONLY from Heritage
    // ─────────────────────────────
    let obs = DimensionObservation {
        dimension: heritage.dimension,
        structural_value: heritage.structural_value,
        activations: heritage.activation_count,
        seed: heritage.origin 
    };

    // ─────────────────────────────
    // 2. Protocol law (pure stateless operator)
    // ─────────────────────────────
    let protocol = QuantumId::new();

    let density = protocol.density(&obs).unwrap_or(0);

    let required = protocol
        .locked_debt(&obs, heritage.tau)
        .unwrap_or(0);

   
    verify_balance(density, required)
}


// 🔹 Canonical Structure (Commitment-Based)
#[derive(Clone)]
pub struct Structure {
    pub coin: CoinType,
    pub commitment: String, // 🔥 qp{...} (public identity)
    pub address: String,    // derived projection
    pub balance: u128,
}

impl Structure {

    pub fn prove(&self, required: u128) -> Option<VerifiedBalanceProof> {
        verify_balance(self.balance, required)
    }
}

//
// 🔹 Economy (Stateful, Indexed)
//
pub struct Economy {
    pub gravity: u128,   // passed in from QuantPerm::transition()
    pub structures: Vec<Structure>,
    index: HashMap<(CoinType, String), usize>,
}


impl Economy {
    /// Build index for fast lookup
    fn build_index(structures: &[Structure]) -> HashMap<(CoinType, String), usize> {
        structures
            .iter()
            .enumerate()
            .map(|(i, s)| ((s.coin, s.address.clone()), i))
            .collect()
    }

    /// Total backing (canonical)
    pub fn total_backing(&self) -> u128 {
        self.structures.iter().map(|s| s.balance).sum()
    }

    /// Refresh balances across all chains in parallel
    pub async fn refresh_balances(&mut self) {
        let futures = self.structures.iter().map(|s| {
            let coin = s.coin;
            let addr = s.address.clone();

            async move {
                let balance = PathsRegistry::query_balance(coin, &addr)
                    .await
                    .unwrap_or(0);
                (coin, addr, balance)
            }
        });

        let results = join_all(futures).await;

        for (coin, addr, balance) in results {
            if let Some(&i) = self.index.get(&(coin, addr.clone())) {
                self.structures[i].balance = balance;
            }
        }
    }
}

//
// 🔹 Economy Construction from QuantPerm Transition
//
impl Economy {
    /// Build from a single transition
    ///
    /// commitment = qp{...}
    /// coordinate NEVER exposed
    pub fn from_transition(
        gravity: u128,
        commitment: String,
        derive_address: impl Fn(CoinType) -> String,
    ) -> Self {
        let mut structures = Vec::new();

        for &coin in CoinType::all() {
            let address = derive_address(coin);
        
            structures.push(Structure {
                coin,
                commitment: commitment.clone(),
                address,
                balance: 0,
            });
        }

        let index = Self::build_index(&structures);

        Economy {
            gravity,
            structures,
            index,
        }
    }
}

//
// 🔹 Global Ledger (Multi-Transition Economy)
//
pub struct EconomyLedger {
    pub states: Vec<Economy>,
}

impl EconomyLedger {
    /// Total backing across ALL transitions
    pub fn total_backing(&self) -> u128 {
        self.states
            .iter()
            .flat_map(|e| &e.structures)
            .map(|s| s.balance)
            .sum()
    }

    /// Refresh all states
    pub async fn refresh_all(&mut self) {
        let futures = self.states.iter_mut().map(|e| e.refresh_balances());
        join_all(futures).await;
    }
}