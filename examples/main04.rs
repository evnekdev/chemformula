/// chemformula::examples/main04.rs
use std::str::FromStr;
use chemformula::Formula;

pub fn main(){
	let name = "Al[3+]";
	let frm = Formula::from_str(name).unwrap();
	println!("name = {:?}, formula = {:?}, wmass = {:?}", &name, &frm, frm.wmass());
}