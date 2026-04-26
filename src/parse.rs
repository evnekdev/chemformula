// parse.rs

//! Collection of `nom`-based internal parsing routines which, combined, ensure the proper parsing of text strings into `Formula` objects.

use crate::{Formula, Element};

use std::str::FromStr;
use nom::{IResult,
bytes::complete::{tag, take_while},
character::{is_digit, is_alphabetic},
combinator::{fail},
sequence::{pair},
multi::{many0}};


/*************************************************************************************************/
/*************************************************************************************************/
/// parses a solution electron in the form 'e(PhaseName)'
pub fn parse_electron(input: &str)->IResult<&str,Element>{
	let (input, _) = tag("e(")(input)?;
	let (input, phase) = take_while(|c: char| c.is_alphanumeric() || c == '#' || c == '-' || c == '_')(input)?;
	let (input, _) = tag(")")(input)?;
	return Ok((input, Element::e(String::from(phase))));
}

/// parses an element from a string
pub fn parse_element(input: &str)->IResult<&str,Element>{
	if input.len() < 1 {return fail("insufficient length")}
	//if input.len() < 1 {return fail(input)}
	if input.len() > 1 {
		match Element::from_str(&input[0..2]) {
			Ok(element) => {return Ok((&input[2..],element));}
			Err(_) => {
				// do nothing, try parsing as a 1-character element
			}
		}
	}
	match Element::from_str(&input[0..1]){
		Ok(element) => {return Ok((&input[1..],element));}
		Err(_) => {
			return fail("Cannot parse an element");
		}
	}
}

/// parses an integer coefficient from a string, such as 2, 5, etc.
pub fn parse_coeff(input: &str)->IResult<&str,f64>{
	let (input, res) = take_while(|c: char| c.is_numeric() || c == '.')(input)?;
	match res.len() {
		0 => {return Ok((input, 1.0f64));}
		_ => {
			match f64::from_str(res){
				Ok(res_num) => {
					return Ok((input, res_num));
				}
				Err(_) => {
					return fail("Cannot parse coefficient");
				}
			}
		}
	}
}
/// parses an element/coefficient group, such as Na2, Ca, H3, etc.
pub fn parse_element_coeff(input: &str)->IResult<&str,(Element, f64)>{
	let (input, res) = pair(parse_element, parse_coeff)(input)?;
	return Ok((input, res));
}

/// parses several element/coefficient groups together
pub fn parse_element_coeff_group(input: &str)->IResult<&str, Formula>{
	let (input, res) =  many0(parse_element_coeff)(input)?;
	let mut frm = Formula::new();
	for (element, coeff) in res {
		let mut f = Formula::from(element);
		f *= coeff;
		frm += &f;
	}
	return Ok((input, frm));
}

