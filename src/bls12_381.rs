use crate::IndifferentiableHash;
use ark_bls12_381::g1::Config;
use ark_bls12_381::Fq;
use ark_ec::short_weierstrass::Affine;
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
    const Z: Fq = MontFp!("501185307051513973337446462668281432142924704371855479526782420057604592581826186485831721800670613054734723765276");
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
        let num0 = inputs[0];
        let num1 = inputs[1];
        let num2 = inputs[2];
        let den = inputs[3];
        let t1 = inputs[4];
        let t2 = inputs[5];

        let v = den.square();
        let u = num0.square() - v;
        let v2 = v.square();
        let v4 = v2.square();
        let v5 = v * v4;
        let v8 = v4.square();

        let theta = u * v5 * (u * v8).pow(Self::M.into_bigint());
        let v = theta * theta * theta * v;

        let mut w_zeta = theta;
        let w2 = Self::W.square();
        if t1 > Self::W * t1 {
            w_zeta *= Self::W;
        }
        if t1 > w2 * t1 {
            w_zeta *= Self::W;
        }

        let (x, y) = if v == u {
            (w_zeta, num0)
        } else if v == Self::W * u {
            (theta * t1, num1)
        } else if v == w2 * u {
            (theta * t2, num2)
        } else {
            panic!("should not arrive here")
        };

        Self::GroupAffine::new_unchecked(x, y / den)
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

        let num0 = MontFp!( "3907323029266142329677629247141145302116574109761409359386547830066801509673825460759676313956143925321184463756739");
        let num1 = MontFp!( "578272923952259724112273745438281857984753465059536553279481107815161821090037190528857633468439930778441935489925");
        let num2 = MontFp!( "823682855771317968884270516493825698933844833638923961461397642987234402518145944551804186068438433764063100887964");
        let den = MontFp!( "1347770150726807382080703071199277727039296615709072948268344845689432783849833566522518562382504519106049522492473");

        let x = MontFp!( "463172938055427656695940778573982304337940308805428225975291306144636365946397580750450928691055305460142008944275");
        let y = MontFp!( "922157006072556689886388480384040432811137800200465340009477125457864501221362576154817069261793790545967567025429");

        let res = <Config as IndifferentiableHash>::h_prime(&[num0, num1, num2, den, t1, t2]);
        assert_eq!(x, res.x);
        assert_eq!(y, res.y);
    }

    #[test]
    fn test_map() {
        // the following test inputs are obtained from the sage code with an input string s = "input to the test function"
        let s = "input to the test function";

        let x = MontFp!( "463172938055427656695940778573982304337940308805428225975291306144636365946397580750450928691055305460142008944275");
        let y = MontFp!( "922157006072556689886388480384040432811137800200465340009477125457864501221362576154817069261793790545967567025429");

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
