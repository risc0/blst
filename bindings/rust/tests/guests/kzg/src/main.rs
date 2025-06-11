#![no_main]

use c_kzg::{
    BYTES_PER_BLOB, BYTES_PER_COMMITMENT, BYTES_PER_PROOF, Blob, Bytes48,
};
use risc0_zkvm::guest::env;
use std::vec::Vec;

risc0_zkvm::guest::entry!(main);

fn read_vecs<T: TryFrom<Vec<u8>>>() -> Vec<T>
where
    <T as TryFrom<Vec<u8>>>::Error: std::fmt::Debug,
{
    let vecs: Vec<Vec<u8>> = env::read();
    vecs.into_iter().map(|v| v.try_into().unwrap()).collect()
}

pub fn main() {
    let blobs = read_vecs::<[u8; BYTES_PER_BLOB]>()
        .into_iter()
        .map(|b| Blob::new(b))
        .collect::<Vec<_>>();
    let commitments = read_vecs::<[u8; BYTES_PER_COMMITMENT]>()
        .into_iter()
        .map(|c| Bytes48::new(c))
        .collect::<Vec<_>>();
    let proofs = read_vecs::<[u8; BYTES_PER_PROOF]>()
        .into_iter()
        .map(|p| Bytes48::new(p))
        .collect::<Vec<_>>();
    env::log("Input read");

    let kzg_settings = c_kzg::ethereum_kzg_settings(0);
    env::log("LOADED SETTINGS");

    for i in 0..blobs.len() {
        assert!(
            kzg_settings
                .verify_blob_kzg_proof(&blobs[i], &commitments[i], &proofs[i])
                .unwrap()
        );
        env::log("Blob verification passed");
    }

    assert!(
        kzg_settings
            .verify_blob_kzg_proof_batch(&blobs, &commitments, &proofs)
            .unwrap()
    );
}
