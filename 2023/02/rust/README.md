# 2023 Day 2 - Rust

## Part 1

### Summary

I'll be honest, I didn't realize from the problem description that it didn't matter which 'reveal' a recorded number of cubes was in, so I originally built a parser that split each game by reveal, and then each reveal by color. I also added error handling in case there were more colors than just 'red', 'blue', and 'green', but it turns out I didn't have to worry about that either.

Again, I didn't use Regex, and unless it makes a problem significantly easier I don't think I will be using it at all.

After solving both part 1 and part 2, I came back and rewrote the parsing code to give me a summary of the max colors for each cube for a given game line, and then did the trivial math for each part from there.

### Complexity

**`O(n)` time, `O(1)` space**

where:

- `n`: total input length

**Time analysis**: the solution iterates over each line, and then splits multiple times within that line to get different parts which it uses once or iterates over once. So if `l` is line length, then iterating over each line is `O(n)`, there are `n/l` lines, and parsing each line is `O(l)`. This simplifies out like so: `O(n + n/l*l)` -> `O(n + n)` -> `O(n)`. You can use the same trick to show that the line parsing is actually `O(l)` as well, but I just skipped that to keep this simple.

**Space analysis**: the solution doesn't allocate anything on the heap and has no recursion, so it only uses up to a fixed amount of memory regardless of how large the input is. Mainly this works because of Rust's zero-copy APIs and iterators.

### Further optimizations

*Unless this was really performance-critical code, I wouldn't apply any further optimizations.* I believe my solution for part 1 is very clear, and while there are some easy optimizations that could be made, I just don't think they're necessary.

There are no further optimizations for worst-case big-O complexity.

High-performance optimizations:

- Using a faster integer parsing library would be an easy improvement since Rust's standard library parsers aren't the most efficient. (This may require certain compiler optimizations like the `target-cpu=native` compiler flag to be fully effective, but if you're going this far already, you're probably looking at every compiler trick to optimize your code too.)

- Another optimization would be to do less string splitting and more 'parse-as-you-go' processing. For example, instead of finding a line first and then parsing it, you could build a single iterator that takes in an entire file and returns game stats for each line as it parses the file byte-by-byte.

- This solution may also be multithreaded or converted into an ETL process if it's running at an insane scale, although at that point usually I/O bottlenecks become the main issue.

## Part 2

### Summary

Part 2 was trivial for me since it was nearly all the same code from part 1, just computing the final sum a little differently.

### Complexity

**`O(n)` time, `O(1)` space**

where:

- `n`: total input length

**Time analysis**: same as for part 1

**Space analysis**: same as for part 1

### Further optimizations

Same as for part 1, since the only thing different is how the sum is computed.
