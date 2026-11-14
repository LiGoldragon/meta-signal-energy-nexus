//! The hand-written meta contract of 0.5.0 (3264d6f), with the ordinary
//! types it carried from signal-energy-nexus 0.5.0 (7ca10eb), against the
//! contract ethos-zero now generates. `hand` is the two 0.5.0 `src/lib.rs`
//! declarations it needs, frozen as they stood. A value of every type and
//! of every variant is archived through the hand type; its bytes are read as
//! the generated type and archived again, and the two archives must be one;
//! the generated archive must read back as the hand value; and under
//! `datom` the two texts must be one. The contract digest is compared last:
//! it is the hash of the authored Ethos text, and the text had to change,
//! since ethos-zero 28 refuses the 0.5.0 text (`Duplicate.Input`).

use signal::Contracted;
use std::fmt::Debug;

#[allow(non_camel_case_types, dead_code)]
mod hand {
    macro_rules! wire {
        ($item:item) => {
            #[derive(
                rkyv::Archive,
                rkyv::Serialize,
                rkyv::Deserialize,
                Clone,
                Debug,
                PartialEq,
                Eq,
                Hash,
            )]
            #[cfg_attr(
                feature = "datom",
                derive(datom_codec::Datomizable, datom_codec::Composing)
            )]
            $item
        };
    }
    // signal-energy-nexus 0.5.0 (7ca10eb)
    wire! { pub enum Provider { Claude, Codex } }
    wire! { pub struct Percent(pub i64); }
    wire! { pub struct Instant(pub i64); }
    wire! { pub struct Minutes(pub i64); }
    wire! { pub enum Name { Session, Week, FableWeek } }
    wire! { pub enum Level { Plenty, Fair, Low, Out } }
    wire! { pub struct Threshold { pub level: Level, pub left: Percent } }
    pub use signal_flow::Address;
    wire! { pub struct Recipient(pub Address); }
    wire! { pub struct Reminder { pub provider: Provider, pub level: Level, pub recipient: Recipient } }
    wire! { pub enum Target { Even, SpendBy(Instant) } }
    wire! { pub struct Caller { pub process_id: i64, pub user_id: i64 } }
    // meta-signal-energy-nexus 0.5.0 (3264d6f)
    wire! { pub struct ProviderTarget { pub provider: Provider, pub target: Target } }
    wire! { pub struct WindowTarget { pub provider: Provider, pub name: Name, pub target: Target } }
    wire! { pub enum Program { Codex, Flow } }
    wire! { pub struct Path(pub String); }
    wire! { pub struct ProgramPath { pub program: Program, pub path: Path } }
    wire! { pub enum Set { Threshold(Threshold), Reminder(Reminder), RemoveReminder(Reminder), RedeemExpiring(Minutes), RedeemUsedAtLeast(Percent), Target(ProviderTarget), WindowTarget(WindowTarget), Program(ProgramPath), Credentials(Path) } }
    wire! { pub enum Query { Configure(Set) } }
    wire! { pub struct Call { pub caller: Caller, pub input: Query } }
    wire! { pub enum Response { Configured(Set), ConfigureRefused(String) } }
    pub const ETHOS: &str = include_str!("fixtures/signal-0.5.0.ethos");
    impl signal::Contracted for Call {
        const CONTRACT_SOURCE: &'static str = ETHOS;
    }
}

use meta_signal_energy_nexus as generated;

type Strict = rkyv::rancor::Error;

/// Every difference found, by the name of the case.
struct Proof {
    differences: Vec<String>,
    cases: usize,
}

trait Proving<Hand, Generated> {
    fn prove(&mut self, name: &str, hand: Hand);
}

