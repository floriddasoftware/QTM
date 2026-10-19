use std::{fs, u64};
use std::path::{Path, PathBuf};
use std::collections::BTreeMap;

use quantom_value::{
    Perm,
    QuantPerm,
    Heritage,
    TransitionHeritage,
    Retain,
};

use qtm_graph::graph::frame::Depth;

use crate::protocolvalue::Qtm;
use crate::qp44::{
    QP44Object,
    QP44Event,
    PQ44Event,
};

pub trait HeritageEvent {
    fn heritage(&self) -> &Heritage;
    fn qtm(&self) -> &Qtm;
    fn depth(&self) -> &Depth;
}

impl HeritageEvent for QP44Event {
    fn heritage(&self) -> &Heritage {
        &self.heritage
    }

    fn qtm(&self) -> &Qtm {
        &self.qtm
    }

    fn depth(&self) -> &Depth {
        &self.depth
    }
}

impl HeritageEvent for PQ44Event {
    fn heritage(&self) -> &Heritage {
        &self.heritage
    }

    fn qtm(&self) -> &Qtm {
        &self.qtm
    }

    fn depth(&self) -> &Depth {
        &self.depth
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
    let depth = event.depth();

    let path = substrate_root().join(substrate);

    fs::create_dir_all(&path)
        .expect("failed to create substrate heritage path");

    let write = |name: &str, value: String| {
        fs::write(path.join(name), value)
            .unwrap_or_else(|error| {
                panic!("failed to persist {}: {}", name, error)
            });
    };

    write(
        "manifold.dimension",
        heritage.state.dimension().to_string(),
    );
    write(
        "manifold.structural_value",
        heritage.state.structural_value().to_string(),
    );
    write(
        "manifold.activations",
        heritage.state.activations().to_string(),
    );
    write(
        "manifold.retained_mass",
        heritage.state.retained_mass().to_string(),
    );

    write(
        "transition.mirror",
        hex::encode(heritage.transition.mirror_bytes),
    );
    write(
        "transition.tau",
        heritage.transition.tau.to_string(),
    );
    write(
        "transition.delta",
        heritage.transition.delta.to_string(),
    );
    write(
        "transition.gross_work",
        heritage.transition.gross_work.to_string(),
    );
    write(
        "transition.net_work",
        heritage.transition.net_work.to_string(),
    );

    write(
        "qtm.commitment",
        hex::encode(qtm.commitment),
    );
    write(
        "qtm.coordinate",
        hex::encode(qtm.coordinate),
    );
    write(
        "qtm.network",
        qtm.net_work.to_string(),
    );
    write(
        "qtm.sigma",
        qtm.sigma.to_string(),
    );

    write("depth.path", depth.path().to_string());

    let mut entries = depth
        .objects()
        .map(|(position, identifier, retain)| {
            format!(
                "{}\t{}\t{}\t{}",
                position,
                hex::encode(identifier.as_bytes()),
                retain.mass,
                retain.from,
            )
        })
        .collect::<Vec<_>>();

    entries.sort_by_key(|entry| {
        entry
            .split('\t')
            .next()
            .and_then(|position| position.parse::<usize>().ok())
            .unwrap_or(usize::MAX)
    });

    write("depth.objects", entries.join("\n"));
}

pub fn load_depth(
    path: &Path,
) -> Depth {
    let depth_path = fs::read_to_string(path.join("depth.path"))
        .expect("missing depth.path");

    let mut depth = Depth::new(depth_path.trim_end_matches('\n').to_string());

    let contents = fs::read_to_string(path.join("depth.objects"))
        .expect("missing depth.objects");

    for (line_number, line) in contents.lines().enumerate() {
        let fields = line.split('\t').collect::<Vec<_>>();

        if fields.len() != 4 {
            panic!(
                "invalid depth.objects entry at line {}",
                line_number + 1,
            );
        }

        let position = fields[0]
            .parse::<usize>()
            .expect("invalid depth position");

        let identifier_bytes = hex::decode(fields[1])
            .expect("invalid depth identifier");

        let identifier = String::from_utf8(identifier_bytes)
            .expect("depth identifier is not UTF-8");

        let mass = fields[2]
            .parse::<u128>()
            .expect("invalid retained mass");

        let from = fields[3]
            .parse::<u64>()
            .expect("invalid retained source dimension");

        depth.insert(
            identifier,
            position,
            Retain { mass, from },
        );
    }

    depth
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
    
    fs::write(path.join("activation"), "0")
    .expect("failed to initialize substrate activation");

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

fn activation_path(substrate: &str) -> PathBuf {
    substrate_root()
        .join(substrate)
        .join("activation")
}

pub fn load_activation(substrate: &str) -> u64 {
    fs::read_to_string(activation_path(substrate))
        .expect("missing substrate activation")
        .trim()
        .parse::<u64>()
        .expect("invalid substrate activation")
}

fn persist_activation(substrate: &str, activation: u64) {
    let path = activation_path(substrate);
    let temporary = path.with_extension("tmp");

    fs::write(&temporary, activation.to_string())
        .expect("failed to write substrate activation");

    fs::rename(&temporary, &path)
        .expect("failed to commit substrate activation");
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventBudget {
    pub min_events: u64,
    pub max_events: u64,
    pub max_retained_mass: u128,
}

impl EventBudget {
    pub fn validate(&self) -> std::io::Result<()> {
        if self.max_events == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "max_events must be at least 1",
            ));
        }

        if self.min_events > self.max_events {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "min_events cannot exceed max_events",
            ));
        }

        if self.max_retained_mass == 0 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "max_retained_mass must be at least 1",
            ));
        }

        Ok(())
    }
}

