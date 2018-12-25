pub type Position = (u32, u32);

pub trait SizedGrid2D {
    fn get_size(&self) -> (u32, u32);
}

pub trait RecursiveCellPosition {
    fn get_x(&self) -> u32;
    fn get_y(&self) -> u32;
    fn north<M: SizedGrid2D>(&self, recursive_map: &M) -> Self;
    fn south<M: SizedGrid2D>(&self, recursive_map: &M) -> Self;
    fn east<M: SizedGrid2D>(&self, recursive_map: &M) -> Self;
    fn west<M: SizedGrid2D>(&self, recursive_map: &M) -> Self;
}

impl RecursiveCellPosition for Position {
    fn get_x(&self) -> u32 {
        return self.0;
    }
    fn get_y(&self) -> u32 {
        return self.1;
    }
    fn north<M: SizedGrid2D>(&self, recursive_map: &M) -> Position {
        let position = (self.get_x(), self.get_y());
        match position {
            (x, y) => {
                if y <= 0 {
                    (x, recursive_map.get_size().1 - 1)
                } else {
                    (x, y - 1)
                }
            }
        }
    }
    fn south<M: SizedGrid2D>(&self, recursive_map: &M) -> Position {
        let position = (self.get_x(), self.get_y());
        match position {
            (x, y) => {
                if (y + 1) > (recursive_map.get_size().1 - 1) {
                    (x, 0)
                } else {
                    (x, y + 1)
                }
            }
        }
    }
    fn east<M: SizedGrid2D>(&self, recursive_map: &M) -> Position {
        let position = (self.get_x(), self.get_y());
        match position {
            (x, y) => {
                if (x + 1) > (recursive_map.get_size().0 - 1) {
                    (0, y)
                } else {
                    (x + 1, y)
                }
            }
        }
    }
    fn west<M: SizedGrid2D>(&self, recursive_map: &M) -> Position {
        let position = (self.get_x(), self.get_y());
        match position {
            (x, y) => {
                if x <= 0 {
                    (recursive_map.get_size().0 - 1, y)
                } else {
                    (x - 1, y)
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::SizedGrid2D;
    
    type SizedMap = (u32, u32);
    
    impl SizedGrid2D for SizedMap {
        fn get_size(&self) -> (u32, u32) {
            self.clone()
        }
    }
    
    #[test]
    /// trying to confirm that trait associated with a type alias not only works with the alias,
    /// but it works with origin type as well
    fn type_alias_compile_test() {
        use super::{Position, RecursiveCellPosition};
        let map: SizedMap = (3, 3);
        let position: Position = (1, 1);
        let north = position.north(&map);
        let not_position: (u32, u32) = (1 as u32, 1 as u32);
        let west = not_position.north(&map);
    }
}
