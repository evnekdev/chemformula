/// chemformula::examples/main01.rs
use std::str::FromStr;
use chemformula::Formula;

pub fn main(){
	let name = "MgAl2O4";
	let frm = Formula::from_str(name).unwrap();
	println!("name = {:?}, formula = {:?}, wmass = {:?}", &name, &frm, frm.wmass());
}