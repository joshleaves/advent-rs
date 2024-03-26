## Day 01: Inverse Captcha

Welcome to another year. First day isn't too complicated, and the only play here is to avoid iterating more than once over a string.

## Day 02: Corruption Checksum

Nothing too hard either, if you know how iterations work. In that case, it's FASTER to only iterate once and store the `min` and `max` as you go along, rather than iterating, collecting, and iterating again to gain the `min`, then iterating once more to gain the `max`.

As for part 2, it's better to use `filter_map()` instead of chaining `filter` then `map` since our verification is based on doing a (costy) calculation.

## Day 03: Spiral Memory

Now we are getting somewhere!

Again, half the battle is knowing exactly what to really look for. While part 2 DOES force us to iterate through the field until we get to a result, part one can be solved by understaing you are faced with an [Ulam Spiral](https://en.wikipedia.org/wiki/Ulam_spiral). From there, a little Google can get you [prettY](https://oeis.org/A268038) [eXciting](https://oeis.org/A268038) [results](https://stackoverflow.com/a/61253346).

## Day 04: High-Entropy Passphrases

Knowing how to use iterators properly is often the key to winning.

## Day 05: A Maze of Twisty Trampolines, All Alike

I was expecting more difficulty here.

## Day 06: Memory Reallocation

No idea why, but I cannot get this one to go faster at all.

## Day 07: Recursive Circus

Little-known fact about me: I hate traversing trees.

That said, you can always find ways to avoid doing it :D

## Day 08: I Heard You Like Registers

Since we only iterate once with no jumps, no need to save instructions.

## Day 09: Stream Processing

The usual: do your best to only iter every character once. Rust's pattern-matching is really good there.

## Day 10: Knot Hash

Funny exercise, the second part was A BIT confusing at first, but it got okay.

## Day 11: Hex Ed

Navigating hexagons is a pain. Naive approach is to start with "only change the Y value if we move up, not down", but this will fail you in the long run. What you can do, is treat each hexagone as "two squares": moving up means `Y minus 2` and moving down means `Y plus 2`, while all diagonals work the way they would with a regular grid.

## Day 12: Digital Plumber

I hate traversing trees. But you don't always have to.
