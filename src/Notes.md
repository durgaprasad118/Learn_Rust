# For loop notes

- when we loop over a vector(let's say) we use &vec so each `i` has &i32 or &str (has the reference)
- in order to not have any reference we use \*i (dereferencing)
- also if a function expects only `x` then we should send \*i in the loop

- **.iter** vec.iter() then it gives immutable reference
  - immutable reference
  - &i (we get )
  - access to original vec
- **.iter_mut()**
  - let's say vec is mutable then
  - &mut i
  - oriiginal collection remains same but we can modify the elements
  - access to original vec
- **.into_iter()**
  - To iterate over elements that take ownership
  - gives i
  - no access to original vec

- lazy evaluation: is a key performance benefit of iterators; operators are only executed when a consuming adaptors like (collect) request in items;

- [ ] traits
- [ ] trait bound
- [ ] copy trait
- [ ] debug trait
- [ ] clone and copy
- [ ] One writer or many readers rule
