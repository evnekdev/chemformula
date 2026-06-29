# Changelog

## [0.2.0] - 2026-Jun-29

### Added
 - New chemical formula parser, now accepts nested formulas (such as `KAl(SO4)2`) and charges (`Al[3+]`, `Al(OH)4[-]`)
 - Full-blown composition transformations in `Transform`, now support mole/gram conversions and fraction normalization, also extended compositions from vectors only to generic matrices (includes vectors too).
 - added `wmasses_initial` and `wmasses_final` methods to `Transform`
 
### Changed
  - Less cryptic methods of `Transform` structure
  - Avoiding costly pseudoinverse in `Transform`, always using cheap QR solve
  
### Fixed
  - `Debug` outputs of formula charges
  - Emply formulas output if the string inputs are incorrect - replaced by explicit errors.