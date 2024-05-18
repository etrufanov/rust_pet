use crate::vector::Vector;

/// Sorts the list of vectors in place by z-axis coordinate (the largest is the first)
pub fn sort_vectors_by_z(vectors: &mut Vec<Vector>) {
    vectors.sort_unstable_by(|a, b| (-(a.z())).partial_cmp(&(-(b.z()))).unwrap());
}

/// Sorts the enumerated list of vectors in place by z-axis coordinate (the largest is the first)
pub fn sort_enumerated_vectors_by_z(vectors: &mut Vec<(usize, Vector)>) {
    vectors.sort_unstable_by(|a, b| (-(a.1.z())).partial_cmp(&(-(b.1.z()))).unwrap());
}
