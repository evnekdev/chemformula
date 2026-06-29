# chemformula

> A Rust library for parsing, representing, and transforming chemical formulas.

![Rust](https://img.shields.io/badge/language-Rust-orange)
![Status](https://img.shields.io/badge/status-active-green)

---

## 📌 Overview

**chemformula** is a Rust crate for working with chemical formulas, providing tools for:

- Parsing chemical formula strings
- Representing formulas as structured data
- Transforming and manipulating formulas
- Working with elements and compositions

It is designed for:

- Chemistry-related software
- Scientific computing
- Data processing pipelines involving chemical compositions

---

## ✨ Features

- 🧪 Parse chemical formulas (e.g., `H2O`, `Al2O3`, `Al(OH)4[-]`, etc.)
- 🧱 Structured representation of formulas
- 🔄 Formula transformation utilities
- 🧬 Element-level abstraction
- ⚡ Lightweight and fast
- 🧠 Designed for extensibility

---

## 🛠️ Tech Stack

- Language: **Rust**
- No heavy dependencies (lightweight core)

---

## 📂 Project Structure

```
src/
├── lib.rs         # Library entry point
├── main.rs        # Example / CLI entry
├── formula.rs     # Core formula representation
├── element.rs     # Element definitions
├── parse.rs       # Parsing logic
├── transform.rs   # Formula transformations
```

---

## 🚀 Getting Started

### 1. Add dependency

```toml
[dependencies]
chemformula = "0.2"
```

---

### 2. Basic usage (parsing a formula)

```rust
use std::str::FromStr;
use chemformula::Formula;

pub fn main(){
	let name = "Al(OH)4[-]";
	let frm = Formula::from_str(name).unwrap();
	println!("name = {:?}, formula = {:?}, wmass = {:?}", &name, &frm, frm.wmass());
}
```

---

## 📊 Usage Examples

Please refer to the `/examples` folder to see how the crate functionality works.

### Working with elements

```rust
use chemformula::Element;

// Example usage (depends on your API)
let oxygen = Element::from_str("O").unwrap();
```

---

### Transforming formulas

```rust
use std::str::FromStr;
use nalgebra::{dvector};

use chemformula::Formula;
use chemformula::Transform;

pub fn main(){
	let binitial = vec!["CaO", "Al2O3", "SiO2"];
	let bfinal   = vec!["Ca", "Al", "Si", "O"];
	let transform = Transform::new(&binitial, &bfinal, false).unwrap();
	let compinitial = dvector![0.4, 0.5, 0.1]; // moles, initial
	let compfinal = transform.transform_init2final(&compinitial, false, false, false);
	println!("FINAL COMPOSITION (moles) = {:.6?}", &compfinal.data.as_slice());
	let compfinal = transform.transform_init2final(&compinitial, false, false, true);
	println!("FINAL COMPOSITION (Xfraction) = {:.6?}", &compfinal.data.as_slice());
	let compfinal = transform.transform_init2final(&compinitial, false, true, false);
	println!("FINAL COMPOSITION (grams) = {:.6?}", &compfinal.data.as_slice());
	let compfinal = transform.transform_init2final(&compinitial, false, true, true);
	println!("FINAL COMPOSITION (Wfraction) = {:.6?}", &compfinal.data.as_slice());
}
```

---

## 🧩 Core Modules

### `parse.rs`
Handles conversion from string → structured formula.

---

### `formula.rs`
Defines internal representation of a chemical formula.

---

### `element.rs`
Represents chemical elements.

---

### `transform.rs`
Provides utilities for modifying formulas.

---

## 🧪 Testing

```bash
cargo test
```

---

## 🤝 Contributing

Contributions are welcome!

1. Fork the repo  
2. Create a branch  
3. Commit changes  
4. Open a PR  

---

## 🐛 Issues

https://github.com/evnekdev/chemformula/issues

---

## 📄 License

MIT

---

## 📬 Contact

**Evgenii Nekhoroshev**  
https://github.com/evnekdev
