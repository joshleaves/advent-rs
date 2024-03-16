# Notes for solving 2016
## Day 1: No Time for a Taxicab

Nice to get back into it!

A funny thing to note in `day_01_v2`: I thought that using `match direction {}` inside the `for _i in 1..=steps {}` loop would prove costly, but doing it outside the loop to store which part of `position` to modify and by how much, ends up being slower.

## Day 02: Bathroom Security

I must admit: the biggest pleasure in learning a new language is when it compiles right on your first time.

## Day 03:Squares With Three Sides

Iterators can be a special kind of hell in Rust, especially when combined, and each type specializes itself further, but in the end, they work.

## Day 04: Security Through Obscurity

Jumping around between `char`, `u8`, and `i32` is a bit of a pain, but it's not too hard once you get it.

Interestingly, when calculating the checksum, it is FASTER to reiterate over the string for each new character we find, rather than query the index straight from a HashMap.

## Day 05: How About a Nice Game of Chess?

Hey look, it's those pesky [MD5 hashes](https://en.wikipedia.org/wiki/MD5) again!

For efficiency purposes, the [MD5 crate that I am using](https://crates.io/crates/md-5) is returning hashes in their hexadecimal value, so when humans are expecting to get 32 characters, the digest is returned as 16 `u8` values (reminder, an `u8` goes from `0` to `255` so it can store `2 (* 16 = 256)` hexadecimal values). It's a bit of a mental gymnastic to juggle with afterwards, but it's not impossible either.
