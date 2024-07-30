//lib.rs

//! This crate helps to parse simple chemical formulas for ChemApp datafiles. The parser is built using [`nom`] crate functionality. For now, the parsing functionality is limited to parsing simple formulas without parentheses and with integer coefficients for elements.

pub mod element;
pub mod formula;
pub mod transform;
pub mod parse;

pub use crate::formula::{Formula};
pub use crate::element::{Element};
pub use crate::transform::{Transform};

use std::fmt;
use std::str::FromStr;
use std::collections::{BTreeSet};
use std::convert::From;

use nalgebra::{DVector, dvector, DMatrix, dmatrix};




/*************************************************************************************************/
/*************************************************************************************************/
/// Derives an ordered set of all elements occurring in a vector of [`Formula`] structures.
fn element_set(basis: &[Formula])->BTreeSet<Element>{
	let mut eset : BTreeSet<Element> = BTreeSet::new();
	for k in 0..basis.len(){
		for (key, _) in basis[k].pairs.iter(){
			eset.insert(key.clone());
		}
	}
	return eset;
}

/// Builds a conversion matrix (as a [`DMatrix<f64>`]) for conversion from one set of [`Formula`] to another one.
pub fn conversion_matrix_f(basis_from: &[Formula], basis_to: &[Formula])->DMatrix<f64>{
	//let eset_from = element_set(basis_from);
	//let eset_to   = element_set(basis_to);
	let mut eset : BTreeSet<Element> = BTreeSet::new();
	for k in 0..basis_from.len(){
		for (key, _) in basis_from[k].pairs.iter(){
			eset.insert(key.clone());
		}
	}
	for k in 0..basis_to.len(){
		for (key, _) in basis_to[k].pairs.iter(){
			eset.insert(key.clone());
		}
	}
	let eset : Vec<Element> = eset.into_iter().collect();
	//println!("eset = {:?}", &eset);
	let mut mfrom : DMatrix<f64> = DMatrix::zeros(eset.len(),basis_from.len());
	let mut mto   : DMatrix<f64> = DMatrix::zeros(eset.len(),basis_to.len());
	/**********filling in***********/
	for k in 0..eset.len(){
		for m in 0..basis_from.len(){
			mfrom[(k,m)] = basis_from[m].coeff(&eset[k]) as f64;
		}
	}
	for k in 0..eset.len(){
		for m in 0..basis_to.len(){
			mto[(k,m)] = basis_to[m].coeff(&eset[k]) as f64;
		}
	}
	//println!("mfrom = {:?}", &mfrom);
	let mto_t = mto.transpose();
	let mm = &mto_t * &mto;
	let mminv = mm.pseudo_inverse(1e-8).unwrap();
	let res   = &mminv * &mto_t * &mfrom;
	//println!("res = {:?}", &res);
	return res;
}

/// Builds a conversion matrix (as a [`DMatrix<f64>`]) for conversion from one set of [`&str`] to another one.
pub fn conversion_matrix_s<T1: AsRef<str>, T2: AsRef<str>>(basis_from: &[T1], basis_to: &[T2])->DMatrix<f64>{
	let mut formulas_from : Vec<Formula> = Vec::new();
	let mut formulas_to   : Vec<Formula> = Vec::new();
	for k in 0..basis_from.len() {
		formulas_from.push(Formula::from_str(basis_from[k].as_ref()).unwrap());
	}
	for k in 0..basis_to.len() {
		formulas_to.push(Formula::from_str(basis_to[k].as_ref()).unwrap());
	}
	return conversion_matrix_f(&formulas_from, &formulas_to);
}

/*************************************************************************************************/
/*************************************************************************************************/

