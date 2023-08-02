use std::io::{self, Write};
use radix_fmt::*;

mod coords;
use coords::Hex;

mod bee;
use bee::Bee;

const BEE: i128 = 11*36*36 + 14*36 + 14;
const LAST_THREE: i128 = 36*36*36;

fn main() {
    print!("Bee location: ");
    io::stdout().flush().expect("Failed to flush buffer");
    let mut location = String::new();
    io::stdin()
        .read_line(&mut location)
        .expect("Failed to read line");

    let location = i128::from_str_radix(&location.trim(), 36).expect("Failed to convert radix");
    let location = Hex { name: location }.to_lattice();

    print!("Bee direction: ");
    io::stdout().flush().expect("Failed to flush buffer");
    let mut direction = String::new();
    io::stdin()
        .read_line(&mut direction)
        .expect("Failed to read line");

    let direction = direction.trim().parse().expect("Failed to make an integer");

    let mut bee = Bee {
        location,
        direction,
    };

    loop {
        bee.shift();

        let name = bee.location.to_hex().name;
        
        if name % LAST_THREE == BEE {
            break;
        }
    }

    println!("Bee found flower at {}", radix_36(bee.location.to_hex().name));
}
