# Notes for solving 2016
## Day 1: No Time for a Taxicab

Nice to get back into it!

A funny thing to note in `day_01_v2`: I thought that using `match direction {}` inside the `for _i in 1..=steps {}` loop would prove costly, but doing it outside the loop to store which part of `position` to modify and by how much, ends up being slower.

## Day 02: Bathroom Security

I must admit: the biggest pleasure in learning a new language is when it compiles right on your first time.

## Day 03:Squares With Three Sides

Iterators can be a special kind of hell in Rust, especially when combined, and each type specializes itself further, but in the end, they work.
