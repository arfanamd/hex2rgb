use std::env;

fn hex2rgb(value: String) {
    let convert = | ch | {
        match ch {
            '0'|'1'|'2'|'3'|'4' => ch.to_digit(10).unwrap(),
            '5'|'6'|'7'|'8'|'9' => ch.to_digit(10).unwrap(),
            'A'|'a' => 10,
            'B'|'b' => 11,
            'C'|'c' => 12,
            'D'|'d' => 13,
            'E'|'e' => 14,
            'F'|'f' => 15,
            _ => 0,
        }
    };
    if value.len() == 7 {
        let mut v_iter = value.chars();
        if v_iter.next() == Some('#') {
            let mut r: u32 = 0;
            let mut g: u32 = 0;
            let mut b: u32 = 0;

            r += convert(v_iter.next().unwrap()) * 16;
            r += convert(v_iter.next().unwrap()) * 1;
            g += convert(v_iter.next().unwrap()) * 16;
            g += convert(v_iter.next().unwrap()) * 1;
            b += convert(v_iter.next().unwrap()) * 16;
            b += convert(v_iter.next().unwrap()) * 1;

            println!("\x1b[48;2;{};{};{}m    \x1b[0m    {:>3} {:>3} {:>3}    {}",
                    r, g, b, r, g, b, value);
        } else {
            println!("[\x1b[31mFatal\x1b[0m]: {} Wrong input!", value);
        }
    } else {
        println!("[\x1b[31mFatal\x1b[0m]: {} Wrong input!", value);
    }
}
fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc: usize = argv.len();

    if argv[0].contains("hex2rgb") {
        for arg in 1..argc {
            hex2rgb(argv[arg].clone());
        }
    } else if argv[0].contains("rgb2hex") {
        if argc == 3 {
            println!("Nothing todo!");
        } else {}
    } else {}
}
