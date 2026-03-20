## v0.5.0
* Hash to scalar: guarantee reproducibility on all platforms [#55]
* Use reproducible & constant-time algorithm for random scalar
  generation [#55]
* Add secp384r1 curve [#59]
* Update `generic-ec-core` & `generic-ec-curves` to v0.3 [#55]

[#55]: https://github.com/LFDT-Lockness/generic-ec/pull/55
[#59]: https://github.com/LFDT-Lockness/generic-ec/pull/59

## v0.4.5
* Add curve-specific aliases (see `generic_ec::curves::{secp256k1, secp256r1, stark, ed25519}::*`) [#52]
* Implement infallible `FromRaw` for `Point<E>` conversion for prime order curves
  such as `secp256k1`, `secp256r1` and `stark` [#50]

[#52]: https://github.com/LFDT-Lockness/generic-ec/pull/52
[#50]: https://github.com/LFDT-Lockness/generic-ec/pull/50

## v0.4.4
* Fix double header menu issue on docs.rs [#49]

[#49]: https://github.com/LFDT-Lockness/generic-ec/pull/49

## v0.4.3
* Add `Point::serialized_len`

## v0.4.2
* Update links, add info about our discord [#44]

[#44]: https://github.com/LFDT-Lockness/generic-ec/pull/44

## v0.4.1
* Fix `no_std` support [#43]

[#43]: https://github.com/LFDT-Lockness/generic-ec/pull/43

## v0.4.0
* Update `udigest` to v0.2 [#40]
* Add hash to scalar primitive [#40]

[#40]: https://github.com/LFDT-Lockness/generic-ec/pull/40

## v0.3.0
* Rework `generic_ec::multiscalar` API, optimize Straus algorithm, add Dalek, remove
  Pippenger [#30]
* Optimize `Scalar::from_{be|le}_bytes_mod_order` [#34]
* Remove `hash_to_curve` primitive from library API [#34]
* Add `Scalar::is_zero` method [#35]

[#30]: https://github.com/LFDT-Lockness/generic-ec/pull/30
[#34]: https://github.com/LFDT-Lockness/generic-ec/pull/34
[#35]: https://github.com/LFDT-Lockness/generic-ec/pull/35

## v0.2.4
* Add `generic_ec::multiscalar` which helps optimizing multiscalar multiplication [#29]

[#29]: https://github.com/LFDT-Lockness/generic-ec/pull/29

## v0.2.3
* Add `generic_ec::serde::PreferCompact` that serializes points/scalars in compact form,
  but deserialization recognizes both compact and non-compact formats [#28]

[#28]: https://github.com/LFDT-Lockness/generic-ec/pull/28

## v0.2.2
* Implement `serde_with::SerializeAs<&T>` for `generic_ec::serde::Compact` when `T` is
  serializable via `Compact` [#27]

[#27]: https://github.com/LFDT-Lockness/generic-ec/pull/27

## v0.2.1
* Make `generic_ec::serde` module always available even when `serde` feature is disabled [#25]

[#25]: https://github.com/LFDT-Lockness/generic-ec/pull/25

## v0.2.0

All changes prior to this version weren't documented
