/// chemformula::examples/main03.rs
use std::str::FromStr;
use chemformula::Formula;

pub fn main(){
	let name = "FeO1.09";
	let frm = Formula::from_str(name).unwrap();
	println!("name = {:?}, formula = {:?}, wmass = {:?}", &name, &frm, frm.wmass());
}