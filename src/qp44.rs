// src/qp44.rs

use qtm_graph::graph::frame::Depth;
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
pub struct PQ44Object {
    pub manifold: QuantPerm,
    pub coin: u128, 
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

    pub fn deep(&self, manifold: &QuantPerm) -> Depth {
        let mut depth = Depth::new("qp44/wallet");

        depth.insert(
            "purpose",
            0,
            manifold.retain(
                self.purpose.saturating_add(HARDENED_OFFSET),
                manifold.dimension(),
            ),
        );

        depth.insert(
            "coin",
            1,
            manifold.retain(
                self.coin.saturating_add(HARDENED_OFFSET),
                manifold.dimension(),
            ),
        );

        depth.insert(
            "account",
            2,
            manifold.retain(
                self.account.saturating_add(HARDENED_OFFSET),
                manifold.dimension(),
            ),
        );

        depth.insert(
            "change",
            3,
            manifold.retain(
                self.change,
                manifold.dimension(),
            ),
        );

        depth.insert(
            "external",
            4,
            manifold.retain(
                self.external,
                manifold.dimension(),
            ),
        );

        depth
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
        Self { manifold, coin }
    }

    pub fn realize(
        self,
    ) -> QP44Event {
        let manifold = self.manifold;

        let external = manifold.activations();

        let total_mass =
            TotalMass::new(
                PURPOSE_44,
                self.coin,
                0,
                0,
                external as u128,
            );

        let retain =
            total_mass.deep(&manifold,);


        let qp =
            crate::protocol_id::QuantumId.quantum_seed(
                crate::economic_gate::verify_balance(0, 0)
                    .expect("Economic gate failed"),
            );

        // ---------------------------------------------------------------------
        // GRAPH IS THE ONLY TRANSITION GATE.
        //
        // QP-HD does not call Quantom-VALUE::transition() directly.
        // ---------------------------------------------------------------------
        let heritage =
            qtm_graph::graph::frame::Direction::transit(
                manifold,
                &retain,
                Some(&qp),
            );

        let event_map =
            qtm_graph::graph::frame::EventMap::now(
                heritage,
                qtm_graph::graph::scale::Scale::Global,
            );

        // ---------------------------------------------------------------------
        // LOCAL OBSERVER
        //
        // Dashboard is intentionally created here so existing QP44 callers
        // require no new dependency or argument.
        //
        // Heritage remains the sole authentic transition receipt.
        // Dashboard only borrows it.
        // ---------------------------------------------------------------------
        let mut dashboard =
            qtm_graph::Dashboard::new();

        dashboard.journey(
            &event_map,
        );

        // ---------------------------------------------------------------------
        // Derived protocol projection.
        // ---------------------------------------------------------------------
        let qtm =
            Qtm::commit(
                &event_map.heritage().state,
                event_map.net_work(),
            );

        // ---------------------------------------------------------------------
        // RETURN THE AUTHENTIC HERITAGE
        //
        // EventMap is consumed only after all observers/projections have
        // finished using it.
        // ---------------------------------------------------------------------
        let heritage =
        event_map.heritage;


        QP44Event {
            heritage,
            qtm,
        }
    }

    pub fn next_receive(
        self,
    ) -> QP44Event {
        self.realize()
    }

    pub fn next_change(
        self,
    ) -> QP44Event {
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

            let coin = heritage.state.retained_mass();


            let manifold =
                heritage.state;
    
            let mass =
                TotalMass::from_memorized(
                    PURPOSE_44,
                    coin,
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


     //MODEL NETWORK
     //network object

     impl PQ44Event {
    
        #[inline(always)]
        pub fn into_object(self) -> PQ44Object {
            PQ44Object {
                manifold: self.heritage.state,
                coin: self.heritage.transition.net_work,
            }
        }
    
        pub fn balance(
            heritage: Heritage,
            qtm: &Qtm,
        ) -> PQ44Event {
            // -------------------------------------------------------------------------
            // VERIFY INCOMING WITNESS
            // -------------------------------------------------------------------------
        
            let committed =
                Qtm::commit(
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
        
            // -------------------------------------------------------------------------
            // CANONICAL SENDER EXIT
            //
            // Graph owns the exile boundary.
            // -------------------------------------------------------------------------
        
            let heritage =
                qtm_graph::graph::frame::Direction::exile(
                    heritage.state,
                );
        
            // -------------------------------------------------------------------------
            // NEW RECEIPT AFTER EXILE
            //
            // The exile operation itself produces the new Heritage.
            // -------------------------------------------------------------------------
        
            let qtm =
                Qtm::commit(
                    &heritage.state,
                    heritage.transition.net_work,
                );
        
            PQ44Event {
                heritage,
                qtm,
            }
        }


        pub fn balance_until(
            mut heritage: Heritage,
            qtm: &Qtm,
            activation: u64,
        ) -> PQ44Event {
            // -------------------------------------------------------------------------
            // VERIFY INITIAL WITNESS ONCE
            // -------------------------------------------------------------------------
        
            let committed =
                Qtm::commit(
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
        
            // -------------------------------------------------------------------------
            // REPEATED CANONICAL EXILE
            //
            // Every exile passes through the graph boundary.
            // -------------------------------------------------------------------------
        
            while heritage.state.activations() > activation {
                heritage =
                    qtm_graph::graph::frame::Direction::exile(
                        heritage.state,
                    );
            }
        
            // -------------------------------------------------------------------------
            // FINAL RECEIPT
            // -------------------------------------------------------------------------
        
            let qtm =
                Qtm::commit(
                    &heritage.state,
                    heritage.transition.net_work,
                );
        
            PQ44Event {
                heritage,
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