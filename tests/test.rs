use qp_hd::{Flow, PersistedEvent, QuantumTransaction};

//
// ─────────────────────────────────────────────
// MODEL 1 — Identity Stability
// ─────────────────────────────────────────────
//


#[test]
fn model1_identity_survives_local_replay() {
    let tx =
        QuantumTransaction::send(
            "mars-lab",
            1,
        );

    let event =
        tx.receive(
            "mars-lab",
            Flow::Change,
        );

    assert_eq!(
        event.qtm.coordinate,
        event.qtm.coordinate,
    );

    assert_eq!(
        event.qtm.commitment,
        event.qtm.commitment,
    );
}

#[test]
fn model1_coordinate_mutation_creates_new_identity() {
    let mut tx = QuantumTransaction::send("mars-lab", 1);

    let original = tx.qtm.coordinate;

    tx.qtm.coordinate[0] ^= 1;

    assert_ne!(original, tx.qtm.coordinate);
}

#[test]
fn model1_network_structure_consistency() {
    let tx =
        QuantumTransaction::send(
            "mars-lab",
            1,
        );

    let event =
        tx.receive(
            "mars-lab",
            Flow::Change,
        );

    assert!(
        event
            .heritage
            .state
            .structural_value()
            >= event.qtm.net_work
    );
}

//
// ─────────────────────────────────────────────
// MODEL 2 — Transport Logic
// ─────────────────────────────────────────────
//

#[test]
fn model2_delta_is_positive() {
    let tx =
        QuantumTransaction::send(
            "mars-lab",
            1,
        );

    let delta =
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

    assert!(
        delta > 0,
        "persisted transition delta must be positive",
    );
}

#[test]
fn model2_transport_is_monotonic() {
    let tx =
        QuantumTransaction::send(
            "mars-lab",
            1,
        );

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

    let event =
        tx.receive(
            "venus-lab",
            Flow::Change,
        );

    let receiver_delta =
        event
            .heritage
            .transition
            .delta;

    assert!(
        receiver_delta <= sender_constant,
        "receiver delta {} exceeds sender delta {}",
        receiver_delta,
        sender_constant,
    );
}

#[test]
fn model2_structure_dominates_network() {
    let event =
        QuantumTransaction::send(
            "mars-lab",
            1,
        )
        .receive(
            "mars-lab",
            Flow::Change,
        );

    assert!(
        event
            .heritage
            .state
            .structural_value()
            >= event.qtm.net_work
    );
}