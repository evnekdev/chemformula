// transform.rs

//! An utility structure, `Transform` is introduced, to handle matrix transformation of one composition set (for example, ['CaO', 'SiO2', 'MgO']) to another (['Ca', 'Mg', 'Si', 'O']). Very useful for practical engineering calculations.

use std::str::FromStr;
use std::collections::{BTreeSet};
use nalgebra::{DMatrix, Dim, Storage, Matrix};
use nom::{Err, error::{Error,ErrorKind}};

use crate::formula::Formula;
use crate::element::Element;

const THRESHOLD : f64 = 1e-12;

/*********************************************************************************************************************************/
/*********************************************************************************************************************************/

#[derive(Debug)]
pub struct Transform {
	basis_initial: Vec<Formula>,                    // initial (usually atomic)
	basis_final:   Vec<Formula>,                    // final (usually endmembers)
	wmasses_initial : Vec<f64>,                     // molar masses of the input basis
	wmasses_final   : Vec<f64>,                     // molar masses of the output basis
	coeffmatrix_initial : DMatrix<f64>,             // Initial basis coefficients
	coeffmatrix_final : DMatrix<f64>,               // Final basis coefficients
	precomputed_init2final : Option<DMatrix<f64>>,  // if `precompute` was set to `true`.
	precomputed_final2init : Option<DMatrix<f64>>,  // if `precompute` was set to `true`.
}

/*********************************************************************************************************************************/
/*********************************************************************************************************************************/

fn check_matrix_d(mat: &mut DMatrix<f64>) {
    for value in mat.iter_mut() {
        if value.abs() < THRESHOLD {
            *value = 0.0;
        }
    }
}

/*********************************************************************************************************************************/
/*********************************************************************************************************************************/

impl Transform {
	/// Create a [`Transform`] struct based on a list of initial chemical formulas and a list of final chemical formulas
	pub fn new<T1: AsRef<str>, T2: AsRef<str>>(list1: &[T1], list2: &[T2], precompute: bool)->Result<Transform, Err<Error<String>>> {
		let basis_initial: Vec<Formula> = list1.iter().map(|s| {Formula::from_str(s.as_ref()).map_err(|_| {Err::Error(Error::new(s.as_ref().to_string(),ErrorKind::MapRes,))})}).collect::<Result<_, _>>()?;
		let basis_final:   Vec<Formula> = list2.iter().map(|s| {Formula::from_str(s.as_ref()).map_err(|_| {Err::Error(Error::new(s.as_ref().to_string(),ErrorKind::MapRes,))})}).collect::<Result<_, _>>()?;
		let wmasses_initial : Vec<f64>  = basis_initial.iter().map(|frm| frm.wmass()).collect();
		let wmasses_final   : Vec<f64>  = basis_final.iter().map(|frm| frm.wmass()).collect();
		let eset = element_set(&basis_initial, &basis_final);
		let coeffmatrix_initial = coeffmatrix(&basis_initial, &eset);
		let coeffmatrix_final   = coeffmatrix(&basis_final,   &eset);
		let precomputed_init2final : Option<DMatrix<f64>> = if precompute {Some(matrix_solve(&coeffmatrix_initial, &coeffmatrix_final))} else {None};
		let precomputed_final2init : Option<DMatrix<f64>> = if precompute {Some(matrix_solve(&coeffmatrix_final, &coeffmatrix_initial))} else {None};
		return Ok(
				Transform {
					basis_initial,
					basis_final,
					wmasses_initial,
					wmasses_final,
					coeffmatrix_initial,
					coeffmatrix_final,
					precomputed_init2final,
					precomputed_final2init,
					});
	}
	
	/// Initial basis size
	pub fn number_initial(&self)->usize{
		return self.basis_initial.len();
	}
	
	/// Final basis size
	pub fn number_final(&self)->usize{
		return self.basis_final.len();
	}
	
	/// molar masses of the input basis
	pub fn wmasses_initial(&self)->&Vec<f64> {
		return &self.wmasses_initial;
	}
	/// Molar masses of the output basis
	pub fn wmasses_final(&self)->&Vec<f64> {
		return &self.wmasses_final;
	}
	
	/// Transform compositions from the initial basis to final using the following flags:
	/// 
	/// `isweight_initial` - `true` if the input compositions are in gram, moles otherwise
	/// `isweight_final` - `true` if the output compositions must be in gram, moles otherwise
	/// `isfraction` - normalize the final compositions (sum(xi) = 1.0)
	pub fn transform_init2final<R: Dim, C: Dim, S: Storage<f64,R,C>>(&self, compositions: &Matrix<f64,R,C,S>, isweight_initial : bool, isweight_final : bool, isfraction: bool)->DMatrix<f64> {
		assert_eq!(compositions.nrows(),self.number_initial(),"Input composition rows must match initial basis size");
		let mut x = DMatrix::from_fn(compositions.nrows(),compositions.ncols(),|i, j| compositions[(i, j)],   );
		// Convert input from weight amounts/fractions to mole amounts/fractions.
		if isweight_initial {divide_weights(&mut x, &self.wmasses_initial);}
		// Convert basis.
		let mut y = match &self.precomputed_init2final {
			Some(t) => t * x,
			None => matrix_solve(&self.coeffmatrix_initial, &self.coeffmatrix_final) * x,
		};
		// Convert output from mole amounts/fractions to weight amounts/fractions.
		if isweight_final {apply_weights(&mut y, &self.wmasses_final);}
		if isfraction {normalize_columns(&mut y);}
		check_matrix_d(&mut y);
		return y;
	}
	
