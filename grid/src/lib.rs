use std::fmt::Debug;

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

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Location {
    pub x: usize,
    pub y: usize,
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
            .and_then(|cell| Some(std::mem::replace(cell, value)))
            .is_some()
    }

    pub fn iter_locations(&self) -> impl Iterator<Item = Location> + '_ {
        (0..self.cells.len()).into_iter().map(|i| Location {
            x: i % self.row_length,
            y: i / self.row_length,
        })
    }
}
