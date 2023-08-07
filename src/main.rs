use radix_fmt::*;
use std::collections::HashMap;

mod coords;
use coords::Hex;

mod bee;
use bee::Bee;

fn main() {
    mini_puzzle();
}

fn mini_puzzle() {
    let location = i128::from_str_radix("c0c0c0", 36).expect("Failed to convert radix");
    let location = Hex { name: location }.to_lattice();

    let mut bee = Bee {
        start: "c0c0c0",
        location,
        direction: 11,
        distance: 0,
    };

    let end = i128::from_str_radix("c0c140", 36).expect("Failed to convert radix");
    let end = Hex { name: end }.to_lattice();

    while bee.location != end {
        bee.shift();
    }

    println!("The bee moved {} steps", bee.distance);
}

fn main_puzzle() {
    let mut bees: Vec<Bee> = [
        ("51d9d3", 10),
        ("ae9601", 6),
        ("53363e", 6),
        ("2a5ebd", 2),
        ("71665b", 3),
        ("ea8d18", 10),
        ("eef37e", 5),
        ("955e7e", 1),
        ("2d0fa9", 7),
        ("e9f33a", 6),
        ("f66293", 9),
        ("fd7c4e", 3),
        /*("174a4a", 0),
        ("11158c", 7),
        ("111588", 5),*/
    ]
    .into_iter()
    .map(|(a, b)| {
        let location = i128::from_str_radix(a, 36).expect("Failed to convert radix");
        let location = Hex { name: location }.to_lattice();

        Bee {
            start: a,
            location,
            direction: b,
            distance: 0,
        }
    })
    .collect();

    let mut visited = HashMap::<_, Vec<Bee>>::new();

    let mut hits = 4;

    while hits != 0 {
        for bee in &mut bees {
            visited
                .entry(bee.location)
                .and_modify(|x| {
                    x.push(*bee);

                    if x.len() == 3 {
                        println!(
                            "Location {} visited 3 times.",
                            radix_36(bee.location.to_hex().name)
                        );

                        println!(
                            "Bee 1 started at {} and moved {} steps.",
                            x[0].start, x[0].distance
                        );
                        println!(
                            "Bee 2 started at {} and moved {} steps.",
                            x[1].start, x[1].distance
                        );
                        println!(
                            "Bee 3 started at {} and moved {} steps.",
                            x[2].start, x[2].distance
                        );

                        println!();

                        hits -= 1;
                    }
                })
                .or_insert(vec![*bee]);

            bee.shift();
        }
    }
}