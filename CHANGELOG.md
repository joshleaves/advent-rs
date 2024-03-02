# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and it kinda adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Of note:
- Major version is the year of advent calendar I'm doing, minor version is the latest exercise I pushed, and everything in-between is a patch.
- The changelog 2015.5.2 has been rewritten from each commit content.
- This file may be amended entirely in the future to adhere to the [GNU Changelog style](https://www.gnu.org/prep/standards/html_node/Style-of-Change-Logs.html#Style-of-Change-Logs)

## [2015.19.1]
### Added
- Solved [exercice for 2015, day 19](src/year_2015/day_19.rs).

## [2015.18.1]
### Added
- Solved [exercice for 2015, day 18](src/year_2015/day_18.rs).

## [2015.17.1]
### Added
- Solved [exercice for 2015, day 17](src/year_2015/day_17.rs).

## [2015.16.1]
### Added
- Solved [exercice for 2015, day 16](src/year_2015/day_16.rs).

## [2015.15.2]
### Added
- Added a (disappointingly) little macro to save time rewriting stuff.

## [2015.15.1]
### Added
- Solved [exercice for 2015, day 15](src/year_2015/day_15.rs).

## [2015.14.1]
### Added
- Solved [exercice for 2015, day 14](src/year_2015/day_14.rs).

## [2015.13.1]
### Added
- Solved [exercice for 2015, day 13](src/year_2015/day_13.rs).
- 
## [2015.12.3]
### Changed
- Documentation for 2015 day 1.
- Small imprivement for 2015 day 9.

## [2015.12.2]
### Changed
- All tests now take input from any `impl Into<String>` (so `String`, `&str`,...).
- Some optimizations there and there.
### Removed
- File samples for 2015, days 6 and 7.
- Method and tests for `code_line_len` on 2015 day 8.

## [2015.12.1]
### Added
- Solved [exercice for 2015, day 12](src/year_2015/day_12.rs).

## [2015.11.1]
### Added
- Solved [exercice for 2015, day 11](src/year_2015/day_11.rs).

## [2015.10.1]
### Added
- Solved [exercice for 2015, day 10](src/year_2015/day_10.rs).

## [2015.9.1]
### Added
- Solved [exercice for 2015, day 9](src/year_2015/day_09.rs).

## [2015.8.2]
### Changed
- Rewrote [solving notes for 2015](2015.md).

## [2015.8.1]
### Added
- Solved [exercice for 2015, day 8](src/year_2015/day_08.rs).

## [2015.7.2]
### Changed
- Fixed `.solve` to solve year 2015, day 7.

## [2015.7.1]
### Added
- Solved [exercice for 2015, day 7](src/year_2015/day_07.rs).

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
