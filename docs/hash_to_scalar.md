Hashes the input and outputs scalar

Input can be any structured data that implements [`Digestable`](udigest::Digestable)
trait (see [udigest] crate).

## How it works
It works by instantiating [`HashRng`](rand_hash::HashRng) CSPRNG seeded from provided data.
Then it's used to derive the scalar.

## Security considerations
It's not constant time. It doesn't follow any existing standards for hash to scalar primitive.
