# Rustoku
Gomoku engine built in Rust! (with a NodeJS/Electron GUI)
## Motivation
To battle against a friend's A.I.
## Board Representation
Initially the board was a struct with 4 different array attributes.

```rust
pub Board {
  horiz: [u32; 15],
  verti: [u32; 15],
  diagr: [u32; 21],
  diagl: [u32; 21]
}
```

This was great as it fit perfectly into my move generation model. Essentially, move generation is done by taking each row as a number, and doing an array lookup using that row as the key.
  