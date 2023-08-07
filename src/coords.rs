#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub struct Lattice {
    pub x: i32,
    pub y: i32,
}

#[derive(PartialEq, Debug)]
pub struct Hex {
    pub name: i32,
}

fn centred_hex(x: i32) -> i32 {
    3 * x * (x + 1) + 1
}

impl Hex {
    pub fn to_lattice(&self) -> Lattice {
        if self.name == 0 {
            return Lattice { x: 0, y: 0 };
        }

        let ring = (0..)
            .map(|x| (x, centred_hex(x)))
            .find(|(_, x)| *x > self.name)
            .unwrap()
            .0;

        let sign = if (self.name % 2 == 0) ^ (ring % 2 == 0) {
            -1
        } else {
            1
        };

        let ring_pos = (self.name - centred_hex(ring - 1)) / 2;
        let side = ring_pos / ring;
        let side_pos = ring_pos % ring;

        if side == 0 {
            let y = sign * (side_pos + 1);
            let x = y - sign * ring;

            Lattice { x, y }
        } else if side == 1 {
            Lattice {
                x: sign * (side_pos + 1),
                y: sign * ring,
            }
        } else {
            Lattice {
                x: sign * ring,
                y: sign * (ring - side_pos - 1),
            }
        }
    }
}

impl Lattice {
    pub fn to_hex(self) -> Hex {
        let (x, y) = (self.x, self.y);

        if x == 0 && y == 0 {
            return Hex { name: 0 };
        }

        let ring = if self.x * self.y >= 0 {
            self.x.abs().max(self.y.abs())
        } else {
            self.x.abs() + self.y.abs()
        };

        let sign = if (y > 0) || (x > 0 && y == 0) { 1 } else { -1 };

        let parity = if (ring % 2 == 0) ^ (sign == 1) { 0 } else { 1 };

        if x * sign <= 0 {
            let side_pos = sign * y - 1;

            Hex {
                name: centred_hex(ring - 1) + side_pos * 2 + parity,
            }
        } else if y * sign >= x * sign {
            let side_pos = sign * x - 1;

            Hex {
                name: centred_hex(ring - 1) + (side_pos + ring) * 2 + parity,
            }
        } else {
            let side_pos = ring - sign * y - 1;

            Hex {
                name: centred_hex(ring - 1) + (side_pos + 2 * ring) * 2 + parity,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TABLE: [(i32, (i32, i32)); 19] = [
        (0, (0, 0)),
        (1, (0, 1)),
        (2, (0, -1)),
        (3, (1, 1)),
        (4, (-1, -1)),
        (5, (1, 0)),
        (6, (-1, 0)),
        (7, (1, -1)),
        (8, (-1, 1)),
        (9, (0, -2)),
        (10, (0, 2)),
        (11, (-1, -2)),
        (12, (1, 2)),
        (13, (-2, -2)),
        (14, (2, 2)),
        (15, (-2, -1)),
        (16, (2, 1)),
        (17, (-2, 0)),
        (18, (2, 0)),
    ];

    #[test]
    fn to_lattice_test() {
        for (name, (x, y)) in TABLE {
            assert_eq!(Hex { name }.to_lattice(), Lattice { x, y });
        }
    }

    #[test]
    fn to_hex_test() {
        for (name, (x, y)) in TABLE {
            assert_eq!(Hex { name }, Lattice { x, y }.to_hex());
        }
    }
}
