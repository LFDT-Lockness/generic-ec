## v0.3.0
* Rework `generic_ec::multiscalar` API, optimize Straus algorithm, add Dalek, remove
  Pippenger [#30]
* Optimize `Scalar::from_{be|le}_bytes_mod_order` [#34]
* Remove `hash_to_curve` primitive from library API [#34]

[#30]: https://github.com/dfns/generic-ec/pull/30
[#34]: https://github.com/dfns/generic-ec/pull/34

## v0.2.4
* Add `generic_ec::multiscalar` which helps optimizing multiscalar multiplication [#29]

[#29]: https://github.com/dfns/generic-ec/pull/29

## v0.2.3
* Add `generic_ec::serde::PreferCompact` that serializes points/scalars in compact form,
  but deserialization recognizes both compact and non-compact formats [#28]

[#28]: https://github.com/dfns/generic-ec/pull/28

## v0.2.2
* Implement `serde_with::SerializeAs<&T>` for `generic_ec::serde::Compact` when `T` is
  serializable via `Compact` [#27]

[#27]: https://github.com/dfns/generic-ec/pull/27

## v0.2.1
* Make `generic_ec::serde` module always available even when `serde` feature is disabled [#25]

[#25]: https://github.com/dfns/generic-ec/pull/25

## v0.2.0

All changes prior to this version weren't documented
