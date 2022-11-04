/* This project is released under
 *
 * "THE BEER-WARE LICENSE" (Revision 42):
 *
 * As long as you retain this notice you can do whatever you want
 * with this stuff. If we meet some day, and you think this stuff
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
    $ hex2rgb \"#ffffff\" abcabc"
	);
	eprint!("    "); print_result(&255u32,&255u32,&255u32);
	eprint!("    "); print_result(&171u32,&202u32,&188u32);

	eprintln!("
    /* convert rgb to hex color. */
    $ rgb2hex 255 255 255"
	);
	eprint!("    "); print_result(&255u32,&255u32,&255u32);
}
fn exerr(msg: &str) {
	eprintln!("error: {}", msg);
	process::exit(CANCEL_OPERATION);
}

fn print_result(r: &u32, g: &u32, b: &u32) {
	println!("\x1B[48;2;{};{};{}m        \x1B[0m   \
					 {:>3} {:>3} {:>3}   #{:0>2x}{:0>2x}{:0>2x}",
					 r, g, b, r, g, b, r, g, b);
}
fn rgb2hex(r: String, g: String, b: String) {
	/* convert string digit into actual digit */
	let str2digit = | st: &String | {
		match st.parse::<u32>() {
			Ok(good) => return good,
			Err(bad) => {
				exerr(&format!("{}", bad));
				return 0;
			},
		}
	};

	let red   = str2digit(&r);
	let green = str2digit(&g);
	let blue  = str2digit(&b);

	if red <= 0xff && green <= 0xff && blue <= 0xff {
		print_result(&red, &green, &blue);
	} else {
		exerr("each value must be less than 255!");
	}
}
fn hex2rgb(hex_color: String) {
	/* convert hex character into hex digit */
	let char2digit = | ch: char | {
		match ch.to_digit(16) {
			Some(d) => return d,
			None    => {
				exerr(&format!("invalid hex value of {}", hex_color));
				return 0;
			},
		}
	};

	let mut r: u32 = 0x0;
	let mut g: u32 = 0x0;
	let mut b: u32 = 0x0;

	/* 16^1 && 16^0 */
	match hex_color.len() {
		6 => {
			let mut iter = hex_color.chars();
			r += char2digit(iter.next().unwrap()) << 0x4;
			r += char2digit(iter.next().unwrap());
			g += char2digit(iter.next().unwrap()) << 0x4;
			g += char2digit(iter.next().unwrap());
			b += char2digit(iter.next().unwrap()) << 0x4;
			b += char2digit(iter.next().unwrap());
		},
		7 => {
			let mut iter = hex_color.chars();
			if iter.next() != Some('#') {
				exerr(&format!("invalid hex value of {}", hex_color));
			}
			r += char2digit(iter.next().unwrap()) << 0x4;
			r += char2digit(iter.next().unwrap());
			g += char2digit(iter.next().unwrap()) << 0x4;
			g += char2digit(iter.next().unwrap());
			b += char2digit(iter.next().unwrap()) << 0x4;
			b += char2digit(iter.next().unwrap());
		},
		_ => {
			exerr(&format!("invalid hex value of {}", hex_color));
		},
	}
	print_result(&r, &g, &b);
}

fn main() {
	let argv: Vec<String> = env::args().collect();
	let argc: usize = argv.len();

	if argc != 1 {
		if argv[0].contains("hex2rgb") {
			for arg in 1..argc {
				hex2rgb(argv[arg].clone());
			}
		} else if argv[0].contains("rgb2hex") {
			if argc == 4 {
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
