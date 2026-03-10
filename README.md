# cp-export

## Usage
Replace `include!()` macro with actuall source code.

### Example
from
```rust
// sol.rs
fn solve() {
  println!("hello, world!");
}
```
```rust
// man.rs
include!("sol.rs);

fn main() {
  solve();
}
```
to
```rust
fn solve() {
  println!("hello, world!");
}

fn main() {
  solve();
}
```

## Installation
```bash
git clone https://github.com/revival0728/cp-export.git
cd cp-export && cargo install --path .
```
