use std::marker::PhantomData;
use ark_ec::AffineCurve;
use ark_ec::SWModelParameters;
use ark_ec::hashing::map_to_curve_hasher::MapToCurve;
use ark_ec::hashing::HashToCurveError;
use ark_ec::CurveGroup;
use ark_ff::PrimeField;
use ark_ff::Zero;
use sha2::Digest;
use sha2::Sha512;

mod bls12_377;
mod bls12_381;

#[cfg(test)]
mod test_vectors;

pub struct BLSIndHash<P>
where
    P: IndifferentiableHash,
    P::BaseField: PrimeField,
{
    phantom: PhantomData<P>,
}

impl<P, T> MapToCurve<T> for BLSIndHash<P>
where
    P: IndifferentiableHash,
    T: CurveGroup<Affine = CurveAffine<P>>,
    P::BaseField: PrimeField,
{
    /// Constructs a new mapping.
    fn new() -> Result<Self, HashToCurveError> {
        Ok(Self {
            phantom: PhantomData::<P>,
        })
    }

    /// Map an arbitary field element to a corresponding curve point.
    fn map_to_curve(&self, point: T::BaseField) -> Result<T::Affine, HashToCurveError> {
        Ok(P::hash_to_curve(input))
    }
}

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

    /// affine curve point
    type GroupAffine: AffineCurve;

    /// map an element in Fq^2 to Group
    fn hash_to_curve<B: AsRef<[u8]>>(input: B) -> Self::GroupAffine {
        let t = Self::eta(input);
        let nums = Self::phi(&t[0], &t[1]);
        let p = Self::h_prime(&[nums[0], nums[1], nums[2], nums[3], t[0], t[1]]);
        // if s1s2 == 0:
        if nums[4] == Self::BaseField::zero() {
            Self::GroupAffine::zero()
        } else if nums[3] == Self::BaseField::zero() {
            Self::GroupAffine::prime_subgroup_generator()
        } else {
            p
        }
    }

    /// rational map Fq^2 -> T(Fq)
    /// returns nums0, nums1, nums2, den, s1s2
    fn phi(t1: &Self::BaseField, t2: &Self::BaseField) -> [Self::BaseField; 5];

    /// hash function to the plane Fq^2
    fn eta<B: AsRef<[u8]>>(input: B) -> [Self::BaseField; 2] {
        let mut s0 = input.as_ref().to_owned();
        s0.push('0' as u8);
        let mut s1 = input.as_ref().to_owned();
        s1.push('1' as u8);

        let mut hasher = Sha512::new();
        hasher.update(s0);
        let output = hasher.finalize();
        let t1 = Self::BaseField::from_be_bytes_mod_order(&output);

        let mut hasher = Sha512::new();
        hasher.update(s1);
        let output = hasher.finalize();
        let t2 = Self::BaseField::from_be_bytes_mod_order(&output);

        [t1, t2]
    }

    // auxiliary map from the threefold T to Eb
    fn h_prime(inputs: &[Self::BaseField; 6]) -> Self::GroupAffine;
}
