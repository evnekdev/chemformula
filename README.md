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

- 🧪 Parse chemical formulas (e.g., `H2O`, `Al2O3`, etc.)
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
chemformula = "0.1"
```

---

### 2. Basic usage

```rust
use chemformula::parse;

fn main() {
    let formula = parse("H2O").unwrap();
    println!("{:?}", formula);
}
```

---

## 📊 Usage Examples

### Parsing a formula

```rust
let f = parse("Al2O3")?;
```

---

### Working with elements

```rust
use chemformula::element::Element;

// Example usage (depends on your API)
let oxygen = Element::new("O");
```

---

### Transforming formulas

```rust
use chemformula::transform;

// Example (conceptual)
let new_formula = transform::normalize(f);
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

## ⚠️ Limitations

- May not yet support:
  - Nested parentheses
  - Charges / ions
  - Isotopes
- API still evolving

---

## 📈 Roadmap

- [ ] Support nested formulas (e.g., `(NH4)2SO4`)
- [ ] Add molar mass calculations
- [ ] Improve parser robustness
- [ ] Add serialization (JSON, etc.)
- [ ] Expand documentation and examples

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
