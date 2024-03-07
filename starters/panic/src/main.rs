use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};
use std::os::fd::AsRawFd;

fn main() {
    println!("Hello, world!");

    let a_file = File::open("a_file.txt");

    match a_file {
        Ok(_f) => {
            println!("this is fun ain't it");
        },
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                let new_file = match File::create("a_file.txt") {
                    Ok(f) => f,
                    Err(e) => panic!("{:?}", e)
                };

                println!("New file got created. {:#?}", new_file.as_raw_fd());
            },
            other_error => {
                panic!("{:?}", other_error);
            }
        }
    }

    let b_file_name = "b_file.txt";
    let _b_file = File::open(b_file_name).unwrap_or_else(|error| {
        println!("Problem creating {} {:?}", b_file_name, error);
        File::create(b_file_name).unwrap_or_else(|error| {
            panic!("There was an error while creating the b_file {}", error);
        })
    });

    let username = read_username_from_file("b_file.txt").unwrap_or_else(|error| {
        panic!("Something wrong happened... {:?}", error);
    });
    println!("username is {}", username);

    let _username_based = read_username_from_file_based("b_file.txt").unwrap_or_else(|error| {
        panic!("read the username from the file. {:?}", error);
    });

    let c_file_name = "c_file.txt";
    let _c_file = File::open(c_file_name).expect("c_file should exist");
}

fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    let file_result = File::open(path);

    let mut file = match file_result {
        Ok(f) => f,
        Err(e) => return Err(e)
    };

    let mut username = String::new();

    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_based(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

#[allow(unused)]
fn read_username_from_file_even_more_based(path: &str) -> Result<String, io::Error> {
    let mut username = String::new();

    File::open(path)?.read_to_string(&mut username)?;

    Ok(username)
}

#[allow(unused)]
fn read_username_from_file_the_most_based(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}
