use nom::{sequence::tuple,IResult,bytes::complete::tag};
use nom::character::complete::{digit1,alpha1,anychar};
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
pub struct Password<'a>{
    pub min: usize,
    pub max: usize,
    pub c: char,
    pub pw: &'a str,
}

impl Password<'_> {
    pub fn validate(&self) -> bool {
        let mut i = self.pw.chars();
        let a: char = i.nth(self.min-1).unwrap_or(' ');
        let b: char = i.nth(self.max-self.min-1).unwrap_or(' ');
        println!("{} {} {} {}",a,b,a == self.c, b == self.c);
        return (a == self.c) ^ (b == self.c);
    }
}

fn read_pass(input: &str) -> IResult<&str,Password> {
    let (input, (min,_,max,_,c,_,_,pw)) =
        tuple((digit1,anychar,digit1,anychar,anychar,anychar,anychar,alpha1))(input)?;
    let min = min.parse().expect("Parser error");
    let max = max.parse().expect("Parser error");
    return Ok((input,Password {min,max,c,pw}));
}

fn main() {
    let input_handle = File::open("./input.txt").expect("Couldn't open input!");
    let lines = io::BufReader::new(input_handle).lines();

    let mut count = 0;
    for line in lines {
        let line = line.unwrap();
        let (_,pass) = read_pass(&line).expect("Nani");
        println!("{:#?}",pass);
        if pass.validate() {
            println!("---VALID---");
            count = count + 1;
        }
    }
    println!("{}",count)
}
