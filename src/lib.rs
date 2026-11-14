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
pub use signal_energy_nexus::{
    Baseline, Caller, Minutes, Name, Override, Percent, Provider, Reminder, ResetId, Span,
    Target, Threshold,
};
wire! { pub struct ProviderTarget { pub provider: Provider, pub target: Target } }
wire! { pub struct WindowTarget { pub provider: Provider, pub name: Name, pub target: Target } }
wire! { pub enum Program { Codex, Flow } }
wire! { pub struct Path(pub String); }
wire! { pub struct ProgramPath { pub program: Program, pub path: Path } }
wire! { pub enum Set { Threshold(Threshold), Reminder(Reminder), RemoveReminder(Reminder), RedeemExpiring(Minutes), RedeemUsedAtLeast(Percent), Target(ProviderTarget), WindowTarget(WindowTarget), Program(ProgramPath), Credentials(Path), Span(Span), RemoveSpan(Span), Override(Override), RemoveOverride(Override), Baseline(Baseline) } }
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
    use ethos_zero::{Actualizing, File, Potential, Validating};
    use signal::{ByteViewable, Restorable, Signal, Signalizable};
    #[test]
    fn validated_binary_round_trip() {
        let value = Call {
            caller: Caller {
                process_id: 1,
                user_id: 2,
            },
            input: Query::Configure(Set::RedeemExpiring(Minutes(720))),
        };
        let bytes = value.signalize().unwrap();
        let restored = Signal::<Call>::from(bytes.bytes().to_vec())
            .restore()
            .unwrap();
        assert_eq!(restored, value);
    }

    /// The provider polling interval is gone: Energy reads on events, so no
    /// setting names an interval.
    #[cfg(feature = "datom")]
    #[test]
    fn a_polling_interval_no_longer_reads() {
        use datom_codec::{Actualizing, Budget, Potential};
        use protos::ReaderBudget;
        let read = |text: &str| {
            let mut budget = Budget {
                remaining: 256,
                reader: ReaderBudget::try_from(256).unwrap(),
                depth: 0,
                maximum_depth: 32,
            };
            Potential::<Query>::from(text).actualize(&mut budget)
        };
        assert!(read("Configure.Polling.5").is_err());
        assert_eq!(
            read("Configure.RedeemExpiring.720").unwrap(),
            Query::Configure(Set::RedeemExpiring(Minutes(720)))
        );
    }

    /// The schedule's settings read as the vision writes them.
    #[cfg(feature = "datom")]
    #[test]
    fn the_schedule_is_configured_one_setting_at_a_time() {
        use datom_codec::{Actualizing, Budget, Potential};
        use protos::ReaderBudget;
        use signal_energy_nexus::{Hour, Instant, Usefulness, Weekday};
        let read = |text: &str| {
            let mut budget = Budget {
                remaining: 256,
                reader: ReaderBudget::try_from(256).unwrap(),
                depth: 0,
                maximum_depth: 32,
            };
            Potential::<Query>::from(text).actualize(&mut budget).unwrap()
        };
        assert_eq!(
            read("Configure.Baseline.20"),
            Query::Configure(Set::Baseline(Baseline(Percent(20))))
        );
        let span = Span {
            weekday_vector: vec![Weekday::Tuesday, Weekday::Thursday],
            from: Hour(19),
            to: Hour(23),
            usefulness: Usefulness(Percent(100)),
        };
        assert_eq!(
            read("Configure.Span.{ [ Tuesday Thursday ] 19 23 100 }"),
            Query::Configure(Set::Span(span.clone()))
        );
        assert_eq!(
            read("Configure.RemoveSpan.{ [ Tuesday Thursday ] 19 23 100 }"),
            Query::Configure(Set::RemoveSpan(span))
        );
        let dentist = Override {
            from: Instant(1794859200),
            to: Instant(1794866400),
            usefulness: Usefulness(Percent(0)),
        };
        assert_eq!(
            read("Configure.Override.{ 1794859200 1794866400 0 }"),
            Query::Configure(Set::Override(dentist.clone()))
        );
        assert_eq!(
            read("Configure.RemoveOverride.{ 1794859200 1794866400 0 }"),
            Query::Configure(Set::RemoveOverride(dentist))
        );
    }

    #[test]
    fn ethos_declaration_checks() {
        let file = Potential::<File>::from(ETHOS).actualize().unwrap();
        file.validate().unwrap();
    }
}