	/// Transform compositions from the final basis to initial using the following flags:
	/// 
	/// `isweight_initial` - `true` if the input compositions are in gram, moles otherwise
	/// `isweight_final` - `true` if the output compositions must be in gram, moles otherwise
	/// `isfraction` - normalize the final compositions (sum(xi) = 1.0)
	pub fn transform_final2init<R: Dim, C: Dim, S: Storage<f64,R,C>>(&self, compositions: &Matrix<f64,R,C,S>, isweight_initial : bool, isweight_final : bool, isfraction: bool)->DMatrix<f64> {
		assert_eq!(compositions.nrows(),self.number_final(),"Input composition rows must match final basis size");
    let mut x = DMatrix::from_fn(compositions.nrows(),compositions.ncols(),|i, j| compositions[(i, j)],);
    // Here the input is in the final basis.
    if isweight_final {divide_weights(&mut x, &self.wmasses_final);}
    let mut y = match &self.precomputed_final2init {
        Some(t) => t * x,
        None => matrix_solve(&self.coeffmatrix_final, &self.coeffmatrix_initial) * x,
    };
    // Here the output is in the initial basis.
    if isweight_initial {apply_weights(&mut y, &self.wmasses_initial);}
    if isfraction {normalize_columns(&mut y);}
    check_matrix_d(&mut y);
    return y;
	}
	
}

/*********************************************************************************************************************************/
/*********************************************************************************************************************************/

impl Default for Transform {
	fn default()->Self{
		return Transform::new::<&str, &str>(&[], &[], false).unwrap();
	}
}

/*********************************************************************************************************************************/
/*********************************************************************************************************************************/

/// Derives an ordered set of all elements occurring in a vector of [`Formula`] structures.
fn element_set(basis1: &[Formula], basis2: &[Formula])->Vec<Element>{
	let mut eset : BTreeSet<Element> = BTreeSet::new();
	for k in 0..basis1.len(){
		for (key, _) in basis1[k].pairs.iter(){
			eset.insert(key.clone());
		}
	}
	for k in 0..basis2.len(){
		for (key, _) in basis2[k].pairs.iter(){
			eset.insert(key.clone());
		}
	}
	return eset.into_iter().collect();
}

/*********************************************************************************************************************************/
/*********************************************************************************************************************************/

/// Constructs a coefficient matrix for a formula basis
fn coeffmatrix(basis: &[Formula], eset: &[Element])->DMatrix<f64> {
	let mut dmat : DMatrix<f64> = DMatrix::zeros(eset.len(), basis.len());
	for e in 0..eset.len(){
		for b in 0..basis.len(){
			dmat[(e,b)] = basis[b].coeff(&eset[e]) as f64;
		}
	}
	return dmat;
}

/*********************************************************************************************************************************/
/*********************************************************************************************************************************/

/// presolve a matrix for multiplication
fn matrix_solve(amat: &DMatrix<f64>, bmat: &DMatrix<f64>)->DMatrix<f64> {
	assert_eq!(amat.nrows(),bmat.nrows(),"Source and target coefficient matrices must have the same number of element rows");
    let qr = bmat.clone().qr();
    qr.solve(amat).unwrap_or_else(|| {panic!("Could not solve basis transformation: target basis probably does not span source basis")})
}

/*********************************************************************************************************************************/
/*********************************************************************************************************************************/

/// Transform from moles to grams
fn apply_weights(mat: &mut DMatrix<f64>, weights: &[f64]) {
    assert_eq!(mat.nrows(),weights.len(),"Number of composition rows must match number of basis formulas");
    for i in 0..mat.nrows() {for j in 0..mat.ncols() {mat[(i, j)] *= weights[i];}}
}

/// Transform from grams to moles
fn divide_weights(mat: &mut DMatrix<f64>, weights: &[f64]) {
    assert_eq!(mat.nrows(),weights.len(),"Number of composition rows must match number of basis formulas");
    for i in 0..mat.nrows() {for j in 0..mat.ncols() {mat[(i, j)] /= weights[i];}}
}

/// Make sum(xi) = 1.0
fn normalize_columns(mat: &mut DMatrix<f64>) {
    for j in 0..mat.ncols() {
        let mut sum = 0.0;
        for i in 0..mat.nrows() {sum += mat[(i, j)];}
        if sum.abs() > THRESHOLD {
            for i in 0..mat.nrows() {mat[(i, j)] /= sum;}
        }
    }
}


/*********************************************************************************************************************************/
/*********************************************************************************************************************************/

