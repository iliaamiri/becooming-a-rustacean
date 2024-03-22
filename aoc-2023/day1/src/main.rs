use std::{fs::File, io::{self, BufReader, BufRead, Lines}, path::Path};

pub mod linked_list;

fn main() {
    // part1();

    part2::run();
}

mod part2 {
    use std::collections::HashMap;

    use crate::read_lines;

    // one, two, three, four, five, six, seven, eight, nine, ten

    pub fn run() {
        let search = HashMap::from([
                                   ('o', HashMap::from([('n', )]))
        ])

        let mut result = 0;
        if let Ok(lines) = read_lines("input") {
            for line in lines.flatten() {
                
            }
        }
    }

}

#[allow(dead_code)]
fn part1() {
    let mut result = 0;

    let mut two_chars = vec!['0'; 2];
    
    if let Ok(lines) = read_lines("input") {
        for line in lines.flatten() {
            two_chars[0] = 'X';
            for c in line.chars() {
                if c.is_numeric() {
                    two_chars[0] = c;
                    break;
                }
            }

            for c in line.chars().rev() {
                if c.is_numeric() {
                    two_chars[1] = c;
                    break;
                }
            }

            let line_sum: i32 = String::from(format!("{}{}", two_chars[0], two_chars[1]))
                .parse::<i32>()
                .unwrap_or(0);

            result += line_sum;
        }
    }

    println!("result is {}", result);
}

fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>, io::Error>
where P: AsRef<Path>, {
    let file = File::open(filename)?;

    Ok(BufReader::new(file).lines())
}
