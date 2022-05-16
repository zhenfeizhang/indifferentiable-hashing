use crate::IndifferentiableHash;
use ark_bls12_381::{g1::Parameters, Fq};
use ark_ec::short_weierstrass_jacobian::GroupProjective;
use ark_ec::ProjectiveCurve;
use ark_ec::SWModelParameters;
use ark_ff::field_new;
use ark_ff::Field;
use ark_ff::PrimeField;
use ark_ff::Zero;
use sha2::Digest;
use sha2::Sha512;

impl IndifferentiableHash for Parameters {
    // m = (q - 10) // 27
    const M: Self::BaseField = field_new!(Fq, "148237390934135829385844067619848302094699363701444736493779930967556727795956957942321764041815394964366454539251");
    // w = b^((q-1) // 3)
    const W: Self::BaseField = field_new!(Fq, "4002409555221667392624310435006688643935503118305586438271171395842971157480381377015405980053539358417135540939436");
    // z = w.nth_root(3)
    const Z: Self::BaseField=field_new!(Fq, "501185307051513973337446462668281432142924704371855479526782420057604592581826186485831721800670613054734723765276");
    // c1 = (b/z).nth_root(3)
    const C: Self::BaseField=field_new!(Fq, "529033685927954107995765316255150655705710311730735691995243315144334423929822497684682959478359149743541419332944");

    /// projective curve point
    type GroupProject = GroupProjective<Self>;

    /// map an element in Fq^2 to
    fn hash_to_curve<B: AsRef<[u8]>>(input: B) -> Self::GroupProject {
        let t = Self::eta(input);
        let nums = Self::phi(&t[0], &t[1]);
        let p = Self::h_prime(&[nums[0], nums[1], nums[2], nums[3], t[0], t[1]]);
        // if s1s2 == 0:
        if nums[4] == Self::BaseField::zero() {
            Self::GroupProject::zero()
        } else if nums[3] == Self::BaseField::zero() {
            Self::GroupProject::prime_subgroup_generator()
        } else {
            p
        }
    }

    /// rational map Fq^2 -> T(Fq)
    fn phi(t1: &Self::BaseField, t2: &Self::BaseField) -> [Self::BaseField; 5] {
        // constants
        let one = Self::BaseField::from(1u64);
        let two = Self::BaseField::from(2u64);
        let three = Self::BaseField::from(3u64);

        // square root of b
        let sb = two;

        let s1 = *t1 * *t1 * *t1;
        let s2 = *t2 * *t2 * *t2;
        let s1s1 = s1 * s1;
        let s2s2 = s2 * s2;
        let s1s2 = s1 * s2;

        let b2 = Self::COEFF_B * Self::COEFF_B;
        let b3 = Self::COEFF_B * b2;
        let b4 = b2 * b2;

        let a20 = b2 * s1s1;
        let a11 = two * b3 * s1s2;
        let a10 = two * Self::COEFF_B * s1;
        let a02 = b4 * s2s2;
        let a01 = two * b2 * s2;

        let num0 = sb * (a20 - a11 + a10 + a02 + a01 - three);
        let num1 = sb * (-three * a20 + a11 + a10 + a02 - a01 + one);
        let num2 = sb * (a20 + a11 - a10 - three * a02 + a01 + one);
        let den = a20 - a11 - a10 + a02 - a01 + one;

        [num0, num1, num2, den, s1s2]
    }

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
    fn h_prime(inputs: &[Self::BaseField; 6]) -> Self::GroupProject {
        let one = Self::BaseField::from(1u64);

        let num0 = inputs[0];
        let num1 = inputs[1];
        let num2 = inputs[2];
        let den = inputs[3];
        let t1 = inputs[4];
        let t2 = inputs[5];

        let v = den * den;
        let u = num0 * num0 - Self::COEFF_B * v;
        let v2 = v * v;
        let v4 = v2 * v2;
        let v8 = v4 * v4;
        let v9 = v * v8;
        let v16 = v8 * v8;
        let v25 = v9 * v16;

        let u2 = u * u;
        let u3 = u * u2;

        // compute theta = u*v8*(u2*v25)^m
        let tmp = u2 * v25;
        let tmp = tmp.pow(Self::M.into_repr());
        let theta = u * v8 * tmp;

        let v = theta * theta * theta * v;
        let v3 = v * v * v;

        let w2 = Self::W * Self::W;
        let z2 = Self::Z * Self::Z;
        let c2 = Self::C * Self::C;

        let mut w_zeta = theta;

        if t1 > Self::W * t1 {
            w_zeta *= Self::W;
        }
        if t1 > w2 * t1 {
            w_zeta *= Self::W;
        }

        let (x, y, z) = if v3 == u3 {
            let (y, z) = {
                if v == u {
                    (one, one)
                } else if v == Self::W * u {
                    (Self::Z, Self::Z)
                } else if v == w2 * u {
                    (z2, z2)
                } else {
                    panic!("should not arrive here")
                }
            };
            let y = y * num0;

            (w_zeta, y, z)
        } else if v3 == Self::W * u3 {
            let x = Self::C * theta * t1;
            let zu = Self::Z * u;
            let (mut y, z) = {
                if v == zu {
                    (one, one)
                } else if v == Self::W * zu {
                    (Self::Z, Self::Z)
                } else if v == w2 * zu {
                    (z2, z2)
                } else {
                    panic!("should not arrive here")
                }
            };
            y = y * num1;
            (x, y, z)
        } else if v3 == w2 * u3 {
            let x = c2 * theta * t2;
            let z2u = z2 * u;
            let (mut y, z) = {
                if v == z2u {
                    (one, one)
                } else if v == Self::W * z2u {
                    (Self::Z, Self::Z)
                } else if v == w2 * z2u {
                    (z2, z2)
                } else {
                    panic!("should not arrive here")
                }
            };
            y = y * num2;
            (x, y, z)
        } else {
            panic!("should not arrive here")
        };

        let x = x * den;
        let z = z * den;

        Self::GroupProject::new(x, y, z)
    }
}

