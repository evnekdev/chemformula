//lib.rs

//! This crate helps to parse simple chemical formulas. Its original purpose was to facilitate composition calculations for thermochemical calculations using ChemApp, but it might be useful for general purposes as well. The parser is built using [`nom`] crate functionality. For now, the parsing functionality is limited to parsing simple formulas without parentheses and with float coefficients for elements.
//! 
//! Main functionality and structures:
//! `Element` structure represents a chemical element from the Periodic table + two pseudoelements for chemical thermodynamics (charge compensation electrons and site vacancies).
//! `Formula` a parsed formula instance: elements + coefficients + formula charge; implement basis arithmetic operations, you can add formulas together and multiply by a coefficient.
//! `Transform` - a structure to transform between two formula basis sets, for example, from ["CaO", "Al2O3", "SiO2"] to ["Ca", "Al", "Si", "O"]


pub mod element;
pub mod formula;
pub mod transform;
pub mod parse;

pub use crate::formula::{Formula};
pub use crate::element::{Element};
pub use crate::transform::{Transform};

