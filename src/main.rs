use std::str::FromStr;
use nalgebra::{DVector, DMatrix, dvector};
use nom::{IResult,
bytes::complete::{tag}};
use chemformula::*;

pub fn main(){
	let input = "H2SO4";
	let res = Formula::from_str(input).unwrap();
	println!("res = {:?}, wmass = {}", &res, res.wmass());
	let from = ["Ca2SiO4", "CaO"];
	let to   = ["Ca", "Si", "O"];
	let matrix = conversion_matrix_s(&from, &to);
	let comps = dvector![0.5, 0.6];
	let comps_ = &matrix * comps;
	println!("comps_ = {:?}", &comps_);
}