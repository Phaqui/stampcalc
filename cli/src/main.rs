use libstampcalc::Solutions;
//use libstampcalc::{solutions_for_price, num_iterations_for};

struct Input {
    price: u32,
    stamps: Vec<u32>,
}

fn parse_cmdline_args() -> Result<Input, &'static str> {
    let args = std::env::args().skip(1)
        .map(|x| x.parse::<u32>().map_err(|_| "error: all numbers must be positive"))
        .collect::<Result<Vec<u32>, _>>()?;

    match args.len() {
        0 | 1 => Err("usage: ./prog <price> <stamp1> <stamp2> ... <stampN>"),
            _ => Ok(Input { price: args[0], stamps: args[1..].to_vec() }),
    }
}

fn run_app() -> Result<(), &'static str> {
    let input = parse_cmdline_args()?;
    let mut i: usize = 0;
    //*
    let mut solutions = Solutions::new(input.price, &input.stamps);
    let n = solutions.num_iterations_for();
    for _x in solutions.make_into_iterator() {
        i += 1;
        if i % 1000 == 0 {
            //println!("{:?}", x);
        }
    }
    //*/
    /*
    let answers = solutions_for_price(input.price, &input.stamps);
    let n = num_iterations_for(input.price, &input.stamps);
    for _ in answers {
        i += 1;
    }
    // */
    println!("Found {} results (checked {} possible combinations)", i, n);
    Ok(())
}

fn main() {
    std::process::exit(match run_app() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    });
}
