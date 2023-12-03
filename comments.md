
# Ownership rules

Rust don't need a garbage collector thanks all the strict ownership rules.
* Each value in Rust has an owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

