# Notes for solving 2018
## Day 01: Chronal Calibration

Again, first day is nothing to write home about: just iterate and sum, nothing too complicated.

## Day 02: Inventory Management System

Oh look, another of those "loop over EVERY-FUCKING-COMBINATION" exercises...

## Day 03: No Matter How You Slice It

Not much we haven't seen last year.

## Day 04: Repose Record

There's something about this "find the most used record" type of exercises that I dislike.

## Day 05: Alchemical Reduction

As a programmer, your MUST have used, at least once, the command [`man ascii`](https://man.archlinux.org/man/core/man-pages/ascii.7.en), if only to find the integer value of a specific number. But were you to look specifically at the hexadecimal table, something may have caught your attention: uppercase letters, and their equivalent lowercase values, are all separated by 32 characters. Now, isn't that a nice number that plays VERY WELL with bit masks? Here's a little [playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=d9a88346debfc3c2470c6b1527202f30) to help you!
