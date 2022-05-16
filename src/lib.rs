use ark_ec::SWModelParameters;
use ark_ff::PrimeField;

mod bls12_381;

pub trait IndifferentiableHash: SWModelParameters
where
    Self::BaseField: PrimeField,
{
    // m = (q - 10) // 27
    const M: Self::BaseField;
    // w = b^((q-1) // 3)
    const W: Self::BaseField;
    // z = w.nth_root(3)
    const Z: Self::BaseField;
    // c1 = (b/z).nth_root(3)
    const C: Self::BaseField;

    /// projective curve point
    type GroupProject;

    /// map an element in Fq^2 to
    fn hash_to_curve<B: AsRef<[u8]>>(input: B) -> Self::GroupProject;

    /// rational map Fq^2 -> T(Fq)
    /// returns nums0, nums1, nums2, den, s1s2
    fn phi(t1: &Self::BaseField, t2: &Self::BaseField) -> [Self::BaseField; 5];

    /// hash function to the plane Fq^2
    fn eta<B: AsRef<[u8]>>(input: B) -> [Self::BaseField; 2];

    // auxiliary map from the threefold T to Eb
    fn h_prime(inputs: &[Self::BaseField; 6]) -> Self::GroupProject;
}
