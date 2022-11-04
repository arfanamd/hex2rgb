/* This project is released under
 *
 * THE BEER-WARE LICENSE (Revision 42):
 *
 * As long as you retain this notice you can do whatever you want
 * with this stuff.  If we meet someday, and you think this stuff
 * is worth it, you can buy us a beer in return.
 *
 * This project is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY;  without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
*/
use std::{
	env,      /* for args.collect() */
	process,  /* for exit() */
};
const CANCEL_OPERATION: i32 = 125;  /* exit code */

fn print_help() {
	eprintln!("
  Convert hex to rgb or vise versa.

  SYNOPSIS
    hex2rgb <color-hex-value ...>
    rgb2hex <red> <green> <blue>

  EXAMPLES
    /* convert multi hex value to rgb. */
    $ hex2rgb \"#ffffff\" abcabc
    \x1b[48;2;255;255;255m        \x1b[0m    255 255 255    #ffffff
    \x1b[48;2;171;202;188m        \x1b[0m    171 202 188    #ababab

    /* convert rgb to hex color. */
    $ rgb2hex 255 255 255
    \x1b[48;2;255;255;255m        \x1b[0m    255 255 255    #ffffff
");
}
fn exerr(msg: &str) {
	eprintln!("error: {}", msg);
	process::exit(CANCEL_OPERATION);
}

fn rgb2hex(v1: String, v2: String, v3: String) {
	// Thanks to this thread:
	// stackoverflow.com/questions/27043268/convert-a-string-to-int
	let r = v1.parse::<u32>().unwrap_or_else(|_e| {
		exerr(&format!("{} Wrong input!", v1));
	});
	let g = v2.parse::<u32>().unwrap_or_else(|_e| {
		exerr(&format!("{} Wrong input!", v2));
	});
	let b = v3.parse::<u32>().unwrap_or_else(|_e| {
		exerr(&format!("{} Wrong input!", v3));
	});

	if r <= 0xff && g <= 0xff && b <= 0xff {
		println!("\x1b[48;2;{};{};{}m        \x1b[0m    \
								 {:>3} {:>3} {:>3}    #{:0>2x}{:0>2x}{:0>2x}",
								 r, g, b, r, g, b, r, g, b);
	} else {
		exerr("The value must be less than 256!");
	}
}

fn hex2rgb(value: String) {
	// convert each character to a decimal.
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
	// if the value starts with the pawn sign '#'
	if value.len() == 0x7 {
		// string iteration.
		let mut v_iter = value.chars();
		if v_iter.next() == Some('#') {
			let mut r: u32 = 0x0;
			let mut g: u32 = 0x0;
			let mut b: u32 = 0x0;

			// 16^0 && 16^1
			r += convert(v_iter.next().unwrap()) * 0x10;
			r += convert(v_iter.next().unwrap()) * 0x1;
			g += convert(v_iter.next().unwrap()) * 0x10;
			g += convert(v_iter.next().unwrap()) * 0x1;
			b += convert(v_iter.next().unwrap()) * 0x10;
			b += convert(v_iter.next().unwrap()) * 0x1;

			println!("\x1b[48;2;{};{};{}m        \x1b[0m    \
										 {:>3} {:>3} {:>3}    {}",
										 r, g, b, r, g, b, value);
		} else {
			exerr(&format!("{} Wrong input!", value));
		}
		// if the value not starts with the pawn sign '#'
	} else if value.len() == 0x6 {
		// string iteration.
		let mut v_iter = value.chars();
		let mut r: u32 = 0x0;
		let mut g: u32 = 0x0;
		let mut b: u32 = 0x0;

		// 16^0 && 16^1
		r += convert(v_iter.next().unwrap()) * 0x10;
		r += convert(v_iter.next().unwrap()) * 0x1;
		g += convert(v_iter.next().unwrap()) * 0x10;
		g += convert(v_iter.next().unwrap()) * 0x1;
		b += convert(v_iter.next().unwrap()) * 0x10;
		b += convert(v_iter.next().unwrap()) * 0x1;

		println!("\x1b[48;2;{};{};{}m        \x1b[0m    \
								 {:>3} {:>3} {:>3}    #{}",
								 r, g, b, r, g, b, value);
	} else {
		exerr(&format!("{} Wrong input!", value));
	}
}

fn main() {
	let argv: Vec<String> = env::args().collect();
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
				print_help();
			}
		} else {
			print_help();
		}
	} else {
		print_help();
	}
}
// vim:ts=2:sw=2:noexpandtab:cindent
