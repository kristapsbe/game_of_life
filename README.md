# John Conway's Game of Life in Rust

[Rules](https://rustwasm.github.io/docs/book/game-of-life/rules.html). Currently contains a hard-coded [Gosper glider gun](https://conwaylife.com/wiki/Gosper_glider_gun).

## TODO: I want to make it a bit more interactive

I'm thinking of adding some keyboard/mouse controls
* <space> start / stop the game clock from ticking forward
* <lmb> toggle cell between alive and dead

I think I may need to use [termion](https://crates.io/crates/termion) - I'd ideally want to write it from scratch, but I've not found any tip on how to do that with just the standard library thus far.

