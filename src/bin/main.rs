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
    /// Display the results: full, value, json, or chart
    #[arg(short, long)]
    display: Option<String>,

    /// Output format: json or text (defaults to text)
    #[arg(short = 'f', long)]
    format: Option<String>,

    /// Run the generator count number of times.
    #[arg(short, long)]
    count: Option<u32>,

    input: String,
}

fn parse_display_and_format(display: &Option<String>, format: &Option<String>) -> (String, String) {
    match (display.as_deref(), format.as_deref()) {
        (Some("json"), _) => ("full".to_string(), "json".to_string()),
        (display_opt, Some(format_str)) => (
            display_opt.unwrap_or("full").to_string(),
            format_str.to_string(),
        ),
        (Some(display_str), None) if display_str != "json" => {
            (display_str.to_string(), "text".to_string())
        }
        _ => ("full".to_string(), "text".to_string()),
    }
}

fn main() {
    let args = Args::parse();
    let input = args.input;

    let g = match generator_parser(input.as_ref()) {
        Ok((_, g)) => g,
        Err(_) => panic!("could not parse `{}`", input),
    };

    let (display_mode, output_format) = parse_display_and_format(&args.display, &args.format);

    match (display_mode.as_str(), output_format.as_str()) {
        ("full", "text") => display_results(&g, args.count.unwrap_or(1)),
        ("full", "json") => display_json(&g, args.count.unwrap_or(1)),
        ("value", "text") => display_value(&g, args.count.unwrap_or(1)),
        ("value", "json") => display_value_json(&g, args.count.unwrap_or(1)),
        ("chart", "text") => display_chart(&g, args.count.unwrap_or(10_000)),
        ("chart", "json") => display_chart_json(&g, args.count.unwrap_or(10_000)),
        _ => display_results(&g, args.count.unwrap_or(1)),
    }
}

fn display_results(g: &Generator, n: u32) {
    let mut rng = rand::rng();
    for _ in 0..n {
        println!("{}: {}", g, g.generate(&mut rng));
    }
}

fn display_json(g: &Generator, n: u32) {
    let mut rng = rand::rng();
    let mut results_array = Vec::new();
    for _ in 0..n {
        results_array.push(g.generate(&mut rng));
    }
    let json = serde_json::to_string(&results_array);
    match json {
        Ok(json) => println!("{}", json),
        Err(err) => println!("{{\"error\": {}}}", err),
    }
}

fn display_value(g: &Generator, n: u32) {
    let mut rng = rand::rng();
    for _ in 0..n {
        println!("{}", g.generate(&mut rng).sum());
    }
}

fn display_value_json(g: &Generator, n: u32) {
    let mut rng = rand::rng();
    let mut values = Vec::new();
    for _ in 0..n {
        values.push(g.generate(&mut rng).sum());
    }
    let json = serde_json::to_string(&values);
    match json {
        Ok(json) => println!("{}", json),
        Err(err) => println!("{{\"error\": \"{}\"}}", err),
    }
}

fn display_chart(g: &Generator, num: u32) {
    let histo = Histo::build(g, num);

    let mut cnt = num as f64;
    let width = if histo.max_cnt < 50 {
        1
    } else {
        histo.max_cnt / 50
    };
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
    pub fn build(g: &Generator, count: u32) -> Histo {
        let mut histo = Histo {
            min: MAX,
            max: 0,
            max_cnt: 0,
            map: BTreeMap::new(),
        };
        let mut rng = rand::rng();
        for _ in 0..count {
            let v = g.generate(&mut rng).sum();
            if v < histo.min {
                histo.min = v;
            }
            if v > histo.max {
                histo.max = v;
            }
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

fn display_chart_json(g: &Generator, num: u32) {
    let histo = Histo::build(g, num);

    let mut chart_data = Vec::new();
    let mut total = 100.0;
    for k in histo.min..=histo.max {
        let count = histo.map.get(&k).unwrap_or(&0);
        let percentage = (*count as f64 / num as f64) * 100.0;
        chart_data.push(serde_json::json!({
            "value": k,
            "count": count,
            "percentage": percentage,
            "total": total,
        }));
        total = total - percentage;
    }

    let json = serde_json::to_string(&chart_data);
    match json {
        Ok(json) => println!("{}", json),
        Err(err) => println!("{{\"error\": \"{}\"}}", err),
    }
}
