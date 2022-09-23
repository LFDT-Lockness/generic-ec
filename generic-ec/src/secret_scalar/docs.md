Scalar representing sensitive information (like secret key)

Secret scalar should be treated with an extra care. You shouldn't do any
branching (e.g. `Eq`, `Ord`) on the secret to avoid timing side-channel
attacks, so it implements only constant time traits (like [`ConstantTimeEq`]).

Also, when `alloc` feature is enabled, we enforce extra measures:

* Secret scalar leaves no trace in RAM after it's dropped \
  Memory is zeroized after use
* All clones of secret scalar refer to the same region in the memory \
  I.e. there will always be only one instance of the scalar in the memory
  no matter how many clones you make

All these guarantees can be bypassed by calling `.as_ref()` and obtaining
`&Scalar<E>` that is not protected from timing attacks, leaving traces in
the memory, etc.

[`ConstantTimeEq`]: subtle::ConstantTimeEq
