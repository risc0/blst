#![no_main]

use blst::{min_pk, min_sig, BLST_ERROR};
use risc0_zkvm::guest::env;
use std::vec::Vec;

risc0_zkvm::guest::entry!(main);

// Define a message and DSTs
const MSG: &[u8] =
    b"This is the message to be signed by BLS12-381 within RISC Zero ZKVM";
// DST for MinPk: G1 public keys, G2 signatures
const DST_MIN_PK: &[u8] = b"BLS_SIG_BLS12381G1_XMD:SHA-256_SSWU_RO_AUG_";
// DST for MinSig: G2 public keys, G1 signatures
const DST_MIN_SIG: &[u8] = b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_AUG_";
const AUG: &[u8] = b"augmentation_data_for_testing";

pub fn main() {
    // Read the input secret keys (Vec of 32-byte arrays, assumed big-endian)
    let sk_be_bytes_vec: Vec<[u8; 32]> = env::read();
    env::log("Input read");
    assert!(!sk_be_bytes_vec.is_empty());

    let mut pks_for_min_pk_aggregation: Vec<min_pk::PublicKey> = Vec::new();
    let mut sigs_for_min_pk_aggregation: Vec<min_pk::Signature> = Vec::new();

    let mut pks_for_min_sig_aggregation: Vec<min_sig::PublicKey> = Vec::new();
    let mut sigs_for_min_sig_aggregation: Vec<min_sig::Signature> = Vec::new();

    for sk_be_bytes in &sk_be_bytes_vec {
        // --- MinPk Scheme (PK in G1, Sig in G2) ---
        let sk = min_pk::SecretKey::from_bytes(sk_be_bytes).unwrap();
        let pk = sk.sk_to_pk();
        assert_eq!(pk, min_pk::PublicKey::uncompress(&pk.compress()).unwrap());
        pks_for_min_pk_aggregation.push(pk);

        let sig = sk.sign(MSG, DST_MIN_PK, AUG);
        let verification_result =
            sig.verify(true, MSG, DST_MIN_PK, AUG, &pk, true);
        assert_eq!(verification_result, BLST_ERROR::BLST_SUCCESS);
        env::log("MinPk verification passed");
        sigs_for_min_pk_aggregation.push(sig);

        // --- MinSig Scheme (PK in G2, Sig in G1) ---
        let sk = min_sig::SecretKey::from_bytes(sk_be_bytes).unwrap();
        let pk = sk.sk_to_pk();
        assert_eq!(pk, min_sig::PublicKey::uncompress(&pk.compress()).unwrap());
        pks_for_min_sig_aggregation.push(pk);

        let sig = sk.sign(MSG, DST_MIN_SIG, AUG);
        let verification_result =
            sig.verify(true, MSG, DST_MIN_SIG, AUG, &pk, true);
        assert_eq!(verification_result, BLST_ERROR::BLST_SUCCESS);
        env::log("MinSig verification passed");
        sigs_for_min_sig_aggregation.push(sig);
    }

    // --- MinPk Scheme (PK in G1, Sig in G2) ---
    let mut iter = pks_for_min_pk_aggregation.iter();
    let mut agg_pk =
        min_pk::AggregatePublicKey::from_public_key(&iter.next().unwrap());
    iter.for_each(|pk| agg_pk.add_public_key(pk, true).unwrap());
    let agg_pk_min_pk = agg_pk.to_public_key();

    let mut iter = sigs_for_min_pk_aggregation.iter();
    let mut agg_sig_min_pk =
        min_pk::AggregateSignature::from_signature(&iter.next().unwrap());
    iter.for_each(|pk| agg_sig_min_pk.add_signature(pk, true).unwrap());
    let agg_sig_min_pk = agg_sig_min_pk.to_signature();

    assert_eq!(
        agg_sig_min_pk.verify(true, MSG, DST_MIN_PK, AUG, &agg_pk_min_pk, true),
        BLST_ERROR::BLST_SUCCESS
    );
    env::log("MinPk aggregated verification passed");

    env::commit_slice(&agg_sig_min_pk.serialize());

    // --- MinSig Scheme (PK in G2, Sig in G1) ---
    let mut iter = pks_for_min_sig_aggregation.iter();
    let mut agg_pk =
        min_sig::AggregatePublicKey::from_public_key(&iter.next().unwrap());
    iter.for_each(|pk| agg_pk.add_public_key(pk, true).unwrap());
    let agg_pk_min_sig = agg_pk.to_public_key();

    let mut iter = sigs_for_min_sig_aggregation.iter();
    let mut agg_sig_min_sig =
        min_sig::AggregateSignature::from_signature(&iter.next().unwrap());
    iter.for_each(|pk| agg_sig_min_sig.add_signature(pk, true).unwrap());
    let agg_sig_min_sig = agg_sig_min_sig.to_signature();

    assert_eq!(
        agg_sig_min_sig.verify(
            true,
            MSG,
            DST_MIN_SIG,
            AUG,
            &agg_pk_min_sig,
            true
        ),
        BLST_ERROR::BLST_SUCCESS
    );
    env::log("MinSig aggregated verification passed");

    env::commit_slice(&agg_sig_min_sig.serialize())
}
