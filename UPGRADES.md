# Upgrades

## 0.5.0 — signal 11.0.0, signal-energy-nexus 0.5.0 on signal-flow 61.0.0

Branch-only preparation for the Flow 0.54.7 rollout; deployed with it, in the same Home generation, with signal-energy-nexus 0.5.0 and energy-nexus 0.3.2.

Repins signal 10.3.0 to 11.0.0 (`6d4eb74`), datom-codec to `09a0526`, protos to `c93b598` and signal-energy-nexus to 0.5.0 (`7ca10eb`), which carries signal-flow 61.0.0 (`4871bb8`, the contract Flow 0.54.7 is built with). `signal` declares `links = "signal"`, so a consumer carries one signal.

This crate's own types, its authored `signal.ethos` and its wire form are unchanged, and so is its contract digest; the Rust types of the recipient it reaches through signal-energy-nexus change (`Address.topic` is the `Topic` newtype). No Ethos output is generated from `signal.ethos` in this repository (it is included as text and checked by a test); nothing to regenerate.

`tests/wire_fixtures.rs` holds the wire against `tests/fixtures/`, made by 9f37cb7 (0.4.0): the rkyv bytes and the datom text of a value of every wire type and every variant, the contract digest, and the greeting and receipt that carry it. They are identical under 0.5.0. The test builds a recipient's `Address`, so signal-flow (`4871bb8`, the revision signal-energy-nexus carries) is a dev-dependency.

## 0.4.0 — no polling interval; signal-energy-nexus 0.4.0

`Set.Polling` is removed: the Energy Nexus reads a provider when a turn ends on it, when a window resets and when a reading is asked for while older than the last turn end, never on an interval, so no setting names one. `Configure.Polling.5` no longer reads, and a `Set` archive no longer carries the variant (every later variant's discriminant moves down by one; the contract digest changes with the authored `signal.ethos`). Pins signal-energy-nexus 0.4.0, which repins signal-flow 57.0.0 at `3d0a4cd`.

## 0.3.0 — signal 10.3.0, datom-codec 0.33.0, protos 0.33.0, signal-energy-nexus 0.3.0

Repins signal 10.3.0 (`7afb`), datom-codec 0.33.0 (`eabe`), protos 0.33.0 (`3c7a`), the dev generator ethos-zero 19.2.0 (`5a5d`) and signal-energy-nexus 0.3.0, whose `Recipient` names a model (signal-flow 57.0.0, `065730f`, the contract Flow 0.53.3 is built with; branch idle-0533-ff3882, not yet a main). The authored `signal.ethos` and this crate's own types are unchanged; the recipient reaches this contract through the re-export.

Known and inherited: signal-flow 57.0.0 and its flow-ethos list ethos-zero 17.0.0 (`07714b0`) as a build dependency, which carries protos 0.32.2 and datom-codec 0.32.2 into the lock beside the 0.33.0 pair.
