extern crate clap;
use clap::{App, Arg};

use dice_nom::parsers::generator_parser;
use dice_nom::generators::Generator;

use std::i32::MAX;
use std::collections::BTreeMap;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("roll")
        .version(VERSION)
        .about("Generates random dice rolls")
        .author("Galen P.")
        .arg(Arg::with_name("display")
            .long("display")
            .short("d")
            .takes_value(true)
            .help("Display the results: full, value, or chart"))
        .arg(Arg::with_name("count")
            .long("count")
            .short("n")
            .takes_value(true)
            .help("Run the generator count number of times."))
        .arg(Arg::with_name("INPUT")
            .help("A dice roll expression is required.")
            .required(true)
            .index(1))
        .get_matches();

    let input = matches.value_of("INPUT").unwrap();
    let count = matches.value_of("count");
    let gen = match generator_parser(input) {
        Ok((_, gen)) => gen,
        Err(_) => panic!("could not parse `{}`", input)
    };
    
    match matches.value_of("display") {
        Some(s) => match s {    
            "display" => display_results(&gen, count),
            "value" => display_value(&gen, count),
            "chart" => display_chart(&gen, count),
            _ => display_results(&gen, count),
        },
        _ => display_results(&gen, count)
    }
}

fn display_results(gen: &Generator, count: Option<&str>) {
    match count {
        Some(n) => {
            let n = n.parse::<usize>().unwrap_or(1);
            for _ in 0..n {
                println!("{}: {}", gen, gen.generate());
            }
        },
        None => println!("{}: {}", gen, gen.generate())
    }
}

fn display_value(gen: &Generator, count: Option<&str>) {
    match count {
        Some(n) => {
            let n = n.parse::<usize>().unwrap_or(1);
            for _ in 0..n {
                println!("{}", gen.generate().sum());
            }
        },
        None => println!("{}", gen.generate().sum())
    }
}

fn display_chart(gen: &Generator, count: Option<&str>) {
    let num = count.unwrap_or("10000")
        .parse::<usize>().unwrap_or(10000);

    let mut min: i32 = MAX;
    let mut max: i32 = 0;
    let mut max_cnt: usize = 0;
    let mut map: BTreeMap<i32, usize> = BTreeMap::new(); 
    for _ in 0..num {
        let v = gen.generate().sum();
        if v < min { min = v; }
        if v > max { max = v; }
        match map.get(&v) {
            Some(n) => {
                let cnt = n + 1;
                if cnt > max_cnt { max_cnt = cnt; }
                map.insert(v, cnt);
            }   
            None    => {
                map.insert(v, 1); 
            }   
        }   
    }   

    let mut cnt = num as f64;
    let width = if max_cnt < 50 {
        1
    } else {
        max_cnt / 50
    };
    for k in min..=max {
        match map.get(&k) {
            Some(n) => {
                print!("{:>3}. {:>5.*}: ", k, 1, (cnt / num as f64) * 100.0);
                for _ in 0..=(n / width) { print!("*"); }
                println!("");
                cnt -= *n as f64;
            }
            None    => {
                println!("{:>3}. {:>5.*}:", k, 1, 0.0);
            }
        }
    }        
}