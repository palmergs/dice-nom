extern crate clap;
use clap::Parser;

use dice_nom::generators::Generator;
use dice_nom::parsers::generator_parser;

use std::collections::BTreeMap;
use std::i32::MAX;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser, Debug)]
#[command(name = "roll")]
#[command(author = "Galen P <galenp@gmail.com>")]
#[command(version = VERSION)]
#[command(about = "Generates random dice rolls")]
struct Args {
    /// Display the results: full, value, or chart
    #[arg(short, long)]
    display: Option<String>,

    /// Run the generator count number of times.
    #[arg(short, long)]
    count: Option<u32>,

    input: String,
}


fn main() {

    let args = Args::parse();
    let input = args.input;

    let gen = match generator_parser(input.as_ref()) {
        Ok((_, gen)) => gen,
        Err(_) => panic!("could not parse `{}`", input),
    };

    match args.display  {
        Some(s) => match s.as_str() {
            "full" => display_results(&gen, args.count.unwrap_or(1)),
            "value" => display_value(&gen, args.count.unwrap_or(1)),
            "chart" => display_chart(&gen, args.count.unwrap_or(10_000)),
            _ => display_results(&gen, args.count.unwrap_or(1)),
        },
        _ => display_results(&gen, args.count.unwrap_or(1)),
    }
}

fn display_results(gen: &Generator, n: u32) {
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        println!("{}: {}", gen, gen.generate(&mut rng));
    }
}

fn display_value(gen: &Generator, n: u32) {
    let mut rng = rand::thread_rng();
    for _ in 0..n {
        println!("{}", gen.generate(&mut rng).sum());
    }
}

fn display_chart(gen: &Generator, num: u32) {
    let histo = Histo::build(gen, num);

    let mut cnt = num as f64;
    let width = if histo.max_cnt < 50 { 1 } else { histo.max_cnt / 50 };
    for k in histo.min..=histo.max {
        match histo.map.get(&k) {
            Some(n) => {
                print!("{:>3}. {:>5.*}: ", k, 1, (cnt / num as f64) * 100.0);
                for _ in 0..=(n / width) {
                    print!("*");
                }
                println!();
                cnt -= *n as f64;
            }
            None => {
                println!("{:>3}. {:>5.*}:", k, 1, 0.0);
            }
        }
    }
}

struct Histo {
    min: i32,
    max: i32,
    max_cnt: u32,
    map: BTreeMap<i32, u32>,
}

impl Histo {
    pub fn build(gen: &Generator, count: u32) -> Histo {
        let mut histo = Histo{ min: MAX, max: 0, max_cnt: 0, map: BTreeMap::new() };
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            let v = gen.generate(&mut rng).sum();
            if v < histo.min { histo.min = v; }
            if v > histo.max { histo.max = v; }
            match histo.map.get(&v) {
                Some(n) => {
                    let cnt = n + 1;
                    if cnt > histo.max_cnt {
                        histo.max_cnt = cnt;
                    }
                    histo.map.insert(v, cnt);
                }
                None => {
                    histo.map.insert(v, 1);
                }
            }
        }
        histo
    }
}
