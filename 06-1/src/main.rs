use std::{fmt, io::Read};

fn main() {
    let input = get_input().unwrap();

    let mut map = Map::parse(&input).unwrap();
    map.run().unwrap();

    println!("Spaces visited: {}", map.spaces_visited);
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
    board: Vec<Vec<Space>>,
    max_x: usize,
    max_y: usize,
    guard: Guard,
    spaces_visited: u64,
}

impl Map {
    fn parse(input: &str) -> Result<Self, Error> {
        let mut board = Vec::new();
        let mut guard: Option<Guard> = None;
        let mut max_x: usize = 0;

        let mut y: usize = 0;
        for line in input.lines() {
            let mut x: usize = 0;
            let mut current_line = Vec::new();
            for c in line.chars() {
                if c == GUARD {
                    guard = Some(Guard {
                        x,
                        y,
                        direction: Direction::North,
                    });
                    // A guard is an empty space that has been visited
                }

                let space = Space::parse(c)?;
                current_line.push(space);
                x += 1;
            }

            max_x = x - 1;
            board.push(current_line);
            y += 1;
        }

        let max_y = y - 1;
        if guard.is_none() {
            return Err(Error::new("Did not find guard in map!".to_string()));
        }

        Ok(Map {
            board,
            max_x,
            max_y,
            guard: guard.unwrap(),
            spaces_visited: 1,
        })
    }

    fn run(&mut self) -> Result<(), Error> {
        loop {
            // Get the next move of the guard
            let (new_x, new_y) = self.guard.get_next_move();
            println!("{}. Next move: ({}, {})", self.guard, &new_x, &new_y);

            if new_x < 0 || new_x > self.max_x as isize || new_y < 0 || new_y > self.max_y as isize
            {
                println!("\t Next move moves out of board! Done");
                break;
            }

            let x = new_x as usize;
            let y = new_y as usize;

            if let Some(line) = self.board.get_mut(y) {
                if let Some(space) = line.get_mut(x) {
                    if space.can_visit() {
                        println!("\t ({}, {}) is visitable. Moving...", &x, &y);
                        self.guard.move_to(x, y);

                        if space.visit() {
                            self.spaces_visited += 1;
                        }
                    } else {
                        println!("\t ({}, {}) is not visitable. Rotating...", &x, &y);
                        self.guard.rotate();
                    }
                } else {
                    return Err(Error::new(format!("Could not get spot at x {}", &x)));
                }
            } else {
                return Err(Error::new(format!("Could not get spot at y {}", &y)));
            }
        }

        Ok(())
    }
}

struct Guard {
    x: usize,
    y: usize,
    direction: Direction,
}

impl Guard {
    fn get_next_move(&self) -> (isize, isize) {
        let (delta_x, delta_y) = self.direction.get_move();
        (self.x as isize + delta_x, self.y as isize + delta_y)
    }

    fn move_to(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    fn rotate(&mut self) {
        self.direction = self.direction.rotate();
    }
}

impl fmt::Display for Guard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Guard at ({}, {}) facing {}",
            self.x, self.y, self.direction
        )
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn get_move(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }

    fn rotate(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let direction = match self {
            Direction::North => "North",
            Direction::South => "South",
            Direction::East => "East",
            Direction::West => "West",
        };

        write!(f, "{}", direction)
    }
}

struct Space {
    space_type: SpaceType,
    visited: bool,
}

impl Space {
    fn parse(space: char) -> Result<Self, Error> {
        Ok(Space {
            space_type: SpaceType::parse(space)?,
            visited: space == GUARD,
        })
    }

    fn can_visit(&self) -> bool {
        self.space_type != SpaceType::Object
    }

    fn visit(&mut self) -> bool {
        let new_vist = !self.visited;
        self.visited = true;

        new_vist
    }
}

#[derive(PartialEq)]
enum SpaceType {
    Empty,
    Object,
}

