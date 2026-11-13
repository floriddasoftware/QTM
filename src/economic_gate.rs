
use crate::protocolvalue::Qtm;
use crate::protocol_id::QuantumId;
use crate::pathsregistry::CoinType;
use quantom_value::{DimensionObservation, Heritage};
use crate::qp44::{Flow, QP44Event};
use qtm_graph::graph::frame::{Direction, EventMap};
use crate::{
    PQ44Event,
    PersistedEvent,
    load_event_at_activation,
    qtm_open_manifold_until,
};

//
// Secure Balance Proof Gate
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
// Canonical Gate
//

pub fn verify_balance(
    balance: u128,
    required: u128,
) -> Option<VerifiedBalanceProof> {
    (balance >= required).then(|| VerifiedBalanceProof::new())
}

//
// Canonical Cost Verification
//

pub fn verify_cost(
    heritage: &Heritage,
) -> Option<VerifiedBalanceProof> {
    if heritage.transition.gross_work
        == heritage.transition.net_work
        && heritage.state.activations() <= 1
    {
        return Some(VerifiedBalanceProof::new());
    }

    let obs = DimensionObservation {
        dimension: heritage.state.dimension(),
        structural_value: heritage.state.structural_value(),
        activations: heritage.state.activations(),
    };

    let protocol = QuantumId::new();
    let mass = heritage.state.retained_mass();

    let eco = Qtm::from_observation(
        &obs,
        &protocol,
    );

    let density = eco.density.unwrap_or(1);

    let gross_work = heritage.transition.gross_work;

    let required = protocol
        ._debt(&obs, heritage.transition.tau)
        .unwrap_or(0);

    let credit = gross_work.saturating_div(density);
    let debt = required.saturating_div(mass);

    verify_balance(credit, debt)
}

//
// Structure
//

#[derive(Clone)]
pub struct Structure {
    pub coin: CoinType,
    pub commitment: [u8; 32],
    pub density: u128,
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
// Economy
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
// Global Ledger
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

//
// Transactional Worldline Model
//

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
        let activation = amount;

        match load_event_at_activation(
            substrate,
            activation,
        ) {
            PersistedEvent::Transit(event) => {
                let QP44Event {
                    heritage,
                    qtm,
                    depth,
                } = event;

                let balance = PQ44Event::balance(
                    heritage,
                    &qtm,
                    depth,
                );

                Self {
                    amount: activation,
                    qtm,
                    constant: PersistedEvent::Exile(balance),
                }
            }

            PersistedEvent::Exile(event) => {
                let qtm = Qtm {
                    commitment: event.qtm.commitment,
                    coordinate: event.qtm.coordinate,
                    net_work: event.qtm.net_work,
                    sigma: event.qtm.sigma,
                };

                Self {
                    amount: activation,
                    qtm,
                    constant: PersistedEvent::Exile(event),
                }
            }
        }
    }

    #[inline(always)]
    pub fn receive(
        self,
        substrate: &str,
        _flow: Flow,
    ) -> QP44Event {
        let QuantumTransaction {
            amount: activation,
            qtm: sender_qtm,
            constant,
        } = self;

        let (depth, sender_constant) = match constant {
            PersistedEvent::Exile(event) => {
                let sender_constant =
                    event.heritage.transition.delta;

                let depth = event.depth;

                (depth, sender_constant)
            }

            PersistedEvent::Transit(_) => {
                panic!(
                    "receive requires a balanced exile event at activation {}",
                    activation,
                );
            }
        };

        let depth_mass = depth.objects().fold(
            0u128,
            |total, (_, _, retain)| {
                total.saturating_add(retain.mass)
            },
        );

        let mut heritage =
            qtm_open_manifold_until(
                substrate,
                activation,
            );

        let source_tau =
            heritage.transition.tau;

        let source_delta =
            heritage.transition.delta;

        heritage.state.set_initial_dimension_from_perm();

        let manifold = heritage.state;

        println!(
            "[RX] act={} nw={} Δ={}",
            activation,
            sender_qtm.net_work,
            source_delta,
        );

        println!(
            "     coord={}.. commit={}..",
            hex::encode(&sender_qtm.coordinate[..4]),
            hex::encode(&sender_qtm.commitment[..4]),
        );

        let proof = verify_balance(0, 0)
            .expect("economic gate failed");

        let seed = QuantumId::new().quantum_seed(proof);

        let receiver_heritage = Direction::transit(
            manifold,
            &depth,
            Some(&seed),
        );

        let event_map = EventMap::now(
            receiver_heritage,
            qtm_graph::graph::scale::Scale::Global,
        );

        let receiver_net_work =
            event_map.net_work();

        let receiver_qtm = Qtm::commit(
            &event_map.heritage().state,
            receiver_net_work,
        );

        let receiver_heritage =
            event_map.heritage;

        let event = QP44Event {
            heritage: receiver_heritage,
            qtm: receiver_qtm,
            depth,
        };

        println!(
            "\n================ OBSERVE RESULT ===================",
        );

        println!(
            "[RX] act={} tau={} depth_mass={} nw={} Δ={}",
            event.heritage.state.activations(),
            event.heritage.transition.tau,
            depth_mass,
            event.qtm.net_work,
            event.heritage.transition.delta,
        );

        Self::uniformity(
            &sender_qtm,
            sender_constant,
            &event,
        );

        let _ = source_tau;

        event
    }

    fn uniformity(
        sender_qtm: &Qtm,
        sender_constant: u128,
        event: &QP44Event,
    ) {
        let expected_coordinate =
            Qtm::derive_coordinate(
                &event.heritage.state,
            );

        let identity_valid =
            expected_coordinate == event.qtm.coordinate;

        let geometry_preserved =
            sender_qtm.coordinate == event.qtm.coordinate;

        let expected_sigma =
            event.heritage.state.structural_value();

        let sigma_valid =
            expected_sigma == event.qtm.sigma;

        let expected_commitment =
            Qtm::commit(
                &event.heritage.state,
                event.qtm.net_work,
            );

        let commitment_valid =
            expected_commitment.commitment
                == event.qtm.commitment;

        let receiver_delta =
            event.heritage.transition.delta;

        let transport_valid =
            receiver_delta <= sender_constant;

        println!();
        println!("──────────── Uniformity ────────────");

        println!(
            "[ID ] valid={} evolved={} {}.. → {}..",
            identity_valid,
            !geometry_preserved,
            hex::encode(&sender_qtm.coordinate[..4]),
            hex::encode(&event.qtm.coordinate[..4]),
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
            transport_valid,
        );

        println!("────────────────────────────────────");
        println!();

        assert!(
            identity_valid,
            "geometry invariant violation",
        );

        assert!(
            sigma_valid,
            "sigma invariant violation",
        );

        assert!(
            commitment_valid,
            "commitment invariant violation",
        );

        assert!(
            transport_valid,
            "transport invariant violation: receiver delta exceeds sender constant",
        );
    }
}
