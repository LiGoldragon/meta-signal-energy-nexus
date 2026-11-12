//! Energy stage-one meta contracts. Branch energy-stage-one-a9a8d6, owned by Mind research Sol.
#[path = "signal.rs"]
mod contract;
pub mod library;
pub use contract::*;
#[path = "nexus.rs"]
pub mod internal;

pub const ETHOS: &str = concat!(
    include_str!("../Cargo.toml"),
    "\n",
    include_str!("../library.ethos"),
    "\n",
    include_str!("../signal.ethos"),
);
pub const INTERNAL_ETHOS: &str = concat!(
    include_str!("../Cargo.toml"),
    "\n",
    include_str!("../library.ethos"),
    "\n",
    include_str!("../nexus.ethos"),
);
impl signal::Contracted for Call {
    const CONTRACT_SOURCE: &'static str = ETHOS;
}
impl signal::Contracted for internal::Call {
    const CONTRACT_SOURCE: &'static str = INTERNAL_ETHOS;
}
