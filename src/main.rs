use structopt::StructOpt;
use rand::Rng;

#[derive(StructOpt)]
struct Cli {
    //long and short flags (-s, --size)
    #[structopt(long, short, default_value = "10")]
    size: i32,
    #[structopt(long, short, default_value = "normal")]
    complexity: String,
}

const NORMAL: [&str; 62] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o",
    "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D",
    "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z", ];

const HARD: [&str; 83] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o",
    "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D",
    "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S",
    "T", "U", "V", "W", "X", "Y", "Z", "!", "#", "$", "%", "&", "(", ")", "=",
    "|", ".", ",", ";", "[", "]", "/", "<", ">", ":", "@", "?", "_"];


fn main() {
    let args = Cli::from_args();
    let mut password = "".to_string();
    let mut rng = rand::thread_rng();
    if &args.complexity == "hard" {
        for _i in 0..args.size {
            password += HARD[rng.gen_range(0..HARD.len())];
        }
    } else {
        for _i in 0..args.size {
            password += NORMAL[rng.gen_range(0..NORMAL.len())];
        }
    }
    println!("{}", password);

}
