
# ♟️ evaldeez
```
        ,....,
      ,::::::<
     ,::/^\"``.
    ,::/, `   e`.
   ,::; |        '.
   ,::|  \___,-.  c)
   ;::|     \   '-'
   ;::|      \
   ;::|   _.=`\
   `;:|.=` _.=`\
     '|_.=`   __\
     `\_..==`` /
      .'.___.-'.
     /          \
    /'--......--'\
    `"--......--"
```

**A modular, high-performance (hopefully) chess engine being written in Rust.**  
Built from the ground up with bitboards, magic, and good ol’ brainpower ( and some AI ... a good some of AI).

---

## 🧠 What's in the Box?

This repo is a full-on **chess engine lab**, split into clean Rust crates:

| Crate       | Role                                                                 |
|-------------|----------------------------------------------------------------------|
| `magician`  | Generates magic bitboards (fast sliding piece attacks) ✨            |
| `arena`     | Coordinates the board state & game logic (like a gladiator ring) 🥊  |
| `prophet`   | Move generation, validation, and maybe some psychic prediction 🔮    |
| `tactition` | Evaluation and search — the brain of the beast 🧮                    |
| `translator`| FEN / UCI / PGN parsing and formatting 📜                            |
| `warden`    | Rules enforcement, move legality, time control — the law 🚔          |

---

## 🧩 Design Goals

- **Modularity** – each crate has one job and does it well
- **Performance** – bitboards, magic tricks, and minimal allocations
- **Clean code** – ergonomic, idiomatic Rust
- **Extensibility** – easy to tweak, add heuristics, or swap components

---

## 🧪 Testing

All crates are testable independently:

```bash
cargo test -p magician     # test magic bitboards
cargo test -p arena        # test game state logic
# etc...
```
Worspace wide:
```bash
cargo test --workspace
```

---

## 🚧 WIP
This engine is under active construction. 

Planned:
    - Bitboards ( figured it out )
    - Magic Bitboards ( figured it out ... for the most part, some of it is still magic to me)
    - FEN / UCI / LAN (long algebric notation)
    - NNUE ( figured it out )
    - Minimax with alpha-beta
    - MVV LVA Ordering
    - Iterative Deepening
    - LMR Null Move Pruning
    - Opening & Engame books
    - Explainer Module?

---
🤝 Contributing

You’re welcome to peek in, break things, or optimize the hell out of something; as long as you fork it, or shoot me a PR. This my fyp so lets wait a year before I accept everyone?

---

🧙‍♂️ Why “evaldeez”?
Because we evaluate... deez ~~nuts~~ moves.

