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

Each attribute contained an array of ternary (base 3), numbers. Each attribute corresponded to a different position the board could be in. If the board was in it's normal state it would be represented by the attribute `horiz`. Each element in horiz would contain a line of the board. So, if there were 4 pieces on the board:

```txt
. W B . . . . . . . . . . . .
. W B . . . . . . . . . . . .
. . . . . . . . . . . . . . .
. . . . . . . . . . . . . . .
. . . . . . . . . . . . . . .
. . . . . . . . . . . . . . .
. . . . . . . . . . . . . . .
. . . . . . . . . . . . . . .
. . . . . . . . . . . . . . .
. . . . . . . . . . . . . . .
. . . . . . . . . . . . . . .
. . . . . . . . . . . . . . .
. . . . . . . . . . . . . . .
. . . . . . . . . . . . . . .
. . . . . . . . . . . . . . .
```
Then, the corresponding horiz array would like this, where each number is :

```rust
[
 012000000000000₃,
 012000000000000₃,
 000000000000000₃,
 000000000000000₃,
 000000000000000₃,
 000000000000000₃,
 000000000000000₃,
 000000000000000₃,
 000000000000000₃,
 000000000000000₃,
 000000000000000₃,
 000000000000000₃,
 000000000000000₃,
 000000000000000₃,
 000000000000000₃
]

```
And the vert array would look like:

```rust
[
 000000000000000₃,
 110000000000000₃,
 220000000000000₃,
 000000000000000₃,
 000000000000000₃,
 000000000000000₃,
 000000000000000₃,
 000000000000000₃,
 000000000000000₃,
 000000000000000₃,
 000000000000000₃,
 000000000000000₃,
 000000000000000₃,
 000000000000000₃,
 000000000000000₃
]

```
This representation did not work out in the end mainly due to extremely large lookup tables and slow move times.


My new representation works in a similar manner, except rather than using arrays of base 3 numbers, it simply has 2 attributes for each board representation in the struct. Each array in these attributes shows the board in a bit representation. So, it looks like this:

```rust
pub struct Board {
  pub horiz_y: [u16; 15],
  pub horiz_o: [u16; 15],

  pub verti_y: [u16; 15],
  pub verti_o: [u16; 15],
  
  pub diagr_y: [u16; 21],
  pub diagr_o: [u16; 21],
  
  pub diagl_y: [u16; 21],
  pub diagl_o: [u16; 21]
}
```