use qp_hd::purpose::{Purpose, SeedSource, BIP39Language};
use qp_hd::qp44::{QP44Wallet, CoinType};

#[test]
fn forensic_qp44_chain() {
    let seed = SeedSource::Raw(vec![
        12, 45, 78, 91, 33, 64, 255, 18,
        7, 88, 120, 9, 10, 11, 12, 13,
        14, 15, 16, 17, 18, 19, 20, 21,
        22, 23, 24, 25, 26, 27, 28, 29,
    ]);

    let base = Purpose::quantperm_from_seed(seed)
        .expect("Failed to build QuantPerm from seed");

    let mut manifold = base;

    println!("\n🔬 FORENSIC QP44 CHAIN START\n");

    // Main loop: per Coin (as you requested)
    for coin in CoinType::all() {
        println!("\n━━━━━━━━━━━━━━━ {} CHAIN ━━━━━━━━━━━━━━━", coin.name());

        for round in 0..10 {
            let mut wallet = QP44Wallet::from_quantperm(
                manifold,
                *coin,
                round as u32,
            );

            let event = wallet.next_receive();
            let h = &event.heritage;
            let q = &event.qtm;

            println!("\n🪙 {} (Round {})", coin.name(), round);
            println!("dim   : {}", h.state.dimension());
            println!("activ : {}", h.state.activations());
            println!("mass  : {}", h.state.retained_mass());
            println!("sigma : {}", h.state.structural_value());
            println!("tau   : {}", h.transition.tau);
            println!("delta : {}", h.transition.delta);
            println!("gross : {}", h.transition.gross_work);
            println!("net   : {}", h.transition.net_work);
            println!("coord : {:02x?}", q.coordinate);
            println!("commit: {:02x?}", q.commitment);

            // Propagate state for next iteration
            manifold = wallet.into_manifold();
        }
    }

    println!("\n✅ FORENSIC CHAIN COMPLETE\n");
}