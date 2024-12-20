## Day 01: Inverse Captcha

Welcome to another year. First day isn't too complicated, and the only play here is to avoid iterating more than once over a string.

## Day 02: Corruption Checksum

Nothing too hard either, if you know how iterations work. In that case, it's FASTER to only iterate once and store the `min` and `max` as you go along, rather than iterating, collecting, and iterating again to gain the `min`, then iterating once more to gain the `max`.

As for part 2, it's better to use `filter_map()` instead of chaining `filter` then `map` since our verification is based on doing a (costly) calculation.

## Day 03: Spiral Memory

Now we are getting somewhere!

Again, half the battle is knowing exactly what to really look for. While part 2 DOES force us to iterate through the field until we get to a result, part one can be solved by understating you are faced with an [Ulam Spiral](https://en.wikipedia.org/wiki/Ulam_spiral). From there, a little Google can get you [prettY](https://oeis.org/A268038) [eXciting](https://oeis.org/A268038) [results](https://stackoverflow.com/a/61253346).

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

The usual: do your best to only iterate every character once. Rust's pattern-matching is really good there.

## Day 10: Knot Hash

Funny exercise, the second part was A BIT confusing at first, but it got okay.

## Day 11: Hex Ed

Navigating hexagons is a pain. Naive approach is to start with "only change the Y value if we move up, not down", but this will fail you in the long run. What you can do, is treat each hexagon as "two squares": moving up means `Y minus 2` and moving down means `Y plus 2`, while all diagonals work the way they would with a regular grid.

## Day 12: Digital Plumber

I hate traversing trees. But you don't always have to.

## Day 13: Packet Scanners

The maths were a bit annoying on this one, and I was hoping for a specific mathematical formula that would prevent me from looping over all scenarios. I found something that felt promising and ported the code from Python, but to no avail, my original code was already faster...

## Day 14: Disk Defragmentation

I kinda liked this one.

## Day 15: Dueling Generators

The most interesting part of these exercises is often [all the obscure maths you can learn about](https://www.reddit.com/r/adventofcode/comments/7jyz5x/2017_day_15_opportunities_for_optimization/drasfzr/?context=3), in that case [Mersenne prime](https://en.wikipedia.org/wiki/Mersenne_prime). But the peculiarities of the number used for the remainder [should have been obvious to people who wrote a lot of C](https://doc.rust-lang.org/std/i32/constant.MAX.html).

## Day 16: Permutation Promenade

Nothing hard, until you see how much ONE BILLION ITERATIONS actually is. Good news though: "All loops are about knowing when to stop". In that case, we can just...look for a repetition of the pattern.

## Day 17: Spinlock

As always, there's a trick to avoid looping too much: since `0` is ALWAYS at the 0th position, we just have to store the last number we are inserting  (allegedly, no need to allocate a real Vec) after it.

## Day 18: Duet

Threading is a bitch.

## Day 19: A Series of Tubes

Worse than maps? Maps that aren't reliably readable by machines.

## Day 20: Particle Swarm

Nothing too hard here.

## Day 21: Fractal Art

That one was both funny and very annoying. Playing with patterns and turning them into fractals wasn't easy.

## Day 22: Sporifica Virus

Very funny exercise, but I'll spend a long time trying to optimize it.

## Day 23: Coprocessor Conflagration

This exercise [is actually a trap](https://www.youtube.com/watch?v=4F4qzPbcFiA): running it as expected would last you around ten minutes. Were you to [rewrite it as pseudo-code](https://docs.rs/advent-of-code/2022.0.66/src/advent_of_code/year2017/day23.rs.html#15), you would see another algorithm, that is [way easier to implement](https://github.com/galenelias/AdventOfCode_2017/blob/master/src/Day23/mod.rs#L70).

##  Day 24: Electromagnetic Moat

Your usual shortest-path algorithm.

## Day 25: The Halting Problem

Implementing a [Turing machine](https://en.wikipedia.org/wiki/Turing_machine) isn't that hard once you get to the principle. Hardest part was actually parsing the input.
