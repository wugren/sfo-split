# sfo-split

Implement splittable object

## Example
```rust
pub struct TestRead {

}

pub struct TestWrite {

}

let splittable = Splittable::new(TestRead{}, TestWrite{});
let (r, w) = splittable.split();
```
