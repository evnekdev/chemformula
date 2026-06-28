// parse.rs

//! Collection of `nom`-based internal parsing routines which, combined, ensure the proper parsing of text strings into `Formula` objects.

use crate::{Formula, Element};

use std::str::FromStr;
use nom::{IResult,
bytes::complete::{tag, take_while1},
branch::alt,
character::complete::{digit1,char},
combinator::{map, map_res, opt, recognize},
sequence::{pair, preceded, delimited},
multi::{many1}
};

/*********************************************************************************************************************************/
/*********************************************************************************************************************************/

/// parses a solution electron in the form 'e(PhaseName)'
pub fn parse_electron(input: &str)->IResult<&str,Element>{
	let (input, _) = tag("e(")(input)?;
	let (input, phase) = take_while1(|c: char| c.is_alphanumeric() || c == '#' || c == '-' || c == '_')(input)?;
	let (input, _) = tag(")")(input)?;
	return Ok((input, Element::E(String::from(phase))));
}

/*********************************************************************************************************************************/
/*********************************************************************************************************************************/

/// Element symbol from the Periodic Table
pub fn parse_element(input: &str)-> IResult<&str, Element>{
	let mut chars = input.char_indices();
	
	let (_, first) = chars.next().ok_or_else(|| {nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Eof))})?;
	
	let two_char_len = chars.next().map(|(i,c)| i + c.len_utf8());
	
	if let Some(len) = two_char_len {
		if let Ok(element) = Element::from_str(&input[..len]){
			return Ok((&input[len..],element));
		}
	}
	
	let one_char_len = first.len_utf8();
	
	match Element::from_str(&input[..one_char_len]) {
        Ok(element) => Ok((&input[one_char_len..], element)),
        Err(_) => Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        ))),
    }
	
}

/*********************************************************************************************************************************/
/*********************************************************************************************************************************/
/// 
pub fn parse_coeff(input: &str)->IResult<&str, f64> {
	let decimal = recognize(pair(digit1, opt(preceded(char('.'), digit1))));
	
	match opt(map_res(decimal, f64::from_str))(input)?{
		(input, Some(value)) => Ok((input,value)),
		(input, None) => Ok((input, 1.0)),
	}
	
}

/*********************************************************************************************************************************/
/*********************************************************************************************************************************/

/// 
pub fn parse_charge(input: &str)->IResult<&str, i32> {
	return delimited(char('['),map(pair(opt(map_res(digit1, str::parse::<i32>)),alt((char('+'), char('-'))),),|(n, sign)| { let n = n.unwrap_or(1); if sign == '+' { n } else { -n }},),char(']'))(input);
}

/*********************************************************************************************************************************/
/*********************************************************************************************************************************/

///parses an element/coefficient group, such as Na2, Ca, H3, etc.
pub fn parse_element_coeff(input: &str)->IResult<&str,(Element, f64)>{
	pair(parse_element, parse_coeff)(input)
}

/*********************************************************************************************************************************/
/*********************************************************************************************************************************/

/// 
pub fn parse_element_term(input: &str)->IResult<&str, Formula> {
	let (input, (element, coeff)) = pair(parse_element, parse_coeff)(input)?;
	
	let mut f = Formula::from(element);
	f *= coeff;
	
	return Ok((input, f));
}

/*********************************************************************************************************************************/
/*********************************************************************************************************************************/

/// 
pub fn parse_parenthesis_term(input: &str)->IResult<&str, Formula>{
	let (input, (mut inner, coeff)) = pair(delimited(tag("("), parse_formula_group, tag(")")), parse_coeff)(input)?;
	
	inner *= coeff;
	
	return Ok((input, inner));
}

/*********************************************************************************************************************************/
/*********************************************************************************************************************************/

/// 
pub fn parse_formula_term(input: &str)->IResult<&str,Formula>{
	return alt((parse_parenthesis_term,parse_element_term))(input);
}

/*********************************************************************************************************************************/
/*********************************************************************************************************************************/

/// 
pub fn parse_formula_group(input: &str)->IResult<&str, Formula>{
	let (input, terms) = many1(parse_formula_term)(input)?;
	let mut frm = Formula::new();
	for term in terms {
		frm += &term;
	}
	return Ok((input, frm));
}

/*********************************************************************************************************************************/
/*********************************************************************************************************************************/

/// This is the entry point for parsing
pub fn parse_formula(input: &str) -> IResult<&str, Formula> {
    let (input, (mut formula, charge)) =
        pair(parse_formula_group, opt(parse_charge))(input)?;

    if let Some(c) = charge {
        formula.charge = c as f64;
    }

    Ok((input, formula))
}

/*********************************************************************************************************************************/
/*********************************************************************************************************************************/