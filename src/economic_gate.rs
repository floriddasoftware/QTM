use crate::protocolvalue::{Qtm, PQNetObject, PQNetEvent};
use crate::qp44::{TotalMass, Flow, QP44Event, QP44Object};
use crate::protocol_id::QuantumId;
use crate::pathsregistry::{CoinType};
use quantom_value::{DimensionObservation, Heritage};
use crate::{
 load_qtm_at_activation, qtm_open_manifold_until

};
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


    // 🛡️ MODEL 1 EXCEPTION
    //
    // Local manifold replay:
    // - no external debt
    // - no external creditor
    // - no settlement required
    //
    if heritage.transition.gross_work
        == heritage.transition.net_work
        && heritage.state.activations() <= 1
    {
        return Some(VerifiedBalanceProof::new());
    }

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
        Qtm::from_observation(
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



//This code is operating at a very specific layer

//It is not a ledger layer.

//It is also not a network layer.

//It is a projection layer (or semantic observation layer).
//Coordinate
//↓
//Commitment
// ↓
//Heritage
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


//Transactional Worldline Model implementation, with a small dependency on Model 1 (Physics)
//Everything here describes a transported state:

//amount      -> activation state
//net_work    -> carried structural mass
//commitment  -> event identity
//coordinate  -> worldline identity
//constant    -> traversed invariant
pub struct QuantumTransaction {
    pub amount: u64,
    pub net_work: u128,
    pub commitment: [u8; 32],
    pub coordinate: [u8; 32],
    pub constant: u128,
}


impl QuantumTransaction {

    pub fn send(
        substrate: &str,
        amount: u64,
    ) -> (PQNetEvent, Self) {
        let activation = amount;

        // 1. Recover historical manifold state up to activation boundary.
        let heritage = qtm_open_manifold_until(
            substrate,
            Some(activation),
        );

        // 2. Recover the historical QTM witness.
        let qtm = load_qtm_at_activation(
            substrate,
            activation,
        );

        // 3. Transform state through the pure balance invariant layer.
        // This enforces co-variant matching of coordinates and commitments.
        let balanced_object = PQNetObject::balance(
            heritage,
            qtm,
        );

        // 4. Collapse the balanced object into an observable PQNetEvent
        let event = balanced_object.observe();

        // -------------------------------------------------
        // Receipt construction
        // -------------------------------------------------
        let change = event.heritage.transition.delta;

        let tx = QuantumTransaction {
            amount,
            net_work: event.qtm.net_work,
            commitment: event.qtm.commitment,
            coordinate: event.qtm.coordinate,
            constant: change,
        };

        (event, tx)
    }


    

        /// Topology creation
        /// Network -> Value Receive
        pub fn receive(
            substrate: &str,
            tx: Self,
            flow: Flow,
        ) -> QP44Event {
    
            // 🌟 The activation threshold used to locate the historic entry surface
            let activation = tx.amount; 
    
            // 1. Recover historical manifold state up to activation boundary.
            let mut heritage =
                qtm_open_manifold_until(
                    substrate,
                    Some(activation),
                );


                println!("\n================ RECEIVE FORENSICS ================");
                println!("Activation Boundary      : {}", activation);
                
                println!("\n--- Recovered Heritage ---");
                println!("Dimension               : {}", heritage.state.dimension());
                println!("Activations             : {}", heritage.state.activations());
                println!("Retained Mass           : {}", heritage.state.retained_mass());
                println!("Structural Value        : {}", heritage.state.structural_value());
                
                  
    
            // 2. 🔥 THE CRITICAL INVARIANT FIXED HERE (Option 1)
            // Before creating any topology or calculating transition costs,
            // re-anchor the recovered state back to native geometric truth.
            heritage.state.set_initial_dimension_from_perm();
    
            // 2. Recover the historical QTM witness.
            let qtm =
                load_qtm_at_activation(
                    substrate,
                    activation,
                );

        
                

            let (account, change) =
                flow.stream(&heritage);

            let manifold =
                heritage.state;
    
            let mass =
                TotalMass::from_memorized(
                    0,
                    heritage.transition.net_work,
                    account,
                    change,
                    manifold.activations() as u128,
                )
                .memorize();
    
            let object = QP44Object {
                manifold,
                coin: mass,
            }
            .realize();
        
                println!("\n--- Historical Witness ---");
                println!("Coordinate              : {}", hex::encode(qtm.coordinate));
                println!("Commitment              : {}", hex::encode(qtm.commitment));
                println!("Net Work                : {}", qtm.net_work);
                
                println!("\n--- Transition Surface ---");
                println!("Delta                   : {}", heritage.transition.delta);
            
                println!("===================================================\n");
                          

                println!("\n================ RECEIVE FORENSICS ================");
println!("Activation Boundary      : {}", activation);



println!("\n--- Historical Witness ---");
println!("Coordinate              : {}", hex::encode(qtm.coordinate));
println!("Commitment              : {}", hex::encode(qtm.commitment));
println!("Net Work                : {}", qtm.net_work);

println!("\n--- Transition Surface ---");
println!("Delta                   : {}", heritage.transition.delta);

println!("===================================================\n");
            
    
            // 6. Produce the network event and commit the finalized progression layer
            let event = object;      // Yields QNetEvent (Processes raw matrix mass)
               
                println!("\n================ OBSERVE RESULT ===================");

                println!("Coordinate              : {}", hex::encode(event.qtm.coordinate));
                println!("Commitment              : {}", hex::encode(event.qtm.commitment));
                
                println!("Dimension               : {}", event.heritage.state.dimension());
                println!("Activations             : {}", event.heritage.state.activations());
                println!("Structural Value        : {}", event.heritage.state.structural_value());
                
                println!("Transition Delta        : {}", event.heritage.transition.delta);
                
                println!("===================================================\n");
            // 7. Authorize progression through the Uniformity Gate
            Self::uniformity(
                &tx,
                &event,
            );
    
            event
        }
    
        fn uniformity(
            tx: &Self,
            event: &QP44Event,
        ) {
            // -------------------------------------------------
            // TOPOLOGICAL CONTINUITY AUTHORIZATION
            // -------------------------------------------------
    
            // Receiver must evaluate against the legal complementary endpoint.
            // NOTE: If your design allows progression, change this to an asset progression check,
            // or ensure it matches the complementary trajectory verified by Option A.
            assert_eq!(
                tx.coordinate,
                event.qtm.coordinate,
                "uniformity violation: coordinate mismatch",
            );
    
            // Receiver may advance but never regress.
            assert!(
                event.heritage.transition.delta
                    >= tx.constant,
                "non-monotonic transition",
            );
        }
    }