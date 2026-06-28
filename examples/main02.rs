/// chemformula::examples/main02.rs
use std::str::FromStr;
use chemformula::Formula;

pub fn main(){
	let name = "KAl(SO4)2";
	let frm = Formula::from_str(name).unwrap();
	println!("name = {:?}, formula = {:?}, wmass = {:?}", &name, &frm, frm.wmass());
}