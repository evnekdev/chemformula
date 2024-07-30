// transform.rs

use nalgebra::{dvector, DVector, SVector, DMatrix, Matrix, DimName, Dim, Owned, Storage, VecStorage, U1, Dyn, constraint::{ShapeConstraint, DimEq, AreMultipliable}};

use crate::{conversion_matrix_s};

const THRESHOLD : f64 = 1e-12;

pub struct Transform{
	basis_i: Vec<String>, // initial (atomic)
	basis_f: Vec<String>, // final (end members)
	pub f2i: Matrix<f64,Dyn,Dyn,VecStorage<f64,Dyn,Dyn>>,
	pub i2f: Matrix<f64,Dyn,Dyn,VecStorage<f64,Dyn,Dyn>>,
}

fn check_vector_d(vec: &mut DVector<f64>){
	for k in 0..vec.len() {
		if vec[k].abs() < THRESHOLD {
			vec[k] = 0.0;
		}
	}
}

impl Transform {
	/// Create a [`Transform`] struct based on a list of initial chemical formulas and a list of final chemical formulas
	pub fn from_formulas_s<T1: AsRef<str>, T2: AsRef<str>>(list1: &[T1], list2: &[T2])->Transform{
		return Transform {
			basis_i: list1.iter().map(|s| String::from(s.as_ref())).collect(),
			basis_f: list2.iter().map(|s| String::from(s.as_ref())).collect(),
			i2f : conversion_matrix_s(list1, list2),
			f2i : conversion_matrix_s(list2, list1),
		};
	}
	/// apply transform final-to-initial, dyn-to-dyn
	pub fn transform_f2i_d2d(&self, x_f: &DVector<f64>)->DVector<f64>{
		let mut res = &self.f2i * x_f;
		check_vector_d(&mut res);
		return res;
	}
	/// apply transform initial-to-final, dyn-to-dyn
	pub fn transform_i2f_d2d(&self, x_i: &DVector<f64>)->DVector<f64>{
		let mut res = &self.i2f * x_i;
		check_vector_d(&mut res);
		return res;
	}
	/// apply transform final-to-initial, static-to-dyn
	pub fn transform_f2i_s2d<const N: usize>(&self, x_f: &SVector<f64,N>)->DVector<f64>{
		let mut res = &self.f2i * x_f;
		check_vector_d(&mut res);
		return res;
	}
	/// apply transform initial-to-final, static-to-dyn
	pub fn transform_i2f_s2d<const N: usize>(&self, x_i: &SVector<f64,N>)->DVector<f64>{
		let mut res = &self.i2f * x_i;
		check_vector_d(&mut res);
		return res;
	}
	/// apply transform final-to-initial, dyn-to-static
	pub fn transform_f2i_d2s<const N: usize>(&self, x_f: &DVector<f64>)->SVector<f64,N>{
		let mut res = &self.f2i * x_f;
		check_vector_d(&mut res);
		return SVector::from(res.fixed_slice::<N,1>(0,0));
	}
	/// apply transform initial-to-final, dyn-to-static
	pub fn transform_i2f_d2s<const N: usize>(&self, x_i: &DVector<f64>)->SVector<f64,N>{
		let mut res = &self.i2f * x_i;
		check_vector_d(&mut res);
		return SVector::from(res.fixed_slice::<N,1>(0,0));
	}
	/// apply transform final-to-initial, static-to-static
	pub fn transform_f2i_s2s<const N1: usize, const N2: usize>(&self, x_f: &SVector<f64,N1>)->SVector<f64,N2>{
		let mut res = &self.f2i * x_f;
		check_vector_d(&mut res);
		return SVector::from(res.fixed_slice::<N2,1>(0,0));
	}
	/// apply transform initial-to-final, static-to-static
	pub fn transform_i2f_s2s<const N1: usize, const N2: usize>(&self, x_i: &SVector<f64,N1>)->SVector<f64,N2>{
		let mut res = &self.i2f * x_i;
		check_vector_d(&mut res);
		return SVector::from(res.fixed_slice::<N2,1>(0,0));
	}
}

impl Default for Transform {
	fn default()->Self{
		return Transform::from_formulas_s::<&str, &str>(&["O"], &["O"]);
	}
}