/// Canonical persistent budget location:
/// <substrate_root>/<substrate>/event/activation/network/budget
pub fn budget_path(substrate: &str) -> PathBuf {
    substrate_root()
        .join(substrate)
        .join("event")
        .join("activation")
        .join("network")
        .join("budget")
}

/// Persist a budget for the specified local substrate.
///
/// The budget is stored at:
/// <substrate_root>/<substrate>/event/activation/network/budget
pub fn persist_budget(
    substrate: &str,
    min_events: u64,
    max_events: u64,
    max_retained_mass: u128,
) -> std::io::Result<()> {
    let budget = EventBudget {
        min_events,
        max_events,
        max_retained_mass,
    };

    budget.validate()?;

    let path = budget_path(substrate);
    let parent = path.parent().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "invalid budget path",
        )
    })?;

    fs::create_dir_all(parent)?;

    let temporary = parent.join(format!(
        ".budget.{}.tmp",
        std::process::id(),
    ));

    let contents = format!(
        "min_events={}\nmax_events={}\nmax_retained_mass={}\n",
        budget.min_events,
        budget.max_events,
        budget.max_retained_mass,
    );

    let result = (|| -> std::io::Result<()> {
        fs::write(&temporary, contents)?;
        fs::rename(&temporary, &path)?;
        Ok(())
    })();

    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }

    result
}

/// Shared read interface for every section of the crate.
pub fn load_budget(substrate: &str) -> std::io::Result<EventBudget> {
    let contents = fs::read_to_string(budget_path(substrate))?;
    let mut values = BTreeMap::<String, String>::new();

    for line in contents.lines() {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };

        values.insert(key.trim().to_owned(), value.trim().to_owned());
    }

    let parse_u64 = |key: &str| -> std::io::Result<u64> {
        values
            .get(key)
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("missing {key}"),
                )
            })?
            .parse::<u64>()
            .map_err(|_| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!("invalid {key}"),
                )
            })
    };

    let max_retained_mass = values
        .get("max_retained_mass")
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "missing max_retained_mass",
            )
        })?
        .parse::<u128>()
        .map_err(|_| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid max_retained_mass",
            )
        })?;

    let budget = EventBudget {
        min_events: parse_u64("min_events")?,
        max_events: parse_u64("max_events")?,
        max_retained_mass,
    };

    budget.validate()?;
    Ok(budget)
}

pub fn load_qtm(
    path: &PathBuf,
) -> Qtm {
    let read = |name: &str| {
        fs::read_to_string(path.join(name))
            .unwrap_or_else(|error| {
                panic!("failed to read {}: {}", name, error)
            })
    };

    let commitment_bytes = hex::decode(read("qtm.commitment").trim())
        .expect("invalid qtm.commitment");

    let coordinate_bytes = hex::decode(read("qtm.coordinate").trim())
        .expect("invalid qtm.coordinate");

    let commitment: [u8; 32] = commitment_bytes
        .try_into()
        .unwrap_or_else(|_| panic!("qtm.commitment must be exactly 32 bytes"));

    let coordinate: [u8; 32] = coordinate_bytes
        .try_into()
        .unwrap_or_else(|_| panic!("qtm.coordinate must be exactly 32 bytes"));

    let net_work = read("qtm.network")
        .trim()
        .parse::<u128>()
        .expect("invalid qtm.network");

    let sigma = read("qtm.sigma")
        .trim()
        .parse::<u128>()
        .expect("invalid qtm.sigma");

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

pub fn qtm_open_manifold(substrate: &str) -> Heritage {
    qtm_open_manifold_until(
        substrate,
        load_activation(substrate),
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
    let heritage = qtm_open_manifold(substrate);
    let activation = heritage.state.activations();

    let transit_path = event_path_at_activation(
        substrate,
        activation,
    )
    .unwrap_or_else(|| {
        panic!(
            "no persisted transit event for activation {}",
            activation,
        )
    });

    let qtm = load_qtm(&transit_path);
    let depth = load_depth(&transit_path);

    let event = PQ44Event::balance(
        heritage,
        &qtm,
        depth,
    );

    let resulting_activation = event.heritage.state.activations();
    let dimension = event.heritage.state.dimension();

    let exile_path = substrate_root()
        .join(substrate)
        .join("events")
        .join(format!(
            "exile_{}_{}",
            resulting_activation,
            dimension,
        ));

    persist_heritage(
        &exile_path,
        &event,
    );

    println!(
        "EXILING {}",
        transit_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy(),
    );

    fs::remove_dir_all(&transit_path)
        .expect("failed to remove transit event");

    println!(
        "ACTIVATIONS AFTER EXILE = {}",
        qtm_open_manifold(substrate)
            .state
            .activations(),
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
    let path = event_path_at_activation(
        substrate,
        activation,
    )
    .unwrap_or_else(|| {
        panic!("event not found at activation {}", activation)
    });

    let name = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let heritage = qtm_open_manifold_until(
        substrate,
        activation,
    );

    let qtm = load_qtm(&path);
    let depth = load_depth(&path);

    if name.starts_with("event_") {
        PersistedEvent::Transit(QP44Event {
            heritage,
            qtm,
            depth,
        })
    } else if name.starts_with("exile_") {
        PersistedEvent::Exile(PQ44Event {
            heritage,
            qtm,
            depth,
        })
    } else {
        panic!(
            "unknown persisted event namespace: {}",
            name,
        );
    }
}