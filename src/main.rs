use clap::Parser;
use rand::prelude::*;
use rand_chacha::ChaCha12Rng;

static AVAILABLE_CHAR: [char; 84] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '!', '"', '#', '$', '%',
    '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<', '=', '>', '?', '@', '[',
    '\\', ']', '^', '_', '`', '{', '|', '}', '~',
];

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value_t = 16)]
    length: u32,
    #[arg(short, long)]
    filter: Option<String>,
}

fn gen_random_vector(size: &u32, ignore_set: &str) -> Vec<char> {
    let mut rng = ChaCha12Rng::from_entropy();

    let mut result: Vec<char> = vec![];

    for i in 0..size.to_owned() {
        let mut position: usize;
        position = rng.gen_range(0..84);

        while (i == 0 && position >= 52) || ignore_set.contains(AVAILABLE_CHAR[position]) {
            position = rng.gen_range(0..84);
        }

        result.push(AVAILABLE_CHAR[position]);
    }

    return result;
}

fn main() {
    let args = Cli::parse();

    if &args.length < &1 {
        eprint!("length must greater than 0");
    }

    let result: Vec<char> = gen_random_vector(&args.length, &args.filter.unwrap_or_default());
    print!("{}", &result.iter().collect::<String>())
}
