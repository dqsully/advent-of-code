# Advent of Code

These are my Advent of Code solutions starting from 2023.

To get started, log into Advent of Code, find your `session` cookie value, and paste it into `session.txt`. Then, you can play around with the `./advent.sh` script to test/run my solutions for different days. Examples below:

```sh
# Test all solutions (session.txt not required)
./advent.sh test all all

# Run all solutions
./advent.sh run all all


# Test the current day's solutions (session.txt not required)
./advent.sh test

# Run the current day's solutions
./advent.sh run


# Test a particular day's solutions (session.txt not required)
./advent.sh test 2023 02

# Run a particular day's solutions
./advent.sh run 2023 02


# Test an entire year's solutions (session.txt not required)
./advent.sh test 2023 all

# Run an entire year's solutions
./advent.sh run 2023 all


# Download the problem input for a given day, preparing to solve it
./advent.sh prepare 2023 02

# Download the current day's problem input
./advent.sh prepare

# Download all problem inputs for a year
./advent.sh prepare 2023 all

# Download all problem inputs for all time
./advent.sh prepare all all
```

## Templates
Inside the `template/` directory, I have starter templates for different languages that I copy to each day to simplify solving it. These templates are designed so that I can attempt to solve a problem as quick as possible when it's released, and then come back later and improve my solution.
