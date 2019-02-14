mod cell;
mod direction;
mod maze;
mod random;
mod presenter;

use crate::maze::*;
use crate::presenter::*;

extern crate rand;

fn main() {
    let mut maze: Maze = Maze::init(10);
    maze.generate();
    let presenter: Presenter = Presenter::init(maze);
    presenter.print_maze();
}
