// https://stackoverflow.com/questions/27043268/convert-a-string-to-int

fn manhelp() {
    eprintln!("
  SYNOPSIS
    hex2rgb <hex-color-values...>
    rgb2hex <red> <green> <blue>

  DESCRIPTION
    Color preview from the inputed value(s).

  EXAMPLE
    $ hex2rgb \"#ffffff\" abcabc
    \x1b[48;2;255;255;255m    \x1b[0m    255 255 255    #ffffff
    \x1b[48;2;171;202;188m    \x1b[0m    171 202 188    #ababab

    $ rgb2hex 255 255 255
    \x1b[48;2;255;255;255m    \x1b[0m    255 255 255    #ffffff
    ");
    std::process::exit(0);
}

fn rgb2hex(v1: String, v2: String, v3: String) {
    let r = v1.parse::<u32>().unwrap_or_else(|_e| {
        println!("[\x1b[31mFatal\x1b[0m]: Wrong input!");
        std::process::exit(1);
    });
    let g = v2.parse::<u32>().unwrap_or_else(|_e| {
        println!("[\x1b[31mFatal\x1b[0m]: Wrong input!");
        std::process::exit(1);
    });
    let b = v3.parse::<u32>().unwrap_or_else(|_e| {
        println!("[\x1b[31mFatal\x1b[0m]: Wrong input!");
        std::process::exit(1);
    });

    if r <= 0xff && g <= 0xff && b <= 0xff {
        println!("\x1b[48;2;{};{};{}m    \x1b[0m    {:>3} {:>3} {:>3}    #{:0>2X}{:0>2X}{:0>2X}",
                 r, g, b, r, g, b, r, g, b);
    } else {
        println!("[\x1b[31mFatal\x1b[0m]: The value must less than 256!");
        std::process::exit(1);
    }
}

fn hex2rgb(value: String) {
    let convert = | ch | {
        match ch {
            '0'|'1'|'2'|'3'|'4' => ch.to_digit(16).unwrap(),
            '5'|'6'|'7'|'8'|'9' => ch.to_digit(16).unwrap(),
            'A'|'a' => 0xa,
            'B'|'b' => 0xb,
            'C'|'c' => 0xc,
            'D'|'d' => 0xd,
            'E'|'e' => 0xe,
            'F'|'f' => 0xf,
            _ => 0x0,
        }
    };
    if value.len() == 0x7 {
        let mut v_iter = value.chars();
        if v_iter.next() == Some('#') {
            let mut r: u32 = 0x0;
            let mut g: u32 = 0x0;
            let mut b: u32 = 0x0;

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
    } else if value.len() == 0x6 {
        let mut v_iter = value.chars();
        let mut r: u32 = 0x0;
        let mut g: u32 = 0x0;
        let mut b: u32 = 0x0;

        r += convert(v_iter.next().unwrap()) * 0x10;
        r += convert(v_iter.next().unwrap()) * 0x1;
        g += convert(v_iter.next().unwrap()) * 0x10;
        g += convert(v_iter.next().unwrap()) * 0x1;
        b += convert(v_iter.next().unwrap()) * 0x10;
        b += convert(v_iter.next().unwrap()) * 0x1;

        println!("\x1b[48;2;{};{};{}m    \x1b[0m    {:>3} {:>3} {:>3}    #{}",
                r, g, b, r, g, b, value);
    } else {
        println!("[\x1b[31mFatal\x1b[0m]: {} Wrong input!", value);
    }
}

fn main() {
    let argv: Vec<String> = std::env::args().collect();
    let argc: usize = argv.len();

    if argc != 0x1 {
        if argv[0].contains("hex2rgb") {
            for arg in 1..argc {
                hex2rgb(argv[arg].clone());
            }
        } else if argv[0].contains("rgb2hex") {
            if argc == 0x4 {
                rgb2hex(argv[1].clone(), argv[2].clone(), argv[3].clone());
            } else {
                manhelp();
            }
        } else {
            manhelp();
        }
    } else {
        manhelp();
    }
}
