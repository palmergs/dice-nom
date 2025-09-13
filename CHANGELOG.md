# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.0] - 2024-12-19

### Added
- Comprehensive documentation for all public APIs
- Module-level documentation with examples
- Documentation for `Generator`, `PoolGenerator`, `PoolOp`, and other core types
- Documentation for `Die`, `Value`, `Pool`, and `Results` types
- Parser module documentation with hierarchy explanation
- Examples demonstrating basic usage and gaming system applications
- Metadata for docs.rs including keywords and categories

### Changed
- Updated Cargo.toml with repository, documentation, and homepage links
- Added package metadata for better docs.rs presentation
- Improved code organization and documentation structure

### Fixed
- Compilation issues in documentation examples
- Type consistency in generator implementations

## [0.3.x] - Previous Versions

### Features
- Dice notation parsing with nom parser combinators
- Support for complex dice expressions (XdY notation)
- Exploding dice operations (!, !!, *, **)
- Pool manipulation (keep high/low/middle, advantage/disadvantage)
- Target number systems and success counting
- Arithmetic operations (+, -, implicit addition)
- Comparison operations (>, <, >=, <=, =, <=>)
- Success threshold systems
- Command-line interface with multiple output formats
- JSON serialization support
- Statistical analysis and charting capabilities

### Supported Operations
- Basic dice rolls (3d6, 1d20, etc.)
- Exploding dice with various modes
- Pool operations (take high, take low, take middle)
- Advantage and disadvantage mechanics
- Target-based success counting
- Threshold-based success levels
- Arithmetic combinations
- Dice pool comparisons

### Dice Operators
- `!` - Explode on maximum values
- `!!` - Explode until not maximum
- `*` - Explode each die on maximum
- `**` - Explode each die until not maximum
- `++n` - Add n to each die
- `--n` - Subtract n from each die
- `` `n `` - Keep lowest n dice
- `^n` - Keep highest n dice  
- `~n` - Keep middle n dice
- `ADV` - Advantage (roll twice, keep higher)
- `DIS` - Disadvantage (roll twice, keep lower)
- `Y` - Best group (keep largest matching set)

### Target Operators
- `[n]` - Count dice ≥ n as hits
- `(n)` - Count dice ≤ n as hits
- `{n}` - Success if total ≥ n
- `{n,m}` - Success levels (base n, +1 per m over)

### Command Line Features
- Multiple output formats (text, JSON, values, chart)
- Statistical analysis with histograms
- Batch rolling with repeat counts
- Expression validation and error reporting