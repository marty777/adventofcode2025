# Puzzle notes

Discussion of the solutions for puzzles completed so far. These notes contain spoilers and shouldn't be viewed until you find your own solution.

### [Day 1: Secret Entrance](https://adventofcode.com/2025/day/1)

Code file: [day1.rs](./src/day1.rs)

<details>
<summary>Discussion</summary>

Nothing too complicated about my solution. For part 2, the number of times the dial crosses zero on each instruction can be found via division rather than simulation, but that gets complicated when turning the dial in the negative direction. Flipping the dial position (to `100 - pos` rather than `pos`) and treating the turn as if it were in the positive direction simplifies the math.

I was noticing some oddly poor performance when running the solution, which turned out to be my utility function for extracting and parsing integer strings via a regex. Regexes take a while to compile, and calling the function for each individual input line was causing a serious hit. I've added a separate utility function to parse integers from multiple input lines at once, requiring only a single compilation. For day 1, I'm now manually extracting and parsing the integers in the strings anyway.
</details>

### [Day 2: Gift Shop](https://adventofcode.com/2025/day/2)

Code file: [day2.rs](./src/day2.rs)

<details>
<summary>Discussion</summary>

My approach was a brute-force approach for finding repetitions in digits for each element in the ranges. It's effective enough, and the digit lengths of each element in the range can be used to restrict the search to pattern lengths that evenly divide the digits of the element.

However, I'd like to hilight a very clever approach developed by [Matthew Luu](https://github.com/nluu175) ([implementation](https://github.com/nluu175/aoc-2025/blob/main/day2/p2-math-optimized.py)). Integers written entirely with repeating digit patterns in any base have common divisors with specific patterns. For example, 1001 divides any integer written in base 10 with a repetition of two groups of three digits like 123123 or 999999, and 1010101 divides any integer with a repetition of four groups of two digits. 

By composing divisors for repeated digit patterns of varying lengths and numbers of repetitions, and then multiplying these divisors by coefficients appropriate to a given range, all the integers in the range which have repeating digit patterns can be quickly generated (with some filtering needed due to the varying numbers of digits at the start and stop of some ranges). Some duplicates will be found in this process (e.g. '2222' matches both 4x'2' and 2x'22'), but using a set container for the generated integers is a quick solution to that issue.

This allows the sum of invalid ids for a range to be obtained efficiently without having to iterate over each element in the range. An ingenious solution.
</details>

### [Day 3: Lobby](https://adventofcode.com/2025/day/3)

Code file: [day3.rs](./src/day3.rs)

<details>
<summary>Discussion</summary>

For part 2, it looked like a tree search would be needed to find the greatest joltage from the available batteries. Since each input line has 100 digits, a naive tree search would have a worst case $\binom{100}{12} = \frac{100!}{12!(100-12)!}$ selections to examine. After some thought, it's apparent that each digit selection in the sequence should be the left-most highest digit that still leaves space for the remaining digits to be selected. A tree search isn't necessary at all, although the recursive DFS I set up was easily adapted to a non-branching digit selection.
</details>

### [Day 4: Printing Department](https://adventofcode.com/2025/day/4)

Code file: [day4.rs](./src/day4.rs)

<details>
<summary>Discussion</summary>

Nothing very clever about my solution, although you can use the part 1 pass to find rolls that can be removed in a first pass on part 2 to slightly reduce the amount of work done. I found some missing methods that I needed to add to my `DefaultHashMap` struct that I'm using to read 2D grids.
</details>

### [Day 5: Cafeteria](https://adventofcode.com/2025/day/5)

Code file: [day5.rs](./src/day5.rs)

<details>
<summary>Discussion</summary>

I realized fairly quickly that you need to merge ranges in order to enumerate the fresh ingredient ids arithmetically for part 2, but I didn't figure out that it could take multiple passes to fully merge all the ranges. I think my conditions for handling range overlaps could probably be simplified a little, but it works.
</details>

### [Day 6: Trash Compactor](https://adventofcode.com/2025/day/6)

Code file: [day6.rs](./src/day6.rs)

<details>
<summary>Discussion</summary>

That was a fun exercise in lateral thinking. It occurs to me that you could easily rotate the input file and read the numbers more conventionally that way, but composing them vertically is good practice.
</details>

### [Day 7: Laboratories](https://adventofcode.com/2025/day/7)

Code file: [day7.rs](./src/day7.rs)

<details>
<summary>Discussion</summary>

My initial idea for part 2 was basically correct, but I had a weirdly hard time trying to store beam counts in a mutable `HashMap` (and then my utility `DefaultHashMap`), with a non-deterministic bug in the final beam counts. Switching to a `Vec` allocated to the width of the grid immediately worked. Odd.

On reflection, you could treat the problem as a directed acyclic graph, and there are matrix multiplication methods for counting distinct paths between nodes in those, but my solution is essentially equivalent to a dynamic programming approach to the same problem.
</details>