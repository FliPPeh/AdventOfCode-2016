type KeypadScheme = &'static[&'static [Option<char>]];

const KEYPAD: KeypadScheme = &[
    &[Some('1'), Some('2'), Some('3')],
    &[Some('4'), Some('5'), Some('6')],
    &[Some('7'), Some('8'), Some('9')]
];

const FANCY_KEYPAD: KeypadScheme = &[
    &[None,      None,      Some('1'), None,      None],
    &[None,      Some('2'), Some('3'), Some('4'), None],
    &[Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
    &[None,      Some('A'), Some('B'), Some('C'), None],
    &[None,      None,      Some('D'), None,      None]
];

#[derive(Copy, Clone, Debug)]
enum Movement {
    Up,
    Down,
    Left,
    Right
}

struct Keypad {
    scheme: KeypadScheme,
    x: i32,
    y: i32
}

impl Keypad {
    fn new() -> Keypad {
        Keypad {
            scheme: KEYPAD,
            x: 1,
            y: 1
        }
    }

    fn new_fancy() -> Keypad {
        Keypad {
            scheme: FANCY_KEYPAD,
            x: 0,
            y: 2
        }
    }

    fn is_valid(&self, x: i32, y: i32) -> bool {
        y >= 0 && y < self.scheme.len() as i32 
            && x >= 0 && x < self.scheme[y as usize].len() as i32
            && self.scheme[y as usize][x as usize].is_some()
    }

    fn move_dir(&mut self, dir: Movement) -> char {
        match dir {
            Movement::Up => if self.is_valid(self.x, self.y - 1) { self.y -= 1 },
            Movement::Down => if self.is_valid(self.x, self.y + 1) { self.y += 1 },
            Movement::Left => if self.is_valid(self.x - 1, self.y) { self.x -= 1 },
            Movement::Right => if self.is_valid(self.x + 1, self.y) { self.x += 1 }
        }

        self.active_button()
    }

    fn enter(&mut self, directions: &Vec<Vec<Movement>>) -> String {
        directions
            .iter()
            .map(|seq| 
                 seq.iter()
                    .fold(self.active_button(), |_, m| self.move_dir(*m))).collect()
    }

    fn active_button(&self) -> char {
        self.scheme[self.y as usize][self.x as usize].expect("Something terrible has happened")
    }
}

fn main() {
    let mut instructions: Vec<Vec<Movement>> = Vec::new();

    loop {
        let mut line = String::new();
        match std::io::stdin().read_line(&mut line).ok() {
            Some(0) => break,
            Some(_) => instructions.push(line.trim().chars().map(|c| match c {
                    'U' => Movement::Up,
                    'D' => Movement::Down,
                    'L' => Movement::Left,
                    'R' => Movement::Right,
                    _ => panic!("Oh no, what's {}", c)
            }).collect()),

            None => panic!("Someone set up us the bomb")
        }
    }

    let mut pad = Keypad::new();
    let mut fancy_pad = Keypad::new_fancy();

    println!("Part 1: {:?}", pad.enter(&instructions));
    println!("Part 2: {:?}", fancy_pad.enter(&instructions));
}
