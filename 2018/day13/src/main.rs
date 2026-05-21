use clap::Parser;
use colored::Colorize;
use shared::Dir;
use shared::Vec2D;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String,
}

fn main() {
    let args = Args::parse();

    let start = Instant::now();
    let contents = fs::read_to_string(args.input).expect("Should have been able to read the file");
    let res = read_contents(&contents);
    println!("\n########################");
    println!("Part 1 answer is {:?}", res.0);
    println!("Part 2 answer is {:?}", res.1);
    let elapsed = start.elapsed();
    println!("Execution lasted {:.2?}", elapsed);
}

fn read_contents(cont: &str) -> ((i64, i64), (i64, i64)) {
    let grid = read_grid(cont);
    let part1 = get_part1(&grid);
    let part2 = get_part2(&grid);
    (part1, part2)
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Object {
    Empty,
    Vertical,
    Horizontal,
    Cross,
    Cart(Dir),
    CornerEN, // East to North
    CornerES, // East to Nouth
}

#[derive(Debug, Clone)]
struct Grid {
    grid: BTreeMap<Vec2D, Object>,
    carts: Vec<Cart>,
}

#[derive(Debug, Clone)]
struct Cart {
    loc: Vec2D,
    dir: Dir,
    turns: usize,
}

impl Grid {
    fn print_grid(&self, x_lim: Option<(i64, i64)>, y_lim: Option<(i64, i64)>) {
        let (min_x, max_x) = if let Some(t) = x_lim {
            t
        } else {
            (
                self.grid.keys().map(|v| v.x).min().unwrap(),
                self.grid.keys().map(|v| v.x).max().unwrap(),
            )
        };

        let (min_y, max_y) = if let Some(t) = y_lim {
            t
        } else {
            (
                self.grid.keys().map(|v| v.y).min().unwrap(),
                self.grid.keys().map(|v| v.y).max().unwrap(),
            )
        };

        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                let mut f = 0;
                let mut out = '.';
                for cart in self.carts.iter() {
                    if cart.loc == (Vec2D { x, y }) {
                        out = cart.dir.get_char();
                        f += 1;
                        if f > 1 {
                            out = 'X';
                        }
                    }
                }
                if f > 0 {
                    print!("{}", out.to_string().red().on_black());
                    continue;
                }
                match self.grid.get(&Vec2D { x, y }) {
                    Some(Object::Horizontal) => {
                        print!("{}", "-".blue().on_black());
                    }
                    Some(Object::Vertical) => {
                        print!("{}", "|".blue().on_black());
                    }
                    Some(Object::CornerEN) => {
                        print!("{}", "/".blue().on_black());
                    }
                    Some(Object::CornerES) => {
                        print!("{}", "\\".blue().on_black());
                    }
                    Some(Object::Cross) => {
                        print!("{}", "+".blue().on_black());
                    }
                    Some(&Object::Empty) => {
                        print!("{}", "b".black().on_black());
                    }
                    Some(&Object::Cart(dir)) => {
                        print!("{}", dir.get_char().to_string().red().on_black());
                    }
                    None => {
                        print!("{}", ".".black().on_black());
                    }
                }
            }
            println!();
        }
    }

    fn evolve(&mut self) -> Vec<Vec2D> {
        self.carts
            .sort_by(|a, b| b.loc.y.cmp(&a.loc.y).then(a.loc.x.cmp(&b.loc.x)));

        let n_carts = self.carts.len();
        let mut crashes = Vec::new();
        let mut carts_to_remove: BTreeSet<usize> = BTreeSet::new();
        for i1 in 0..n_carts {
            let cart1 = self.carts.get_mut(i1).unwrap();
            if carts_to_remove.contains(&i1) {
                continue;
            }
            cart1.loc = cart1.loc + cart1.dir.get_dir_true_vec();
            match self.grid.get(&cart1.loc).unwrap() {
                Object::Horizontal | Object::Vertical => {} // Continue as normal
                Object::CornerEN => {
                    //  /
                    cart1.dir = match cart1.dir {
                        Dir::W => Dir::S,
                        Dir::E => Dir::N,
                        Dir::N => Dir::E,
                        Dir::S => Dir::W,
                    }
                }
                Object::CornerES => {
                    // \
                    cart1.dir = match cart1.dir {
                        Dir::W => Dir::N,
                        Dir::E => Dir::S,
                        Dir::N => Dir::W,
                        Dir::S => Dir::E,
                    }
                }
                Object::Cross => {
                    if cart1.turns % 3 == 0 {
                        cart1.dir = cart1.dir.ccw();
                    }
                    if cart1.turns % 3 == 2 {
                        cart1.dir = cart1.dir.cw();
                    }
                    cart1.turns += 1;
                }
                _ => {
                    panic!("Should not happen");
                }
            }
            let loc = cart1.loc;
            for i2 in 0..n_carts {
                if i1 == i2 {
                    continue;
                }
                let cart2 = self.carts.get(i2).unwrap();
                if loc == cart2.loc {
                    println!("Crash at {:?}", loc);
                    let x_lim = Some((loc.x - 4, loc.x + 4));
                    let y_lim = Some((loc.y - 4, loc.y + 4));
                    self.print_grid(x_lim, y_lim);
                    crashes.push(loc);
                    carts_to_remove.insert(i1);
                    carts_to_remove.insert(i2);
                }
            }
        }
        for i in carts_to_remove.iter().rev() {
            self.carts.remove(*i);
        }
        crashes
    }
}

