# MEMORY SAFE CELLULAR AUTOMATA

A cellular automata simulation built from scratch using **pixels** and **winit**.

Use **Left Click** to add cells, use **Right Click** to remove.
Use **Scroll Wheel** to speed up/slow down the simulation.
Use **Space** to pause/play the simulation

Try running the executable in Releases!

# DESIGN
There are three main files as of **July 5th 2026**:

[`main.rs`](src/main.rs): starts off winit/pixels in `window.rs`.

[`window.rs`](src/window.rs): custom-built interface to handle window creation, 
drawing, scaffolding for the cellular automata in `logic.rs`

[`logic.rs`](src/logic.rs): contains the cellular automata logic, 
and uses the interface in `window.rs` to display changes to the screen.
