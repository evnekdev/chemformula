// element.rs

use std::str::FromStr;
use crate::parse::{parse_electron};

/// Enum representing a chemical element. Includes vacancies (Va), and e(phase_name) electrons for charge balance of solutions with charged endmembers.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Element {
	e(String),
	Va,
	H,	He,
	Li,	Be,	B,	C,	N,	O,	F,	Ne,
	Na,	Mg,	Al,	Si,	P, S, Cl, Ar,
	K, Ca, Sc, Ti, V, Cr, Mn, Fe, Co, Ni,
	Cu, Zn, Ga, Ge, As, Se, Br, Kr,
	Rb, Sr, Y, Zr, Nb, Mo, Tc, Ru, Rh, Pd,
	Ag, Cd, In, Sn, Sb, Te, I, Xe,
	Cs, Ba, La, Ce, Pr, Nd, Pm, Sm, Eu, Gd, Tb, Dy, Ho, Er, Tm, Yb, Lu, Hf, Ta, W, Re, Os, Ir, Pt,
	Au, Hg, Tl, Pb, Bi, Po, At, Rn,
	Fr, Ra, Ac, Th, Pa, U, Np, Pu, Am, Cm, Bk, Cf, Es, Fm, Md, No, Lr, Rf, Db, Sg, Bh, Hs,
}

