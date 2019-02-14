use super::cell::*;
use super::direction::*;
use super::random::*;

struct Coordinates {
    x: usize,
    y: usize,
}

pub trait MazeInterface {
   fn init(size: usize) -> Self; 
   fn generate(&mut self);

}

pub struct Maze {
    pub size: usize,
    pub cells: Vec<Vec<Cell>>
}

impl MazeInterface for Maze {
    fn init(size: usize) -> Maze {
        let mut cells: Vec<Vec<Cell>> = Vec::new();
        for _ in 0..size {
            let mut inner: Vec<Cell> = Vec::new();
            for _ in 0..size{
                inner.push(Cell::init())
            }
            cells.push(inner);
        };
        Maze { size, cells }
    }

    fn generate(&mut self) {
        let start = Coordinates { x: 0, y: 0 };
        self.mark_cell(start, &Direction::Left);
    }
}

impl Maze {
    fn mark_cell(&mut self, coordinate: Coordinates, origin: &Direction) {
        self.cells[coordinate.x][coordinate.y].visit();
        self.cells[coordinate.x][coordinate.y].remove_wall(origin);
        if coordinate.x == self.size - 1 && coordinate.y == self.size - 1 {
            self.cells[coordinate.x][coordinate.y].remove_wall(&Direction::Right);
        }
        let mut walls: Vec<Direction> = vec![Direction::Left, Direction::Right, Direction::Top, Direction::Bottom];
        while walls.len() > 0 {
            let index = rand_between(0, walls.len());
            let relative: Direction;
            let mut x: isize = coordinate.x as isize;
            let mut y: isize = coordinate.y as isize;
            match walls[index] {
                Direction::Left => {
                    x = x - 1;
                    relative = Direction::Right;
                },
                Direction::Right => {
                    x = x + 1;
                    relative = Direction::Left;
                },
                Direction::Top => {
                    y = y - 1;
                    relative = Direction::Bottom;
                },
                Direction::Bottom => {
                    y = y + 1;
                    relative = Direction::Top;
                }
            }
            if x >= 0 && x < self.size as isize && y >= 0 && y < self.size as isize && !self.cells[x as usize][y as usize].is_visited() {
                let next: Coordinates = Coordinates{x: x as usize, y: y as usize};
                self.cells[coordinate.x][coordinate.y].remove_wall(&walls[index]);
                self.mark_cell(next, &relative);
            }
            walls.swap_remove(index);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn init_maze() {
        let _maze = Maze::init(10);
    }

    #[test]
    fn first_cell_should_visited() {
        let mut maze = Maze::init(1);
        maze.generate();
        assert!(maze.cells[0][0].is_visited())
    }
}