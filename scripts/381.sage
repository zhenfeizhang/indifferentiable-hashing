# credit: Dmitrii Koshelev
# https://github.com/dishport/Indifferentiable-hashing-to-ordinary-elliptic-curves-of-j-0-with-the-cost-of-one-exponentiation

# Dmitrii Koshelev (the author of the code) was supported by Web3 Foundation (W3F).
# Throughout the code the notation is consistent with author's article
# [1] "Indifferentiable hashing to ordinary elliptic Fq-curves of j = 0 with the cost of one exponentiation in Fq",
# Designs, Codes and Cryptography, 90:3 (2022), 801-812.

import hashlib
import random
import string

# We assume that the finite field order q != 1 (mod 27) and
# b is a cubic (resp. quadratic) non-residue (resp. residue) in Fq.
# Parameters for the BLS12-381 curve Eb: y^2 = x^3 + b:
u = -0xd201000000010000
r = u^4 - u^2 + 1
q = ((u - 1)^2 * r) // 3 + u
assert( ceil(log(q,2).n()) == 381 )
assert(q.is_prime())
assert(q % 27 == 10)
m = (q - 10) // 27

Fq = GF(q)
sb = Fq(2)   # sqrt(b)
b = sb^2
w = b^((q-1) // 3)   # the cubic residue symbol
assert(w != 1)   # w is a primitive 3rd root of unity
w2 = w^2
z = w.nth_root(3)   # z (i.e., zeta in [1, Section 3]) is a primitive 9th root of unity
z2 = z^2
c1 = (b/z).nth_root(3)
c2 = c1^2


##############################################################################


# In [1, Section 2] we deal with a Calabi-Yau threefold defined as
# the quotient T := Eb x Eb' x Eb'' / [w] x [w] x [w],
# where Eb', Eb'' are the cubic twists of Eb
# and [w](x, y) -> (wx, y) is an automorphism of order 3 on Eb, Eb', and Eb''.
# Auxiliary map h': T(Fq) -> Eb(Fq):
def hPrime(num0,num1,num2,den, t1,t2):
	v = den^2
	u = num0^2 - b*v
	v2 = v^2
	v4 = v2^2
	v8 = v4^2
	v9 = v*v8
	v16 = v8^2
	v25 = v9*v16
	u2 = u^2
	th = u*v8*(u2*v25)^m   # theta from [1, Section 3]

	v = th^3*v
	v3 = v^3
	u3 = u*u2
	L = [t1, w*t1, w2*t1]
	L.sort()
	n = L.index(t1)

	if v3 == u3:
		X = w^n*th
		if v == u:
			Y = 1; Z = 1
		if v == w*u:
			Y = z; Z = z
		if v == w2*u:
			Y = z2; Z = z2
		Y = Y*num0
	if v3 == w*u3:
		X = c1*th*t1
		zu = z*u
		if v == zu:
			Y = 1; Z = 1
		if v == w*zu:
			Y = z; Z = z
		if v == w2*zu:
			Y = z2; Z = z2
		Y = Y*num1
	if v3 == w2*u3:
		X = c2*th*t2
		z2u = z2*u
		if v == z2u:
			Y = 1; Z = 1
		if v == w*z2u:
			Y = z; Z = z
		if v == w2*z2u:
			Y = z2; Z = z2
		Y = Y*num2
	# elif is not used to respect constant-time execution

	X = X*den
	Z = Z*den
	return X,Y,Z


# [1, Lemma 1] states that T is given in the affine space A^5(y0,y1,y2,t1,t2) by the two equations
# y1^2 - b = b*(y0^2 - b)*t1^3,
# y2^2 - b = b^2*(y0^2 - b)*t2^3,
# where tj := xj/x0.
# The threefold T can be regarded as an elliptic curve in A^3(y0,y1,y2) over the function field F := Fq(s1,s2),
# where sj := tj^3.
# By virtue of [1, Theorem 2] the non-torsion part of the Mordell-Weil group T(F) is generated by phi from [1, Theorem 1].
# Rational map phi: (Fq)^2 -> T(Fq):
def phi(t1,t2):
	s1 = t1^3
	s2 = t2^3
	s1s1 = s1^2
	s2s2 = s2^2
	global s1s2
	s1s2 = s1*s2

	b2 = b^2
	b3 = b*b2
	b4 = b2^2
	a20 = b2*s1s1
	a11 = 2*b3*s1s2
	a10 = 2*b*s1
	a02 = b4*s2s2
	a01 = 2*b2*s2

	# yi = numi/den
	num0 = sb*(a20 - a11 + a10 + a02 + a01 - 3)
	num1 = sb*(-3*a20 + a11 + a10 + a02 - a01 + 1)
	num2 = sb*(a20 + a11 - a10 - 3*a02 + a01 + 1)
	den = a20 - a11 - a10 + a02 - a01 + 1
	return num0,num1,num2,den


# Map h: (Fq)^2 -> Eb(Fq)
def h(t1,t2):
	num0,num1,num2,den = phi(t1,t2)
	X,Y,Z = hPrime(num0,num1,num2,den, t1,t2)
	if s1s2 == 0:
		X = 0; Y = sb; Z = 1
	# Without loss of the admissibility property, h can return any other Fq-point on Eb in the case s1s2 == 0 (see [1, Section 4])
	if den == 0:
		X = 0; Y = 1; Z = 0
	return X,Y,Z


# Indifferentiable hash function eta: {0,1}* -> (Fq)^2
def eta(s):
	s = s.encode("utf-8")
	s0 = s + b'0'
	s1 = s + b'1'
	# 512 > 510 = 381 + 128 + 1, hence sha512 provides the 128-bit security level
	# in according to Lemma 14 of the article
	# Brier E., et al.: Efficient indifferentiable hashing into ordinary elliptic curves.
	# In: Rabin T. (ed) Advances in Cryptology - CRYPTO 2010, LNCS, 6223, pp. 237-254. Springer, Berlin (2010).
	hash0 = hashlib.sha512(s0).hexdigest()
	hash0 = int(hash0, base=16)
	hash1 = hashlib.sha512(s1).hexdigest()
	hash1 = int(hash1, base=16)
	return Fq(hash0), Fq(hash1)


# Resulting hash function H: {0,1}* -> Eb(Fq)
def H(s):
	t1,t2 = eta(s)
	return h(t1,t2)


##############################################################################


# Main
# symbols = string.ascii_letters + string.digits
# length = random.randint(0,50)
# s = ''.join( random.choices(symbols, k=length) )
# Eb = EllipticCurve(Fq, [0,b])
# X,Y,Z = H(s)
# print( f"\nH({s})   =   ({X} : {Y} : {Z})   =   {Eb(X,Y,Z)}\n" )
