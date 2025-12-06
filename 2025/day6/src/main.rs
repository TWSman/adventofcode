use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input: String
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.input)
        .expect("Should have been able to read the file");
    let res = read_contents(&contents);
    println!("Part 1 answer is {}", res.0);  
    println!("Part 2 answer is {}", res.1);  
}


#[derive(Debug)]
enum Operation {
    Sum,
    Product
}

fn get_result(numbers: &Vec<Vec<i64>>, operations: &Vec<Operation>) -> i64 {
    let p = operations.len();
    let n = numbers.len();
    (0..p).map(|i|
        match operations.get(i).unwrap() {
            Operation::Sum => {
                (0..n).map(|j| {
                    numbers[j][i]
                }).sum::<i64>()
            }
            Operation::Product =>  {
                (0..n).map(|j| {
                    numbers[j][i]
                }).product()
            }
        }).sum()
}

fn read_contents(cont: &str) -> (i64, i64) {
    let part1 = get_part1(cont);
    let part2 = get_part2(cont);
    (part1, part2)
}

fn get_part2(cont: &str) -> i64 {
    let n = cont.lines().count();
    let number_length = n -1;
    let line_len = cont.lines().next().unwrap().len();
    let lines: Vec<Vec<char>> = cont.lines().map(|ln| ln.chars().collect::<Vec<char>>()).collect();
    let operations_line = cont.lines().last().unwrap();

    let mut operations: Vec<Operation> = Vec::new();
    let mut operations_ind: Vec<usize> = Vec::new();
    // Loop over the operations line, and get index of each operation
    // Input is organized such that the operation is in the first column of each calculation
    // There is an empty column before the operation column
    for (i,c) in operations_line.chars().enumerate() {
        match c {
            '*' => {
                operations.push(Operation::Product);
                operations_ind.push(i);
            }
            '+' => {
                operations.push(Operation::Sum);
                operations_ind.push(i);
            }
            _ => {
                continue;
            }
        }
    }
    let mut result: i64 = 0;
    for j in 0..operations.len() {
        let operation = operations.get(j).unwrap();
        let start = operations_ind.get(j).unwrap();

        let end = match operations_ind.get(j+1) {
            Some(v) => v - 1,
            None => line_len,
        };
        let number_count = end - start;

        let numbers: Vec<i64>= (0..number_count).map(|k| {
            let s = (0..number_length).map(|i| {
                lines[i][start+k]
            }).collect::<String>();
            s.trim().parse::<i64>().unwrap()
        }).collect();
        
        result += match operation {
            Operation::Product => numbers.iter().product::<i64>(),
            Operation::Sum => numbers.iter().sum::<i64>(),
        }
    }
    result
}

fn get_part1(cont: &str) -> i64 {
    let cand = cont.lines().map(|ln| {
        ln.split_whitespace().collect::<Vec<&str>>()
    }).collect::<Vec<Vec<&str>>>();
    let n = cand.len();
    let p = cand.get(0).unwrap().len();
    assert!(cand.iter().all(|m| m.len() == p));

    let operations: Vec<Operation> = cand.get(n-1).unwrap().iter().map(|o| {
        match *o {
            "*" => Operation::Product,
            "+" => Operation::Sum,
            _ => panic!("Unknown character"),
        }}).collect();

    let numbers: Vec<Vec<i64>> = cand.iter().take(n-1).map(|v| {
        v.iter().map(|c| c.parse().unwrap()).collect()
    }).collect();
    get_result(&numbers, &operations)
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn part1() {

//123 328  51 64 
// 45 64  387 23 
//  6 98  215 314
//*   +   *   +  
        let a = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!(get_part1(&a), 4277556);
    }

    #[test]
    fn part2() {

//123 328  51 64 
// 45 64  387 23 
//  6 98  215 314
//*   +   *   +  
        let a = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +";
        println!("{}", &a);
        assert_eq!(get_part2(&a), 3263827);
    }
}
