use std::str::FromStr;

use chemformula::{Element, Formula, Transform};
use nalgebra::dvector;

const EPS: f64 = 1.0e-2;

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < EPS,
        "actual {actual} differs from expected {expected}"
    );
}

fn formula(input: &str) -> Formula {
    Formula::from_str(input).unwrap_or_else(|err| panic!("failed to parse {input:?}: {err}"))
}

#[test]
fn example_01_parses_spinel_formula() {
    let frm = formula("MgAl2O4");

    assert_close(frm.coeff(&Element::Mg), 1.0);
    assert_close(frm.coeff(&Element::Al), 2.0);
    assert_close(frm.coeff(&Element::O), 4.0);
    assert_close(frm.charge, 0.0);
    assert_close(frm.wmass(), 142.2656772);
}

#[test]
fn example_02_parses_parenthesised_sulfate_formula() {
    let frm = formula("KAl(SO4)2");

    assert_close(frm.coeff(&Element::K), 1.0);
    assert_close(frm.coeff(&Element::Al), 1.0);
    assert_close(frm.coeff(&Element::S), 2.0);
    assert_close(frm.coeff(&Element::O), 8.0);
    assert_close(frm.charge, 0.0);
    assert_close(frm.wmass(), 258.2040386);
}

#[test]
fn example_03_parses_fractional_coefficient() {
    let frm = formula("FeO1.09");

    assert_close(frm.coeff(&Element::Fe), 1.0);
    assert_close(frm.coeff(&Element::O), 1.09);
    assert_close(frm.charge, 0.0);
    assert_close(frm.wmass(), 73.284346);
}

#[test]
fn example_04_parses_positive_charge() {
    let frm = formula("Al[3+]");

    assert_close(frm.coeff(&Element::Al), 1.0);
    assert_close(frm.charge, 3.0);
    assert_close(frm.wmass(), 26.9815386);
}

#[test]
fn example_05_parses_parenthesised_hydroxide_anion() {
    let frm = formula("Al(OH)4[-]");

    assert_close(frm.coeff(&Element::Al), 1.0);
    assert_close(frm.coeff(&Element::O), 4.0);
    assert_close(frm.coeff(&Element::H), 4.0);
    assert_close(frm.charge, -1.0);
    assert_close(frm.wmass(), 95.0108986);
}

#[test]
fn example_06_transforms_oxide_moles_to_element_moles() {
    let binitial = vec!["CaO", "Al2O3", "SiO2"];
    let bfinal = vec!["Ca", "Al", "Si", "O"];
    let transform = Transform::new(&binitial, &bfinal, false).unwrap();
    let compinitial = dvector![0.4, 0.5, 0.1];

    let compfinal = transform.transform_init2final(&compinitial, false, false, false);

    assert_eq!(compfinal.nrows(), 4);
    assert_eq!(compfinal.ncols(), 1);
    assert_close(compfinal[(0, 0)], 0.4);
    assert_close(compfinal[(1, 0)], 1.0);
    assert_close(compfinal[(2, 0)], 0.1);
    assert_close(compfinal[(3, 0)], 2.1);
}

#[test]
fn example_06_transforms_oxide_moles_to_element_mole_fractions() {
    let binitial = vec!["CaO", "Al2O3", "SiO2"];
    let bfinal = vec!["Ca", "Al", "Si", "O"];
    let transform = Transform::new(&binitial, &bfinal, false).unwrap();
    let compinitial = dvector![0.4, 0.5, 0.1];

    let compfinal = transform.transform_init2final(&compinitial, false, false, true);

    assert_close(compfinal[(0, 0)], 0.4 / 3.6);
    assert_close(compfinal[(1, 0)], 1.0 / 3.6);
    assert_close(compfinal[(2, 0)], 0.1 / 3.6);
    assert_close(compfinal[(3, 0)], 2.1 / 3.6);
}

#[test]
fn example_06_transforms_oxide_moles_to_element_grams() {
    let binitial = vec!["CaO", "Al2O3", "SiO2"];
    let bfinal = vec!["Ca", "Al", "Si", "O"];
    let transform = Transform::new(&binitial, &bfinal, false).unwrap();
    let compinitial = dvector![0.4, 0.5, 0.1];

    let compfinal = transform.transform_init2final(&compinitial, false, true, false);

    assert_close(compfinal[(0, 0)], 0.4 * 40.078);
    assert_close(compfinal[(1, 0)], 1.0 * 26.9815386);
    assert_close(compfinal[(2, 0)], 0.1 * 28.0855);
    assert_close(compfinal[(3, 0)], 2.1 * 15.9994);
}

#[test]
fn example_06_transforms_oxide_moles_to_element_weight_fractions() {
    let binitial = vec!["CaO", "Al2O3", "SiO2"];
    let bfinal = vec!["Ca", "Al", "Si", "O"];
    let transform = Transform::new(&binitial, &bfinal, false).unwrap();
    let compinitial = dvector![0.4, 0.5, 0.1];

    let compfinal = transform.transform_init2final(&compinitial, false, true, true);
    let weights = [0.4 * 40.078, 1.0 * 26.9815386, 0.1 * 28.0855, 2.1 * 15.9994];
    let total: f64 = weights.iter().sum();

    assert_close(compfinal[(0, 0)], weights[0] / total);
    assert_close(compfinal[(1, 0)], weights[1] / total);
    assert_close(compfinal[(2, 0)], weights[2] / total);
    assert_close(compfinal[(3, 0)], weights[3] / total);
}
