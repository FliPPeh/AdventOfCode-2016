use std::env::args;

fn is_wall(num: i32, x: i32, y: i32) -> bool {
    let sum = ((x * x) + (3 * x) + (2 * x * y) + y + (y * y)) + num;
    // (0 .. (8*mem::size_of::<i32>())).filter(|i| (sum & (1 << i)) > 0).count() % 2 != 0
    sum.count_ones() % 2 != 0
}

const W: i32 = 100;
const H: i32 = 100;

type Coord = (i32, i32);

fn search<F>(h: i32, w: i32, start: Coord, end: Coord, wall_fn: F) -> Vec<(Coord, i32)>
    where F: Fn(i32, i32) -> bool
{
    let mut grid = Vec::new();

    for _ in 0 .. h {
        let mut row = Vec::new();

        for _ in 0 .. w {
            row.push(None);
        }

        grid.push(row);
    }

    grid[start.1 as usize][start.0 as usize] = Some(0);

    let mut marked = vec![(start, 0)];

    loop {
        let mut new = Vec::new();

        for &((x, y), i) in &marked {
            for &(xoff, yoff) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if     (y + yoff) >= 0 && (y + yoff) < h
                    && (x + xoff) >= 0 && (x + xoff) < w
                    && !wall_fn(x + xoff, y + yoff)
                    && grid[(y + yoff) as usize][(x + xoff) as usize].is_none()
                {
                    new.push(((x + xoff, y + yoff), i + 1));
                    grid[(y + yoff) as usize][(x + xoff) as usize] = Some(i + 1);
                }
            }
        }

        marked.extend(new.drain(0..));

        if grid[end.1 as usize][end.0 as usize].is_some() {
           break;
        }
    }

    marked
}

fn main() {
    let start = (1, 1);
    let end = (31, 39);

    let num: i32 = if let Some(n) = args().nth(1) {
        match n.parse() {
            Ok(n) => n,
            Err(e) => {
                println!("error: {}", e);
                return;
            }
        }
    } else {
        println!("usage: {} <number>",
                 args().nth(0).unwrap_or("./a.out".to_string()));
        return;
    };

    /*
     * Generate wave propagation
     */
    let wave = search(H, W, start, end, |x, y| is_wall(num, x, y));

    /*
     * Get what we need
     */
    let part1 = wave.iter().find(|&&((ix, iy), _)| (ix, iy) == end).unwrap().1;
    let part2 = wave.iter().filter(|&&((_, _), n)| n <= 50).count();

    /*
     * Get more than we need, generate path
     */
    let mut path = vec![(end, part1)];

    fn is_neighbor((x1, y1): Coord, (x2, y2): Coord) -> bool {
        let x_diff = (x1 - x2).abs();
        let y_diff = (y1 - y2).abs();

        ((x_diff == 1) && (y_diff == 0)) || ((x_diff == 0) && (y_diff == 1))
    }

    loop {
        let &(last_coord, last_step) = path.last().unwrap();
        let &point = wave
            .iter()
            .find(|&&(c, step)| step < last_step && is_neighbor(c, last_coord))
            .unwrap();

        path.push(point);

        if point.0 == start {
            break;
        }
    }

    /*
     * Make pretty
     */
    for y in 0 .. H {
        for x in 0 .. W {
            let (fg, bg, s) = if x == start.0 && y == start.1 {
                ('4', '0', 'O')
            } else if x == end.0 && y == end.1 {
                ('1', '0', 'X')
            } else if path.iter().find(|&&((ix, iy), _)| (ix, iy) == (x, y)).is_some() {
                ('3', '3', '+')
            } else if is_wall(num, x, y) {
                ('7', '7', '#')
            } else {
                ('7', '0', '.')
            };

            print!("\x1b[3{};1m\x1b[4{};1m{}\x1b[0m", fg, bg, s);
        }

        println!("");
    }

    println!("Part 1: Can reach ({},{}) in {} steps", end.0, end.1, part1);
    println!("Part 2: Can reach {} unique locations in 50 steps", part2);
}
