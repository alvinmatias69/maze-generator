use super::direction::Direction;

struct Wall {
    left: bool,
    right: bool,
    top: bool,
    bottom: bool
}

pub trait CellInterface {
    fn init() -> Self;
    fn visit(&mut self);
    fn remove_wall(&mut self, wall: &Direction);
    fn is_visited(&self) -> bool;
    fn has_wall(&self, wall: &Direction) -> bool;
}

pub struct Cell {
    visited: bool,
    walls: Wall
}

impl CellInterface for Cell {
    fn init() -> Cell {
        let wall = Wall {
            left: true,
            right: true,
            top: true,
            bottom: true,
        };

        Cell{
            visited: false,
            walls: wall
        }
    }

    fn visit(&mut self) {
        self.visited = true
    }

    fn is_visited(&self) -> bool {
        self.visited
    }

    fn remove_wall(&mut self, wall: &Direction) {
        match wall {
            Direction::Top => self.walls.top = false,
            Direction::Bottom => self.walls.bottom = false,
            Direction::Left => self.walls.left = false,
            Direction::Right => self.walls.right = false,
        };
    }

    fn has_wall(&self, wall: &Direction) -> bool {
        let status = match wall {
            Direction::Top => self.walls.top,
            Direction::Bottom => self.walls.bottom,
            Direction::Left => self.walls.left,
            Direction::Right => self.walls.right,
        };

        status
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_cell() {
        let cell = Cell::init();
        assert!(!cell.is_visited() &&
                cell.has_wall(&Direction::Left) &&
                cell.has_wall(&Direction::Right) &&
                cell.has_wall(&Direction::Top) &&
                cell.has_wall(&Direction::Bottom)
                )
    }

    #[test]
    fn visit_cell() {
        let mut cell = Cell::init();
        cell.visit();
        assert!(cell.is_visited());
    }

    #[test]
    fn remove_wall() {
        let mut cell = Cell::init();
        cell.remove_wall(&Direction::Bottom);
        assert!(!cell.has_wall(&Direction::Bottom))
    }
}