# Notes for solving 2015
## Day 01: Not Quite Lisp

<details>
<summary>📊Tests and benchmarks</summary>

```
test year_2015::day_01::tests::works_with_samples_v1 ... ok
test year_2015::day_01::tests::works_with_samples_v2 ... ok
test year_2015::tests::day_01 ... ok

year_2015::day_01/year_2015::day_01_v1
                        time:   [6.2931 µs 6.8094 µs 7.3834 µs]
year_2015::day_01/year_2015::day_01_v2
                        time:   [1.9841 µs 1.9896 µs 1.9955 µs]
year_2015::day_01_v1/Naive/7000
                        time:   [63.548 µs 64.204 µs 64.821 µs]
year_2015::day_01_v1/Fast/7000
                        time:   [5.6477 µs 5.8044 µs 6.0121 µs]
```
</details>

<details>
<summary>Ruby version comments</summary>

> Ruby's `String` class is very well-furnished, even without all of Rail's `ActiveSupport` goodness. In that case, just using [`#count()`](https://apidock.com/ruby/String/count) is enough to get us out of trouble quickly.
> 
> I am pretty sure there must be an algorithm that doesn't include iterating through the whole string, but so far, the only idea I got would be to use bisecting until I get to the proper index, which just felt like a hassle.
</details>

First day wasn't very complicated. I chose to use `i16` out of pragmatic reasons, I don't believe Santa would go this high or this low.

I've added [a benchmark](benches/year_2015_day_01.rs) for this day. ~~The first version had me naively reusing Ruby functionalities, when given Rust's ease of navigating memory, it was faster to just iterate through the data.~~

There is an EVEN BIGGER optimization that can be done if you treat your string as a bunch of u16 instead of u8.

## Day 02: I Was Told There Would Be No Math

<details>
<summary>📊Tests and benchmarks</summary>

```
test year_2015::day_02::tests::works_with_samples_v1 ... ok
test year_2015::day_02::tests::works_with_samples_v2 ... ok
test year_2015::tests::day_02 ... ok

year_2015::day_02/year_2015::day_02_v1
                        time:   [74.835 µs 74.904 µs 74.977 µs]
year_2015::day_02/year_2015::day_02_v2
                        time:   [74.878 µs 74.969 µs 75.067 µs]
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
test year_2015::tests::day_03 ... ok

year_2015::day_03/year_2015::day_03_v1
                        time:   [232.27 µs 232.77 µs 233.35 µs]
year_2015::day_03/year_2015::day_03_v2
                        time:   [250.97 µs 251.71 µs 252.48 µs]
year_2015::day_03_v1/BTreeSet/8192
                        time:   [401.94 µs 402.21 µs 402.50 µs]
year_2015::day_03_v1/HashSet/8192
                        time:   [234.20 µs 234.41 µs 234.64 µs]
year_2015::day_03_v2/BTreeSet/8192
                        time:   [432.48 µs 432.72 µs 432.98 µs]
year_2015::day_03_v2/HashSet/8192
                        time:   [252.37 µs 252.70 µs 253.00 µs]
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
test year_2015::day_04::tests::works_with_samples_v1 ... ok
test year_2015::day_04::tests::works_with_samples_v2 ... ok
test year_2015::tests::day_04 ... ok

year_2015::day_04/year_2015::day_04_v1
                        time:   [36.404 ms 36.439 ms 36.457 ms]
Warning: Unable to complete 10 samples in 5.0s. You may wish to increase target time to 10.6s.
year_2015::day_04/year_2015::day_04_v2
                        time:   [1.0535 s 1.0541 s 1.0545 s]
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
test year_2015::tests::day_05 ... ok

year_2015::day_05/year_2015::day_05_v1
                        time:   [51.142 µs 51.447 µs 51.928 µs]

year_2015::day_05/year_2015::day_05_v2
                        time:   [181.19 µs 181.42 µs 181.68 µs]

year_2015::day_05_v1/contains/17000
                        time:   [95.128 µs 95.245 µs 95.361 µs]
year_2015::day_05_v1/chars/17000
                        time:   [50.388 µs 50.474 µs 50.563 µs]
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
test year_2015::tests::day_06 ... ok

year_2015::day_06/year_2015::day_06_v1
                        time:   [13.186 ms 13.320 ms 13.522 ms]
year_2015::day_06/year_2015::day_06_v2
                        time:   [13.437 ms 13.466 ms 13.499 ms]
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
test year_2015::tests::day_07 ... ok

year_2015::day_07/year_2015::day_07_v1
                        time:   [62.800 µs 62.873 µs 62.940 µs]
year_2015::day_07/year_2015::day_07_v2
                        time:   [127.35 µs 127.56 µs 127.82 µs]
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
                        time:   [13.389 µs 13.434 µs 13.486 µs]
year_2015::day_08/year_2015::day_08_v2
                        time:   [8.7562 µs 8.8298 µs 8.9084 µs]
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
test year_2015::tests::day_09 ... ok

year_2015::day_09/year_2015::day_09_v1
                        time:   [1.2927 ms 1.2995 ms 1.3064 ms]
year_2015::day_09/year_2015::day_09_v2
                        time:   [4.0988 ms 4.1071 ms 4.1157 ms]
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
test year_2015::tests::day_10 ... ok

year_2015::day_10/year_2015::day_10_v1
                        time:   [870.36 µs 872.61 µs 875.35 µs]
year_2015::day_10/year_2015::day_10_v2
                        time:   [12.131 ms 12.153 ms 12.177 ms]
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
test year_2015::tests::day_11 ... ok

year_2015::day_11/year_2015::day_11_v1
                        time:   [4.8713 ms 4.8800 ms 4.8927 ms]
year_2015::day_11/year_2015::day_11_v2
                        time:   [24.898 ms 24.958 ms 25.035 ms]
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
test year_2015::tests::day_12 ... ok

year_2015::day_12/year_2015::day_12_v1
                        time:   [17.760 µs 17.955 µs 18.183 µs]
year_2015::day_12/year_2015::day_12_v2
                        time:   [30.888 µs 31.305 µs 31.736 µs]
```
</details>

<details>
<summary>Ruby version comments</summary>

> That day looked so easy that I could not believe my eyes when I solved it in a simple one-liner: `3.0.0 :001 > pbpaste.scan(/-?\d+/).map(&:to_i).inject(&:+)`.
> 
> The second part is a bit more convoluted, but navigating JSON nodes isn't really a pain, you either have a Hash (explore), an Array (explore), or a value (return). Nothing too hard so far.
</details>

You know what's faster than using a Regex matcher on a string, or converting it to JSON before traversing it? Iterating though it only once at byte-level.


## Day 13: Knights of the Dinner Table

<details>
<summary>📊Tests and benchmarks</summary>

```
test year_2015::day_13::tests::parses_input_lines ... ok
test year_2015::day_13::tests::works_with_samples_v1 ... ok
test year_2015::tests::day_13 ... ok

year_2015::day_13/year_2015::day_13_v1
                        time:   [46.791 ms 47.177 ms 47.561 ms]
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 9.9s, or reduce sample count to 50.
year_2015::day_13/year_2015::day_13_v2
                        time:   [86.755 ms 87.306 ms 87.891 ms]
```
</details>

<details>
<summary>Ruby version comments</summary>

> Again, refer to [Day 09](##day-09) because we are in a similar configuration, with the exception that we must rejoin our starting point at the end. Nothing too complicated.
> 
> Second part is just adding another node with a 0 relationship to all nodes. Surprisingly, it seems our presence lowered the general happiness...
</details>

Why do these people even bother having a Christmas dinner together if they hate each other (and us) so much? On the Rust front, beginning to understand ownership a little better, I managed to write this one without resorting to lifetimes. Just makes everything more readable.

Something I also gained over my Ruby implementation is saving a loop: since we are looping back to our initial point at the end of each cycle, a permutation like `[0, 1, 2, 3, 4]` will have the same value as `[1, 2, 3, 4, 0]`. Therefore, we only need to generate (brute-force?) sequences starting with our first element. Once we reach our first depth, we can iterate over all available results.


## Day 14: Reindeer Olympics

<details>
<summary>📊Tests and benchmarks</summary>

```
test year_2015::day_14::tests::works_with_samples_v1 ... ok
test year_2015::day_14::tests::works_with_samples_v2 ... ok
test year_2015::tests::day_14 ... ok

year_2015::day_14/year_2015::day_14_v1
                        time:   [3.5302 ms 3.5346 ms 3.5403 ms]
year_2015::day_14/year_2015::day_14_v2
                        time:   [3.5312 ms 3.5359 ms 3.5412 ms]
```
</details>

<details>
<summary>Ruby version comments</summary>

> This day doesn't have complicated concepts. If you want to dig, you can think of each reindeer as a [finite-state machine](https://en.wikipedia.org/wiki/Finite-state_machine), which is [one of the concepts that are best solved thanks to Object-oriented programming](https://eev.ee/blog/2013/03/03/the-controller-pattern-is-awful-and-other-oo-heresy/).
</details>

Nothing too different here.


## Day 15: Science for Hungry People

<details>
<summary>📊Tests and benchmarks</summary>

```
test year_2015::day_15::tests::works_with_samples_v1 ... ok
test year_2015::day_15::tests::works_with_samples_v2 ... ok
test year_2015::tests::day_15 ... ok

year_2015::day_15/year_2015::day_15_v1
                        time:   [35.365 ms 35.613 ms 35.944 ms]
year_2015::day_15/year_2015::day_15_v2
                        time:   [35.463 ms 35.498 ms 35.539 ms]
```
</details>

<details>
<summary>Ruby version comments</summary>

> Is there a day where you don't have to iterate through all possible solutions?
> 
> I am (partly) joking. I am pretty sure there must be a way to solve n-sided equations, but quite frankly, there's a reason I've always thought of programming as "languages" and not "mathematics". Again, the best thing we can do to avoid an `O(n*n)` complexity is finding ways to avoid calculations we know will result in a 0 anywhere.
> 
> So far, I mainly used loops and other control flow structures, but I decided to go with an [`Enumerator`](https://ruby-doc.org/core-3.0.1/Enumerator.html) for this one. No specific change or speedup, but it's the kind of structure we should think of building more often, as [`Enumerable`](https://ruby-doc.org/core-3.0.1/Enumerable.html) is the class that made me fall in love with Ruby.
</details>

This one was much harder.

After trying many times to use my previous algorithm to mark ranges where numbers were "forbidden", things got heated, and in the end, I reworked something myself by [following a proper implementation](https://docs.rs/advent-of-code/2022.0.66/src/advent_of_code/year2015/day15.rs.html). However, being in Rust makes the execution of all these loops way faster than it used to be in Ruby.

## Day 16: Aunt Sue

<details>
<summary>📊Tests and benchmarks</summary>

```
test year_2015::tests::day_16 ... ok

year_2015::day_16/year_2015::day_16_v1
                        time:   [115.32 µs 115.43 µs 115.53 µs]
year_2015::day_16/year_2015::day_16_v2
                        time:   [81.179 µs 81.202 µs 81.226 µs]
```
</details>

<details>
<summary>Ruby version comments</summary>

> See, when I say Ruby's `Enumerable` is the perfect, this is the kind of problems I'm talking about.
</details>

This one was already easy in Ruby, and it's just as easy in Rust.

## Day 17: No Such Thing as Too Much

<details>
<summary>📊Tests and benchmarks</summary>

```
test year_2015::day_17::tests::works_with_samples_v1 ... ok
test year_2015::day_17::tests::works_with_samples_v2 ... ok
test year_2015::tests::day_17 ... ok

year_2015::day_17/year_2015::day_17_v1
                        time:   [9.3768 ms 9.3931 ms 9.4143 ms]
year_2015::day_17/year_2015::day_17_v2
                        time:   [9.3911 ms 9.4139 ms 9.4419 ms]
```
</details>

<details>
<summary>Ruby version comments</summary>

> Nothing really complicated here, or that we haven't seen yet.
</details>

Heh, I'm getting to used to this stuff that it compiled on the second try, and passed the tests on the second. Reworking my Ruby algorithm into Ruby wasn't complicated either.


## Day 18: Like a GIF For Your Yard

<details>
<summary>📊Tests and benchmarks</summary>

```
test year_2015::day_18::tests::works_with_samples_v1 ... ok
test year_2015::day_18::tests::works_with_samples_v2 ... ok
test year_2015::tests::day_18 ... ok

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 5.4s, or reduce sample count to 90.
year_2015::day_18/year_2015::day_18_v1
                        time:   [54.055 ms 54.104 ms 54.155 ms]

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 6.5s, or reduce sample count to 70.
year_2015::day_18/year_2015::day_18_v2
                        time:   [64.643 ms 64.735 ms 64.828 ms]
```
</details>

<details>
<summary>Ruby version comments</summary>

> upon [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) earlier, and this exercise is a good example of it.
> 
> I thought I was being smart by only keeping track of positions of only ON lights, which was a good implementation for a six-by-six data set, but proved UTTERLY SLOW on the final data set. Using an array of strings (as is the most naive implementation) is indeed the fastest way to do it. I still kept the old code commented.
</details>

That was fun.

My [distinguished competitor](https://docs.rs/advent-of-code/2022.0.66/src/advent_of_code/year2015/day18.rs.html) used a single-dimensional vector approach, which may have a better performance, but quite frankly, I prefer having representation closer to "physical" reality. But it's just me, don't listen to me too much.

## Day 19: Medicine for Rudolph

<details>
<summary>📊Tests and benchmarks</summary>

```
test year_2015::day_19::tests::works_with_samples_v1 ... ok
test year_2015::day_19::tests::works_with_samples_v2 ... ok
test year_2015::tests::day_19 ... ok

year_2015::day_19/year_2015::day_19_v1
                        time:   [565.05 µs 565.71 µs 566.40 µs]
year_2015::day_19/year_2015::day_19_v2
                        time:   [6.0105 µs 6.0169 µs 6.0237 µs]
```
</details>

<details>
<summary>Ruby version comments</summary>

> Part one is nothing to write home about. However, part two... First idea was the good ol' bruteforce approach, but when searching if there was an algorithm I didn't know, I stumbled upon a [very interesting Reddit comment](https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/cy4h7ji/)...
</details>

Okay, that was less painful than I remembered it, and my Rust code is actually clearer than my Ruby code.

## Day 20: Infinite Elves and Infinite Houses

<details>
<summary>📊Tests and benchmarks</summary>

```
test year_2015::day_20::tests::works_with_samples_v1 ... ok
test year_2015::tests::day_20 ... ok

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 45.9s, or reduce sample count to 10.
year_2015::day_20/year_2015::day_20_v1
                        time:   [454.92 ms 455.72 ms 456.56 ms]
Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 50.1s, or reduce sample count to 10.
year_2015::day_20/year_2015::day_20_v2
                        time:   [497.94 ms 498.66 ms 499.45 ms]
```
</details>

<details>
<summary>Ruby version comments</summary>

> I've had to calculate list of primes and divisors in the past, and it always sucked. My previous algorithms were correct but always too long (but what is "long" when you're nearing infinity?).
> 
> Again, [outside help](https://dmatrix.dev/posts/advent-of-code-year-2015-day-20/) is a great resource to find algorithms better than yours.
</details>

I remembered this one was A PIECE OF SHIT to run in Ruby, as it lasted around over minute on my machine the last time I ran it. Having it run in only one second here is a real pleasure. Some issues flip-flapping between types, but nothing too brutal.

## Day 21: RPG Simulator 20XX

<details>
<summary>📊Tests and benchmarks</summary>

```
test year_2015::day_21::tests::wins_the_fight_with_samples_v1 ... ok
test year_2015::tests::day_21 ... ok

year_2015::day_21/year_2015::day_21_v1
                        time:   [4.8485 µs 4.8549 µs 4.8618 µs]
year_2015::day_21/year_2015::day_21_v2
                        time:   [4.8473 µs 4.8539 µs 4.8609 µs]
```
</details>

<details>
<summary>Ruby version comments</summary>

> ...really nothing to say here.
</details>

Couldn't believe this one is running so fast.

## Day 22: Wizard Simulator 20XX

<details>
<summary>📊Tests and benchmarks</summary>

```
test year_2015::tests::day_22 ... ok

year_2015::day_22/year_2015::day_22_v1
                        time:   [18.188 ms 18.206 ms 18.225 ms]
year_2015::day_22/year_2015::day_22_v2
                        time:   [29.689 ms 29.734 ms 29.782 ms]
```
</details>

<details>
<summary>Ruby version comments</summary>

> ["Any sufficiently advanced bruteforce is indistinguishable from Dijkstra's algorithm"](https://en.wikipedia.org/wiki/Clarke%27s_three_laws)
> 
> As for this exercise, a key lesson is: make commits BEFORE you rewrite your code. In that occasion, rewriting my code made it unworkable, no matter how many times I tried, and I must have made the same input mistake many times because my second day results were always off by twenty, even when [carefully copying an algorithm that works](https://github.com/rHermes/adventofcode/blob/master/2015/22/y2015_d22_p02.py). In the end, it all boiled down to an off-by-one error.
</details>

I just hate this one, let's get it over with.

## Day 23: Opening the Turing Lock

<details>
<summary>📊Tests and benchmarks</summary>

```
test year_2015::day_23::tests::iterates_over_program ... ok
test year_2015::tests::day_23 ... ok

ar_2015::day_23/year_2015::day_23_v1
                        time:   [6.7568 µs 6.7660 µs 6.7749 µs]
year_2015::day_23/year_2015::day_23_v2
                        time:   [4.8121 µs 4.8186 µs 4.8261 µs]
```
</details>

<details>
<summary>Ruby version comments</summary>

> We are not [Turing complete](https://en.wikipedia.org/wiki/Turing_completeness) yet, but learning to write a virtual machine is always a fun exercise to understand how computers work.
> 
> As a small bonus on this exercise, instead of [re-interpreting each instruction as they come](https://github.com/rHermes/adventofcode/blob/master/2015/23/y2015_d23_p01.py), which can be costly on the long term, I used an approach similar to [Ahead-of-time compilation](https://en.wikipedia.org/wiki/Ahead-of-time_compilation) to have all my instructions in ~~machine code~~ Ruby code and ready to reuse as needed.
> 
> That said: make sure you ALWAYS read the exercise WORD. BY. WORD. Because if `jie` means "jump-if-even", it does not mean `jio` will mean "jump-if-odd".
</details>

Heh, I love this one, whether it's in Ruby or in Rust.

## Day 24: It Hangs in the Balance

<details>
<summary>📊Tests and benchmarks</summary>

```
test year_2015::day_24::tests::works_with_samples_v1 ... ok
test year_2015::day_24::tests::works_with_samples_v2 ... ok
test year_2015::tests::day_24 ... ok

year_2015::day_24/year_2015::day_24_v1
                        time:   [15.294 ms 15.326 ms 15.360 ms]
year_2015::day_24/year_2015::day_24_v2
                        time:   [3.5474 ms 3.5596 ms 3.5719 ms]
```
</details>

<details>
<summary>Ruby version comments</summary>

>While this looks like YABA (Yet Another Bruteforce Algorithm), Ruby's got a very useful [`#combination`](https://ruby-doc.org/3.2.2/Array.html#method-i-combination) method that will generate permutations around for you. And of course, knowing when to exit the loop at the best time is half the battle.
</details>

Would you believe it? The `combinations(n)` method also exists in Rust!
