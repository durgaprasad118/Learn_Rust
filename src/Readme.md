# Memory management

- Rust has its own ownership model for memory management.
- Makes it extremely safe to memory errors.

- (no garbage collection- auto removing variables after their role is done) so this makes rust fast
  - it takes time to garbage collect so it's slow

# Mutability

- Immutable data thread safe because if no thread alters data then no sync is needed when data is accessed concurrently
- Knowing that certain data will not change allows the compiler to optimise code better

## Stack vs Heap memory

- Stack

  - fast allocation and deallocation. For primitive data types and for the data where size is known at compile time.
  - Numbers
  - booleans
  - array(fixed size)

- Heap

  - Used to store data that can grow at runtime, such as strings and vectors
  - For strings suppose we have a string "hello" the address to the first byte is stored in stack memory and the value is stored in heap memory. (we point from stack to heap)
  -

- cargo build = rust program => binary (COMPILATION)
- ./target/debug/program = actually running binary code (RUNTIME)
