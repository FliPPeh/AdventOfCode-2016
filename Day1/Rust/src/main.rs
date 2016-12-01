use std::collections::HashSet;

#[derive(Copy, Clone, Debug)]
enum Turn { Left, Right }

#[derive(Copy, Clone, Debug)]
enum Facing { North, South, East, West }

impl Facing {
    fn turn(self, t: Turn) -> Facing {
        match t {
            Turn::Left => match self {
                Facing::North => Facing::West,
                Facing::South => Facing::East,
                Facing::East => Facing::North,
                Facing::West => Facing::South
            },

            Turn::Right => self.turn(Turn::Left).turn(Turn::Left).turn(Turn::Left)
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Location(i32, i32);

impl Location {
    fn forward(self, facing: Facing, distance: i32) -> Location {
        match facing {
            Facing::North => Location(self.0, self.1 - distance),
            Facing::South => Location(self.0, self.1 + distance),
            Facing::West => Location(self.0 + distance, self.1),
            Facing::East => Location(self.0 - distance, self.1),
        }
    }

    fn distance_from(self, other: Location) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs()
    }
}

fn main() {
    let mut line = String::new();

    std::io::stdin().read_line(&mut line).unwrap();

    let mut facing = Facing::North;

    let path = line.split(", ")
        .map(|s| {
            let (dir, n) = s.trim().split_at(1);
            let turn = match dir {
                "L" => Turn::Left,
                "R" => Turn::Right,
                _   => panic!("Oh god what did you do")
            };

            (turn, n.parse().unwrap())
        })

        .flat_map(|(turn, n)| {
            facing = facing.turn(turn);

            (0..n).map(move |_| (facing, 1))
        })

        .fold(vec![Location(0, 0)], |mut p, (f, d)| {
            let next = p.last().unwrap().forward(f, d);
            p.push(next);
            p
        });

    println!("Part 1: {:}", path.last().unwrap().distance_from(Location(0, 0)));

    // Find dups
    let mut known = HashSet::new();

    for node in path {
        if known.contains(&node) {
            println!("Part 2: {}", node.distance_from(Location(0, 0)));

            break;
        } else {
            known.insert(node);
        }
    }
}
