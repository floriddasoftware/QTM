use std::{fs, u64};
use std::path::{Path, PathBuf};

use quantom_value::{Perm, QuantPerm, Heritage, TransitionHeritage};
use crate::protocolvalue::Qtm;
use crate::qp44::{
    QP44Object,  
    QP44Event, 
    PQ44Event 
};
pub trait HeritageEvent {
    fn heritage(&self) -> &Heritage;
    fn qtm(&self) -> &Qtm;
}

impl HeritageEvent for QP44Event {
    fn heritage(&self) -> &Heritage {
        &self.heritage
    }

    fn qtm(&self) -> &Qtm {
        &self.qtm
    }
}

impl HeritageEvent for PQ44Event {
    fn heritage(&self) -> &Heritage {
        &self.heritage
    }

    fn qtm(&self) -> &Qtm {
        &self.qtm
    }
}

pub fn persist_heritage<E>(
    substrate: &Path,
    event: &E,
)
where
    E: HeritageEvent,
{
    let heritage = event.heritage();
    let qtm = event.qtm();

    let path = substrate_root()
        .join(substrate);

    fs::create_dir_all(&path)
        .expect("failed to create substrate heritage path");

    fs::write(
        path.join("manifold.dimension"),
        heritage.state.dimension().to_string(),
    ).ok();

    fs::write(
        path.join("manifold.structural_value"),
        heritage.state.structural_value().to_string(),
    ).ok();

    fs::write(
        path.join("manifold.activations"),
        heritage.state.activations().to_string(),
    ).ok();

    fs::write(
        path.join("manifold.retained_mass"),
        heritage.state.retained_mass().to_string(),
    ).ok();

    fs::write(
        path.join("transition.mirror"),
        hex::encode(heritage.transition.mirror_bytes),
    ).ok();

    fs::write(
        path.join("transition.tau"),
        heritage.transition.tau.to_string(),
    ).ok();

    fs::write(
        path.join("transition.delta"),
        heritage.transition.delta.to_string(),
    ).ok();

    fs::write(
        path.join("transition.gross_work"),
        heritage.transition.gross_work.to_string(),
    ).ok();

    fs::write(
        path.join("transition.net_work"),
        heritage.transition.net_work.to_string(),
    ).ok();

    fs::write(
        path.join("qtm.commitment"),
        hex::encode(qtm.commitment),
    ).ok();

    fs::write(
        path.join("qtm.coordinate"),
        hex::encode(qtm.coordinate),
    ).ok();

    fs::write(
        path.join("qtm.network"),
        qtm.net_work.to_string(),
    ).ok();

    fs::write(
        path.join("qtm.sigma"),
        qtm.sigma.to_string(),
    ).ok();
}

pub fn substrate_root() -> PathBuf {
    let home = std::env::var("HOME")
        .expect("HOME not set");

    PathBuf::from(home)
        .join(".qp")
        .join("substrates")
}

