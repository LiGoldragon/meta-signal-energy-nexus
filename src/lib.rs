#![allow(non_camel_case_types)]
macro_rules! wire {
    ($item:item) => {
        #[derive(
            rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash,
        )]
        #[cfg_attr(
            feature = "datom",
            derive(datom_codec::Datomizable, datom_codec::Composing)
        )]
        $item
    };
}
pub use signal_energy_nexus::{Caller, Minutes, Percent, Provider, Reminder, ResetId, Target, Threshold};
wire! { pub struct Target_Data { pub provider: Provider, pub target: Target } }
wire! { pub enum Program { Codex, Flow } }
wire! { pub struct Path(pub String); }
wire! { pub struct ProgramPath { pub program: Program, pub path: Path } }
wire! { pub enum Set { Polling(Minutes), Threshold(Threshold), Reminder(Reminder), RemoveReminder(Reminder), RedeemExpiring(Minutes), RedeemUsedAtLeast(Percent), Target(Target_Data), Program(ProgramPath), Credentials(Path) } }
wire! { pub enum Query { Configure(Set) } }
wire! { pub struct Call { pub caller: Caller, pub input: Query } }
wire! { pub enum Response { Configured(Set), ConfigureRefused(String) } }
pub const ETHOS: &str = include_str!("../signal.ethos");
impl signal::Contracted for Call {
    const CONTRACT_SOURCE: &'static str = ETHOS;
}
#[cfg(test)]
mod tests {
    use super::*;
    use signal::{ByteViewable, Restorable, Signal, Signalizable};
    #[test]
    fn validated_binary_round_trip() {
        let value = Call {
            caller: Caller { process_id: 1, user_id: 2 },
            input: Query::Configure(Set::Polling(Minutes(5))),
        };
        let bytes = value.signalize().unwrap();
        let restored = Signal::<Call>::from(bytes.bytes().to_vec())
            .restore()
            .unwrap();
        assert_eq!(restored, value);
    }
}
