// src/main.rs

use qp_hd::qp44::{CoinType, Purpose, QP44, WalletRequest};
use qp_hd::purpose::{SeedSource};
use qp_hd::pathsregistry::PathsRegistry;


use k256::SecretKey;
use k256::elliptic_curve::sec1::ToEncodedPoint;
use ed25519_dalek::SigningKey;


use sha2::{Sha256, Digest};
use ripemd::Ripemd160;
use tiny_keccak::{Keccak, Hasher};

use bs58;
use hex;

#[tokio::main]
async fn main() {
    println!("🚀 QP-44 Thermodynamic HD Wallet — Forensic Mode\n");

    // 1️⃣ Seed source
    let seed = SeedSource::Raw(vec![0x11; 32]);

    // 2️⃣ Wallet request
    let request = WalletRequest {
        seed,
        purpose: Purpose::BIP44,
        coins: vec![
            CoinType::Bitcoin,
            CoinType::Ethereum,
            CoinType::Solana,
            CoinType::Tron,
        ],
        account: 0,
        index: 0,
    };

    // 3️⃣ Derive wallets
    let wallets = QP44::derive_wallet(request)
        .expect("Wallet derivation failed");

    println!("🌐 Derived Wallets:\n");

    for w in &wallets {
        println!("Coin: {}", w.coin.name());
    
        // 🔐 
        println!("Commitment: qp{}", hex::encode(w.commitment));
        println!();
    
        // 🔒 
        let priv_bytes = w.coordinate;
    



        
        // 5️⃣ Address derivation
        let address = match w.coin {
            CoinType::Bitcoin => {
                let secret = SecretKey::from_slice(&priv_bytes)
                    .expect("Invalid BTC key");
                let public = secret.public_key();
                let pub_bytes = public.to_encoded_point(false);

                let sha = Sha256::digest(pub_bytes.as_bytes());
                let ripe = Ripemd160::digest(&sha);

                let mut payload = vec![0x00]; // mainnet
                payload.extend_from_slice(&ripe);

                let checksum = Sha256::digest(&Sha256::digest(&payload));
                payload.extend_from_slice(&checksum[..4]);

                bs58::encode(payload).into_string()
            }
            CoinType::Ethereum => {
                let secret = SecretKey::from_slice(&priv_bytes)
                    .expect("Invalid ETH key");
                let public = secret.public_key();
                let uncompressed = public.to_encoded_point(false);

                let mut keccak = Keccak::v256();
                let mut out = [0u8; 32];
                keccak.update(&uncompressed.as_bytes()[1..]);
                keccak.finalize(&mut out);

                format!("0x{}", hex::encode(&out[12..]))
            }
            CoinType::Tron => {
                let secret = SecretKey::from_slice(&priv_bytes)
                    .expect("Invalid TRON key");
                let public = secret.public_key();
                let uncompressed = public.to_encoded_point(false);

                let mut keccak = Keccak::v256();
                let mut out = [0u8; 32];
                keccak.update(&uncompressed.as_bytes()[1..]);
                keccak.finalize(&mut out);

                let mut payload = vec![0x41];
                payload.extend_from_slice(&out[12..]);

                let checksum = Sha256::digest(&Sha256::digest(&payload));
                payload.extend_from_slice(&checksum[..4]);

                bs58::encode(payload).into_string()
            }
            CoinType::Solana => {
                let bytes: [u8; 32] = priv_bytes.as_slice()
                    .try_into()
                    .expect("Invalid SOL key length");
                let signing_key = SigningKey::from_bytes(&bytes);
                let public = signing_key.verifying_key();

                let mut full_key = Vec::new();
                full_key.extend_from_slice(&bytes);
                full_key.extend_from_slice(public.as_bytes());

                println!("  ├─ Solana Coordinate (Priv): 0x{}", hex::encode(bytes));
                println!("  ├─ Solana 64‑byte PrivKey   : {}", hex::encode(&full_key));

                bs58::encode(public.to_bytes()).into_string()
            }
        };

        println!("Blockchain Address: {}", address);

        // 6️⃣ Balance query
        match PathsRegistry::query_balance(w.coin, &address).await {
            Ok(balance) => println!("Balance: {}\n", balance),
            Err(err) => println!("Error: {}\n", err),
        }
    }

    println!("✨ Done.");

}