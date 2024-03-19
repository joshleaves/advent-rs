# advent-rs

[![dependency status](https://deps.rs/repo/github/joshleaves/advent-rs/status.svg)](https://deps.rs/repo/github/joshleaves/advent-rs)

As I said, [Consistency is hard](https://github.com/joshleaves/advent-rb), and I haven't taken the time to pick up a new language in quite a while.

I'm also adding notes that may be useful if you're (like me) discovering Rust:
- [2015, complete!](NOTES_2015.md)
- [2016, up to day 15](NOTES_2016.md)

# Regarding style rules
I'm gonna use a mix of what `cargo fmt` does, with some stuff that feels more natural to me.

# Testing guidelines
Strangely, testing feels more natural in Rust than in Ruby. Since I've [already done these exercises before](https://github.com/joshleaves/advent-rb), I will use [Test-driven Development](https://en.wikipedia.org/wiki/Test-driven_development) again to write the test, then make the code function.

# Benchmarking
I use [criterion.rs](https://github.com/bheisler/criterion.rs) for benchmarking, as it feels like the best option. I try to strive for the most effective solutions, without cutting corners. I could probably gain a lot more time in some spaces, but I'm going at my own speed for now.

The speeds I'm reporting are what my Mac M2 reports, depending on how many Chrome tabs I got opened. It's just to give myself a pat on the back.

# Documentation
I am playing with [RustDoc](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html) and am trying a few things. Maybe all sample tests will be moved into [documentation tests](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html), which I think is a very neat feature.
