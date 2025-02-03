Generates random non-zero scalar

# Guarantees
1. Uniform distribution \
   Output scalar is uniformly distributed (given that provided PRNG outputs uniformly
   distributed bytes), with possible negligible bias respective to curve target security
   level
2. Constant-time \
   Random generation algorithm does not branch on input randomness, and in particular,
   it does not use rejection-sampling strategy (might not be the case with negligible
   probability respective to the curve target security level[^1])
3. Reproducible \
   When random generation algorithm is given the same PRNG with the same seed, it always
   outputs the same scalar on all platforms

If you don't need constant-time/reproducibility properties, you can use [`random_vartime`](
Self::random_vartime) instead which is typically faster.

[^1]: we use rejection-sampling strategy in case if a zero scalar is generated, however,
probability of that happening is negligible

# Panics
Panics if randomness source returned 100 zero scalars in a row. It happens with negligible
probability, e.g. for secp256k1 curve it's about $2^{-25600}$, which practically means that
randomness source is broken.

