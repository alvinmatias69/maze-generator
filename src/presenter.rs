use super::maze::*;
use super::direction::*;
use super::cell::{CellInterface, Cell};

pub trait PresenterInterface {
    fn init(maze: Maze) -> Self;
    fn print_maze(&self);
}

pub struct Presenter {
    maze: Maze,
}

struct Coordinate {
    x: usize,
    y: usize,
}

impl PresenterInterface for Presenter {
    fn init(maze: Maze) -> Presenter {
        let presenter = Presenter { maze };
        presenter
    }

    fn print_maze(&self) {
        for y in 0..self.maze.size {
            self.print_top_left(&Coordinate { x: 0, y });
            for x in 0..self.maze.size {
                let coordinate = Coordinate { x, y };
                self.print_top(&coordinate);
                self.print_top(&coordinate);
                self.print_top_right(&coordinate);
            }
            println!("");

            self.print_left(&Coordinate { x: 0, y });
            for x in 0..self.maze.size {
                let coordinate = Coordinate { x, y };
                print!(" ");
                print!(" ");
                self.print_right(&coordinate);
            }
            println!("");

            if y == self.maze.size - 1 {
                print!("╚");
                for x in 0..self.maze.size {
                    print!("═");
                    print!("═");
                    self.print_bottom_right(&Coordinate{ x, y })
                }
                println!("");
            }
        }
    }
}

impl Presenter {
    fn print_top_left(&self, coordinate: &Coordinate) {
        let cell = self.find_cell(coordinate);
        if cell.has_wall(&Direction::Left) {
            if cell.has_wall(&Direction::Top) {
                print!("╠"); 
            } else {
                print!("║");
            }
        } else {
            print!("╔");
        }
    }

    fn print_top(&self, coordinate: &Coordinate) {
        let cell = self.find_cell(coordinate);
        if cell.has_wall(&Direction::Top) {
            print!("═");
        } else {
            print!(" ");
        }
    }

    fn print_top_right(&self, coordinate: &Coordinate) {
        let cell = self.find_cell(coordinate);
        if coordinate.y == 0 {
            if coordinate.x == self.maze.size - 1 {
                print!("╗");
            } else {
                if cell.has_wall(&Direction::Right) {
                    print!("╦");
                } else {
                    print!("═");
                }
            }
        } else if coordinate.x == self.maze.size - 1 {
            if cell.has_wall(&Direction::Top) {
                print!("╣");
            } else {
                print!("║");
            }
        } else {
            let mut walls: Vec<Direction> = Vec::new();
            if self.find_cell(&Coordinate { x: coordinate.x, y: coordinate.y - 1 }).has_wall(&Direction::Right) {
                walls.push(Direction::Top);
            }
            if self.find_cell(&Coordinate { x: coordinate.x + 1, y: coordinate.y }).has_wall(&Direction::Top) {
                walls.push(Direction::Right);
            }
            if cell.has_wall(&Direction::Top) {
                walls.push(Direction::Left);
            }
            if cell.has_wall(&Direction::Right) {
                walls.push(Direction::Bottom);
            }

            match walls.len() {
                4 => print!("╬"),
                3 => {
                    if !walls.contains(&Direction::Left) {
                        print!("╠");
                    } else if !walls.contains(&Direction::Right) {
                        print!("╣");
                    } else if !walls.contains(&Direction::Top) {
                        print!("╦");
                    } else if !walls.contains(&Direction::Bottom) {
                        print!("╩");
                    }
                },
                2 => {
                    if walls.contains(&Direction::Top) {
                        if walls.contains(&Direction::Left) {
                            print!("╝");
                        } else if walls.contains(&Direction::Right) {
                            print!("╚");
                        } else {
                            print!("║");
                        }
                    } else if walls.contains(&Direction::Bottom) {
                        if walls.contains(&Direction::Left) {
                            print!("╗");
                        } else {
                            print!("╔");
                        }
                    } else {
                        print!("═");
                    }
                },
                1 => {
                    if walls.contains(&Direction::Left) || walls.contains(&Direction::Right) {
                        print!("═");
                    } else {
                        print!("║");
                    }
                },
                _ => print!(" "),
            }
        }
    }

    fn print_left(&self, coordinate: &Coordinate) {
        let cell = self.find_cell(coordinate);
        if cell.has_wall(&Direction::Left) {
            print!("║");
        } else {
            print!(" ");
        }
    }

    fn print_right(&self, coordinate: &Coordinate) {
        let cell = self.find_cell(coordinate);
        if cell.has_wall(&Direction::Right) {
            print!("║");
        } else {
            print!(" ");
        }
    }

    fn print_bottom_right(&self, coordinate: &Coordinate) {
        let cell = self.find_cell(coordinate);
        if cell.has_wall(&Direction::Right) {
            print!("╩");
        } else if coordinate.x == self.maze.size -1 {        
            print!("╝");
        } else {
            print!("═");
        }
    }

    fn find_cell(&self, coordinate: &Coordinate) -> &Cell {
        let cell = &self.maze.cells[coordinate.x][coordinate.y];
        &cell
    }
}