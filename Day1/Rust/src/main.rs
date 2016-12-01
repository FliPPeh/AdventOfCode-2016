use std::str::FromStr;
use std::collections::HashSet;

#[derive(Debug)]
enum Instruction {
    Left(i32),
    Right(i32)
}

impl Instruction {
    fn distance(self) -> i32 {
        match self {
            Instruction::Left(n) => n,
            Instruction::Right(n) => n
        }
    }
}

#[derive(Copy, Clone)]
enum Facing {
    North,
    South,
    East,
    West
}

impl Facing {
    fn turn_left(self) -> Facing {
        match self {
            Facing::North => Facing::West,
            Facing::South => Facing::East,
            Facing::East => Facing::North,
            Facing::West => Facing::South
        }
    }

    fn turn_right(self) -> Facing {
        self.turn_left().turn_left().turn_left()
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

struct Person {
    facing: Facing,
    position: Location,
    known_locations: HashSet<Location>
}

impl Person {
    fn new() -> Person {
        Person {
            facing: Facing::North,
            position: Location(0, 0),
            known_locations: HashSet::new()
        }
    }

    fn walk(&mut self, ins: Instruction) {
        match ins {
            Instruction::Left(_) => self.facing = self.facing.turn_left(),
            Instruction::Right(_) => self.facing = self.facing.turn_right(),
        }

        for _ in 0..ins.distance() {
            self.position = self.position.forward(self.facing, 1);

            if self.known_locations.contains(&self.position) {
                println!("I already know {:?}, it's {} from center!",
                    self.position,
                    self.position.distance_from(Location(0, 0)));
            } else {
                println!("Now at {:?}, {} from center",
                    self.position,
                    self.position.distance_from(Location(0, 0)));

                self.known_locations.insert(self.position);
            }
        }
    }

    fn distance(&self) -> i32 {
        self.position.distance_from(Location(0, 0))
    }
}

fn main() {
    let mut line = String::new();

    std::io::stdin().read_line(&mut line).unwrap();

    let mut me = Person::new();

    line.split(", ")
        .map(|s| s.trim())
        .map(|s| {
            let (dir, n) = s.split_at(1);

            match dir {
                "L" => Instruction::Left(n.parse().unwrap()),
                "R" => Instruction::Right(n.parse().unwrap()),
                _   => panic!("Oh god what did you do")
            }
        })
        .fold((), |_, d| me.walk(d));

    println!("{:?} @ {:?}", me.position, me.distance())
}
