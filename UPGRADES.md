# Upgrades

## 0.4.0 — no polling interval; signal-energy-nexus 0.4.0

`Set.Polling` is removed: the Energy Nexus reads a provider when a turn ends on it, when a window resets and when a reading is asked for while older than the last turn end, never on an interval, so no setting names one. `Configure.Polling.5` no longer reads, and a `Set` archive no longer carries the variant (every later variant's discriminant moves down by one; the contract digest changes with the authored `signal.ethos`). Pins signal-energy-nexus 0.4.0, which repins signal-flow 57.0.0 at `3d0a4cd`.

## 0.3.0 — signal 10.3.0, datom-codec 0.33.0, protos 0.33.0, signal-energy-nexus 0.3.0

Repins signal 10.3.0 (`7afb`), datom-codec 0.33.0 (`eabe`), protos 0.33.0 (`3c7a`), the dev generator ethos-zero 19.2.0 (`5a5d`) and signal-energy-nexus 0.3.0, whose `Recipient` names a model (signal-flow 57.0.0, `065730f`, the contract Flow 0.53.3 is built with; branch idle-0533-ff3882, not yet a main). The authored `signal.ethos` and this crate's own types are unchanged; the recipient reaches this contract through the re-export.

Known and inherited: signal-flow 57.0.0 and its flow-ethos list ethos-zero 17.0.0 (`07714b0`) as a build dependency, which carries protos 0.32.2 and datom-codec 0.32.2 into the lock beside the 0.33.0 pair.
