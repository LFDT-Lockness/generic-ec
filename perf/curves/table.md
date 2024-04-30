|                         | secp256k1 | secp256r1 | stark    | ed25519 |
|-------------------------|-----------|-----------|----------|---------|
| A+B                     | 300ns     | 453ns     | 505ns    | 215ns   |
| \[k\]P                  | 52.5μs    | 140.8μs   | 153.5μs  | 36.6μs  |
| SmallFactorCheck        | 8ns       | 5ns       | 6ns      | 36606ns |
| EncodeCompressedPoint   | 5.8μs     | 21.1μs    | 14.9μs   | 3.5μs   |
| EncodeUncompressedPoint | 5.8μs     | 21.1μs    | 14.9μs   | 3.5μs   |
| DecodeCompressedPoint   | 5.7μs     | 7.0μs     | 2599.6μs | 4.0μs   |
| DecodeUncompressedPoint | 129ns     | 225ns     | 336ns    | 3966ns  |
| a+b                     | 11ns      | 11ns      | 11ns     | 35ns    |
| a*b                     | 41ns      | 58ns      | 29ns     | 98ns    |
| inv(a)                  | 13.2μs    | 21.9μs    | 8.3μs    | 11.4μs  |
| RandomScalar            | 23ns      | 23ns      | 1803ns   | 146ns   |
| EncodeScalarBE          | 8ns       | 8ns       | 23ns     | 5ns     |
| EncodeScalarLE          | 12ns      | 12ns      | 22ns     | 3ns     |
| DecodeScalarBE          | 5ns       | 5ns       | 2853ns   | 114ns   |
| DecodeScalarLE          | 4ns       | 4ns       | 2852ns   | 112ns   |
| BeBytesModOrder/32      | 30ns      | 30ns      | 2811ns   | 79ns    |
| LeBytesModOrder/32      | 29ns      | 29ns      | 2807ns   | 77ns    |
| BeBytesModOrder/64      | 59ns      | 121ns     | 8423ns   | 148ns   |
| LeBytesModOrder/64      | 46ns      | 120ns     | 8416ns   | 140ns   |
| BeBytesModOrder/128     | 184ns     | 264ns     | 14077ns  | 566ns   |
| LeBytesModOrder/128     | 166ns     | 261ns     | 14066ns  | 544ns   |
| BeBytesModOrder/512     | 691ns     | 1126ns    | 48027ns  | 2143ns  |
| LeBytesModOrder/512     | 626ns     | 1111ns    | 47862ns  | 2065ns  |
| ReduceBe/32             | 13ns      | 13ns      | 2745ns   | 63ns    |
| ReduceLe/32             | 12ns      | 12ns      | 2734ns   | 61ns    |
| ReduceBe/64             | 42ns      |           |          | 130ns   |
| ReduceLe/64             | 28ns      |           |          | 124ns   |
