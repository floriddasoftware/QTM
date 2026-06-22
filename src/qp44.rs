// src/qp44.rs

use quantom_value::{QuantPerm, Heritage};
use crate::purpose::{Purpose as SeedPurpose, SeedSource};
use crate::protocolvalue::Qtm;
use crate::pathsregistry::{CoinType, Purpose, WalletOutput,
    PURPOSE_44,
    HARDENED_OFFSET,
};


#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TotalMass {
    pub purpose: u128,
    pub coin: u128,
    pub account: u128,
    pub change: u128,
    pub external: u128,
}
// ─────────────────────────────────────────────
// 🔹 Internal HD State Output (FOR FORENSICS)
// ─────────────────────────────────────────────
#[repr(C)]
pub enum Flow {
    /// External mass entering the manifold.
    Receive,
    /// Internal network decay.
    Change,
}

#[repr(C)]
pub struct QP44Event {
    pub heritage: Heritage,
    pub qtm: Qtm,
}
#[repr(C)]
#[derive(Debug)]
pub struct PQ44Object {
    pub qtm: Qtm,
}
pub struct PQ44Event {
    pub heritage: Heritage,
    pub qtm: Qtm,
}

pub struct Memorize {
    pub heritage: Heritage,
}
// ─────────────────────────────────────────────
// 🔹 Stateful Wallet Engine
// ─────────────────────────────────────────────

pub struct QP44Object {
    pub manifold: QuantPerm,
    pub coin: u128,
}

impl Flow {
    pub fn stream(self, heritage: &Heritage)->(u128, u128){
        match self{Flow::Receive => {
            let external = heritage.state.activations() as u128;
            (external, 0u128)
        }
            Flow::Change => {
                let change = heritage.transition.delta;
                (0u128, change)
            }
    }
 }
}
impl TotalMass {

    pub fn new(
        purpose: u128,
        coin: u128,
        account: u128,
        change: u128,
        external: u128,
    ) -> Self {Self {purpose, coin, account, change, external,
        }
    }

    pub fn realize(&self) -> u128 {

        self.purpose.saturating_add(HARDENED_OFFSET)
            .saturating_add(
                self.coin.saturating_add(HARDENED_OFFSET)
            )
            .saturating_add(
                self.account.saturating_add(HARDENED_OFFSET)
            )
            .saturating_add(self.change)
            .saturating_add(self.external)
    }


    pub fn from_memorized(
        purpose: u128,
        coin: u128,
        account: u128,
        change: u128,
        external: u128,
    ) -> Self {
        Self {
            purpose,
            coin,
            account,
            change,
            external,
        }
    }
    
    pub fn memorize(&self) -> u128 {
        self.purpose.saturating_sub(HARDENED_OFFSET)
            .saturating_add(
                self.coin.saturating_sub(HARDENED_OFFSET)
            )
            .saturating_add(
                self.account.saturating_sub(HARDENED_OFFSET)
            )
            .saturating_add(self.change)
            .saturating_add(self.external)
}
}

impl QP44Object {
    pub fn from_quantperm(
        manifold: QuantPerm,
        coin: u128,
    ) -> Self {
        Self { manifold, coin}
    }



        pub fn realize(self) -> QP44Event {
    
            let mut manifold = self.manifold;

            let external = manifold.activations();
    
            manifold.set_initial_dimension_from_perm();
    
            let total_mass =
                TotalMass::new(
                    PURPOSE_44,
                    self.coin,
                    0,
                    0,
                    external as u128,
                );
    
            let mass =
                total_mass.realize();
    
            let before_dim =
                manifold.dimension();
    
            let qp =
                crate::protocol_id::QuantumId.quantum_seed(
                    crate::economic_gate::verify_balance(0, 0)
                        .expect("Economic gate failed"),
                );
    
            let retain =
                manifold.retain(
                    mass,
                    before_dim,
                );
    
            let heritage =
                manifold.transition(
                    &retain,
                    Some(&qp),
                );
    
            let qtm =
                Qtm::commit(
                    &heritage.state,
                    heritage.transition.net_work,
                );
    
            QP44Event {
                heritage,
                qtm,
            }
        }
    
        pub fn next_receive(self) -> QP44Event {
            self.realize()
        }
    
        pub fn next_change(self) -> QP44Event {
            self.realize()
        }
    
        pub fn into_manifold(self) -> QuantPerm {
            self.manifold
        }
    }



    impl QP44Event {

        pub fn memorize(
            self,
            flow: Flow,
        ) -> QP44Event {
    
            let heritage =
                self.heritage;
    
            
    
            let (account, change) =
                flow.stream(&heritage);

            let manifold =
                heritage.state;
    
            let mass =
                TotalMass::from_memorized(
                    PURPOSE_44,
                    heritage.transition.net_work,
                    account,
                    change,
                    manifold.activations() as u128,
                )
                .memorize();
    
            QP44Object {
                manifold,
                coin: mass,
            }
            .realize()
        }
    }

impl PQ44Object {

    pub fn trigger(
        heritage: &Heritage,
        qtm: Qtm,
    ) -> Self {

        let committed = Qtm::commit(
            &heritage.state,
            heritage.transition.net_work,
        );

        assert_eq!(
            qtm.coordinate,
            committed.coordinate,
            "coordinate mismatch",
        );

        assert_eq!(
            qtm.commitment,
            committed.commitment,
            "commitment mismatch",
        );

        Self {
            qtm,
        }
    }
}


//Model 1 — Physical Manifold Model

//This is the invariant physics layer.

//Its job is only to answer:

//Where am I?
//How much structure exists?
//How much resistance exists?
//How much work occurred?

//Pipeline:

//PERM
//  ↓
//Euclid
//  ↓
//BiasMirror
//  ↓
//Gravity
//  ↓
//QuantPerm
// ─────────────────────────────────────────────
// 🔹 Wallet Request (SDK)
// ─────────────────────────────────────────────

pub struct WalletRequest {
    pub seed: SeedSource,
    pub purpose: Purpose,
    pub coins: Vec<CoinType>,
    pub account: u32,
    pub index: u32,
}

//Model 5
//this code is not a ledger model and not an account model.

//It is a wal_let projection engine that converts deterministic state transitions into
// commitment-coordinate pairs, which are then consumed by the higher Economic/Structure layers
// ─────────────────────────────────────────────
// 🔹 QP44 SDK Engine
// ─────────────────────────────────────────────

pub struct QP44;

impl QP44 {
    pub fn derive_wallet(
        request: WalletRequest,
    ) -> Result<Vec<WalletOutput>, String> {
        let mut outputs = Vec::new();

        for coin in &request.coins {
            // 🔹 Base manifold from seed
            let base = SeedPurpose::quantperm_seed(request.seed.clone())?;

            // 🔹 Stateful driver
            let wallet = QP44Object::from_quantperm(base, coin.retained_mass());

            // 🔹 Perform transition (THIS produces real state)
            let result = wallet.next_receive();


            // 🔥 CRITICAL: commit using POST-TRANSITION manifold
            let qtm = result.qtm;
            
            outputs.push(WalletOutput {
                coin: *coin,
                coordinate: qtm.coordinate,
                commitment: qtm.commitment,  // ✅ economic binding
            });
        }

        Ok(outputs)
    }
}