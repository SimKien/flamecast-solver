mod embedding;
mod initial_flamecast;
mod neighborhood;

pub use embedding::*;
pub use initial_flamecast::*;
pub use neighborhood::*;

#[cfg(test)]
mod util;

#[cfg(test)]
pub use util::*;

#[cfg(test)]
pub const EPSILON: f64 = 0.000001;
#[cfg(test)]
pub const WEISZFELD_EPSILON: f64 = 0.0000001;