fn get_part1(grid: &Grid) -> (i64, i64) {
    let mut grid = grid.clone();
    let mut loop_count = 0;
    loop {
        loop_count += 1;
        if loop_count > 100_000 {
            return (0, 0);
        }
        let crashes = grid.evolve();
        if !crashes.is_empty() {
            let t = crashes[0];
            return (t.x, -t.y);
        }
    }
}

fn get_part2(grid: &Grid) -> (i64, i64) {
    let mut grid = grid.clone();
    let mut loop_count = 0;
    loop {
        loop_count += 1;
        if loop_count > 100_000 {
            return (0, 0);
        }
        let _ = grid.evolve();
        if grid.carts.len() == 1 {
            let t = grid.carts.first().unwrap().loc;
            println!("One cart remaining after {loop_count} ticks");
            return (t.x, -t.y);
        }
    }
}

fn read_grid(cont: &str) -> Grid {
    let mut grid = cont
        .lines()
        .enumerate()
        .fold(BTreeMap::new(), |mut grid, (y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                let obj = match c {
                    ' ' => Object::Empty,
                    '|' => Object::Vertical,
                    '-' => Object::Horizontal,
                    '+' => Object::Cross,
                    '/' => Object::CornerEN,
                    '\\' => Object::CornerES,
                    '>' => Object::Cart(Dir::E),
                    '<' => Object::Cart(Dir::W),
                    '^' => Object::Cart(Dir::N),
                    'v' => Object::Cart(Dir::S),
                    c => {
                        dbg!(&c);
                        todo!()
                    }
                };
                grid.insert(
                    Vec2D {
                        x: x as i64,
                        y: -(y as i64),
                    },
                    obj,
                );
            });
            grid
        });

    let mut carts = Vec::new();
    for (i, obj) in grid.iter() {
        if let Object::Cart(dir) = obj {
            let cart = Cart {
                loc: *i,
                dir: *dir,
                turns: 0,
            };
            carts.push(cart);
        }
    }
    for cart in &carts {
        let new_o = match cart.dir {
            Dir::N | Dir::S => Object::Vertical,
            Dir::E | Dir::W => Object::Horizontal,
        };
        grid.insert(cart.loc, new_o);
    }
    dbg!(&carts.len());
    Grid { grid, carts }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let a = r"/-----\
|     |
|  /--+--\
|  |  |  |
\--+--/  |
   |     |
   \-----/";

        let grid = read_grid(a);
        grid.print_grid(None, None);
        println!();

        let b = r"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/";

        let grid = read_grid(b);
        grid.print_grid(None, None);
        //grid.evolve();

        assert_eq!(get_part1(&grid), (7, 3));
    }

    #[test]
    fn part2() {
        let a = r"/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/";

        let grid = read_grid(&a);
        grid.print_grid(None, None);
        assert_eq!(get_part2(&grid), (6, 4));
    }
}
