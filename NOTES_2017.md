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
