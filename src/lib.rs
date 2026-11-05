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
pub use signal_energy_nexus::{Minutes, Percent, Reminder, ResetId, Threshold};
wire! { pub enum Set { Polling(Minutes), Threshold(Threshold), Reminder(Reminder), RemoveReminder(Reminder), RedeemExpiring(Minutes), RedeemUsedAtLeast(Percent) } }
wire! { pub enum Query { Configure(Set) } }
wire! { pub enum Response { Configured(Set), ConfigureRefused(String) } }
pub const ETHOS: &str = include_str!("../signal.ethos");
impl signal::Contracted for Query {
    const CONTRACT_SOURCE: &'static str = ETHOS;
}
#[cfg(test)]
mod tests {
    use super::*;
    use signal::{ByteViewable, Restorable, Signal, Signalizable};
    #[test]
    fn validated_binary_round_trip() {
        let value = Query::Configure(Set::Polling(Minutes(5)));
        let bytes = value.signalize().unwrap();
        let restored = Signal::<Query>::from(bytes.bytes().to_vec())
            .restore()
            .unwrap();
        assert_eq!(restored, value);
    }
}
