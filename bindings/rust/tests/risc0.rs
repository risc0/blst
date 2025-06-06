#![cfg(feature = "r0vm-tests")]

use anyhow::ensure;
use blst::{min_pk, min_sig, BLST_ERROR};
use guests::{BLS_ELF, BLS_ID};
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use tracing_subscriber::fmt::writer::TestWriter;

// Define a message and DSTs
const MSG: &[u8] =
    b"This is the message to be signed by BLS12-381 within RISC Zero ZKVM";
// DST for MinPk: G1 public keys, G2 signatures
const DST_MIN_PK: &[u8] = b"BLS_SIG_BLS12381G1_XMD:SHA-256_SSWU_RO_AUG_";
// DST for MinSig: G2 public keys, G1 signatures
const DST_MIN_SIG: &[u8] = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_AUG_";
const AUG: &[u8] = b"augmentation_data_for_testing";

fn bls(seed: [u8; 32], n: usize) -> anyhow::Result<()> {
    let mut rng = ChaCha20Rng::from_seed(seed);

    let sk_be_bytes_vec = (0..n)
        .map(|_| {
            let mut ikm = [0u8; 32];
            rng.fill_bytes(&mut ikm);
            min_pk::SecretKey::key_gen(&ikm, &[]).unwrap().to_bytes()
        })
        .collect::<Vec<_>>();

    let env = ExecutorEnv::builder()
        .stdout(TestWriter::new())
        .write(&sk_be_bytes_vec)?
        .build()?;

    println!("Generating proof...");
    let prove_info = default_prover().prove(env, BLS_ELF)?;
    println!("Proof finished: {:?}", prove_info.stats);

    println!("Verifying proof...");
    prove_info.receipt.verify(BLS_ID)?;

    println!("Validating Journal...");
    let journal = &prove_info.receipt.journal;

    // --- Check aggregated MinPk signature (PK in G1, Sig in G2) ---
    let sig_min_pk =
        min_pk::Signature::deserialize(&journal.bytes[..192]).unwrap();

    let mut pks = sk_be_bytes_vec
        .iter()
        .map(|b| min_pk::SecretKey::from_bytes(b).unwrap().sk_to_pk());
    let mut agg_pk =
        min_pk::AggregatePublicKey::from_public_key(&pks.next().unwrap());
    pks.for_each(|pk| agg_pk.add_public_key(&pk, true).unwrap());
    let pk_min_pk = agg_pk.to_public_key();

    ensure!(
        sig_min_pk.verify(true, MSG, DST_MIN_PK, AUG, &pk_min_pk, true)
            == BLST_ERROR::BLST_SUCCESS,
        "invalid MinPk signature"
    );

    // --- Check aggregated MinSig signature (PK in G2, Sig in G1) ---
    let sig_min_sig =
        min_sig::Signature::deserialize(&journal.bytes[192..]).unwrap();

    let mut pks = sk_be_bytes_vec
        .iter()
        .map(|b| min_sig::SecretKey::from_bytes(b).unwrap().sk_to_pk());
    let mut agg_pk =
        min_sig::AggregatePublicKey::from_public_key(&pks.next().unwrap());
    pks.for_each(|pk| agg_pk.add_public_key(&pk, true).unwrap());
    let pk_min_sig = agg_pk.to_public_key();

    ensure!(
        sig_min_sig.verify(true, MSG, DST_MIN_SIG, AUG, &pk_min_sig, true)
            == BLST_ERROR::BLST_SUCCESS,
        "invalid MinSig signature"
    );

    Ok(())
}

#[test]
fn r0vm_prove_bls_signatures() -> anyhow::Result<()> {
    bls([0u8; 32], 2)
}
