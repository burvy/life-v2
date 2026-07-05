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

## `logic.rs`
`logic.rs` is by far the most interesting file in my code, as it contains the actual cellular automata logic. 
Lets see it!!

### STRUCTURES
#### `Behavior` 
An enum with 3 members: `Birth`, `Earth`, `Death`.  
I chose these names because they all have 5 letters in them.
- `Birth` signals that a cell should be turned on no matter what, 
- `Earth` signals a cell should persist if it existed in the last generation,
- `Death` signals that a cell should be turned off no matter what.

#### `NeighborBehavior`
A struct containing an array of 9 `Behavior`s.  
This list represents the configuration for the cellular automata universe.  
It must be 9 members to represent all possibilities, a cell may have 0 to 8 members around them, 
inclusive of 0 and 8.  
`[0, 1, 2, 3, 4, 5, 6, 7, 8]` is 9 members,whether you like it or not.

A simple array is enough for my needs.

### FUNCTIONS
#### `draw_fn`
An orchestrator function that is meant to draw the different elements.  
It essentially acts as an abstraction layer.
