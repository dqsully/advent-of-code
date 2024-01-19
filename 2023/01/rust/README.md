# 2023 Day 1 - Rust

## Part 1

### Summary

This an easy solution for me, both before and after cleaning it up. All I did was iterate over each character in a line, keep track of the first and last character digits, convert them into a 2-digit number, and sum those together.

I didn't really think to use regex here because it's not as easy in Rust as it is in other languages, and I was racing to try and get on the Advent of Code leaderboard for my initial implementation so I didn't have time to worry about it.

(Also, when I came back to clean this code up, it was the first time I had ever done 'table-driven tests' in Rust, and it's awesome! The downside is that Rust's `macro_rules!` system is confusing, especially to beginners, but the upside is that all of the Rust tooling (including rust-analyzer) can track the text inputs into how they're used, understanding that each line in the macro invocation is its own test! In comparison, Go and JS/TS 'discover' tests at runtime, so table-driven tests have less IDE integration since they're not statically defined.)

### Complexity

**`O(n)` time, `O(1)` space**

where:

- `n`: total input length

**Time analysis**: the solution iterates over each line, and then each character in that line, effectively reading each character twice (`O(n)`). From there, processing each character is `O(1)`. Giving `O(n*1)` or just `O(n)`.

**Space analysis**: the solution doesn't allocate anything on the heap and has no recursion, so it only uses up to a fixed amount of memory regardless of how large the input is.

### Further optimizations

_Unless this was really performance-critical code, I wouldn't apply any further optimizations._ I believe my solution for part 1 is very clear, and the code complexity introduced by any further optimizations wouldn't be worth the small increases in efficiency.

There are no further optimizations for worst-case big-O complexity.

High-performance optimizations:

- Iterating over bytes instead of chars would likely provide a small improvement in runtime. UTF-8 puts all multi-byte code points outside of the ASCII range, and we're only concerned about ASCII characters, so we can ignore everything else.

- Parsing all the text in one go instead of splitting by lines and then parsing each line should help as well because we read each byte only once.

- OR, if we still split by each line and then search forwards for the first digit and search backwards for the last digit, that _might possibly_ be faster. This only works if somehow parsing individual bytes as digits is significantly more expensive than just finding a `\n` byte in a line. There's so many factors that go into this, including the distribution of digits in a line and how long the line is, so you would have to do lots of benchmarking to figure out if and when one optimization is better than the other. Also, this optimization works out to `O(n + n/l)` on average (given a known probability of digits in a line) and `O(n)` worst-case.

- This solution may benefit from explicit vectorization on modern hardware, assuming the compiler isn't doing it all for you already.

- This solution may also be multithreaded or converted into an ETL process if it's running at an insane scale, although at that point usually I/O bottlenecks become the main issue.

## Part 2

### Summary

Part 2 was harder, but the fact that I didn't want to use regex in Rust kinda saved me here. The main difference between part 1 was that I would also match each position in each line with a lookup table of digit-words when searching for digits.

### Complexity

**`O(n)` time, `O(1)` space**

where:

- `n`: total input length

**Time analysis**: same as part 1 since the digit-word matching is also `O(1)` time.

**Space analysis**: same as part 1 since the digit-word matching is also `O(1)` space.

### Further optimizations

_Just like part 1, I wouldn't optimize any of this further unless it was really performance-critical code._ The digit-word matching is already efficient and readable too.

There are no further optimizations for worst-case big-O complexity.

High-performance optimizations:

- Just like in part 1, iterating over bytes would also probably improve runtime. However, it would make the digit-word matching more complicated.

- The 'parse-as-you-go' optimization from part 1 would also most likely be an improvement over my solution.

- _However_, the probabilistic optimization from part 1 (finding a line, then parsing from the start and from the end for a single digit) is probably going to be better than the 'parse-as-you-go' optimization for part 2, since parsing a digit now requires a lot more work. Again, you would need to do lots of benchmarking to figure out which makes the most sense.

- The digit-word matching could probably be optimized by writing out a regex-like state machine for matching individual bytes, or more likely through explicit vectorization.

- Many of the other places in part 1 where vectorization could help could apply to part 2 as well.

- This solution may also be multithreaded or converted into an ETL process, just like part 1.
