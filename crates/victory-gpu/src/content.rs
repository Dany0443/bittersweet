//! Portable texture-content checks, adapted from klbf-lab PR #28.

/// Keep small checks serial; large textures use BLAKE3's identical tree hash
/// on Rayon's workers. Shader and pipeline cache keys keep their existing path.
pub(crate) fn content_hash(bytes: &[u8]) -> blake3::Hash {
    if bytes.len() < 1 << 20 {
        return blake3::hash(bytes);
    }
    let mut hash = blake3::Hasher::new();
    hash.update_rayon(bytes);
    hash.finalize()
}

#[cfg(test)]
mod tests {
    use super::content_hash;

    #[test]
    fn texture_hashes_match_at_chunk_and_parallel_boundaries() {
        const MIB: usize = 1 << 20;
        let data: Vec<u8> = (0..9 * MIB + 4097)
            .map(|i| (i * 131 + i / 977) as u8)
            .collect();
        for len in [
            0,
            1,
            1024,
            1025,
            MIB - 1,
            MIB,
            MIB + 1,
            MIB + 1024,
            3 * MIB + 7,
            8 * MIB,
            data.len(),
        ] {
            assert_eq!(
                content_hash(&data[..len]),
                blake3::hash(&data[..len]),
                "{len} bytes"
            );
        }
    }
}
