use crate::IndifferentiableHash;
use ark_bls12_377::g1::Config;
use ark_ec::short_weierstrass::Affine;
use ark_ff::Field;
use ark_ff::MontFp;
use ark_ff::PrimeField;

impl IndifferentiableHash for Config {
    // m = (q - 7) // 9
    const M: Self::BaseField = MontFp!( "28740491779218788223405859299432614837377056972768295615542695851857829816482313641663209793285928902715591273130");
    // w is a primitive 3rd root of unity
    // w = b^((q-1) // 3)
    const W: Self::BaseField = MontFp!( "80949648264912719408558363140637477264845294720710499478137287262712535938301461879813459410945");
    // z (i.e., zeta in [1, Section 3]) is a primitive 9th root of unity
    // z = w.nth_root(3)
    const Z: Self::BaseField = MontFp!("0");
    // sqrt(c) = w2
    const C: Self::BaseField = MontFp!( "80949648264912719408558363140637477264845294720710499478137287262712535938301461879813459410945");
    // sb = b.nth_root(2)
    const SB: Self::BaseField = MontFp!("258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458176");

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
    use crate::{test_vectors::bls12_377_test, IndifferentiableHash};
    use ark_bls12_377::g1::Config;
    use ark_ff::MontFp;
    use itoa::Buffer;

    #[test]
    fn test_phi() {
        // the following test inputs are obtained from the sage code with an input string s = "input to the test function"
        let t1 = MontFp!( "147370668475511062768593417078575852502166305238356083047569242797625942237381383297554976390154627247147926493198");
        let t2 = MontFp!( "224774355318043699772479778485840064101168681398573284663454398463891850089724106885361203908127740164911167151215");

        let num0 = MontFp!( "234449642914633392584521837562757607648909920067153793849353346054582237272063982518508425009724223737848316641755");
        let num1 = MontFp!( "160423037891784530716005957016289640841111948835936426937484387262516599348586141547616157165397722747451244936808");
        let num2 = MontFp!( "200854173085308118897803307825269596310366771608609042760418432649291337856319134054493677223219947068309205676416");
        let den = MontFp!( "78398001865787854177025635014529777727601615001869942467487640632949237780287612570680483119195173304728124338625");

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
        let t1 = MontFp!( "147370668475511062768593417078575852502166305238356083047569242797625942237381383297554976390154627247147926493198");
        let t2 = MontFp!( "224774355318043699772479778485840064101168681398573284663454398463891850089724106885361203908127740164911167151215");

        let res = <Config as IndifferentiableHash>::eta(s);
        assert_eq!(res[0], t1);
        assert_eq!(res[1], t2);
    }

    #[test]
    fn test_h_prime() {
        // the following test inputs are obtained from the sage code with an input string s = "input to the test function"
        let t1 = MontFp!( "147370668475511062768593417078575852502166305238356083047569242797625942237381383297554976390154627247147926493198");
        let t2 = MontFp!( "224774355318043699772479778485840064101168681398573284663454398463891850089724106885361203908127740164911167151215");

        let num0 = MontFp!( "234449642914633392584521837562757607648909920067153793849353346054582237272063982518508425009724223737848316641755");
        let num1 = MontFp!( "160423037891784530716005957016289640841111948835936426937484387262516599348586141547616157165397722747451244936808");
        let num2 = MontFp!( "200854173085308118897803307825269596310366771608609042760418432649291337856319134054493677223219947068309205676416");
        let den = MontFp!( "78398001865787854177025635014529777727601615001869942467487640632949237780287612570680483119195173304728124338625");

        let x = MontFp!( "88447843811798607965089937473865912423924078263559752807725536262741898732229175112055733585000923536178427677939");
        let y = MontFp!( "139324808532316606671650275155567853806912817623105000585824704086139150798338823307830046341449999254302587526332");

        let res = <Config as IndifferentiableHash>::h_prime(&[num0, num1, num2, den, t1, t2]);
        assert_eq!(x, res.x);
        assert_eq!(y, res.y);
    }

    #[test]
    fn test_map() {
        // the following test inputs are obtained from the sage code with an input string s = "input to the test function"
        let s = "input to the test function";

        let x = MontFp!( "88447843811798607965089937473865912423924078263559752807725536262741898732229175112055733585000923536178427677939");
        let y = MontFp!( "139324808532316606671650275155567853806912817623105000585824704086139150798338823307830046341449999254302587526332");

        let res = <Config as IndifferentiableHash>::hash_to_curve_unchecked(s);
        assert_eq!(x, res.x);
        assert_eq!(y, res.y);

        assert!(res.is_on_curve());
    }

    #[test]
    fn check_test_vectors() {
        let test_vectors = bls12_377_test();
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
