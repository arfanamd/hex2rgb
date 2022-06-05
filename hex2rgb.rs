use std::env;

fn manhelp() {
    eprintln!("
  SYNOPSIS
    hex2rgb <hex-color-values...>
    rgb2hex <red> <green> <blue>

  DESCRIPTION
    Show the preview color from the inputed value(s).

  EXAMPLE
    $ hex2rgb #ffffff #ababab
    $ rgb2hex 255 255 255
    ");
}
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

            r += convert(v_iter.next().unwrap()) * 0x10;
            r += convert(v_iter.next().unwrap()) * 0x1;
            g += convert(v_iter.next().unwrap()) * 0x10;
            g += convert(v_iter.next().unwrap()) * 0x1;
            b += convert(v_iter.next().unwrap()) * 0x10;
            b += convert(v_iter.next().unwrap()) * 0x1;

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
        if argc == 4 {
            println!("Nothing todo!");
        } else {
            manhelp();
        }
    } else {
        manhelp();
    }
}
