## Bisection Key

> insert between two different keys infinitely.

Charset:

```
+-/0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz
```

numeral repersentation: `0` ~ `64`, middle size is `32`, corresponding to `T`.

### Usage

`LexiconKey` is in lexicographic order:

```rust
use bisection_key::{LexiconKey}

let k0 = LexiconKey::default(); // defaults to "T"
k0.bisect_beginning()
k0.bisect_end()
let k1 = LexiconKey::new("a").unwrap();
b0.bisect(&k1)
```

`BalancedKey` is a different attempt, like said, it's balanced around `T` like "balanced ternary" and has its own implementation of `Ord` trait:

```rust
use bisection_key::{BalancedKey}

let k0 = LexiconKey::default(); // defaults to "T"
k0.bisect_beginning()
k0.bisect_end()
let k1 = LexiconKey::new("a").unwrap();
b0.bisect(&k1)
```

### License

MIT
