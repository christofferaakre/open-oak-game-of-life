# open-oak-game-of-life
`open-oak-game-of-life` is an implementation of Conway's Game of Life
(See https://en.wikipedia.org/wiki/Conway's_Game_of_Life) using
[conlife](https://github.com/christofferaakre/conlife) as the backend,
and [open-oak](https://github.com/christofferaakre/open-oak) to render the front-end.

You provide the width and height of the grid, as well as the starting configuration,
and it will open a window and run the simulation in real-time. The window can be closed
by pressing Escape.

## Installation
The simplest way to install is to run `cargo install open-oak-game-of-life`.
Alternatively, you can clone this repository and build it from source: `cargo build --release`.
Then you will find the compiled binary in `target/release/open-oak-game-of-life`.

## Usage
Run `open-oak-game-of-life --help` to see the required arguments. If you
provide just these, you will be greeted with an empty grid. To actually do something,
you should provide the program with a starting configuration. To do so, use the `-o` or `--object` flag.
The syntax is `-o filename,xoffset,yoffset`, for example `-o objects/glider.life,31,27`. This flag
would load a glider object onto the grid at the position `(31, 27)`. Object should be defined in
`.life` files. Users can look at the example files in `objects/` to get an idea of how to specify their own custom
objects, or they can refer to the [conlife documentation](https://docs.rs/conlife/latest/conlife/object/struct.Object.html#method.from_string)
