# Upgrades

## 0.3.0 — signal 10.3.0, datom-codec 0.33.0, protos 0.33.0, signal-energy-nexus 0.3.0

Repins signal 10.3.0 (`7afb`), datom-codec 0.33.0 (`eabe`), protos 0.33.0 (`3c7a`), the dev generator ethos-zero 19.2.0 (`5a5d`) and signal-energy-nexus 0.3.0, whose `Recipient` names a model (signal-flow 51.0, `9f15`, a published branch revision). The authored `signal.ethos` and this crate's own types are unchanged; the recipient reaches this contract through the re-export.

Known and inherited: signal-flow 51.0 lists ethos-zero 17.0.0 (`07714b0`) as a build dependency, which carries protos 0.32.2 and datom-codec 0.32.2 into the lock beside the 0.33.0 pair.
