use crate::IndifferentiableHash;
use ark_bls12_381::g1::Config;
use ark_bls12_381::Fq;
use ark_ec::short_weierstrass::Affine;
use ark_ec::short_weierstrass::SWCurveConfig;
use ark_ff::Field;
use ark_ff::MontFp;
use ark_ff::PrimeField;

impl IndifferentiableHash for Config {
    // m = (q - 10) // 27
    const M: Fq = MontFp!("148237390934135829385844067619848302094699363701444736493779930967556727795956957942321764041815394964366454539251");
    // w is a primitive 3rd root of unity
    // w = b^((q-1) // 3)
    const W: Fq = MontFp!("793479390729215512621379701633421447060886740281060493010456487427281649075476305620758731620350");
    // z (i.e., zeta in [1, Section 3]) is a primitive 9th root of unity
    // z = w.nth_root(3)
    const Z: Fq = MontFp!("656279539151453036372723733049135970080835961207516703218496207152846698634665245028822411104358743008817256364884");
    // c1 = (b/z).nth_root(3)
    const C: Fq = MontFp!("656279539151453036372723733049135970080835961207516703218496207152846698634665245028822411104358743008817256364884");
    // sb = b.nth_root(2)
    const SB: Fq = MontFp!("4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559785");

    /// affine curve point
    type GroupAffine = Affine<Self>;

    ///  Auxiliary map h': T(Fq) -> Eb(Fq):
    //
    //  In [1, Section 2] we deal with a Calabi-Yau threefold defined as
    //  the quotient T := Eb x Eb' x Eb'' / [w] x [w] x [w],
    //  where Eb', Eb'' are the cubic twists of Eb
    //  and [w](x, y) -> (wx, y) is an automorphism of order 3 on Eb, Eb', and Eb''.
    //
    fn h_prime(inputs: &[Self::BaseField; 6]) -> Self::GroupAffine {
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
        let tmp = tmp.pow(Self::M.into_bigint());
        let theta = u * v8 * tmp;

        let v = theta * theta * theta * v;
        let v3 = v * v * v;

        let w2 = Self::W * Self::W;
        let z2 = Self::Z * Self::Z;

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
            let x = theta * t1;
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
            let x = theta * t2;
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
        let z_inv = z.inverse().unwrap();
        Self::GroupAffine::new_unchecked(x * z_inv, y * z_inv)
    }
}

#[cfg(test)]
mod test {
    use crate::test_vectors::bls12_381_test;
    use crate::IndifferentiableHash;
    use ark_bls12_381::g1::Config;
    use ark_ff::MontFp;
    use itoa::Buffer;

    #[test]
    fn test_phi() {
        // the following test inputs are obtained from the sage code with an input string s = "input to the test function"
        let t1 = MontFp!( "1637916486738181879757594354935247698146190377973924295856087059563097387500579915402466902218127343335463775185097");
        let t2 =  MontFp!( "3084368236562539678793686966099022796947242601500183975334286593823404552243658178662185836974209583527845605498635");

        let num0 =  MontFp!( "668793913132438851216583034173410129819241113864860331755488532912633001474798319621970922570168293026704984270511");
        let num1 =  MontFp!( "3212829122310676996023797374386516136736852421021963527590835393030480338172734951597532304343302753388335391481734");
        let num2 =  MontFp!( "1548716369932015580776602644005522146811092687079938004520416189245103671602049774350095885666396762320849868946947");
        let den =  MontFp!( "2715169702687565714008491526282724206683593110983380931933370057594108505624791522784799556289933904367945122349596");

        let res = <Config as IndifferentiableHash>::phi(&t1, &t2);

        assert_eq!(res[0], num0);
        assert_eq!(res[1], num1);
        assert_eq!(res[2], num2);
        assert_eq!(res[3], den);
    }

    #[test]
    fn test_eta() {
        // the following test inputs are obtained from the sage code with an input string s = "input to the test function"
        let s = "input to the test function";
        let t1 = MontFp!( "1637916486738181879757594354935247698146190377973924295856087059563097387500579915402466902218127343335463775185097");
        let t2 = MontFp!( "3084368236562539678793686966099022796947242601500183975334286593823404552243658178662185836974209583527845605498635");

        let res = <Config as IndifferentiableHash>::eta(s);
        assert_eq!(res[0], t1);
        assert_eq!(res[1], t2);
    }

    #[test]
    fn test_h_prime() {
        // the following test inputs are obtained from the sage code with an input string s = "input to the test function"
        let t1 = MontFp!( "1637916486738181879757594354935247698146190377973924295856087059563097387500579915402466902218127343335463775185097");
        let t2 = MontFp!( "3084368236562539678793686966099022796947242601500183975334286593823404552243658178662185836974209583527845605498635");

        let num0 =  MontFp!( "668793913132438851216583034173410129819241113864860331755488532912633001474798319621970922570168293026704984270511");
        let num1 =  MontFp!( "3212829122310676996023797374386516136736852421021963527590835393030480338172734951597532304343302753388335391481734");
        let num2 =  MontFp!( "1548716369932015580776602644005522146811092687079938004520416189245103671602049774350095885666396762320849868946947");
        let den =  MontFp!( "2715169702687565714008491526282724206683593110983380931933370057594108505624791522784799556289933904367945122349596");

        let x = MontFp!( "1816253950397860200714343084334638831538056055256723554500548354781499946331741127336760011522698677680124556029416");
        let y = MontFp!( "3244022000566907360019058064254357188251810714491513483291828058507316467039183172973529080475518022136703508585130");

        let res = <Config as IndifferentiableHash>::h_prime(&[num0, num1, num2, den, t1, t2]);
        assert_eq!(x, res.x);
        assert_eq!(y, res.y);
    }

    #[test]
    fn test_map() {
        // the following test inputs are obtained from the sage code with an input string s = "input to the test function"
        let s = "input to the test function";

        let x = MontFp!( "1816253950397860200714343084334638831538056055256723554500548354781499946331741127336760011522698677680124556029416");
        let y = MontFp!( "3244022000566907360019058064254357188251810714491513483291828058507316467039183172973529080475518022136703508585130");

        let res = <Config as IndifferentiableHash>::hash_to_curve_unchecked(s);
        assert_eq!(x, res.x);
        assert_eq!(y, res.y);

        assert!(res.is_on_curve());
    }

    #[test]
    fn check_test_vectors() {
        let test_vectors = bls12_381_test();
        assert!(test_vectors.len() % 2 == 0);
        for i in 0..test_vectors.len() / 2 {
            let mut buffer = Buffer::new();
            let printed = buffer.format(i);
            let res = <Config as IndifferentiableHash>::hash_to_curve_unchecked(printed);
            assert_eq!(test_vectors[i * 2], res.x);
            assert_eq!(test_vectors[i * 2 + 1], res.y);

            assert!(res.is_on_curve());
        }
    }
}
