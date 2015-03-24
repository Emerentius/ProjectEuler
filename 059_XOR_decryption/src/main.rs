#![feature(old_path, io)]
#![warn(deprecated)]
extern crate test;

use std::io::prelude::*;
use std::fs::File;
use std::string::String;

fn main() {
	// read file
	let path = Path::new(r".\p059_cipher.txt");
	let mut file = match File::open(&path) {
                Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
		Ok(file) => file,
	};
	let mut cipher_str = String::new();
	file.read_to_string(&mut cipher_str);

	// parse into byte vector
	let cipher : Vec<u8> = cipher_str.trim()
		.split(',')
		.map(|str| str.parse().unwrap() )
		.collect();

	// what to loop over, what string to check for
	let (a,z) = ('a' as u8, 'z' as u8 + 1);
	let necessary_words = [" the "];

	'k1: for k1 in a..z {
		'k2: for k2 in a..z {
			'k3: for k3 in a..z {

				let mut decipher_attempt = cipher.clone();
				let keys = [k1,k2,k3];
				let keys_it = keys.iter().cycle();
				for (byte, key) in decipher_attempt.iter_mut().zip(keys_it) {
					*byte = *byte ^ *key;
				}


				if let Ok(decipher_str) = String::from_utf8(decipher_attempt) {
					for &word in &necessary_words {
						if !decipher_str.contains(&word) { continue 'k3 }
					}

					// string passed checks here
					let result : u32 = decipher_str.as_bytes()
						.iter()
						.fold(0, |sum, &byte| sum + byte as u32);

					println!("{}", decipher_str);
					println!("password: {}{}{}, sum: {}", k1 as char,k2 as char,k3 as char, result);
					break 'k1;
				}

			}
		}
	}
}

#[bench]
fn bench (b: &mut test::Bencher) {
	b.iter(|| main())
}
