# KD-Tree sort implemention

Implements a non-no_std compatible sort function for usage with the `kd_tree` crate.

## Benchmark
For benchmarking purposed, criterion.rs was used, comparing our kd-tree against a basic linear_search.

Run: `cargo bench` in this directory.

## Fuzzy Test
The implemented fuzzy test uses `cargo fuzz`  
Install it before running the fuzzy test (a nightly toolchain may be required).  

It compares the result of the linear search algorithm against the result of the kd-tree search.  
The fuzzy test input is filtering out `Inf` and `NaN` values since our kd-tree is 
supposed to find the nearest-neighbor on valid input data only.

For starting the fuzzy test run: `cargo fuzz run kd_tree_search --sanitizer=leak` in this directory.  
*Note: the fuzzy test will run until it finds an error*