use askama::Template;
use lazy_static::lazy_static;
use std::collections::BTreeMap as Map;
use std::{fmt::Display, time::Instant};
use themelio_structs::{Denom, MICRO_CONVERTER};

mod block;
pub mod homepage;
mod pool;
mod transaction;

pub use block::*;
pub use homepage::*;
pub use pool::*;
pub use transaction::*;

#[derive(serde::Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
// A wrapper for microunit-denominated values
pub struct MicroUnit(pub u128, pub String);
lazy_static! {
    static ref TOOLTIPS_STR: &'static str = include_str!("../tooltips.json");
    pub static ref TOOLTIPS: Map<String, InfoBubble> = serde_json::from_str(&TOOLTIPS_STR).unwrap();
}
impl Display for MicroUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}.{:06} {}",
            self.0 / MICRO_CONVERTER,
            self.0 % MICRO_CONVERTER,
            self.1
        )
    }
}

// A wrapper for calculating rendering times
struct RenderTimeTracer<'a> {
    start_time: Instant,
    label: &'a str,
}

impl<'a> Drop for RenderTimeTracer<'a> {
    fn drop(&mut self) {
        log::debug!(
            "rendering {} took {:?}",
            self.label,
            self.start_time.elapsed()
        );
    }
}

impl<'a> RenderTimeTracer<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            start_time: Instant::now(),
            label,
        }
    }
}
fn friendly_denom(denom: Denom) -> String {
    match denom {
        Denom::Mel => "MEL".into(),
        Denom::Sym => "SYM".into(),
        Denom::Erg => "ERG".into(),
        Denom::Custom(hash) => format!("Custom ({}..)", hex::encode(&hash.0[..5])),
        Denom::NewCoin => "(new denom)".into(),
    }
}

#[derive(Template, serde::Deserialize, serde::Serialize)]
#[template(path = "info-bubble.html", escape = "none")]
pub struct InfoBubble(String);