#[cfg(test)]
mod test {
    use crate::IndifferentiableHash;
    use ark_bls12_381::{g1::Parameters, Fq};
    use ark_ff::field_new;

    #[test]
    fn test_phi() {
        // the following test inputs are obtained from the sage code with an input string s = "input to the test function"
        let t1 = field_new!(Fq, "1637916486738181879757594354935247698146190377973924295856087059563097387500579915402466902218127343335463775185097");
        let t2 = field_new!(Fq, "3084368236562539678793686966099022796947242601500183975334286593823404552243658178662185836974209583527845605498635");

        let num0 = field_new!(Fq, "3907323029266142329677629247141145302116574109761409359386547830066801509673825460759676313956143925321184463756739");
        let num1 = field_new!(Fq, "578272923952259724112273745438281857984753465059536553279481107815161821090037190528857633468439930778441935489925");
        let num2 = field_new!(Fq, "823682855771317968884270516493825698933844833638923961461397642987234402518145944551804186068438433764063100887964");
        let den = field_new!(Fq, "1347770150726807382080703071199277727039296615709072948268344845689432783849833566522518562382504519106049522492473");

        let res = <Parameters as IndifferentiableHash>::phi(&t1, &t2);

        assert_eq!(res[0], num0);
        assert_eq!(res[1], num1);
        assert_eq!(res[2], num2);
        assert_eq!(res[3], den);
    }

    #[test]
    fn test_eta() {
        // the following test inputs are obtained from the sage code with an input string s = "input to the test function"
        let s = "input to the test function";
        let t1 = field_new!(Fq, "1637916486738181879757594354935247698146190377973924295856087059563097387500579915402466902218127343335463775185097");
        let t2 = field_new!(Fq, "3084368236562539678793686966099022796947242601500183975334286593823404552243658178662185836974209583527845605498635");

        let res = <Parameters as IndifferentiableHash>::eta(s);
        assert_eq!(res[0], t1);
        assert_eq!(res[1], t2);
    }

    #[test]
    fn test_h_prime() {
        // the following test inputs are obtained from the sage code with an input string s = "input to the test function"
        let t1 = field_new!(Fq, "1637916486738181879757594354935247698146190377973924295856087059563097387500579915402466902218127343335463775185097");
        let t2 = field_new!(Fq, "3084368236562539678793686966099022796947242601500183975334286593823404552243658178662185836974209583527845605498635");

        let num0 = field_new!(Fq, "3907323029266142329677629247141145302116574109761409359386547830066801509673825460759676313956143925321184463756739");
        let num1 = field_new!(Fq, "578272923952259724112273745438281857984753465059536553279481107815161821090037190528857633468439930778441935489925");
        let num2 = field_new!(Fq, "823682855771317968884270516493825698933844833638923961461397642987234402518145944551804186068438433764063100887964");
        let den = field_new!(Fq, "1347770150726807382080703071199277727039296615709072948268344845689432783849833566522518562382504519106049522492473");

        let x = field_new!(Fq, "1138317366648914730625947360205889613576807069530093434329853736040379797074055378501266468834618916351200246830021");
        let y = field_new!(Fq, "3907323029266142329677629247141145302116574109761409359386547830066801509673825460759676313956143925321184463756739");
        let z = field_new!(Fq, "1347770150726807382080703071199277727039296615709072948268344845689432783849833566522518562382504519106049522492473");

        let res = <Parameters as IndifferentiableHash>::h_prime(&[num0, num1, num2, den, t1, t2]);
        assert_eq!(x, res.x);
        assert_eq!(y, res.y);
        assert_eq!(z, res.z);
    }

    #[test]
    fn test_map() {
        // the following test inputs are obtained from the sage code with an input string s = "input to the test function"
        let s = "input to the test function";

        let x = field_new!(Fq, "1138317366648914730625947360205889613576807069530093434329853736040379797074055378501266468834618916351200246830021");
        let y = field_new!(Fq, "3907323029266142329677629247141145302116574109761409359386547830066801509673825460759676313956143925321184463756739");
        let z = field_new!(Fq, "1347770150726807382080703071199277727039296615709072948268344845689432783849833566522518562382504519106049522492473");

        let res = <Parameters as IndifferentiableHash>::hash_to_curve(s);
        assert_eq!(x, res.x);
        assert_eq!(y, res.y);
        assert_eq!(z, res.z);
    }
}
