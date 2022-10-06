#![feature(test)]
extern crate test;

struct Numeral {
    symbol: &'static str,
    val: u32,
}

const NUMERALS: [Numeral; 13] = [
    Numeral {symbol: "M",  val: 1000},
    Numeral {symbol: "CM", val: 900},
    Numeral {symbol: "D",  val: 500},
    Numeral {symbol: "CD", val: 400},
    Numeral {symbol: "C",  val: 100},
    Numeral {symbol: "XC", val: 90},
    Numeral {symbol: "L",  val: 50},
    Numeral {symbol: "XL", val: 40},
    Numeral {symbol: "X",  val: 10},
    Numeral {symbol: "IX", val: 9},
    Numeral {symbol: "V",  val: 5},
    Numeral {symbol: "IV", val: 4},
    Numeral {symbol: "I",  val: 1}
];

fn parse_numeral(numeral: &str) -> u32 {
    match NUMERALS.iter().find(|sym| numeral.starts_with(sym.symbol)) {
        Some(sym) => sym.val + parse_numeral(&numeral[sym.symbol.len()..]),
        None => 0,
    }
}

fn minimal_numeral(mut number: u32) -> String {
    let mut numeral = String::new();
    for sym in NUMERALS.iter() {
        while sym.val <= number {
            numeral = numeral + sym.symbol;
            number -= sym.val;
        }
    }
    numeral
}

fn main() {
    let numerals = include_str!("p089_roman.txt");
    let minimise_numeral = |numeral| minimal_numeral( parse_numeral(numeral) );
    let saved_lines = numerals.lines()
        .fold(0, |sum, n| sum + n.len() - minimise_numeral(n).len() );
    println!("{}", saved_lines);
}

#[bench]
fn bench(b: &mut test::Bencher) {
    b.iter(|| main())
}