pub fn qtm_create(
    name: String,
    indices: Option<String>,
    entropy: Option<String>,
) {

    // --------------------------------------------------
    // 🔹 Root Namespace
    // --------------------------------------------------

    let root =
        substrate_root();

    let path =
        root.join(&name);

    // --------------------------------------------------
    // 🔹 Existing Substrate Check
    // --------------------------------------------------

    if path.exists() {

        println!(
            "⚠️ substrate already exists: {}",
            name
        );

        return;
    }

    // --------------------------------------------------
    // 🔹 Create Namespace
    // --------------------------------------------------

    fs::create_dir_all(&path)
        .expect("failed to create substrate");

    fs::create_dir_all(path.join("events"))
        .expect("failed to create substrate events path");

    // --------------------------------------------------
    // 🔹 Domain Indices
    // --------------------------------------------------

    let domain_indices: [u16; Perm::NUM_INDICES] =
        match indices {

            Some(raw) => {

                let parsed: Vec<u16> =
                    raw.split(',')
                        .map(|v| {
                            v.trim()
                                .parse::<u16>()
                                .expect("invalid domain index")
                        })
                        .collect();

                if parsed.len() != Perm::NUM_INDICES {

                    panic!(
                        "expected {} indices",
                        Perm::NUM_INDICES
                    );
                }

                parsed
                    .try_into()
                    .expect("failed to convert indices")
            }

            None => [0u16; Perm::NUM_INDICES],
        };

    // --------------------------------------------------
    // 🔹 Entropy Surface
    // --------------------------------------------------

    let entropy_surface =
        entropy.unwrap_or_else(|| {
            name.clone()
        });

    // --------------------------------------------------
    // 🔹 PERM Genesis
    // --------------------------------------------------

    let perm =
        Perm::genesis_construct(
            &domain_indices,
            entropy_surface.as_bytes(),
        )
        .expect("failed to construct PERM");

    // --------------------------------------------------
    // 🔹 QuantPerm Manifold
    // --------------------------------------------------

    let manifold =
        QuantPerm::new(
            perm
        );

    // --------------------------------------------------
    // 🔹 Initial Commitment
    // --------------------------------------------------

    let qtm =
        Qtm::commit(
            &manifold,
            0,
        );

    // --------------------------------------------------
    // 🔹 Persist Surfaces
    // --------------------------------------------------

    fs::write(
        path.join("perm.dimension"),
        perm.dimension().to_string(),
    )
    .expect("failed to write perm");

    fs::write(
        path.join("perm.indices"),
        domain_indices
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(","),
    )
    .expect("failed to write indices");

    fs::write(
        path.join("perm.entropy"),
        entropy_surface.as_bytes(),
    )
    .expect("failed to write entropy");

    fs::write(
        path.join("qtm.commitment"),
        hex::encode(
            qtm.commitment
        ),
    )
    .expect("failed to write commitment");

    fs::write(
        path.join("qtm.coordinate"),
        hex::encode(
            qtm.coordinate
        ),
    )
    .expect("failed to write coordinate");

    fs::write(
        path.join("qtm.network"),
        qtm.net_work.to_string(),
    )
    .expect("failed to write network");

    fs::write(
        path.join("qtm.sigma"),
        qtm.sigma.to_string(),
    )
    .expect("failed to write sigma");

    // --------------------------------------------------
    // 🔹 Observer Output
    // --------------------------------------------------

    println!();
    println!("✅ substrate created");
    println!();

    println!("🌌 namespace:");
    println!("{}", path.display());

    println!();

    println!("🔹 indices:");
    println!(
        "{}",
        domain_indices
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );

    println!();

    println!("🔹 entropy:");
    println!("{}", entropy_surface);

    println!();

    println!("🔹 PERM dimension:");
    println!("{}", perm.dimension());

    println!();

    println!("🔹 commitment:");
    println!("qp{}", hex::encode(qtm.commitment));

    println!();

    println!("🔹 coordinate:");
    println!("0x{}", hex::encode(qtm.coordinate));

    println!();

    println!("🔹 sigma:");
    println!("{}", qtm.sigma);

    println!();

    println!("🔹 network:");
    println!("{}", qtm.net_work);

    println!();
}

pub fn load_qtm(
    path: &PathBuf,
) -> Qtm {

    let commitment_hex =
        fs::read_to_string(
            path.join("qtm.commitment")
        )
        .expect("missing qtm.commitment");

    let coordinate_hex =
        fs::read_to_string(
            path.join("qtm.coordinate")
        )
        .expect("missing qtm.coordinate");

    let net_work =
        fs::read_to_string(
            path.join("qtm.network")
        )
        .expect("missing qtm.network")
        .trim()
        .parse::<u128>()
        .expect("invalid qtm.network");

    let sigma =
        fs::read_to_string(
            path.join("qtm.sigma")
        )
        .expect("missing qtm.sigma")
        .trim()
        .parse::<u128>()
        .expect("invalid qtm.sigma");

    let mut commitment =
        [0u8; 32];

    commitment.copy_from_slice(
        &hex::decode(
            commitment_hex.trim()
        )
        .expect("invalid qtm.commitment")
    );

    let mut coordinate =
        [0u8; 32];

    coordinate.copy_from_slice(
        &hex::decode(
            coordinate_hex.trim()
        )
        .expect("invalid qtm.coordinate")
    );

    Qtm {
        commitment,
        coordinate,
        net_work,
        sigma,
    }
}



pub fn load_qtm_at_activation(
    substrate: &str,
    activation:u64,
) -> Qtm {

    let heritage =
        qtm_open_manifold_until(
            substrate,
            activation,
        );

    Qtm::economy(&heritage)
}

pub fn load_latest_qtm(
    substrate: &str,
) -> Qtm {

    let heritage =
        qtm_open_manifold(
            substrate
        );

    Qtm::economy(
        &heritage
    )
}


