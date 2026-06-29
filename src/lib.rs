//lib.rs

//! This crate helps to parse simple chemical formulas. Its original purpose was to facilitate composition calculations for thermochemical calculations using ChemApp, but it might be useful for general purposes as well. The parser is built using [`nom`] crate functionality. For now, the parsing functionality is limited to parsing simple formulas without parentheses and with float coefficients for elements.

pub mod element;
pub mod formula;
pub mod transform;
pub mod parse;

pub use crate::formula::{Formula};
pub use crate::element::{Element};
pub use crate::transform::{Transform};

