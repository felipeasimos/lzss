use std::simd::{cmp::SimdPartialEq, Simd};

const SIMD_WIDTH : usize = 16;

pub fn find_first_match(sequence: &[u8], target: u8) -> Option<usize> {
    let target_vector : Simd<u8, SIMD_WIDTH>  = Simd::splat(target);
    let chunks = sequence.chunks_exact(SIMD_WIDTH);
    let chunks_len = chunks.len();
    let remainder = chunks.remainder();
    for (chunk_index, chunk) in chunks.enumerate() {
        let simd_chunk: Simd<u8, SIMD_WIDTH> = Simd::from_slice(chunk);
        let mask  = simd_chunk.simd_eq(target_vector);
        if mask.any() {
            let lane_index= mask.to_array().iter().position(|&is_match| is_match).unwrap();
            return Some(chunk_index * SIMD_WIDTH + lane_index);
        }
    }
    if let Some(index) = remainder.iter().position(|&c| c == target) {
        return Some(chunks_len * SIMD_WIDTH + index);
    }
    None
}
