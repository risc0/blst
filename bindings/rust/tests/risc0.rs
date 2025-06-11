use anyhow::ensure;
use blst::{min_pk, min_sig, BLST_ERROR};
use guests::{BLS_ELF, BLS_ID};
use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha20Rng,
};
use risc0_zkvm::{default_prover, ExecutorEnv};
use test_log::test;
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

    let prover = default_prover();

    println!("Generating proof ({})...", prover.get_name());
    let prove_info = prover.prove(env, BLS_ELF)?;

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

pub fn kzg(n: u8) -> anyhow::Result<()> {
    let blobs = (0..n)
        .map(|i| Blob::new([i; BYTES_PER_BLOB]))
        .collect::<Vec<_>>();

    // compute KZG proofs
    let kzg_settings = c_kzg::ethereum_kzg_settings(0);

    let commits = blobs
        .iter()
        .map(|b| kzg_settings.blob_to_kzg_commitment(b).unwrap().to_bytes())
        .collect::<Vec<_>>();
    let proofs = zip(blobs.iter(), commits.iter())
        .map(|(b, c)| {
            kzg_settings
                .compute_blob_kzg_proof(b, c)
                .unwrap()
                .to_bytes()
        })
        .collect::<Vec<_>>();

    let blobs = blobs.iter().map(|b| b.to_vec()).collect::<Vec<_>>();
    let commits = commits.iter().map(|b| b.to_vec()).collect::<Vec<_>>();
    let proofs = proofs.iter().map(|b| b.to_vec()).collect::<Vec<_>>();
    let env = ExecutorEnv::builder()
        .write(&blobs)?
        .write(&commits)?
        .write(&proofs)?
        .build()?;

    println!("Generating proof...");
    let prove_info = default_prover().prove(env, KZG_ELF)?;

    println!("Verifying proof...");
    prove_info.receipt.verify(KZG_ID)?;

    Ok(())
}

#[test]
#[cfg_attr(not(feature = "cuda"), ignore = "proving takes a long time")]
fn r0vm_prove_bls_signatures() -> anyhow::Result<()> {
    bls([0u8; 32], 3)
}