pub fn qtm_open(
    name: String,
) {

    // --------------------------------------------------
    // 🔹 Namespace Resolution
    // --------------------------------------------------

    let root =
        substrate_root();

    let path =
        root.join(&name);

    // --------------------------------------------------
    // 🔹 Existence Validation
    // --------------------------------------------------

    if !path.exists() {

        println!(
            "❌ substrate does not exist: {}",
            name
        );

        return;
    }

    // --------------------------------------------------
    // 🔹 Recover Surfaces
    // --------------------------------------------------

    let dimension =
        fs::read_to_string(
            path.join("perm.dimension")
        )
        .expect("failed to read dimension");

    let indices =
        fs::read_to_string(
            path.join("perm.indices")
        )
        .expect("failed to read indices");

    let entropy =
        fs::read_to_string(
            path.join("perm.entropy")
        )
        .expect("failed to read entropy");

    let commitment =
        fs::read_to_string(
            path.join("qtm.commitment")
        )
        .expect("failed to read commitment");

    let coordinate =
        fs::read_to_string(
            path.join("qtm.coordinate")
        )
        .expect("failed to read coordinate");

    let sigma =
        fs::read_to_string(
            path.join("qtm.sigma")
        )
        .expect("failed to read sigma");

    let network =
        fs::read_to_string(
            path.join("qtm.network")
        )
        .expect("failed to read network");

    // --------------------------------------------------
    // 🔹 Observer Surface
    // --------------------------------------------------

    println!();

    println!("🌌 substrate opened");
    println!();

    println!("🔹 namespace:");
    println!("{}", path.display());

    println!();

    println!("🔹 indices:");
    println!("{}", indices.trim());

    println!();

    println!("🔹 entropy:");
    println!("{}", entropy.trim());

    println!();

    println!("🔹 PERM dimension:");
    println!("{}", dimension.trim());

    println!();

    println!("🔹 commitment:");
    println!("qp{}", commitment.trim());

    println!();

    println!("🔹 coordinate:");
    println!("0x{}", coordinate.trim());

    println!();

    println!("🔹 sigma:");
    println!("{}", sigma.trim());

    println!();

    println!("🔹 network:");
    println!("{}", network.trim());

    println!();
}

pub fn qtm_open_manifold(
    substrate: &str,
) -> Heritage {

    qtm_open_manifold_until(
        substrate,
        u64::MAX,
    )
}




pub fn qtm_open_manifold_until(
    substrate: &str,
    stop_activation: u64,
) -> Heritage {

    let root =
        substrate_root()
            .join(substrate);

    //
    // Reconstruct canonical PERM.
    //
    let indices_raw =
        fs::read_to_string(
            root.join("perm.indices")
        )
        .expect("missing perm.indices");

    let entropy =
        fs::read(
            root.join("perm.entropy")
        )
        .expect("missing perm.entropy");



    let indices:
        [u16; Perm::NUM_INDICES] =
        indices_raw
            .trim()
            .split(',')
            .map(
                |v|
                    v.parse::<u16>()
                        .expect("invalid index")
            )
            .collect::<Vec<_>>()
            .try_into()
            .expect("invalid index count");



    let perm =
        Perm::genesis_construct(
            &indices,
            &entropy,
        )
        .expect(
            "failed to reconstruct PERM"
        );



    let mut manifold =
        QuantPerm::new(
            perm
        );

    manifold
        .set_initial_dimension_from_perm();



    let mut heritage =
        Heritage {

            state:
                manifold,

            transition:
                TransitionHeritage {

                    tau: 0,

                    delta: 0,

                    gross_work: 0,

                    net_work: 0,

                    mirror_bytes: [0u8; 32],
                },
        };



    //
    // Pure manifold replay.
    //
    // Activation drives the replay,
    // not the filesystem.
    //
    while
        heritage
            .state
            .activations()
            < stop_activation
    {

        let retained_mass =
            heritage
                .state
                .retained_mass();



        let event =
            QP44Object::from_quantperm(
                heritage.state,
                retained_mass,
            )
            .next_receive();



        heritage =
            event.heritage;
    }



    heritage
}


pub fn qtm_close_manifold_until(
    mut heritage: Heritage,
    stop_activation: u64,
) -> Heritage {

    while heritage.state.activations() > stop_activation {

        heritage =
            heritage
                .state
                .exile();
    }

    heritage
}

