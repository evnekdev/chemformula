/// chemformula::examples/main05.rs
use std::str::FromStr;
use chemformula::Formula;

pub fn main(){
	let name = "Al(OH)4[-]";
	let frm = Formula::from_str(name).unwrap();
	println!("name = {:?}, formula = {:?}, wmass = {:?}", &name, &frm, frm.wmass());
}