impl FromStr for Element {
	type Err = String;
	fn from_str(input: & str)->Result<Self, Self::Err>{
		match input {
			"Va"  => {return Ok(Element::Va);}
			"H"  => {return Ok(Element::H);}
			"He" => {return Ok(Element::He);}
			"Li" => {return Ok(Element::Li);}
			"Be" => {return Ok(Element::Be);}
			"B"  => {return Ok(Element::B);}
			"C"  => {return Ok(Element::C);}
			"N"  => {return Ok(Element::N);}
			"O"  => {return Ok(Element::O);}
			"F"  => {return Ok(Element::F);}
			"Ne" => {return Ok(Element::Ne);}
			"Na" => {return Ok(Element::Na);}
			"Mg" => {return Ok(Element::Mg);}
			"Al" => {return Ok(Element::Al);}
			"Si" => {return Ok(Element::Si);}
			"P"  => {return Ok(Element::P);}
			"S"  => {return Ok(Element::S);}
			"Cl" => {return Ok(Element::Cl);}
			"Ar" => {return Ok(Element::Ar);}
			"K"  => {return Ok(Element::K);}
			"Ca" => {return Ok(Element::Ca);}
			"Sc" => {return Ok(Element::Sc);}
			"Ti" => {return Ok(Element::Ti);}
			"V"  => {return Ok(Element::V);}
			"Cr" => {return Ok(Element::Cr);}
			"Mn" => {return Ok(Element::Mn);}
			"Fe" => {return Ok(Element::Fe);}
			"Co" => {return Ok(Element::Co);}
			"Ni" => {return Ok(Element::Ni);}
			"Cu" => {return Ok(Element::Cu);}
			"Zn" => {return Ok(Element::Zn);}
			"Ga" => {return Ok(Element::Ga);}
			"Ge" => {return Ok(Element::Ge);}
			"As" => {return Ok(Element::As);}
			"Se" => {return Ok(Element::Se);}
			"Br" => {return Ok(Element::Br);}
			"Kr" => {return Ok(Element::Kr);}
			"Rb" => {return Ok(Element::Rb);}
			"Sr" => {return Ok(Element::Sr);}
			"Y"  => {return Ok(Element::Y);}
			"Zr" => {return Ok(Element::Zr);}
			"Nb" => {return Ok(Element::Nb);}
			"Mo" => {return Ok(Element::Mo);}
			"Tc" => {return Ok(Element::Tc);}
			"Ru" => {return Ok(Element::Ru);}
			"Rh" => {return Ok(Element::Rh);}
			"Pd" => {return Ok(Element::Pd);}
			"Ag" => {return Ok(Element::Ag);}
			"Cd" => {return Ok(Element::Cd);}
			"In" => {return Ok(Element::In);}
			"Sn" => {return Ok(Element::Sn);}
			"Sb" => {return Ok(Element::Sb);}
			"Te" => {return Ok(Element::Te);}
			"I"  => {return Ok(Element::I);}
			"Xe" => {return Ok(Element::Xe);}
			"Cs" => {return Ok(Element::Cs);}
			"Ba" => {return Ok(Element::Ba);}
			"La" => {return Ok(Element::La);}
			"Ce" => {return Ok(Element::Ce);}
			"Pr" => {return Ok(Element::Pr);}
			"Nd" => {return Ok(Element::Nd);}
			"Pm" => {return Ok(Element::Pm);}
			"Sm" => {return Ok(Element::Sm);}
			"Eu" => {return Ok(Element::Eu);}
			"Gd" => {return Ok(Element::Gd);}
			"Tb" => {return Ok(Element::Tb);}
			"Dy" => {return Ok(Element::Dy);}
			"Ho" => {return Ok(Element::Ho);}
			"Er" => {return Ok(Element::Er);}
			"Tm" => {return Ok(Element::Tm);}
			"Yb" => {return Ok(Element::Yb);}
			"Lu" => {return Ok(Element::Lu);}
			"Hf" => {return Ok(Element::Hf);}
			"Ta" => {return Ok(Element::Ta);}
			"W"  => {return Ok(Element::W);}
			"Re" => {return Ok(Element::Re);}
			"Os" => {return Ok(Element::Os);}
			"Ir" => {return Ok(Element::Ir);}
			"Pt" => {return Ok(Element::Pt);}
			"Au" => {return Ok(Element::Au);}
			"Hg" => {return Ok(Element::Hg);}
			"Tl" => {return Ok(Element::Tl);}
			"Pb" => {return Ok(Element::Pb);}
			"Bi" => {return Ok(Element::Bi);}
			"Po" => {return Ok(Element::Po);}
			"At" => {return Ok(Element::At);}
			"Rn" => {return Ok(Element::Rn);}
			"Fr" => {return Ok(Element::Fr);}
			"Ra" => {return Ok(Element::Ra);}
			"Ac" => {return Ok(Element::Ac);}
			"Th" => {return Ok(Element::Th);}
			"Pa" => {return Ok(Element::Pa);}
			"U"  => {return Ok(Element::U);}
			"Np" => {return Ok(Element::Np);}
			"Pu" => {return Ok(Element::Pu);}
			"Am" => {return Ok(Element::Am);}
			"Cm" => {return Ok(Element::Cm);}
			"Bk" => {return Ok(Element::Bk);}
			"Cf" => {return Ok(Element::Cf);}
			"Es" => {return Ok(Element::Es);}
			"Fm" => {return Ok(Element::Fm);}
			"Md" => {return Ok(Element::Md);}
			"No" => {return Ok(Element::No);}
			"Lr" => {return Ok(Element::Lr);}
			"Rf" => {return Ok(Element::Rf);}
			"Db" => {return Ok(Element::Db);}
			"Sg" => {return Ok(Element::Sg);}
			"Bh" => {return Ok(Element::Bh);}
			"Hs" => {return Ok(Element::Hs);}
			_ => {
				match parse_electron(input){
					Ok((input, electron)) => {
						return Ok(electron);
					}
					Err(_) => {
						return Err(String::from("Cannot parse an element"));
					}
				}
				return Err(String::from("Cannot parse an element"));
			}
		}
	}
}