impl<Hand, Generated> Proving<Hand, Generated> for Proof
where
    Hand: rkyv::Archive
        + for<'a> rkyv::Serialize<
            rkyv::api::high::HighSerializer<
                rkyv::util::AlignedVec,
                rkyv::ser::allocator::ArenaHandle<'a>,
                Strict,
            >,
        > + PartialEq
        + Debug
        + Datom,
    Hand::Archived: for<'a> rkyv::bytecheck::CheckBytes<rkyv::api::high::HighValidator<'a, Strict>>
        + rkyv::Deserialize<Hand, rkyv::api::high::HighDeserializer<Strict>>,
    Generated: rkyv::Archive
        + for<'a> rkyv::Serialize<
            rkyv::api::high::HighSerializer<
                rkyv::util::AlignedVec,
                rkyv::ser::allocator::ArenaHandle<'a>,
                Strict,
            >,
        > + Debug
        + Datom,
    Generated::Archived: for<'a> rkyv::bytecheck::CheckBytes<rkyv::api::high::HighValidator<'a, Strict>>
        + rkyv::Deserialize<Generated, rkyv::api::high::HighDeserializer<Strict>>,
{
    fn prove(&mut self, name: &str, hand: Hand) {
        self.cases += 1;
        let hand_bytes = rkyv::to_bytes::<Strict>(&hand).unwrap();
        let generated = match rkyv::from_bytes::<Generated, Strict>(&hand_bytes) {
            Ok(value) => value,
            Err(error) => {
                self.differences
                    .push(format!("{name}: the hand bytes do not read as the generated type: {error}"));
                return;
            }
        };
        let generated_bytes = rkyv::to_bytes::<Strict>(&generated).unwrap();
        if hand_bytes.as_slice() != generated_bytes.as_slice() {
            self.differences.push(format!(
                "{name}: hand {:02x?} generated {:02x?}",
                hand_bytes.as_slice(),
                generated_bytes.as_slice()
            ));
        }
        match rkyv::from_bytes::<Hand, Strict>(&generated_bytes) {
            Ok(back) if back == hand => {}
            Ok(back) => self
                .differences
                .push(format!("{name}: the generated bytes read back as {back:?}, not {hand:?}")),
            Err(error) => self
                .differences
                .push(format!("{name}: the generated bytes do not read as the hand type: {error}")),
        }
        let (hand_text, generated_text) = (hand.text(), generated.text());
        if hand_text != generated_text {
            self.differences.push(format!(
                "{name}: hand text {hand_text:?} generated text {generated_text:?}"
            ));
        }
    }
}

/// A value's datom text, under the `datom` feature; nothing without it.
trait Datom {
    fn text(&self) -> Option<String>;
}

#[cfg(feature = "datom")]
impl<T: datom_codec::Datomizable> Datom for T {
    fn text(&self) -> Option<String> {
        use protos::{Compactable, Protosizable};
        Some(self.datomize(Vec::new()).protosize().compact())
    }
}

#[cfg(not(feature = "datom"))]
impl<T> Datom for T {
    fn text(&self) -> Option<String> {
        None
    }
}

fn address(aspect: signal_flow::Aspect, model: signal_flow::Model) -> hand::Address {
    hand::Address {
        aspect,
        topic: signal_flow::Topic("energyNexus".to_string()),
        model,
    }
}

fn reminder(provider: hand::Provider, level: hand::Level, model: signal_flow::Model) -> hand::Reminder {
    hand::Reminder {
        provider,
        level,
        recipient: hand::Recipient(address(signal_flow::Aspect::Mind, model)),
    }
}

fn path() -> hand::Path {
    hand::Path("/home/li/.codex/auth.json".to_string())
}

