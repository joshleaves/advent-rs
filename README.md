# advent-rs

[![dependency status](https://deps.rs/repo/github/joshleaves/advent-rs/status.svg)](https://deps.rs/repo/github/joshleaves/advent-rs)

As I said, [Consistency is hard](https://github.com/joshleaves/advent-rb), and I haven't taken the time to pick up a new language in quite a while.

I'm also adding notes that may be useful if you're discovering Rust (like I am).

Notes for solving:
* [2015, up to day 08](year_2015.md)

# Regarding style rules
I'm gonna use a mix of what `cargo fmt` does, with some stuff that feels more natural to me.

# Testing guidelines
Strangely, testing feels more natural in Rust than in Ruby. Since I've [already done these exercises before](https://github.com/joshleaves/advent-rb), I will use [Test-driven Development](https://en.wikipedia.org/wiki/Test-driven_development) again to write the test, then make the code function.

# Benchmarking
I use [criterion.rs](https://github.com/bheisler/criterion.rs) for benchmarking, as it feels like the best option. I try to strive for the most effective solutions, without cutting corners. I could probably gain a lot more time in some spaces, but I'm going at my own speed for now.