impl Element {
	pub fn wmass(&self)->f64{
		match self {
			Element::e(_) => {return 0.0;}
			Element::Va => {return 0.0;}
			Element::H  => {return 1.00794;}
			Element::He => {return 4.002602;}
			Element::Li => {return 6.941;}
			Element::Be => {return 9.012182;}
			Element::B  => {return 10.811;}
			Element::C  => {return 12.0107;}
			Element::N  => {return 14.0067;}
			Element::O  => {return 15.9994;}
			Element::F  => {return 18.9984032;}
			Element::Ne => {return 20.1797;}
			Element::Na => {return 22.98976;}
			Element::Mg => {return 24.305;}
			Element::Al => {return 26.9815386;}
			Element::Si => {return 28.0855;}
			Element::P  => {return 30.973762;}
			Element::S  => {return 32.065;}
			Element::Cl => {return 35.453;}
			Element::Ar => {return 39.948;}
			Element::K  => {return 39.0983;}
			Element::Ca => {return 40.078;}
			Element::Sc => {return 44.955912;}
			Element::Ti => {return 47.867;}
			Element::V  => {return 50.9415;}
			Element::Cr => {return 51.9961;}
			Element::Mn => {return 54.938045;}
			Element::Fe => {return 55.845;}
			Element::Co => {return 58.933195;}
			Element::Ni => {return 58.6934;}
			Element::Cu => {return 63.546;}
			Element::Zn => {return 65.38;}
			Element::Ga => {return 69.723;}
			Element::Ge => {return 72.63;}
			Element::As => {return 74.9216;}
			Element::Se => {return 78.96;}
			Element::Br => {return 79.904;}
			Element::Kr => {return 83.798;}
			Element::Rb => {return 85.4678;}
			Element::Sr => {return 87.62;}
			Element::Y  => {return 88.90585;}
			Element::Zr => {return 91.224;}
			Element::Nb => {return 92.90638;}
			Element::Mo => {return 95.96;}
			Element::Tc => {return 98.0;}
			Element::Ru => {return 101.07;}
			Element::Rh => {return 102.9055;}
			Element::Pd => {return 106.42;}
			Element::Ag => {return 107.8682;}
			Element::Cd => {return 112.411;}
			Element::In => {return 114.818;}
			Element::Sn => {return 118.71;}
			Element::Sb => {return 121.76;}
			Element::Te => {return 127.6;}
			Element::I  => {return 126.90447;}
			Element::Xe => {return 131.293;}
			Element::Cs => {return 132.9054;}
			Element::Ba => {return 137.327;}
			Element::La => {return 138.90547;}
			Element::Ce => {return 140.116;}
			Element::Pr => {return 140.90765;}
			Element::Nd => {return 144.242;}
			Element::Pm => {return 145.0;}
			Element::Sm => {return 150.36;}
			Element::Eu => {return 151.964;}
			Element::Gd => {return 157.25;}
			Element::Tb => {return 158.92535;}
			Element::Dy => {return 162.5;}
			Element::Ho => {return 164.93032;}
			Element::Er => {return 167.259;}
			Element::Tm => {return 168.93421;}
			Element::Yb => {return 173.054;}
			Element::Lu => {return 174.9668;}
			Element::Hf => {return 178.49;}
			Element::Ta => {return 180.94788;}
			Element::W  => {return 183.84;}
			Element::Re => {return 186.207;}
			Element::Os => {return 190.23;}
			Element::Ir => {return 192.217;}
			Element::Pt => {return 195.084;}
			Element::Au => {return 196.966569;}
			Element::Hg => {return 200.59;}
			Element::Tl => {return 204.3833;}
			Element::Pb => {return 207.2;}
			Element::Bi => {return 208.9804;}
			Element::Po => {return 209.0;}
			Element::At => {return 210.0;}
			Element::Rn => {return 222.0;}
			Element::Fr => {return 223.0;}
			Element::Ra => {return 226.0;}
			Element::Ac => {return 227.0;}
			Element::Th => {return 232.04;}
			Element::Pa => {return 231.04;}
			Element::U  => {return 238.03;}
			Element::Np => {return 237.0;}
			Element::Pu => {return 244.0;}
			Element::Am => {return 243.0;}
			Element::Cm => {return 247.0;}
			Element::Bk => {return 247.0;}
			Element::Cf => {return 251.0;}
			Element::Es => {return 252.0;}
			Element::Fm => {return 257.0;}
			Element::Md => {return 258.0;}
			Element::No => {return 259.0;}
			Element::Lr => {return 266.0;}
			Element::Rf => {return 267.0;}
			Element::Db => {return 268.0;}
			Element::Sg => {return 269.0;}
			Element::Bh => {return 270.0;}
			Element::Hs => {return 277.0;}
		}
	}
}