fn sets() -> Vec<hand::Set> {
    use hand::{Level, Provider, Target};
    let mut sets = Vec::new();
    for level in [Level::Plenty, Level::Fair, Level::Low, Level::Out] {
        sets.push(hand::Set::Threshold(hand::Threshold {
            level: level.clone(),
            left: hand::Percent(40),
        }));
        sets.push(hand::Set::Reminder(reminder(Provider::Codex, level.clone(), signal_flow::Model::Sol)));
        sets.push(hand::Set::RemoveReminder(reminder(Provider::Claude, level, signal_flow::Model::Fable)));
    }
    sets.push(hand::Set::RedeemExpiring(hand::Minutes(720)));
    sets.push(hand::Set::RedeemUsedAtLeast(hand::Percent(90)));
    for provider in [Provider::Claude, Provider::Codex] {
        for target in [Target::Even, Target::SpendBy(hand::Instant(1_789_200_000))] {
            sets.push(hand::Set::Target(hand::ProviderTarget {
                provider: provider.clone(),
                target: target.clone(),
            }));
            for name in [hand::Name::Session, hand::Name::Week, hand::Name::FableWeek] {
                sets.push(hand::Set::WindowTarget(hand::WindowTarget {
                    provider: provider.clone(),
                    name,
                    target: target.clone(),
                }));
            }
        }
    }
    for program in [hand::Program::Codex, hand::Program::Flow] {
        sets.push(hand::Set::Program(hand::ProgramPath { program, path: path() }));
    }
    sets.push(hand::Set::Credentials(path()));
    sets
}

macro_rules! prove {
    ($proof:expr, $name:expr, $hand:ty, $generated:ty, $value:expr) => {
        <Proof as Proving<$hand, $generated>>::prove(&mut $proof, &$name, $value)
    };
}

#[test]
fn every_hand_type_archives_as_its_generated_type() {
    let mut proof = Proof {
        differences: Vec::new(),
        cases: 0,
    };
    let caller = hand::Caller {
        process_id: 4242,
        user_id: 1001,
    };
    prove!(proof, "Caller", hand::Caller, generated::Caller, caller.clone());
    for program in [hand::Program::Codex, hand::Program::Flow] {
        prove!(proof, format!("Program.{program:?}"), hand::Program, generated::Program, program.clone());
        prove!(
            proof,
            format!("ProgramPath→Program_Data.{program:?}"),
            hand::ProgramPath,
            generated::Program_Data,
            hand::ProgramPath { program, path: path() }
        );
    }
    prove!(proof, "Path", hand::Path, generated::Path, path());
    for set in sets() {
        let name = format!("{set:?}");
        if let hand::Set::Target(target) = &set {
            prove!(proof, format!("ProviderTarget.{name}"), hand::ProviderTarget, generated::ProviderTarget, target.clone());
        }
        if let hand::Set::WindowTarget(target) = &set {
            prove!(proof, format!("WindowTarget.{name}"), hand::WindowTarget, generated::WindowTarget, target.clone());
        }
        prove!(proof, format!("Set.{name}"), hand::Set, generated::Set, set.clone());
        prove!(proof, format!("Query→Input.{name}"), hand::Query, generated::Input, hand::Query::Configure(set.clone()));
        prove!(
            proof,
            format!("Call.{name}"),
            hand::Call,
            generated::Call,
            hand::Call {
                caller: caller.clone(),
                input: hand::Query::Configure(set.clone())
            }
        );
        prove!(proof, format!("Response→Output.Configured.{name}"), hand::Response, generated::Output, hand::Response::Configured(set));
    }
    prove!(
        proof,
        "Response→Output.ConfigureRefused",
        hand::Response,
        generated::Output,
        hand::Response::ConfigureRefused("the program is not installed".to_string())
    );
    assert!(
        proof.differences.is_empty(),
        "{} of {} cases differ:\n{}",
        proof.differences.len(),
        proof.cases,
        proof.differences.join("\n")
    );
    eprintln!("{} cases: every hand archive is the generated archive", proof.cases);
}

/// The digest is the hash of the authored Ethos text. It is the one part of
/// the wire that moves: a 0.5.0 peer is refused at its greeting.
#[test]
fn the_contract_digest_moves_with_the_ethos_text() {
    let hand = hand::Call::contract_digest();
    let generated = generated::Call::contract_digest();
    eprintln!("ContractDigest 0.5.0 {} generated {}", hand.0, generated.0);
    assert_ne!(hand.0, generated.0, "the 0.5.0 text and the generated contract's text are two texts");
    assert_eq!(hand.0, -4620172360499517031, "the 0.5.0 digest, as tests/fixtures/contract.txt held it");
}
