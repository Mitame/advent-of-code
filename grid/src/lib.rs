use std::{
    fmt::{Debug, Display},
    ops::{Add, Mul, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
        }
    }

    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Offset {
    pub x: isize,
    pub y: isize,
}

impl Location {
    pub fn up(&self) -> Option<Location> {
        if self.y > 0 {
            Some(Location {
                x: self.x,
                y: self.y - 1,
            })
        } else {
            None
        }
    }

    pub fn left(&self) -> Option<Location> {
        if self.x > 0 {
            Some(Location {
                x: self.x - 1,
                y: self.y,
            })
        } else {
            None
        }
    }

    pub fn right(&self) -> Location {
        Location {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn down(&self) -> Location {
        Location {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn to(&self, direction: Direction) -> Option<Location> {
        match direction {
            Direction::Up => self.up(),
            Direction::Right => Some(self.right()),
            Direction::Down => Some(self.down()),
            Direction::Left => self.left(),
        }
    }

    pub fn manhattan_distance(&self, other: &Location) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl Sub<&Location> for &Location {
    type Output = Offset;

    fn sub(self, rhs: &Location) -> Self::Output {
        Offset {
            x: self.x as isize - rhs.x as isize,
            y: self.y as isize - rhs.y as isize,
        }
    }
}

impl Add<&Offset> for &Location {
    type Output = Location;

    fn add(self, rhs: &Offset) -> Self::Output {
        Location {
            x: (self.x as isize + rhs.x) as usize,
            y: (self.y as isize + rhs.y) as usize,
        }
    }
}

impl Mul<isize> for &Offset {
    type Output = Offset;

    fn mul(self, rhs: isize) -> Self::Output {
        Offset {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Sub<&Offset> for &Location {
    type Output = Location;

    fn sub(self, rhs: &Offset) -> Self::Output {
        Location {
            x: (self.x as isize - rhs.x) as usize,
            y: (self.y as isize - rhs.y) as usize,
        }
    }
}

impl Add<&Offset> for &Offset {
    type Output = Offset;

    fn add(self, rhs: &Offset) -> Self::Output {
        Offset {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Clone)]
pub struct Grid<T> {
    cells: Vec<T>,
    row_length: usize,
}

impl<T> Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Grid")
            .field("row_length", &self.row_length)
            .field("height", &(self.cells.len() / self.row_length))
            .finish()
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Display grid
        let mut last_y = 0;
        for location in self.iter_locations() {
            if location.y != last_y {
                last_y = location.y;
                writeln!(f)?;
            }
            write!(f, "{}", self.get(&location).unwrap())?;
        }
        writeln!(f)?;
        Ok(())
    }
}

impl<T> Grid<T> {
    pub fn new(cells: impl IntoIterator<Item = T>, row_length: usize) -> Grid<T> {
        Grid {
            cells: cells.into_iter().collect(),
            row_length,
        }
    }

    fn get_index_from_location(&self, location: &Location) -> Option<usize> {
        if location.x < self.row_length {
            Some(location.y * self.row_length + location.x)
        } else {
            None
        }
    }

    pub fn get(&self, location: &Location) -> Option<&T> {
        self.get_index_from_location(location)
            .and_then(|i| self.cells.get(i))
    }

    pub fn set(&mut self, location: &Location, value: T) -> bool {
        self.get_index_from_location(location)
            .and_then(|i| self.cells.get_mut(i))
            .map(|cell| std::mem::replace(cell, value))
            .is_some()
    }

    pub fn iter_locations(&self) -> impl Iterator<Item = Location> + '_ {
        (0..self.cells.len()).map(|i| Location {
            x: i % self.row_length,
            y: i / self.row_length,
        })
    }

    pub fn is_within_bounds(&self, location: &Location) -> bool {
        let max_y = self.cells.len() / self.row_length;
        location.y < max_y && location.x < self.row_length
    }

    pub fn cells(&self) -> &'_ [T] {
        &self.cells
    }

    pub fn into_inner(self) -> Vec<T> {
        self.cells
    }

    pub fn width(&self) -> usize {
        self.row_length
    }
}
