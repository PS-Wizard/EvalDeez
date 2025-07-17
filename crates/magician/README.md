# The Magician

**Magic Bitboard Implementation In Rust**
Fast sliding piece attacks using precomputed magic numbers — bishops, rooks, queens.

## What’s inside?
* Generate and load magic numbers for sliding pieces
* Precompute attack tables for all squares
* Fast lookup functions for bishop, rook, and queen attacks
* Utilities for blocker configs and notation conversions

---

## Who’s this for?
This crate is intended for folks who want a **working, magic bitboard implementation** with easy-to-use, high-level interfaces without needing to dive into all the magic bitboard details. 


>[!TIP]
> This crate was originally built for personal use, so I bundled the most useful stuff in `prelude::*` for easy access.
> The design is not perfect as this is my first time doing rust, thus PRs, and forks are very welcome!

---

## Quickstart:

```rust
use magician::prelude::*;

fn main() {
    // Initialize magic tables (must call once before lookups)
    init_magician();

    // Convert algebraic notation to index
    let square = notation_to_index("e4"); 

    // Create blockers bitboard from squares (e.g. blocking pieces)
    let blockers = blockers_from_squares(&["e5", "c4", "g4"]);

    // Get attacks for bishop on e4 given blockers
    let bishop_attacks = get_bishop_attacks(square, blockers);

    // Prints Colored Centered Board
    print_board(bishop_attacks);

    // Get attacks for rook on e4
    let rook_attacks = get_rook_attacks(square, blockers);
    print_board(rook_attacks);

    // Get attacks for queen on e4
    let queen_attacks = get_queen_attacks(square, blockers);
    print_board(queen_attacks);
}
```

---

## Export Format Notes

* **Magic numbers binary files (`*.bin`)** store entries as 9 bytes each:

  * 8 bytes: `u64` magic number (little-endian)
  * 1 byte: `u8` shift value

* **Occupancy mask binary files** store sequences of 8-byte `u64` values only (no shift).

Exporting functions follow this pattern for correct loading/saving.

---

