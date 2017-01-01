extern crate permutohedron;

use std::io::{stdin, BufRead, Result as IoResult};
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Tile {
    Floor,
    Wall,
    Waypoint(u32)
}

type Coord = (isize, isize);

struct Map {
    map: Vec<Vec<Tile>>
}

impl Map {
    fn new(map: Vec<Vec<Tile>>) -> Map {
        Map {
            map: map
        }
    }

    fn get(&self, (x, y): Coord) -> Tile {
        self.map[y as usize][x as usize]
    }

    fn height(&self) -> usize {
        self.map.len()
    }

    fn width(&self) -> usize {
        self.map[0].len()
    }

    fn waypoints(&self) -> Vec<(Coord, u32)> {
        let mut res = Vec::new();

        for y in 0..self.height() {
            for x in 0..self.width() {
                match self.map[y][x] {
                    Tile::Waypoint(i) => res.push(((x as isize, y as isize), i)),
                    _ => ()
                }
            }
        }

        res
    }
}

fn is_neighbor((x1, y1): Coord, (x2, y2): Coord) -> bool {
    let x_diff = (x1 as i32 - x2 as i32).abs();
    let y_diff = (y1 as i32 - y2 as i32).abs();

    ((x_diff == 1) && (y_diff == 0)) || ((x_diff == 0) && (y_diff == 1))
}

fn search(start: Coord, end: Coord, map: &Map) -> Vec<(Coord, usize)> {
    let mut grid = Vec::new();

    let h = map.height() as isize;
    let w = map.width() as isize;

    for _ in 0..h {
        let mut row = Vec::new();

        for _ in 0..w {
            row.push(None);
        }

        grid.push(row);
    }

    grid[start.1 as usize][start.0 as usize] = Some(0);

    /*
     * Generate wave propagation
     */
    let mut marked = vec![(start, 0)];

    loop {
        let mut new = Vec::new();

        for &((x, y), i) in &marked {
            for &(xoff, yoff) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if     (y + yoff) >= 0 && (y + yoff) < h
                    && (x + xoff) >= 0 && (x + xoff) < w
                    && map.get(((x + xoff), (y + yoff))) != Tile::Wall
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

    /*
     * Backtrace
     */
    let mut path = vec![(end, marked.iter().find(|&&(c, _)| c == end).unwrap().1)];

    loop {
        let &(last_coord, last_step) = path.last().unwrap();
        let &point = marked
            .iter()
            .find(|&&(c, step)| step < last_step && is_neighbor(c, last_coord))
            .unwrap();

        path.push(point);

        if point.0 == start {
            break;
        }
    }

    path
}

fn main() {
    let stdin = stdin();
    let mapdata = stdin
        .lock()
        .lines()
        .map(IoResult::unwrap)
        .map(|l| l.chars().map(|c| match c {
            '.' => Tile::Floor,
            '#' => Tile::Wall,
             c if c.is_digit(10) => Tile::Waypoint(c.to_digit(10).expect("bad digit")),
             _ => panic!("bad tile type {}", c)
        }).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let map = Map::new(mapdata);

    for y in 0..map.height() as isize {
        for x in 0..map.width() as isize {
            let (fg, bg, s) = match map.get((x, y)) {
                Tile::Floor       => (7, 0, '.'),
                Tile::Wall        => (7, 7, '#'),
                Tile::Waypoint(i) => (3, 3,  std::char::from_digit(i, 10).unwrap_or('?'))
            };

            print!("\x1b[3{};1m\x1b[4{};1m{}\x1b[0m", fg, bg, s);
        }

        println!("");
    }


    let mut waypoints = map.waypoints();
    waypoints.sort_by(|&(_, i1), &(_, i2)| i1.cmp(&i2));

    let start = waypoints.swap_remove(0);

    /*
     * Cache individual start->end path lengths
     */
    let mut cache = HashMap::new();
    let mut shortest_path = usize::max_value();

    permutohedron::heap_recursive(&mut waypoints, |perm| {
        let mut total_path_length = 0;

        for i in 0..perm.len() {
            let (ac, ai) = if i == 0 {start} else {perm[i - 1]};
            let (bc, bi) = perm[i];

            let path_len: usize = *cache
                .entry((ai, bi))
                .or_insert_with(|| search(ac, bc, &map).len() - 1);

            total_path_length += path_len;
        }

        shortest_path = std::cmp::min(shortest_path, total_path_length);
    });

    println!("Part 1: {}", shortest_path);

    /*
     * Part 2
     */
    shortest_path = usize::max_value();

    permutohedron::heap_recursive(&mut waypoints, |perm| {
        let mut total_path_length = 0;

        for i in 0..perm.len() + 1 {
            let (ac, ai) = if i == 0          {start} else {perm[i - 1]};
            let (bc, bi) = if i == perm.len() {start} else {perm[i]};

            let path_len: usize = *cache
                .entry((ai, bi))
                .or_insert_with(|| search(ac, bc, &map).len() - 1);

            total_path_length += path_len;
        }

        shortest_path = std::cmp::min(shortest_path, total_path_length);
    });

    println!("Part 2: {}", shortest_path);
}