impl SpaceType {
    fn parse(space: char) -> Result<Self, Error> {
        match space {
            OBJECT => Ok(SpaceType::Object),
            OPEN => Ok(SpaceType::Empty),
            GUARD => Ok(SpaceType::Empty),
            _ => Err(Error::new(format!("Invalid space: '{}'", space))),
        }
    }
}

#[derive(Debug)]
struct Error {
    message: String,
}

impl Error {
    fn new(message: String) -> Self {
        Error { message }
    }
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

// Things to define on the map
const OBJECT: char = '#';
const GUARD: char = '^';
const OPEN: char = '.';

const DEFAULT_INPUT: &str = "......##...#...#....#.......#....................##............#.#..#.......#.........................................#...........
..................................#.............#................................#..........##..................#.................
....#......................#................#...........................................#....................................#....
............#..............#...#...#...............#..........#.#....#..........................#....##...........................
....#.............#.....#.....................................................................#...#..........................#....
.................#............#......................#.................#.............#..................#.........................
................#...............#....................#...#..#...#.#.....................................................#.........
..............#.#...............................................................#....................#.......#....................
.............#...#..#............................#..............#..#.........................................#....................
.........................#.........#..#.......#...#..............#..#..............................#.........#............#.......
#...............................................................................................................#.................
..........................#.....#.........................#...................#.......#.........................#.................
...........#...........#........#........................#..........#.#.................#........#............#...............#...
..#..#................#...................................................#..................................................#....
.....#............................#..........................................................................#....................
................#.....#...........................#..........................................#.........#........................#.
..#...........................................##............#.........#..........................#.......................#.....#..
.....................#.............#.....#.......#.##.....#......................................#.#..#.#.........................
.......#...##.....#..........#...............#....#..........................#..............................................##....
..#.......#..................##..............#..........................#.#....#....#....................#.#.#...............#....
....................#.......#.......#............#..............#.......#.........#...#.....#........#.#..........................
...............................................#.....#........................................#..#.........#..........#...........
............#...#.............................................................................#.......##..........................
##..........................#...........................#..............................#..........................................
..............................................#.............................................#.................................#...
.............#..............#................##...........#...............#.......................................................
......#............#.......#..#....#.............................................................................#.#..............
..#..##..#..................#...............................................##......#........................................#....
..#...............................#.#........#................................................#.....#........#.#..................
.........#.......#.....................................#............................#..............................#..............
..........................................................................................................#.....................#.
....................#......................................#..........................#.........................##................
........#..............................................................................#.#..................#..................#..
........................................................^..#...................#....#..............##.............................
..#.#........................................................#..#..................#.....................................#....#...
.........................#....#............................#......................................................................
.#......#.............................................................................................#....................###....
...........#...#.#............#........................................................................#..........................
......#..........#.............................#.#.................................#...#.............#.#........................#.
...............................................#..........#.......#.........#........................................#.#..........
.................#.....................#.......#..................................................................................
.#...............#.........#...................#..............#........................#.....#....................................
...........#..............#..................................................................#...................................#
.................#.............#...........#.......................................#......................#......#..........#.....
........#..#.................#..........#....................................................#.....#...........#.................#
.....................#..#.................#......#......#..#..#..........#........................#...........................##..
.................#........#....#.................................................#.#.........................#............#..#....
.....................#...............##...................#.....#..#............................#.#.............#.................
.#..#......##...............#.......#..........................................................................#......#...........
#..#...................................#.#................#.....#.................................#...........................#...
..............#............................#............#...#...#.............#..........#............#...#............#..........
..........##.................#................................#...........#.#....#...............................................#
......#...........#.............#..........#.............#............#...................#.......................#...............
..............#.......#...........................................................................................................
...#................................................................#...............................#....#...............#........
...#..##................#..............................................................................#..........##.#............
.#........#..........#..................#......................#...........#......................................................
....#.......................#...#...........................................#..................#...............................#..
.............................#...........#..........#.................#.....................................................#.....
...........#.........#............#...............................#................#..............#.........................#.....
..........#........#.....#.....................................#.................#.....................................#..#.......
...#.......#...................................................................#.................#................................
.#....#........#..............................#........#.....#........#..#...#.#......................#..........................#
.....................................................................#....................#.......................................
.............................#..............................#.#....................................................#............#.
..#.....#.......#.......#......................#.#.#.........................................................#.....#........#.....
...............#..........#..#....#..................#..........................................#.................................
............#...................#................#.............................................................................#..
........#...#......................................##....................#..................#..........##............#...#........
......................#......#......#.#.........................................................................#.............#...
........#......................................................................................#.................#................
.................................................#....................#..............##.......................................#..#
.............#..........................................................#.................#..........................#....#......#
......................#.....................#............................#..................#...........#.........................
#..............#...........................#...#.#................................................................................
.......#............................#...........................................................#...............#.................
........................#..................................................................................#......................
...................#..........................#.........#........#...........................#....................................
.............#............#.......................................................................................##....#.........
................#.........#.....................................................................................................#.
........#.............................#.................................#.......##..............#............................#....
#........#.....#.....#.......#...............................#....................................................#..............#
........................#.............................#.....#...........#........#............#...................................
..................#...........#...#.............................................................................#...............#.
...................#..................#...................#..............................#....#...........#..............#........
...............#................#.#....................#.........#.........#.............#.........#......#..................#....
..................#.............................#...........................#....#....#......#..#.#....................#..........
...#.......................................#...............................................................#................#.....
...................#.........................................................#....................................................
.................#..............................#.................................................................................
.............#.......#...#....................................................#........#....................................#...#.
..............#...................#.#..........#....................................................#....#...#.##....#............
........#..#...........................................#.........#...#...............#.........................#..................
#.....#...........................................#.................................................#.............................
#...........................................#..........#..........................................#...............#...............
............................#.................................................#...#...............................................
.#...................#.......................................................................................#......#...##..#.....
.....#..#...............#........#....#.........#........................#.............#..........................................
...................#................#........................................#.#..................................................
......#.#...........................#...................#..........#..........#......#............................................
..........................#.#..............#............#.#.......................................................................
...#..#........................#..............................................#..#...#.....#........#.............................
..........................#.....................#.........................................................#............#.#........
............................#.....................#.............#.................................................................
..........................................#..#.................................................................#......#.#.#.......
..........##......................................#................................##....................#..#.....................
..............#.......................................................#...........................................................
..........#........#........................................................................................#........#...#........
......................#.#...................#...............#.....#......................#.....#.......#.......#..........#.......
......#..#....#..................#.........................................#......................................................
...............#.....#..#..........................................##.........................................##.............#....
.....................#..................#..................................#...................#............................#.#...
..#................#..#................................................#...........#..........#.............#...................#.
.......#.............#.......................................................................#...#.........................##..#..
...#...................#..................#.............#..#.........................................#........##..................
.................................#...........#........#.....................#.........................#..#........................
.......#..................#..................................#......................................#.#....................#......
..................................................................................#...........#....#.......#......#..........#....
........##.........................................................#.......................#........#............#................
...#...............##.#..................#....................................#.............#.................#...................
..................##.........#...................#......#................#......................................................#.
.#....................#............#..............#.............#.........#........................................#......#..#....
...............#...........................................................#........................#.............#...............
#............................#...................#...............................##..#...............#..........................#.
................#.............#..........#.........#..........................#....#..............................#..............#
...................................#.........................................#....................................................
....#................................................................#.#..#..........#....#.............................#.........
...........#.............#..........#........#...#...#........#.............#.......................................##............
.........#.......................#..........#....#.......................................#.....#...#.#..........#.................
.......................#...........#.......#......#.#...............................................#.................#.#.........";