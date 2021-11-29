# KD-Tree implementation

Implements only functions required for searching an already initialized KD-Tree.  
Because of this, this crate is no_std compatible  

Code for building the KD-Tree can be found in `kd_tree_sort`.  
`kd_tree_sort` is not no_std compatible which is why these two crates were seperated.  
A fuzzy tester as well as a benchmark verifying this implementation can be found in `kd_tree_sort`.