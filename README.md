# randstr

Generates random strings based on given alphabets

## Example

```rust
use randstr::randstr;

let mut generator = randstr()
  .len(10)
  .all()
  .build();
let rand = generator.generate();
assert_eq!(10, rand.len());
```
