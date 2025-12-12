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

### [Day 8: Playground](https://adventofcode.com/2025/day/8)

Code file: [day8.rs](./src/day8.rs)

<details>
<summary>Discussion</summary>

I did get a bit confused by trying to avoid enumerating the distances between all junction box positions, anticipating that this might blow up in complexity in part 2. It didn't, and by treating each junction box as a member of a separate set, then merging sets as connections are made in order of ascending distance, the puzzle is relatively straightforward. My handling of the set merging isn't very efficient, but it works.
</details>

### [Day 9: Movie Theater](https://adventofcode.com/2025/day/9)

Code file: [day9.rs](./src/day9.rs)

<details>
<summary>Discussion</summary>

It took quite a while to work out the bugs involving points directly on the polygon perimeter in part 2, but my approach relied on ray-casting to determine if a rectangle is contained within the polygon. For each rectangle with all four corners inside the polygon, each side can be tested to see if it's entirely within the polygon by casting a ray from one endpoint to another and checking if the ray crosses any horizontal or vertical perimeter segments of the polygon. If a rectangle has all four corners in the polygon and no sides cross the polygon perimeter, the rectangle is entirely inside the polygon.

This is another puzzle where the input file is noticeably friendly, since the polygon has no two horizontal or vertical perimeter segments sharing the same row or column (at least for me). It simplifies things quite a bit and must have been difficult to set up.
</details>

### [Day 10: Factory](https://adventofcode.com/2025/day/10)

Code file: [day10.rs](./src/day10.rs)

<details>
<summary>Discussion</summary>

The puzzle obviously describes systems of linear equations, with part 1 in the field of integers modulo 2. However, because these systems have more variables than equations, they don't have exact solutions that can be quickly arrived at using traditional methods.

Since the operation of each button is effectively an XOR operation with a particular mask on the indicators in part 1, the button presses are both commutative and their own inverse. This means it doesn't matter which order the buttons are pressed, and pressing any button more than once cancels out its changes to the system. The minimal number of button presses would press no button more than once and since the order of presses doesn't matter it's simple to enumerate all possible combinations of single button presses, test which ones produce the required output, and take the one with the smallest number of buttons pressed. That was my solution for part 1.

For part 2 the search space is much larger and exhaustive enumeration isn't very feasible. I fell back on the [Z3 prover](https://github.com/Z3Prover/z3) to reach the solution. While there's a Z3 wrapper available in Rust, I was having issues compiling it so the part 2 solution was arrived at via a separate Python script. It would be interesting to try solving the systems more directly.
</details>

### [Day 11: Reactor](https://adventofcode.com/2025/day/11)

Code file: [day11.rs](./src/day11.rs)

<details>
<summary>Discussion</summary>

That's quite a good puzzle. An approach like Dijkstra's algorithm seems like a good bet because the connected devices form a directed graph, but requiring the `dac` and `fft` nodes in the path betweed `svr` and `out` makes the direct application of that a little difficult. The trick is to find the number of paths between each of the nodes in in the distinct orders they can occur (`svr` -> `dac` -> `fft` -> `out` and `svr` -> `fft` -> `dac` -> `out`) and multiply the inter-node path counts together for each ordering, then take the sum. I opted for a DFS to count the paths, which is only feasible once a memoization cache is introduced.

I haven't had to deal with Rust lifetimes before, but I did when I tried to use `&str` keys in the cache for the DFS. I'm not sure my switch to a constant length character array type was the right approach, but at least it compiles.
</details>

### [Day 12: Christmas Tree Farm](https://adventofcode.com/2025/day/12)

Code file: [day12.rs](./src/day12.rs)

<details>
<summary>Discussion</summary>

I did think about the eventual solution within the first few minutes of seeing the puzzle, dismissed it as silly, and spend over an hour poking at the edges of much more hopeless approaches. A fun little prank to cap off the year.
</details>