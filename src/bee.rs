use crate::coords::Lattice;

#[derive(Debug, Clone, Copy)]
pub struct Bee<'a> {
    pub start: &'a str,
    pub location: Lattice,
    //Corresponds to rotating 30 degrees anticlockwise from facing right
    pub direction: u8,
    pub distance: u32,
}

impl Bee<'_> {
    pub fn shift(&mut self) {
        let (dx, dy) = match self.direction {
            0 => (2, 1),
            1 => (1, 1),
            2 => (1, 2),
            3 => (0, 1),
            4 => (-1, 1),
            5 => (-1, 0),
            6 => (-2, -1),
            7 => (-1, -1),
            8 => (-1, -2),
            9 => (0, -1),
            10 => (1, -1),
            11 => (1, 0),
            _ => panic!("Invalid direction"),
        };

        self.location.x += dx;
        self.location.y += dy;

        if self.direction % 2 == 0 {
            self.distance += 3;
        } else {
            self.distance += 2;
        }
    }
}
