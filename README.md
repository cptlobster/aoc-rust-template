# Advent of Code Rust Template
This repository is a quick template to get started on Advent of Code. It includes unit testing for each day (based on
the provided example cases) and a unified execution harness for reading input files and getting results.

## Structure
The following locations are relevant:
- `input`: The location where your input files are saved. They must be stored with the naming scheme `day01_1.txt`, 
  where the first number is the day (two digits w/ leading zero) and the second number is which part (part 1 or 2).
- `src/days`: This is where you will write most of your code. Each day has its own file with solution functions for
  parts 1 and 2, and unit tests for those functions.
- `src/common.rs`: Common functions used across days. You can write your own functions here to be used on multiple days.
- `src/main.rs`: Execution harness for reading full input / getting results.

## Test Files
Test files will have functions `solve_1` and `solve_2`, which is where your implementation will go. It is assumed that
these tests will take a string as input and output an integer value (as this is usually how Advent of Code goes), if not
then we have a problem.

These files also have a tests section for testing with the examples provided by Advent of Code. You will need to fill in
the values with the correct example values. Here's an example of the test section:
```rust
#[cfg(test)]
mod tests {
    #[test]
    #[ignore] // delete this line when ready to use
    fn sample_1() {
        // Fill in with the examples provided
        let str1: &str = "";
        let str2: &str = "";
        let res1: i32 = 1;
        let res2: i32 = 2;
        ...
    }
}
```
*Note: All tests are initially ignored to avoid clogging your test results with failures from days you haven't reached
yet. When you setup your test examples, **remember to delete the `#[ignore]` line above the test!!***

## Running
To get the result for a specific day / part, run the following:
```shell
cargo run -- [DAY] [PART]
```
Both are numeric values.

To run unit tests (using examples provided by Advent of Code), use:
```shell
cargo test
```