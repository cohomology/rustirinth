# rustirinth

This is a game, written to learn Rust. It uses gtk 3 and cairo.

Goal: The user draws a labyrinth on a cairo surface and afterwards sets start and endpoint. Then the shortest way is shown to the user (if it exists).

At the moment, only the labyrinth can be drawn.

As [gtk-rs](https://github.com/gtk-rs/gtk) uses all different kinds of number types (i32, f64, u32) by chance (often a function `draw_region` takes `f64`, a similar function `draw_area` takes i32 and so on), this game became very picky of all the different conversions going on. Also all kind of mathematical weird stuff is done, to determine the exact regions of the screen which have to be repainted. This makes the game not any faster, but produces more, difficult coding.
