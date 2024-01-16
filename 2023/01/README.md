# 2023 Day 1

Solutions:
* [Rust](./rust)

## Part 1

Today's puzzle is to take an input split into lines, and for each line, find the first and last digit, and convert that into a 2-digit number. Then sum each 2-digit number together for the final answer.

For example:

```
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
```

If we extract the first and last digits from each line, we get

```
12
28
15
77
```

(Note that the last line `treb7uchet` only has one digit, which is itself both the first and last digits in the line, therefore `treb7uchet` becomes `77`.)

From there, adding up each 2-digit number `12 + 28 + 15 + 77 = 142`, so the answer for this example is `142`.

## Part 2

Now for part 2, not only do we have to find the digits (`0`, `1`, `2`, etc.), but also the words for each digit except for zero (`one`, `two`, `three`, etc.). And the official example doesn't show it well, but there's technically no guarantee that the digit-words don't overlap. For example the line `eightwo` should become `82`, even though both digit-words share the same `t`.

The example that Advent of Code gave was:

```
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
```

Which turned into 2-digit numbers should be:

```
29
83
13
24
42
14
76
```

And then summed up, `29 + 83 + 13 + 24 + 42 + 14 + 76 = 281`, so the answer for part 2's example is `281`.
