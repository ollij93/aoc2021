# Advent of Code 2021 (My first rust!)

Advent of Code solutions for 2021 written in rust.
My first time using rust so a lot of this is probably very questionable!

The inputs for each day are stored in the `inputs/` directory, along with some
test examples from the problem descriptions.

The code is stored in `src/dayX.rs` where each day has a public `p1()` and
`p2()` method which get invoked from `main()` in `src/main.rs`. To run the code
for a given day and part specify the CLI options like so:

```
cargo run -- -d 1 -p 2 < inputs/day1.txt
```