pub fn qtm_close_heritage_until(
    mut heritage: Heritage,
    stop_activation: u64,
) -> Heritage {

    while heritage.state.activations() > stop_activation {

        heritage =
            heritage
                .state
                .exile();
    }

    heritage
}




     pub fn qtm_transit(
        substrate: &str,
        payload: u128,
        amount: u64,
    ) -> QP44Event {

        let activation = amount;

        
        let heritage = qtm_open_manifold_until(substrate, activation);


        let manifold = heritage.state;
    
        
    
            let object =
            QP44Object::from_quantperm(
                manifold,
                payload,
            );
    
        let new_event =
            object.next_receive();
    
        let activation =
            new_event
                .heritage
                .state
                .activations();
    
        let dimension =
            new_event
                .heritage
                .state
                .dimension();
    
        let path =
            substrate_root()
                .join(substrate)
                .join("events")
                .join(
                    format!(
                        "event_{}_{}",
                        activation,
                        dimension,
                    )
                );
    
        persist_heritage(
            &path,
            &new_event,
        );
    
        new_event
    }


    pub fn qtm_exile(
        substrate: &str,
    ) -> PQ44Event {
    
        //
        // Recover canonical manifold.
        //
        let heritage =
            qtm_open_manifold(
                substrate,
            );
    
        //
        // Recover canonical QTM.
        //
        let qtm =
            load_latest_qtm(
                substrate,
            );
    
        //
        // Materialize balance event.
        //
        let event =
            PQ44Event::balance(
                heritage,
                &qtm,
            );
    
        let activation =
            event
                .heritage
                .state
                .activations();
    
        let dimension =
            event
                .heritage
                .state
                .dimension();
    
        //
        // Resolve the transit event being exiled.
        //
        let transit_path =
            event_path_at_activation(
                substrate,
                activation.saturating_add(1),
            );
    
        //
        // Persist exile record.
        //
        let exile_path =
            substrate_root()
                .join(substrate)
                .join("events")
                .join(
                    format!(
                        "exile_{}_{}",
                        activation,
                        dimension,
                    )
                );
    
        persist_heritage(
            &exile_path,
            &event,
        );
    
        //
        // Remove the corresponding transit event.
        //
        if let Some(path) = transit_path {
    
            println!(
                "EXILING {}",
                path.file_name()
                    .unwrap()
                    .to_string_lossy()
            );
    
            fs::remove_dir_all(
                &path,
            )
            .expect(
                "failed to remove transit event",
            );
        }
    
        println!(
            "ACTIVATIONS AFTER EXILE = {}",
            qtm_open_manifold(
                substrate,
            )
            .state
            .activations()
        );
    
        event
    }


    fn activation_from_name(
        name: &str,
    ) -> Option<u64> {
    
        let rest =
            name
                .strip_prefix("event_")
                .or_else(|| name.strip_prefix("exile_"))?;
    
        rest.split('_')
            .next()?
            .parse::<u64>()
            .ok()
    }
    pub fn event_path_at_activation(
        substrate: &str,
        activation: u64,
    ) -> Option<PathBuf> {
    
        let events_root =
            substrate_root()
                .join(substrate)
                .join("events");
    
        fs::read_dir(events_root)
            .ok()?
            .filter_map(Result::ok)
            .find_map(|entry| {
    
                let name =
                    entry
                        .file_name()
                        .to_string_lossy()
                        .to_string();
    
                if !name.starts_with("event_") {
                    return None;
                }
    
                match activation_from_name(&name) {
                    Some(a) if a == activation => {
                        Some(entry.path())
                    }
    
                    _ => None,
                }
            })
    }
    

    pub enum PersistedEvent {
        Transit(QP44Event),
        Exile(PQ44Event),
    }
    
    pub fn load_event_at_activation(
        substrate: &str,
        activation: u64,
    ) -> PersistedEvent {
    
        let path =
            event_path_at_activation(
                substrate,
                activation,
            )
            .expect("event not found");
    
        let name =
            path.file_name()
                .unwrap()
                .to_string_lossy();
    
        let heritage =
            qtm_open_manifold_until(
                substrate,
                activation,
            );
    
        let qtm =
            load_qtm(&path);
    
        if name.starts_with("event_") {
    
            PersistedEvent::Transit(
                QP44Event {
                    heritage,
                    qtm,
                }
            )
    
        } else if name.starts_with("exile_") {
    
            PersistedEvent::Exile(
                PQ44Event {
                    heritage,
                    qtm,
                }
            )
    
        } else {
    
            panic!(
                "unknown persisted event namespace: {}",
                name,
            );
        }
    }
