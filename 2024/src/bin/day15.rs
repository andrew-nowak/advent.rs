use std::time::Instant;

use aoclib::{Direction, Point};
use rustc_hash::FxHashMap as HashMap;

#[derive(Clone, Copy, Debug)]
enum Tile {
    Wall,
    Crate,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Width {
    Thin,
    Wide,
}

fn print_map(m: &HashMap<Point, Tile>, bot: &Point, width: i64, height: i64) {
    for y in 0..height {
        for x in 0..width {
            if bot.x == x && bot.y == y {
                print!("@");
            } else {
                match m.get(&Point { x, y }) {
                    None => print!("."),
                    Some(Tile::Crate) => print!("O"),
                    Some(Tile::Wall) => print!("#"),
                }
            }
        }
        println!();
    }
}

fn run(data: &str) {
    let start = Instant::now();

    let (mapdata, movedata) = data.split_once("\n\n").expect("didn't split to 2 sections");

    let width = mapdata
        .as_bytes()
        .iter()
        .position(|&b| b == b'\n')
        .expect("no newline")
        + 1;

    let height = mapdata.len() / width + 1;

    let mut bot_start = None;
    let mut map_start: HashMap<Point, Tile> = HashMap::default();

    for (i, b) in mapdata.as_bytes().iter().enumerate() {
        let p = Point {
            x: (i % width) as i64,
            y: (i / width) as i64,
        };
        match b {
            b'@' => bot_start = Some(p),
            b'O' => {
                map_start.insert(p, Tile::Crate);
                ()
            }
            b'#' => {
                map_start.insert(p, Tile::Wall);
                ()
            }
            _ => (),
        }
    }

    let bot_start = bot_start.expect("Didn't find bot start point");

    let mut bot = bot_start.clone();
    let mut map = map_start.clone();

    for arrow in movedata.as_bytes().iter() {
        let dir = match *arrow {
            b'<' => Some(Direction::Left),
            b'^' => Some(Direction::Up),
            b'>' => Some(Direction::Right),
            b'v' => Some(Direction::Down),
            b'\n' => continue,
            _ => None,
        }
        .expect(&format!("unknown direction arrow {}", *arrow as char));

        let mut crate_queue: Vec<Point> = vec![];
        while let Some(Tile::Crate) = map.get(&crate_queue.last().unwrap_or(&bot).go(&dir)) {
            crate_queue.push(crate_queue.last().unwrap_or(&bot).go(&dir));
        }
        let beyond = map.get(&crate_queue.last().unwrap_or(&bot).go(&dir));

        if let Some(Tile::Crate) = beyond {
            panic!(
                "At end of crate_queue {:?} from {:?} was another crate??",
                dir, bot
            );
        } else if let Some(Tile::Wall) = beyond {
            // do nothing!
            ()
        } else {
            // None - open space to push crates and bot
            bot = bot.go(&dir);
            if let Some(pos) = crate_queue.first() {
                map.remove(pos);
            }
            if let Some(pos) = crate_queue.last() {
                map.insert(pos.go(&dir), Tile::Crate);
            }
        }
    }

    let mut gps = 0;

    for (pos, tile) in map.iter() {
        if let Tile::Crate = tile {
            gps += 100 * pos.y + pos.x;
        }
    }
    //print_map(&map, &bot, width as i64, height as i64);

    println!("Part 1: {}", gps);

    // part 2:
    // don't actually represent the double width walls and crates; instead only mark their left
    // positions, and behave correctly when you interact with the rightmost position, which
    // otherwise appears to be empty space. (this is tricky to get right and you will have
    // edgecases, but keeps the map simple at 1:1 coordinate->item)

    let mut map = map_start
        .iter()
        .map(|(p, &tile)| (Point { x: p.x * 2, y: p.y }, tile))
        .collect::<HashMap<Point, Tile>>();

    let mut bot = Point {
        x: bot_start.x * 2,
        y: bot_start.y,
    };

    //print_map(&map, &bot, 2 * width as i64, height as i64);
    for arrow in movedata.as_bytes().iter() {
        //println!("=====");
        //println!("{}", *arrow as char);
        let dir = match *arrow {
            b'<' => Some(Direction::Left),
            b'^' => Some(Direction::Up),
            b'>' => Some(Direction::Right),
            b'v' => Some(Direction::Down),
            b'\n' => continue,
            _ => None,
        }
        .expect(&format!("unknown direction arrow {}", *arrow as char));

        if let Some(updates) = p2_move_crates(&map, &bot, &dir, Width::Thin) {
            for (from, _to) in updates.iter() {
                map.remove(&from);
            }
            for (from, to) in updates.iter() {
                if *from != bot {
                    //println!("crate from {:?} to {:?}", from, to);
                    map.insert(*to, Tile::Crate);
                }
            }
            bot = bot.go(&dir);
        }
        //print_map(&map, &bot, 2 * width as i64, height as i64);
    }
    let mut gps = 0;

    for (pos, tile) in map.iter() {
        if let Tile::Crate = tile {
            gps += 100 * pos.y + pos.x;
        }
    }
    print_map(&map, &bot, 2 * width as i64, height as i64);

    println!("Part 2: {}", gps);

    //println!("Part 2: {}", p2);

    let end = Instant::now();
    println!("in {}μs", (end - start).as_micros());
}

// recursively find all the crates that move in this step, along with their destination
// so the map can be updated in the main loop
// width: robot is "Thin" (only exists in 1 sq on x-dim), all others are "Wide" (exist in 2sq)
// Thin things will only interact with forward and forward-left, wide will also interact with
// forward-right.
fn p2_move_crates(
    map: &HashMap<Point, Tile>,
    position: &Point,
    direction: &Direction,
    width: Width,
) -> Option<HashMap<Point, Point>> {
    match direction {
        Direction::Up | Direction::Down => {
            let forward = position.go(&direction);
            let fw_left = forward.left();
            let fw_right = forward.right();

            match (map.get(&fw_left), map.get(&forward), map.get(&fw_right)) {
                (Some(Tile::Wall), _, _) | (_, Some(Tile::Wall), _) => None,
                (_, _, Some(Tile::Wall)) if width == Width::Wide => None,

                (None, None, Some(Tile::Crate)) if width == Width::Wide => {
                    p2_move_crates(map, &fw_right, direction, Width::Wide).map(|mut crate_moves| {
                        crate_moves.insert(*position, forward);
                        crate_moves
                    })
                }

                (Some(Tile::Crate), None, Some(Tile::Crate)) if width == Width::Wide => {
                    match (
                        p2_move_crates(map, &fw_left, direction, Width::Wide),
                        p2_move_crates(map, &fw_right, direction, Width::Wide),
                    ) {
                        (Some(updl), Some(updr)) => {
                            let mut crate_moves = updl.clone();
                            crate_moves.extend(updr);
                            crate_moves.insert(*position, forward);
                            Some(crate_moves)
                        }
                        _ => None,
                    }
                }

                (None, None, _) => Some(HashMap::from_iter([(*position, position.go(direction))])),

                (None, Some(Tile::Crate), _) => {
                    p2_move_crates(map, &forward, direction, Width::Wide).map(|mut crate_moves| {
                        crate_moves.insert(*position, forward);
                        crate_moves
                    })
                }
                (Some(Tile::Crate), None, _) => {
                    p2_move_crates(map, &fw_left, direction, Width::Wide).map(|mut crate_moves| {
                        crate_moves.insert(*position, forward);
                        crate_moves
                    })
                }
                o => panic!("what on earth {:?}", o),
            }
        }
        Direction::Left | Direction::Right => {
            let distance = if *direction == Direction::Right && width == Width::Thin {
                1
            } else {
                2
            };
            match map.get(&position.go_by(&direction, distance)) {
                Some(Tile::Wall) => None,
                Some(Tile::Crate) => p2_move_crates(
                    map,
                    &position.go_by(direction, distance),
                    direction,
                    Width::Wide,
                )
                .map(|mut crate_moves| {
                    crate_moves.insert(*position, position.go(direction));
                    crate_moves
                }),
                None => Some(HashMap::from_iter([(*position, position.go(direction))])),
            }
        }
    }
}

fn main() {
    let example = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    //    let example = "\
    //#######
    //#...#.#
    //#.....#
    //#..OO@#
    //#..O..#
    //#.....#
    //#######
    //
    //<vv<<^^<<^>>";

    let real = include_str!("../../in/15.txt").trim();

    println!("--- example ---");
    run(example);

    println!("--- real ---");
    run(real);
}
