load("381.sage")


f = open("../src/test_vectors.rs", "w")

f.write("use ark_ff::field_new;")
f.write("use ark_bls12_381::Fq as Fq381;")
f.write("use ark_bls12_377::Fq as Fq377;")
f.write("use ark_ff::Zero;")

f.write("pub(crate) fn bls12_381_test()-> Vec<Fq381> {\n")
f.write("let mut a = vec![Fq381::zero();200];")


for a in range(100):
    X, Y, Z = H(str(a))

    f.write( "a[" + str( a*2 +0) + "] = field_new!(Fq381, \"" + str(X/Z) + "\");" )
    f.write( "\n" )
    f.write( "a[" + str( a*2 +1) + "] = field_new!(Fq381, \"" + str(Y/Z) + "\");" )
    f.write( "\n" )

f.write("a")

f.write("}")

load("377.sage")


f.write("pub(crate) fn bls12_377_test()-> Vec<Fq377> {\n")
f.write("let mut a = vec![Fq377::zero();200];")


for a in range(100):
    X, Y, Z = H(str(a))

    f.write( "a[" + str( a*2 +0) + "] = field_new!(Fq377, \"" + str(X/Z) + "\");" )
    f.write( "\n" )
    f.write( "a[" + str( a*2 +1) + "] = field_new!(Fq377, \"" + str(Y/Z) + "\");" )
    f.write( "\n" )

f.write("a")

f.write("}")



f.close()


