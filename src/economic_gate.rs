use crate::protocolvalue::{Qtm};
use crate::qp44::{TotalMass, Flow, QP44Event, QP44Object};
use crate::protocol_id::QuantumId;
use crate::pathsregistry::{CoinType};
use quantom_value::{DimensionObservation, Heritage};
use crate::{
 PQ44Event, PersistedEvent, load_qtm_at_activation, qtm_open_manifold_until

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
    };

    // 2. Protocol (stateless law)
    let protocol = QuantumId::new();
    // 🔥 Use NETWORK (tau) as instantaneous field
    let mass = heritage.state.retained_mass();
    
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
        gross_work.saturating_div(density);

    let debt =
        required.saturating_div(mass);




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
    pub qtm: Qtm,
    pub constant: PersistedEvent,
}


impl QuantumTransaction {
    pub fn send(
        substrate: &str,
        amount: u64,
    ) -> Self {

        let activation =
            amount;

        let qtm =
            load_qtm_at_activation(
                substrate,
                activation,
            );

        let heritage =
            qtm_open_manifold_until(
                substrate,
                activation,
            );

        let balance =
            PQ44Event::balance(
                heritage,
                &qtm,
            );

        QuantumTransaction {
            amount: activation,
            constant:
                PersistedEvent::Exile(
                    balance,
                ),
            qtm
        }
    }


    #[inline(always)]
    pub fn receive(
        &self,
        substrate: &str,
        flow: Flow,
    ) -> QP44Event {
    
        // -------------------------------------------------------------------------
        // The transaction itself is the recovered manifold.
        // `amount` is the activation boundary established by `send()`.
        // -------------------------------------------------------------------------
    
        let activation =self.amount;
    
        let net_work = self.qtm.net_work;
    
        // -------------------------------------------------------------------------
        // Recover historical manifold state up to activation boundary.
        // -------------------------------------------------------------------------
    
        let mut heritage =
            qtm_open_manifold_until(
                substrate,
                activation,
            );
    
        let prev_mass =
            heritage.state.retained_mass();
    
        let amount =
            heritage.state.activations();
    
        // -------------------------------------------------------------------------
        // Re-anchor recovered state to native PERM geometry before
        // constructing the receive topology.
        // -------------------------------------------------------------------------
    
        heritage.state
        .set_initial_dimension_from_perm();
    
        // -------------------------------------------------------------------------
        // Recover the historical QTM witness.
        // -------------------------------------------------------------------------
    
        let (account, change) =
            flow.stream(
                &heritage,
            );
    
        let manifold =
            heritage.state;
    
        let mass =
            TotalMass::from_memorized(
                0,
                prev_mass,
                account,
                change,
                amount as u128,
            )
            .memorize();
    
        let object =
            QP44Object {
                manifold,
                coin: mass,
            }
            .realize();
    
        println!(
            "[RX] act={} nw={} Δ={}",
            activation,
            net_work,
            heritage.transition.delta,
        );
    
        println!(
            "     coord={}.. commit={}..",
            hex::encode(
                &self.qtm.coordinate[..4],
            ),
            hex::encode(
                &self.qtm.commitment[..4],
            ),
        );
    
        let event =
            object;
    
        println!(
            "\n================ OBSERVE RESULT ===================",
        );
    
        println!(
            "[RX] act={} tau={} mass={} nw={} Δ={}",
            activation,
            heritage.transition.tau,
            mass,
            self.qtm.net_work,
            heritage.transition.delta,
        );
    
        // -------------------------------------------------------------------------
        // Uniformity gate authorizes the receive topology.
        // -------------------------------------------------------------------------
    
        Self::uniformity(
            &self,
            &event,
        );
    
        event
    }
    
        // The gate currently proves neither is preserved.
// It only proves both were derived correctly.

fn uniformity(
    tx: &Self,
    event: &QP44Event,
) {
    use crate::protocolvalue::Qtm;

    // ─────────────────────────────────────
    // [ID] Geometry layer
    // ─────────────────────────────────────

    let expected_coordinate =
        Qtm::derive_coordinate(
            &event.heritage.state,
        );

    let identity_valid =
        expected_coordinate
            ==
        event.qtm.coordinate;

    let geometry_preserved =
        tx.qtm.coordinate
            ==
        event.qtm.coordinate;

    // ─────────────────────────────────────
    // [ECON] Economic layer
    // ─────────────────────────────────────

    let expected_sigma =
        event
            .heritage
            .state
            .structural_value();

    let sigma_valid =
        expected_sigma
            ==
        event.qtm.sigma;

    let expected_commitment =
        Qtm::commit(
            &event.heritage.state,
            event.qtm.net_work,
        );

    let commitment_valid =
        expected_commitment.commitment
            ==
        event.qtm.commitment;

    // ─────────────────────────────────────
    // [TX] Transport layer
    // ─────────────────────────────────────
    //
    // `constant` is the persisted event itself.
    // Derive the canonical sender Δ from that witness.
    //

    let sender_constant =
    match &tx.constant {
        PersistedEvent::Transit(event) =>
            event
                .heritage
                .transition
                .delta,

        PersistedEvent::Exile(event) =>
            event
                .heritage
                .transition
                .delta,
    };

    let receiver_delta =
        event
            .heritage
            .transition
            .delta;

    println!(
        "[TX ] constant={} delta={}",
        sender_constant,
        receiver_delta,
    );

    // ─────────────────────────────────────
    // Reporting
    // ─────────────────────────────────────

    println!();
    println!("──────────── Uniformity ────────────");
    
    println!(
        "[ID ] valid={} evolved={} {}.. → {}..",
        identity_valid,
        !geometry_preserved,
        hex::encode(
            &tx.qtm.coordinate[..4]
        ),
        hex::encode(
            &event.qtm.coordinate[..4]
        ),
    );

    println!(
        "[ECON] sigma={} commit={}",
        sigma_valid,
        commitment_valid,
    );

    println!(
        "[TX ] sender_delta={} receiver_delta={}",
        sender_constant,
        receiver_delta,
    );

    println!(
        "[TX ] transport_valid={}",
        receiver_delta <= sender_constant,
    );

    println!("────────────────────────────────────");
    println!();

    // Geometry must always be valid

    assert!(
        identity_valid,
        "geometry invariant violation",
    );

    // Economic state must always be valid

    assert!(
        sigma_valid,
        "sigma invariant violation",
    );

    // Transport commitment must always be valid

    assert!(
        commitment_valid,
        "commitment invariant violation",
    );

    // Receiver may not exceed the sender's persisted
    // manifold resistance.

   }
}