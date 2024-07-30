use std::str::FromStr;
use nalgebra::{DVector, DMatrix, dvector};
use nom::{IResult,
bytes::complete::{tag}};
use chemformula::*;

fn parse_tag(input: &[u8])->IResult<&[u8],()>{
	let (input, res) = tag("##")(input)?;
	println!("res1 = {:?}", &res);
	return Ok((input, ()));
}

pub fn main(){
	let input = b"##RTF";
	let res = parse_tag(input);
	println!("res2 = {:?}", &res);
	let input = "e(Mullite)";
	//let (remainder, res) = parse_element_coeff_group(input).unwrap();
	//let (_, res) = parse_electron(input).unwrap();
	//println!("res = {:?}, input = {:?}", &res, input);
	//let res = Element::from_str(input).unwrap();
	let res = Formula::from_str(input).unwrap();
	//for (key, value) in &res.pairs {
	//	println!("key = {:?}, value = {}", key, value);
	//}
	println!("res = {:?}, wmass = {}", &res, res.wmass());
	let from = ["Ca2SiO4", "CaO"];
	let to   = ["Ca", "Si", "O", "e(Mullite)", "e(Spinel)"];
	let matrix = conversion_matrix_s(&from, &to);
	let comps = dvector![0.5, 0.6];
	let comps_ = &matrix * comps;
	println!("comps_ = {:?}", &comps_);
}