# Notes for solving 2015
## Day 01: Not Quite Lisp

<details>
<summary>📊Tests and benchmarks</summary>

```
test year_2015::day_01::tests::works_with_samples_v1 ... ok
test year_2015::day_01::tests::works_with_samples_v2 ... ok
test year_2015_day_01 ... ok

year_2015::day_01/year_2015::day_01_v1
                        time:   [5.1060 µs 5.1104 µs 5.1152 µs]
year_2015::day_01/year_2015::day_01_v2
                        time:   [1.4617 µs 1.4620 µs 1.4623 µs]

year_2015::day_01_v1/Slow/7000
                        time:   [59.965 µs 60.418 µs 60.893 µs]
year_2015::day_01_v1/Fast/7000
                        time:   [5.6145 µs 5.9122 µs 6.2267 µs]
```
</details>

<details>
<summary>Ruby version comments</summary>

> Ruby's `String` class is very well-furnished, even without all of Rail's `ActiveSupport` goodness. In that case, just using [`#count()`](https://apidock.com/ruby/String/count) is enough to get us out of trouble quickly.
> 
> I am pretty sure there must be an algorithm that doesn't include iterating through the whole string, but so far, the only idea I got would be to use bisecting until I get to the proper index, which just felt like a hassle.
</details>

First day wasn't very complicated. I chose to use `i16` out of pragmatic reasons, I don't believe Santa would go this high or this low.

I've added [a benchmark](benches/year_2015_day_01.rs) for this day. The first version had me naively reusing Ruby functionalities, when given Rust's ease of navigating memory, it was faster to just iterate through the data.

## Day 02: I Was Told There Would Be No Math

<details>
<summary>📊Tests and benchmarks</summary>

```
test year_2015::day_02::tests::works_with_samples_v1 ... ok
test year_2015::day_02::tests::works_with_samples_v2 ... ok
test year_2015_day_02 ... ok

year_2015::day_02/year_2015::day_02_v1
                        time:   [74.716 µs 74.839 µs 74.945 µs]
year_2015::day_02/year_2015::day_02_v2
                        time:   [74.636 µs 74.685 µs 74.727 µs]
```
</details>

<details>
<summary>Ruby version comments</summary>

> I..am not even sure this one was complicated in any way.
</details>

I used an object-like approach on this one. Using `(&self)` as he first argument wasn't too much of a stretch since [that's exactly how Ruby handles its Object-Oriented pattern in its C code](https://silverhammermba.github.io/emberb/c/#Methods).

I'm not sure yet whether `impl FromStr for PresentBox` was useful or whether I could have just made my own custom method, but I had fun trying it out. I also like the separation between `struct` and `impl`, very neat for converting from/to other C-like languages.

The most annoying part here is dealing with many integers size. Clearly everything here is `unsigned`, but while I'm sure my box dimensions all fit within `u8(0..255)`, the second I multiply them, I get into `u16(0..65_535)` range, and when I reach the results, I'm clearly overflowing up to `u32(0..4_294_967_295)` size. I would love to have each level work nicely with each other, but no, I have to manually cast.

## Day 03: Perfectly Spherical Houses in a Vacuum

<details>
<summary>📊Tests and benchmarks</summary>

```
test year_2015::day_03::tests::moves_characters_properly ... ok
test year_2015::day_03::tests::works_with_samples_v1 ... ok
test year_2015::day_03::tests::works_with_samples_v2 ... ok
test year_2015_day_03 ... ok

year_2015::day_03/year_2015::day_03_v1
                        time:   [415.91 µs 416.24 µs 416.60 µs]
year_2015::day_03/year_2015::day_03_v2
                        time:   [440.35 µs 441.72 µs 442.91 µs]

year_2015::day_03_v1/BTreeSet/8192
                        time:   [410.11 µs 411.30 µs 413.11 µs]
year_2015::day_03_v1/HashSet/8192
                        time:   [169.64 µs 169.95 µs 170.28 µs]
year_2015::day_03_v2/BTreeSet/8192
                        time:   [440.56 µs 441.85 µs 443.34 µs]
year_2015::day_03_v2/HashSet/8192
                        time:   [179.96 µs 180.06 µs 180.15 µs]
```
</details>

<details>
<summary>Ruby version comments</summary>

> The only thing to be wary of is on line 16: without the call to `#dup`, all of Santa's and Robo-Santa's positions will be overwritten, since Ruby's object model has a tendancy to pass references when you expect to pass values.
> 
> Passing by value or reference is a really wonky subject, but this [blog post](https://robertheaton.com/2014/07/22/is-ruby-pass-by-reference-or-pass-by-value/) got nice examples that will get you started.
</details>

Again, remember to clone your references before modifying, and everything will work out nicely.

This time, [the benchmark](benches/year_2015_day_03.rs) checks which of [`BTreeSet`](https://doc.rust-lang.org/stable/std/collections/struct.BTreeSet.html) or [`HashSet`](https://doc.rust-lang.org/stable/std/collections/hash_set/struct.HashSet.html) is faster. Funnily enough, while `HashSet` is faster at querying elements, it seems `BTreeSet` is [faster at removing them](https://github.com/ssomers/rust_bench_sets_compared). In our case, we only need to insert, so we can safely go with `HashSet`.

## Day 04:The Ideal Stocking Stuffer

<details>
<summary>📊Tests and benchmarks</summary>

```
test year_2015::day_04::tests::works_with_samples_v1 ... ignored
test year_2015::day_04::tests::works_with_samples_v2 ... ignored
test year_2015_day_04 ... ok

year_2015::day_04/year_2015::day_04_v1
                        time:   [65.402 ms 65.534 ms 65.640 ms]
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 18.9s.
year_2015::day_04/year_2015::day_04_v2
                        time:   [1.8911 s 1.8941 s 1.8969 s]
```
</details>

<details>
<summary>Ruby version comments</summary>

> Not gonna lie, brute-forcing [MD5 hashes](https://en.wikipedia.org/wiki/MD5) is not something interesting.
</details>

Of course, the bastard child had to be annoying, no matter what language we are in. We can actually win A BIT of time here, but we have to fiddle with bytes.

## Day 05: Doesn't He Have Intern-Elves For This?

<details>
<summary>📊Tests and benchmarks</summary>

```
test year_2015::day_05::tests::finds_nice_strings_v1 ... ok
test year_2015::day_05::tests::finds_nice_strings_v2 ... ok
test year_2015::day_05::tests::works_with_samples_v1 ... ok
test year_2015::day_05::tests::works_with_samples_v2 ... ok
test year_2015_day_05 ... ok

year_2015::day_05/year_2015::day_05_v1
                        time:   [142.32 µs 142.58 µs 142.91 µs]
year_2015::day_05/year_2015::day_05_v2
                        time:   [299.92 µs 300.33 µs 300.68 µs]
```
</details>

<details>
<summary>Ruby version comments</summary>

> Again, [Regexp](https://ruby-doc.org/core-2.5.1/Regexp.html) really are one of the best tools in your developer arsenal. In this specific exercise, we can look for repetition by using `\1`, which will reference a previously-captured group. Nothing specifically hard beyond that.
</details>

Another [benchmark](benches/year_2015_day_05.rs), and again it's about the performance of [`.contains()`](https://doc.rust-lang.org/std/primitive.str.html#method.contains). Using [`.try_fold()`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.try_fold) to return ASAP when any element doesn't satisfy our needs is also a nice touch.

## Day 06: Probably a Fire Hazard

<details>
<summary>📊Tests and benchmarks</summary>

```
test year_2015::day_06::tests::works_with_samples_v1 ... ok
test year_2015::day_06::tests::works_with_samples_v2 ... ok
test year_2015_day_06 ... ok

year_2015::day_06/year_2015::day_06_v1
                        time:   [13.269 ms 13.341 ms 13.423 ms]
year_2015::day_06/year_2015::day_06_v2
                        time:   [13.481 ms 13.585 ms 13.720 ms]
```
</details>

<details>
<summary>Ruby version comments</summary>

> This one actually gave me SOME trouble. My first solution was iterating on each element one by one and was clearly too long. Thanksfully, Ruby is really smart when it comes to replacing slices of an array.
> 
> There is an even more beautiful solution for part 2 that consist of only tracking the total numbers of flicks on/off/toggle, but in the off chance that a light already off is turned off again, the results would become false.
> 
> Also of note: remember what was discussed earlier about references? Well, the [documentation covers that too](https://ruby-doc.org/core-3.0.1/Array.html#class-Array-label-Creating+Arrays). Quote:
> 
> > Note that the second argument populates the array with references to the same object. Therefore, it is only recommended in cases when you need to instantiate arrays with natively immutable objects such as Symbols, numbers, true or false.
> >
> > To create an array with separate objects a block can be passed instead. This method is safe to use with mutable objects such as hashes, strings or other arrays:
</details>

I started doing a copy of my original "naive" algorithm, and as it was too slow, I decided to learn [how to pass closures in Rust](https://doc.rust-lang.org/book/ch13-01-closures.html).

## Day 07: Some Assembly Required

<details>
<summary>📊Tests and benchmarks</summary>

```
test year_2015::day_07::tests::works_with_samples_v1 ... ok
test year_2015::day_07::tests::works_with_samples_v2 ... ok
test year_2015_day_07 ... ok

year_2015::day_07/year_2015::day_07_v1
                        time:   [63.406 µs 63.509 µs 63.606 µs]
year_2015::day_07/year_2015::day_07_v2
                        time:   [129.92 µs 130.54 µs 131.09 µs]
```
</details>

<details>
<summary>Ruby version comments</summary>

> We already discovered bitwise operators in the previous exercises, so that shouldn't be too hard. The complication comes from building the wires.
> 
> The naive implementation, that works very well with the sample input, consists of interpreting each line one by one, storing the value of each wire every time. Unfortunately, not all inputs are indicated in a linear way.
> 
> The answer lies in to store all wires, setting up operations with [lazy evaluation](https://betterprogramming.pub/how-lazy-evaluation-works-in-ruby-a90237e99ac3), and letting intepretation work itself all the way back.
</details>

This one was already complicated in Ruby, but it gets even worse when you have to deal with [Rust's lifetimes](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html). The concept in itself is kinda okay to understand, but the way it has to be used sometimes makes no sense. I guess I'll get used to it with time. A [nice crate](https://docs.rs/advent-of-code/2022.0.66/src/advent_of_code/year2015/day07.rs.html) helped me see through it a bit more clearly.

## Day 08: Matchsticks

<details>
<summary>📊Tests and benchmarks</summary>

```
test year_2015::day_08::tests::calculates_length_of_code_strings ... ok
test year_2015::day_08::tests::calculates_length_of_memory_strings ... ok
test year_2015::day_08::tests::calculates_length_of_dumped_strings ... ok
test year_2015::day_08::tests::works_with_samples_v1 ... ok
test year_2015::day_08::tests::works_with_samples_v2 ... ok

year_2015::day_08/year_2015::day_08_v1
                        time:   [11.833 µs 11.907 µs 11.995 µs]
year_2015::day_08/year_2015::day_08_v2
                        time:   [8.3510 µs 8.6640 µs 9.1839 µs]
```
</details>

<details>
<summary>Ruby version comments</summary>

> Understanding how characters escaping works is a massive PAIN, and misunderstand the concept is a reason why [PHP MySQL injections](https://www.php.net/manual/en/security.database.sql-injection.php) were so infamous. Things get even more hairy when you have to work with MULTIPLE type of injections (paths, web, sql,...), or even multiple types of string that don't escape the same way,
> 
> In that case, we are lucky, since Ruby already implements [dump](https://ruby-doc.org/3.2.2/String.html#method-i-dump) and [undump](https://ruby-doc.org/3.2.2/String.html#method-i-undump), which happens to work exactly as the exercise require. But since we're here to learn, the methods will alternate at runtime between the Ruby methods and the manual implementation.
</details>

This one was actually very funny. For a while, I thought it would be a pain to not be able to index strings, but extracting slices actually works better.

## Day 09: All in a Single Night

<details>
<summary>📊Tests and benchmarks</summary>

```
test year_2015::day_09::tests::works_with_samples_v1 ... ok
test year_2015::day_09::tests::works_with_samples_v2 ... ok
test year_2015_day_09 ... ok

year_2015::day_09/year_2015::day_09_v1
                        time:   [4.1046 ms 4.1171 ms 4.1298 ms]
year_2015::day_09/year_2015::day_09_v2
                        time:   [4.1209 ms 4.1370 ms 4.1531 ms]
```
</details>

<details>
<summary>Ruby version comments</summary>

> Any programming school worth its salt will one day ask of you the shortest path between many points. Often, the idea is that you'll use graph theory and implement [Dijkstra's algorithm](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm). Sometimes, the school wants your brain for dinner and you'll be asked to further solve the [Travelling Salesman Problem](https://en.wikipedia.org/wiki/Travelling_salesman_problem). Both are very interested concepts in themselves, and a good first approach to [PathFinding](https://en.wikipedia.org/wiki/Pathfinding).
> 
> If you're not fond of graph transversal, the best answer is often to start using the [A* (A-Star)](https://en.wikipedia.org/wiki/A*_search_algorithm) algorithm to iterate through all possible paths.
> 
> You can then optimize it, for instance instructing the algorithm to stop searching once it's on a path longer than a previously explored one.
> 
> As for finding the "longest path"...just imagine you're not looking for a "shortest" or "longest" path, but a "best" path, and change how that path is selected among others.
</details>

This one got me a bit closer to understand how lifetimes function.

While my [more experienced counterpart](https://docs.rs/advent-of-code/2022.0.66/src/advent_of_code/year2015/day09.rs.html) used a cleaner perform-all-permutations approach, I wanted to keep the recursivity of my original algorithm, if only to force myself to make lifetimes work across recursion.

## Day 10: Elves Look, Elves Say

<details>
<summary>📊Tests and benchmarks</summary>

```
test year_2015::day_10::tests::looks_and_says_over_strings ... ok
test year_2015_day_10 ... ok

year_2015::day_10/year_2015::day_10_v1
                        time:   [2.7216 ms 2.7336 ms 2.7457 ms]
year_2015::day_10/year_2015::day_10_v2
                        time:   [37.291 ms 37.419 ms 37.551 ms]
```
</details>

<details>
<summary>Ruby version comments</summary>

> This exercise is based on John H. Conway's [Look-and-say sequences](https://en.wikipedia.org/wiki/Look-and-say_sequence), you probably know him for the [Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life), but the mathematician provided us with lot of science.
> 
> Nothing really hard in this exercise, except complexity quickly running high, it becomes important to use the best algorithm to generate the next sequence.
</details>

Casting between string, chars, bytes,... is slowly becoming more and more natural!

## Day 11: Corporate Policy

<details>
<summary>📊Tests and benchmarks</summary>

```
test year_2015::day_11::tests::passwords_are_valid ... ok
test year_2015::day_11::tests::works_with_samples_v1 ... ok
test year_2015_day_11 ... ok

year_2015::day_11/year_2015::day_11_v1
                        time:   [5.6672 ms 5.6736 ms 5.6797 ms]
year_2015::day_11/year_2015::day_11_v2
                        time:   [29.014 ms 29.036 ms 29.057 ms]
```
</details>

<details>
<summary>Ruby version comments</summary>

> Another exercise that is actually quite simple when you don't have to account for PERFORMANCE: the complexity in this exercise can quickly become huge and it's important that each of your iterations is as fast as you can. A good tool for that is [using Ruby's built-in Benchmark class](https://blog.appsignal.com/2018/02/27/benchmarking-ruby-code.html) to compare which of two implementations is the fastest.
> 
> Another thing to do is to use heuristics and pre-sanitization: in this exercise, you don't need to iterate and test from `iaaaaaaa` to `izzzzzzz`.
</details>

Nothing specific to add on this one. I'm just missing the optimization

## Day 12: JSAbacusFramework.io

<details>
<summary>📊Tests and benchmarks</summary>

```
test year_2015::day_12::tests::works_with_samples_v1 ... ok
test year_2015::day_12::tests::works_with_samples_v2 ... ok
test year_2015_day_12 ... ok

year_2015::day_12/year_2015::day_12_v1
                        time:   [24.003 µs 24.055 µs 24.112 µs]
year_2015::day_12/year_2015::day_12_v2
                        time:   [39.613 µs 39.848 µs 40.091 µs]
```
</details>

<details>
<summary>Ruby version comments</summary>

> That day looked so easy that I could not believe my eyes when I solved it in a simple one-liner: `3.0.0 :001 > pbpaste.scan(/-?\d+/).map(&:to_i).inject(&:+)`.
> 
> The second part is a bit more convoluted, but navigating JSON nodes isn't really a pain, you either have a Hash (explore), an Array (explore), or a value (return). Nothing too hard so far.
</details>

You know what's faster than using a Regex matcher on a string, or converting it to JSON before traversing it? Iterating though it only once at byte-level.
