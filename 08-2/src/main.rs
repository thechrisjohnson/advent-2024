use std::{collections::HashMap, io::Read};

fn main() {
    let input = get_input().unwrap();

    let map = Map::parse(&input);

    println!("Antinodes: {}", map.num_antinodes())
}

fn get_input() -> Result<String, std::io::Error> {
    let mut input = String::new();
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut input)?;

    if input.is_empty() {
        input = DEFAULT_INPUT.to_string();
    }

    Ok(input)
}

struct Map {
    max_x: isize,
    max_y: isize,
    antennas: HashMap<char, Vec<Point>>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut max_x: isize = 0;
        let mut y: isize = 0;
        let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();
        for line in input.lines() {
            if !line.is_empty() {
                for (x, frequency) in line.char_indices() {
                    if frequency != EMPTY_SPACE {
                        let antenna = Point { x: x as isize, y };

                        if let Some(vec) = antennas.get_mut(&frequency) {
                            vec.push(antenna);
                        } else {
                            antennas.insert(frequency, vec![antenna]);
                        }
                    }
                    max_x = x as isize;
                }
                y += 1;
            }
        }

        let max_y = y - 1;

        Self {
            max_x,
            max_y,
            antennas,
        }
    }

    fn num_antinodes(&self) -> u64 {
        let mut antinodes = 0;
        let mut existing_locations = Vec::new();
        for antennas in self.antennas.values() {
            for i in 0..antennas.len() {
                if let Some(current) = antennas.get(i) {
                    for j in i + 1..antennas.len() {
                        if let Some(other) = antennas.get(j) {
                            println!(
                                "Checking ({}, {}) ({}, {})",
                                &current.x, &current.y, &other.x, &other.y
                            );
                            for antinode in self.calculate_antinodes(current, other) {
                                println!("\tAntinode ({}, {})", &antinode.x, &antinode.y);
                                if self.in_map(antinode.x, antinode.y)
                                    && !existing_locations.contains(&antinode)
                                {
                                    antinodes += 1;
                                    existing_locations.push(antinode);
                                }
                            }
                        } else {
                            panic!("Unable to get at index {}", i);
                        }
                    }
                } else {
                    panic!("Unable to get at index {}", i);
                }
            }
        }

        antinodes
    }

    fn in_map(&self, x: isize, y: isize) -> bool {
        x >= 0 && x <= self.max_x && y >= 0 && y <= self.max_y
    }

    fn calculate_antinodes(&self, current: &Point, other: &Point) -> Vec<Point> {
        // The two nodes will be antinodes
        let mut antinodes = vec![
            Point {
                x: current.x,
                y: current.y,
            },
            Point {
                x: other.x,
                y: other.y,
            },
        ];

        let delta_x = other.x - current.x;
        let delta_y = other.y - current.y;

        let mut other_x = other.x + delta_x;
        let mut other_y = other.y + delta_y;
        while self.in_map(other_x, other_y) {
            antinodes.push(Point {
                x: other_x,
                y: other_y,
            });

            other_x += delta_x;
            other_y += delta_y;
        }

        let mut current_x = current.x - delta_x;
        let mut current_y = current.y - delta_y;
        while self.in_map(current_x, current_y) {
            antinodes.push(Point {
                x: current_x,
                y: current_y,
            });

            current_x -= delta_x;
            current_y -= delta_y;
        }

        antinodes
    }
}

#[derive(PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

const EMPTY_SPACE: char = '.';

const DEFAULT_INPUT: &str = "....h.....Q..............Y........................
...............................Y........C.........
...............m..........x................B......
........................Y..............qB.........
......g4.........................h..Y.....q...c...
................n.....R...........................
.......................................w........5.
........g...m...........................w5........
..n...........R.1................W.......q.5......
.........h...n.................e..................
...............................R..........B....C..
.........4................................5.e.....
.......0..4......n.......x..w.....................
.......g.....m........x..b.....W.....B.......w....
..............m........................3......C...
........q...0.......h....................C.3......
..................3.....................D.........
...............R..........3.............X.........
..............................W............k2.....
..........7............................2..........
...............A.............................X...2
.......................c...x......................
....................................d.............
.....1......................d.....................
...........1...........................e..........
.........0.7K.........................2.........W.
...b......0.....A.................................
......................1....ic.....................
......b......................i....................
..Q.....b..........................A..E...........
...7.........................V....................
........A.....................v......d............
........v............c...................8E.......
..............................V........8.....E..N.
......................6...........................
.......I....M....................V................
...G......................a.......8...............
.........r.9........a...i..................X......
...............r..i...............e............N..
.....H...........k....9.....6...............8.....
.v.....................6................V.........
.........v.......a........k..........D............
Ha..........k.........K........E.......d..........
...............y.MG..............6....D...........
.........H..G...M......9.K..............N.........
.......G.........................K................
...............M.........I.......D................
..................................................
....r....y................9.......................
....y................................N............";
