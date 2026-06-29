/// chemformula::examples/main05.rs
use std::str::FromStr;
use nalgebra::{dvector};

use chemformula::Formula;
use chemformula::Transform;

pub fn main(){
	let binitial = vec!["CaO", "Al2O3", "SiO2"];
	let bfinal   = vec!["Ca", "Al", "Si", "O"];
	let transform = Transform::new(&binitial, &bfinal, false).unwrap();
	let compinitial = dvector![0.4, 0.5, 0.1];
	let compfinal = transform.transform_init2final(&compinitial, false, false, false);
	println!("FINAL COMPOSITION (moles) = {:.6?}", &compfinal.data.as_slice());
	let compfinal = transform.transform_init2final(&compinitial, false, false, true);
	println!("FINAL COMPOSITION (Xfraction) = {:.6?}", &compfinal.data.as_slice());
	let compfinal = transform.transform_init2final(&compinitial, false, true, false);
	println!("FINAL COMPOSITION (grams) = {:.6?}", &compfinal.data.as_slice());
	let compfinal = transform.transform_init2final(&compinitial, false, true, true);
	println!("FINAL COMPOSITION (Wfraction) = {:.6?}", &compfinal.data.as_slice());
}