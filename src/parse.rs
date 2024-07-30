// parse.rs

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
	let (input, phase) = take_while(|c: char| c.is_alphanumeric())(input)?;
	let (input, _) = tag(")")(input)?;
	return Ok((input, Element::e(String::from(phase))));
}

/// parses an element from a string
pub fn parse_element(input: &str)->IResult<&str,Element>{
	if input.len() < 1 {return fail("insufficient length")}
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
pub fn parse_coeff(input: &str)->IResult<&str,u32>{
	let (input, res) = take_while(|c: char| c.is_numeric())(input)?;
	match res.len() {
		0 => {return Ok((input, 1u32));}
		_ => {
			match u32::from_str(res){
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
pub fn parse_element_coeff(input: &str)->IResult<&str,(Element, u32)>{
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
		//println!("element = {:?}, coeff = {}, f = {:?}", &element, coeff, &f);
		frm += &f;
	}
	return Ok((input, frm));
}

