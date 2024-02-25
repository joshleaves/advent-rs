# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and it kinda adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Of note:
- Major version is the year of advent calendar I'm doing, minor version is the latest exercise I pushed, and everything in-between is a patch.
- The changelog 2015.5.2 has been rewritten from each commit content.
- This file may be amended entirely in the future to adhere to the [GNU Changelog style](https://www.gnu.org/prep/standards/html_node/Style-of-Change-Logs.html#Style-of-Change-Logs)

## [2015.6.1]
### Added
- Solved [exercice for 2015, day 6](src/year_2015/day_06.rs).

## [2015.5.3]
### Added
- Using [cargo-mutants](https://github.com/sourcefrog/cargo-mutants) to check untested code.
- Adding a GitHub action.
### Changed
- All `.solve` methods return a `None` and only `main()` will be returning errors.

## [2015.5.2]
### Added
- Benchmark tests with [criterion.rs](https://github.com/bheisler/criterion.rs).
- Now following a strange version of SemVer.
### Changed
- Implementations have been rewritten in favor of faster execution times.
- Tests now use the macro [`include_str!`](https://doc.rust-lang.org/std/macro.include_str.html) for ease of reading.

## [2015.5.1]
### Added
- Solved [exercice for 2015, day 4](src/year_2015/day_04.rs).
- Solved [exercice for 2015, day 5](src/year_2015/day_05.rs).

## [2015.3.3]
### Added
- Unit test for the command line options.

## [2015.3.2]
### Changed
- Simpler way to read input from file or stdin.
### Removed
- No need for documentation tests...yet.

## [2015.3.1]
### Added
- Solved [exercice for 2015, day 3](src/year_2015/day_03.rs).

## [2015.2.1]
### Added
- Solved [exercice for 2015, day 2](src/year_2015/day_02.rs).

## [2015.1.1]
### Added
- Solved [exercice for 2015, day 1](src/year_2015/day_01.rs).
- Wrote a [binary to solve arbitrary inputs](src/main.rs).

## [2015.0.0]
### Added

- Initial project barebones structure.
- Set the tab format to two spaces.
