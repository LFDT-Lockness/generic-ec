Generates random non-zero scalar using variable time algorithm

# Guarantees
1. Uniform distribution \
   Output scalar is uniformly distributed (given that provided PRNG outputs uniformly
   distributed bytes), with possible negligible bias respective to curve target security
   level

Unlike [`random`](Self::random) method, this one **is not** constant-time nor reproducible, but
often it's faster.

# Panics
Panics if randomness source returned 100 zero scalars in a row. It happens with negligible
probability, e.g. for secp256k1 curve it's about $2^{-25600}$, which practically means that
randomness source is broken.

