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

## Day 06: Signals and Noise

Playing with specific types for closures can be a pain, but it's often very funny to understand how things work under the hood.

## Day 07: Internet Protocol Version 7

Some iterators are annoying, but sometimes, like when calculating whethere there is an interesection of two sets, it's great to not generate the whole intersection and just peek at the first element.

## Day 08: Two-Factor Authentication

I wouldn't call it an "object model", but I like the way Rust works with `impl X for Y`.

Also, remember the [`splice()` method](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.splice), it's very useful to replace large parts of a vector at once.

## Day 09: Explosives in Cyberspace

I knew the Rust version would be faster than my [Ruby implementation](https://github.com/joshleaves/advent-rb/blob/master/year_2016/day_09.rb), but I still felt I wasn't fast enough. I compared myself to [galenelias's implementation](https://github.com/galenelias/AdventOfCode_2016/blob/master/src/Day9/mod.rs) and indeed I wasn't fast enough.

I went back to the workbench to try another implementation, and to my delight, I was the faster. But still not faster than [fornwall's implementation](https://github.com/fornwall/advent-of-code/blob/main/crates/core/src/year2016/day09.rs).

Looking at it carefully proved very smart: both galenelias and me lost too much time **creating a string, then getting its length**, when the exercise only asked to **get the string's length**.

## Day 10: Balance Bots

This exercise got me to face the worst part of Rust: iterating over a hashmap while mutating it.

## Day 11: Radioisotope Thermoelectric Generators

Ugh. I hated this just as much as the first time.

## Day 12: Leonardo's Monorail

Finally, we are back into interesting exercises!

## Day 13: A Maze of Twisty Little Cubicles

...

## Day 14: One-Time Pad

Computing MD5 hashes is never interesting.

## Day 15: Timing is Everything

Finally, something funny to do.
