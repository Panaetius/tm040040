# Changelog

## tm040040 [0.3.0](https://github.com/Panaetius/tm040040/tree/0.3.0) (2024-11-3)

### Added

Added check of hardware ready signal fo check if there's data to read.
This fixes trying to read in sleep mode where there won't be any i2c response if there's no data, 
which causes an error.

### Changed

### Fixed


## tm040040 [0.2.2](https://github.com/Panaetius/tm040040/tree/0.2.2) (2024-10-25)

### Added

### Changed

### Fixed

- fixed setting power mode to use `update_reg`

## tm040040 [0.2.0](https://github.com/Panaetius/tm040040/tree/0.2.0) (2024-10-04)

### Added

### Changed

- Changed to more ergonomic API using type state pattern
- Updated rustfmt rules and fixed formatting

### Fixed

## tm040040 [0.1.0](https://github.com/Panaetius/tm040040/tree/0.1.0) (2024-10-04)

### Added

- Initial support for TM040040
