// src/protocolvalue.rs

use quantom_value::{QuantPerm, DimensionObservation, Dimension, Heritage};
use crate::protocol_id::QuantumId;
use crate::qp44::{TotalMass, Flow, QP44Event, QP44Object};
use blake3;



#[derive(Debug)]
pub struct Value {
    pub dimension: Dimension,
    pub structural_value: u128,
    pub activations: u64,
    pub density: Option<u128>,
}

#[repr(C)]
#[derive(Debug)]
pub struct Qtm {
    pub commitment: [u8; 32],
    pub coordinate: [u8; 32],
    pub net_work: u128,
    pub sigma: u128,
}

#[repr(C)]
#[derive(Debug)]
pub struct Eco {
    pub value: Value,
    pub net_work: u128,
    pub gross_work: u128,
}

impl Qtm {
    pub fn economy(
        heritage: &Heritage,
    ) -> Qtm {
    
        let protocol = QuantumId::new();
    
        let gross_work = heritage.transition.gross_work;
        let net_work = heritage.transition.net_work;
    
        // 🔹 Build observation directly from live manifold
        let obs = DimensionObservation {
            dimension: heritage.state.dimension(),
            structural_value: heritage.state.structural_value(),
            activations: heritage.state.activations(),
            seed: heritage.transition.origin,
        };
    
        // 🔹 Inert economic value projection
        let value = Self::from_observation(&obs, &protocol);
    
        // 🔹 Economic layer
        let _eco = Eco {
            value,
            net_work,
            gross_work,
        };
    
        // 🔹 Commit directly from live manifold geometry
        Qtm::commit(
            &heritage.state,
            net_work,
        )
    }

           

            

    /// Build a ValueSlice from an observation
    pub fn from_observation(
        obs: &DimensionObservation,
        protocol: &QuantumId,
    ) -> Value {
        let density = protocol.density(obs);

        Value {
            dimension: obs.dimension,
            structural_value: obs.structural_value,
            activations: obs.activations,
            density,
        }
    }


    // ─────────────────────────────────────────────
    // 🔹 Coordinate (delegated from QP44)
    // ─────────────────────────────────────────────
    pub fn derive_coordinate(manifold: &QuantPerm) -> [u8; 32] {
        let mut h = blake3::Hasher::new();
        h.update(b"QP44::COORD::V1");
        h.update(&manifold.dimension().to_le_bytes());
        h.update(&manifold.structural_value().to_le_bytes());
    
        let hash = h.finalize();
        let mut out = [0u8; 32];
        out.copy_from_slice(hash.as_bytes());
        out
    }
    
    
    

        pub fn commit(
            manifold: &QuantPerm,
            net_work: u128,
        ) -> Self {
            use blake3;
    
            // 🔹 Deterministic coordinate from manifold geometry
            let coordinate =
                crate::protocolvalue::Qtm::derive_coordinate(manifold);
    
            // 🔹 Σ after transition (structural state)
            let sigma = manifold.structural_value();
    
            // 🔹 Pure commitment binding
            let mut h = blake3::Hasher::new();
    
            h.update(b"QTM::V1");
    
            // 🔐 Identity layer (geometry only)
            h.update(&coordinate);
    
            // 🔐 Economic state layer
            h.update(&sigma.to_le_bytes());
            h.update(&net_work.to_le_bytes());
    
            let hash = h.finalize();
    
            let mut commitment = [0u8; 32];
            commitment.copy_from_slice(hash.as_bytes());
    
            Qtm {
                commitment,
                coordinate,
                net_work,
                sigma,
            }
        }
}

impl Value {
    pub fn heritage_value(
        heritage: &Heritage,
    ) -> Self {
        let protocol = QuantumId::new();

        let obs = DimensionObservation {
            dimension: heritage.state.dimension(),
            structural_value: heritage.state.structural_value(),
            activations: heritage.state.activations(),
            seed: heritage.transition.origin,
        };

        Qtm::from_observation(
            &obs,
            &protocol,
        )
    }
}

//MODEL NETWORK
//network object

pub struct PQNetEvent {
    pub heritage: Heritage,
    pub qtm: Qtm, //<-invirant

}

impl PQNetEvent {
    pub fn memorize(self, flow: Flow) -> QP44Event {

        let heritage =
        self.heritage;

    

    let (account, change) =
        flow.stream(&heritage);

    let manifold =
        heritage.state;

    let purpose = 1;

    let mass =
        TotalMass::from_memorized(
            purpose,
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



    pub fn next_change(self) -> QP44Event {
        self.memorize(Flow::Change) 
    }
}

pub struct PQNetObject {
    heritage: Heritage,
    qtm: Qtm,
}

impl PQNetObject {

    pub fn balance(
        heritage: Heritage,
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
            heritage,
            qtm,
        }
    }

    pub fn observe(self) -> PQNetEvent {
        PQNetEvent {
            heritage: self.heritage,
            qtm: self.qtm,
        }
    }
}