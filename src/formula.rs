// formula.rs

//! `Formula` struct handles parsing formula names.

use std::fmt;
use std::str::FromStr;
use std::ops::{MulAssign, Mul, AddAssign, Add};
use crate::{parse::{parse_electron, parse_element_coeff_group}, Element};

/// A structure representing a chemical formula, i.e. a ordered set of pairs (Element, coefficient) + a charge value.
#[derive(Clone)]
pub struct Formula {
	pub pairs: Vec<(Element, f64)>,
	pub charge: f64,
}

impl Formula {
	/// Initializes an empty formula
	pub fn new()->Formula{
		return Formula{
			pairs: Vec::new(),
			charge: 0.0,
		};
	}
	
	/// return the molar mass of a formula unit
	pub fn wmass(&self)->f64{
		let mut wm = 0.0;
		for (key, value) in self.pairs.iter(){
			wm += key.wmass() * (*value as f64);
		}
		return wm;
	}
	/// returns a stoichiometry coefficient for an element; 0 if the element is not contained in the formula
	pub fn coeff(&self, element: &Element)->f64{
		for (key, value) in self.pairs.iter(){
			if key == element {
				return *value;
			}
		}
		return 0.0;
	}
}
/*************************************************************************************************/
/// Converts an [`Element`] into a corresponding [`Formula`]
impl From<Element> for Formula {
	fn from(element: Element)->Self{
		let mut frm = Formula::new();
		match element {
			Element::e(_)=> {
				frm.charge -= 1.0;
			}
			_ => {}
		}
		frm.pairs.push((element, 1.0f64));
		return frm;
	}
}

/// Use this trait to parse a &str value into a [`Formula`] structure.
impl FromStr for Formula {
	type Err = String;
	fn from_str(input: &str)->Result<Self,String>{
		match parse_element_coeff_group(input){
			Ok((input, frm)) => {
				if input.len() == 0 {
				return Ok(frm);
				}
			}
			Err(_) => {}
		}
		match parse_electron(input){
			Ok((input, electron)) => {
				return Ok(Formula::from(electron));
			}
			Err(_) => {
				return Err(format!("cannot parse the string {:?}", input));
			}
		}
	}
}

impl fmt::Debug for Formula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn fmt_coeff(value: f64) -> String {
            if value.fract() == 0.0 {
                format!("{}", value as i64)
            } else {
                format!("{}", value)
            }
        }

        for (key, value) in self.pairs.iter() {
            if *value == 1.0 {
                write!(f, "{:?}", key)?;
            } else {
                write!(f, "{:?}{}", key, fmt_coeff(*value))?;
            }
        }

        if self.charge != 0.0 {
            write!(f, "[{}]", fmt_coeff(self.charge))?;
        }

        Ok(())
    }
}

impl MulAssign<f64> for Formula {
	fn mul_assign(&mut self, rhs: f64){
		for (key, value) in self.pairs.iter_mut(){
			*value *= rhs;
		}
		self.charge *= rhs;
	}
}

impl Mul<f64> for Formula {
	type Output = Formula;
	fn mul(self, rhs: f64)->Self::Output{
		let mut res = Formula::new();
		for (key, value) in self.pairs.iter(){
			res.pairs.push((key.clone(), value*rhs));
		}
		res.charge = self.charge * rhs as f64;
		return res;
	}
}

impl AddAssign<&Formula> for Formula {
	fn add_assign(&mut self, rhs: &Formula){
		for (key2, value2) in rhs.pairs.iter(){
			for (key1, value1) in self.pairs.iter_mut(){
				if key1 == key2{
					*value1 += value2;
					break;
				}
			}
			self.pairs.push((key2.clone(), *value2));
		}
		self.charge += rhs.charge;
	}
}

impl Add<&Formula> for Formula {
	type Output = Formula;
	fn add(self, rhs: &Formula)->Self::Output{
		let mut res = Formula::new();
		res += &self;
		res += rhs;
		return res;
	}
}
