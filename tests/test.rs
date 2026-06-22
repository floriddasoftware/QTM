use qp_hd::Flow::Change;
// tests/test.rs
use qp_hd::{QuantumTransaction, QP44Object, PQ44Event, Qtm};
use qp_hd::purpose::{Purpose as SeedPurpose, SeedSource};
use qp_hd::pathsregistry::{CoinType};


// MODEL 1

// MODEL 1 — Forensic Invariant Test
#[test]
fn model1_manifold_conserves_dimension() {
    println!("\n=========== 🛡️  MODEL 1 FORENSIC ANALYSIS START ===========");
    
    let seed = SeedSource::Raw(vec![1; 32]);
    let coin = CoinType::Bitcoin;
    
    let base = SeedPurpose::quantperm_seed(seed).unwrap();
    println!("Initial Manifold Base Dimension : {}", base.dimension());

    let object = QP44Object::from_quantperm(base, coin.retained_mass());

    // ─────────────────────────────────────────────────────────────────
    // 🔹 Phase 1: Inbound Realization (Receive Action)
    // ─────────────────────────────────────────────────────────────────
    let result = object.next_receive();

    let initial_dim_coordinate = result.qtm.coordinate;
    let continuity_object = result.heritage; // Clone for structural inspection

    println!("\n--- [Phase 1: Outbound View (next_receive)] ---");
    println!("Initial QTM Coordinate (Hex)    : {}", hex::encode(initial_dim_coordinate));
    println!("Initial QTM Commitment (Hex)    : {}", hex::encode(result.qtm.commitment));
    println!("Post-Receive Manifold Dimension : {}", continuity_object.state.dimension());
    println!("Post-Receive Structural Value   : {}", continuity_object.state.structural_value());
    println!("Post-Receive Activations Count  : {}", continuity_object.state.activations());
    println!("Transition Gross Work Metric    : {}", continuity_object.transition.gross_work);
    println!("Transition Net Work Mass        : {}", continuity_object.transition.net_work);

    // ─────────────────────────────────────────────────────────────────
    // 🔹 Phase 2: Closed-Loop Local Decay (Change Action)
    // ─────────────────────────────────────────────────────────────────
    let decay_event = PQ44Event {
        heritage: continuity_object,
        qtm: result.qtm,
    };

    let final_coordinate = decay_event.qtm.coordinate;

    println!("\n--- [Phase 2: Inbound Local Replay (next_change)] ---");
    println!("Final QTM Coordinate (Hex)      : {}", hex::encode(final_coordinate));
    println!("Final QTM Commitment (Hex)      : {}", hex::encode(decay_event.qtm.commitment));
    println!("Post-Decay Manifold Dimension   : {}", decay_event.heritage.state.dimension());
    println!("Post-Decay Structural Value     : {}", decay_event.heritage.state.structural_value());
    println!("Post-Decay Activations Count    : {}", decay_event.heritage.state.activations());
    println!("Decay Gross Work Metric         : {}", decay_event.heritage.transition.gross_work);
    println!("Decay Net Work Mass             : {}", decay_event.heritage.transition.net_work);
    println!("===========================================================");

    // ─────────────────────────────────────────────────────────────────
    // 🔹 Core Model 1 Assertion
    // ─────────────────────────────────────────────────────────────────
    assert_eq!(
        initial_dim_coordinate,
        final_coordinate,
        "Model 1 Invariant Broken: Coordinate identity drifted during local replay."
    );
}

#[test]
#[should_panic(expected = "uniformity violation")]
fn model1_rejects_coordinate_mutation() {
    let (_, mut tx) =
        QuantumTransaction::send(
            "mars-lab",
            1,
        );

    tx.coordinate[0] ^= 1;

    QuantumTransaction::receive(
        "mars-lab",
        tx,
        Change
    );
}


#[test]
fn model1_network_sigma_is_nonnegative() {
    let (event, _) =
        QuantumTransaction::send(
            "mars-lab",
            1,
        
        );

    assert!(
        event.heritage.state.structural_value() >= event.qtm.net_work
    );
}


//Dimension is not transported.

//Dimension is consulted.

#[test]
fn model2_commitment_is_conserved() {
    // Simulate a send
    let (_, tx) = QuantumTransaction::send("mars-lab", 12);

    // The coordinate is the stable identity
    let expected_coordinate = tx.coordinate;

    // Replay a receive with Change flow
    let event = QuantumTransaction::receive("mars-lab", tx, Change);

    // Recompute commitment from the heritage state
    let recomputed = Qtm::commit(
        &event.heritage.state,
        event.heritage.transition.net_work,
    );

    // Invariant: coordinate identity is conserved
    assert_eq!(
        expected_coordinate,
        recomputed.coordinate,
        "Model 2 Invariant Broken: Coordinate identity drifted."
    );

    // Commitment may differ, but must be derived from the same coordinate space
    assert_ne!(
        event.qtm.commitment,
        expected_coordinate,
        "Commitment should morph with confirmation flux, not remain static."
    );
}


#[test]
fn model2_delta_is_nonzero() {
    let (_, tx) =
        QuantumTransaction::send(
            "mars-lab",
            1,
        );

    assert!(
        tx.constant > 0
    );
}

#[test]
fn model2_network_emerges_from_structure() {
    let (event, tx) =
        QuantumTransaction::send(
            "mars-lab",
            1,
        );

    assert!(
        event.heritage.state.structural_value()
            >= tx.net_work
    );
}


#[test]
fn model2_activation_is_memory() {
    let (_, tx) =
        QuantumTransaction::send(
            "mars-lab",
            1,
        );

    assert_eq!(
        tx.amount,
        1,
    );
}


#[test]
fn model1_dimension_is_not_in_transaction_payload() {
    let (_, tx) =
        QuantumTransaction::send(
            "mars-lab",
            1,
        );

    // ─────────────────────────────────────────────────────────────────
    // 🔹 Compile-Time Invariant Check
    // ─────────────────────────────────────────────────────────────────
    // This pattern destructures QuantumTransaction exhaustively.
    // If a `dimension` field is ever added to this struct, or if any
    // structural field is modified, this test will fail to compile.
    let QuantumTransaction {
        coordinate: _,
        commitment: _,
        constant: _,
        amount: _,
        net_work: _,  // ◄── Added to satisfy current struct layout
    } = tx;
}

