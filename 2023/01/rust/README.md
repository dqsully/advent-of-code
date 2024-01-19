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

*Unless this was really performance-critical code, I wouldn't apply any further optimizations.* I believe my solution for part 1 is very clear, and the code complexity introduced by any further optimizations wouldn't be worth the small increases in efficiency.

There are no further optimizations for worst-case big-O complexity.

There is a probabilistic optimization that can be made by searching forwards for the first digit and searching backwards for the last digit. If lines had a consistent distribution of digits within them, this would make the line parsing code tend towards `O(1)` on average as the lines get longer and longer. There are `n/l` lines in the input, where `l` is line length, so this would make the overall solution `O(n + n/l)` on average, `O(n)` worst-case.

This solution may benefit from explicit vectorization on modern hardware for a proportional reduction in time at a very high scale. This does not affect big-O complexity.

This solution may also be multithreaded or converted into an ETL process if it's running at an insane scale, although at that point usually I/O bottlenecks become the main issue.

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

*Just like part 1, I wouldn't optimize any of this further unless it was really performance-critical code.* The digit-word matching is already efficient and readable too.

There are no further optimizations for worst-case big-O complexity.

The same probabilistic optimization from part 1 also applies here too, although because of the digit-word matching, this would complicate the part 2 solution much more than for part 1.

The digit-word matching could maybe be optimized by writing out a regex-like state machine for matching individual bytes, or more likely through explicit vectorization. Either way this would be much more complicated than `partial_line.starts_with(num_text)` for a small efficiency boost.

I don't believe part 2 could be vectorized in most of the the same ways as part 1 because of the digit-word matching.

This solution may also be multithreaded or converted into an ETL process, just like part 1.
