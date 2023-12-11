use clap::Parser;
use std::fs;
use std::collections::VecDeque;
use itertools::Itertools;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

struct Fifo {
    queue: VecDeque<char>,
    max_len: usize,
    len: usize,
}

impl Fifo {
    fn new(max_len: usize) -> Fifo {
        Fifo {queue: VecDeque::new(), len: 0, max_len: max_len}
    }

    fn add(&mut self, c: char) -> bool {
        if self.len < self.max_len {
            self.len += 1;
            self.queue.push_back(c);
        } else {
            self.queue.pop_front();
            self.queue.push_back(c);
        }
        if self.queue.iter().unique().count() == self.max_len {
            true
        } else {
            false
        }
    }
}

fn main() {
    let args = Args::parse();

    let contents = fs::read_to_string(&args.input)
        .expect("Should have been able to read the file");
    // In part 1 we add 1 one row/column for each empty one.
    // In other words multiply amount of empty space by 2
    let res1 = read_contents(&contents, 4);
    println!("Part 1 answer is {}", res1);

    let res2 = read_contents(&contents, 14);
    println!("Part 2 answer is {}", res2);
}

fn read_contents(cont: &str, count: usize) -> i64 {
    let mut deq = Fifo::new(count);
    let i = cont.chars().into_iter().enumerate().find_map(|(i,v)| {
        if deq.add(v) {
            Some(i + 1)
        } else {
            None
        }
    }).unwrap();
    i as i64
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conts() {
        let a1 = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let a2 = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let a3 = "nppdvjthqldpwncqszvftbrmjlhg";
        let a4 = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let a5 = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(read_contents(&a1, 4), 7);
        assert_eq!(read_contents(&a2, 4), 5);
        assert_eq!(read_contents(&a3, 4), 6);
        assert_eq!(read_contents(&a4, 4), 10);
        assert_eq!(read_contents(&a5, 4), 11);

        assert_eq!(read_contents(&a1, 14), 19);
        assert_eq!(read_contents(&a2, 14), 23);
        assert_eq!(read_contents(&a3, 14), 23);
        assert_eq!(read_contents(&a4, 14), 29);
        assert_eq!(read_contents(&a5, 14), 26);
    }
}
