extern crate test;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::cmp::{Ord, Ordering};
use std::cmp::max;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Suit {
	Spade,
	Club,
	Diamond,
	Heart,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
enum Face {
	Pip(u8),
	Jack,
	Queen,
	King,
	Ace,
}

impl Face {
	fn val(&self) -> u8 {
		match *self {
		Face::Pip(n) => n,
		Face::Jack => 11,
		Face::Queen => 12,
		Face::King => 13,
		Face::Ace => 14,
		}
	}
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct Card {
	face : Face,
	suit : Suit,
}

impl Card {
	fn val(&self) -> u8 { self.face.val() }
}

impl Ord for Card {
	fn cmp(&self, other: &Self) -> Ordering { (self.face).cmp(&other.face) }
}
impl PartialOrd for Card {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Hand {
	HighCard(u8),
	Pair(u8),
	TwoPairs(u8),
	ThreeOfAKind(u8),
	Straight(u8),
	Flush(u8),
	FullHouse(u8,u8), // threes, twos
	FourOfAKind(u8),
	StraightFlush(u8),
	RoyalFlush, //just StraightFlush(max)
}

fn parse_single_card ( card_str : &str ) -> Card {
	let face = match card_str.char_at(0) {
		ch @ '2'...'9' => Face::Pip(ch.to_digit(10).unwrap() as u8),
		'T' => Face::Pip(10),
		'A' => Face::Ace,
		'Q' => Face::Queen,
		'K' => Face::King,
		'J' => Face::Jack,
		_   => unreachable!(),
	};
	let suit = match card_str.char_at(1) {
		'S' => Suit::Spade,
		'H' => Suit::Heart,
		'D' => Suit::Diamond,
		'C' => Suit::Club,
		_ => unreachable!(),
	};
	Card{ suit : suit, face : face }
}

fn parse_cards ( cards_str : &str ) -> Vec<Card> {
	let mut cards = vec![];
	for card in cards_str.words() { cards.push( parse_single_card(card) ) }
	cards.sort();
	cards
}

fn find_high_card ( cards: &Vec<Card>, hands : &Vec<Hand> ) -> Option<Hand> { 
	// find highest card that isn't part of some pattern
	// sorted cards, unsorted hands
	let mut taken_values = vec![];
	for hand in hands {
		match *hand {
			Hand::HighCard(n) | Hand::Pair(n) | Hand::TwoPairs(n) | 
			Hand::ThreeOfAKind(n) | Hand::Straight(n) | Hand::Flush(n) | 
			Hand::FourOfAKind(n) | Hand::StraightFlush(n) => {
				taken_values.push(n);
			},
			Hand::FullHouse(n,m) => {
				taken_values.push(n);
				taken_values.push(m)
			},
			Hand::RoyalFlush => {},
		}
	}
	for card in cards.iter().rev() {
		if !taken_values.contains(&card.val()) {
			return Some(Hand::HighCard( card.val() ));
		}
	}
	None
}

fn find_same_kinds_and_fullhouse ( hand : &Vec<Card> ) -> Vec<Hand> {
	// pairs are also a 'same kind'
	let mut pairs = vec![];
	let mut four_kind = vec![];
	let mut three_kind = vec![];
	
	let vals_equal = |cards : &[Card], val : u8| {
		for card in cards.iter() {
			if card.val() != val { return false }
		}
		true
	};
	
	'cards4: for cards in hand.windows(4) {
		let value = cards[0].val();
		if !vals_equal(cards, value) { continue 'cards4 }
		four_kind.push( value )
	}
	'cards3: for cards in hand.windows(3) {
		let value = cards[0].val();
		if four_kind.contains( &value ) 
		|| !vals_equal(cards, value) {
			continue 'cards3
		}
		three_kind.push( value )
	}
	'cards2: for cards in hand.windows(2) {
		let value = cards[0].val();
		if four_kind.contains( &value )
		|| three_kind.contains( &value )
		|| !vals_equal(cards, value) {
			continue 'cards2
		}
		pairs.push( value )
	}
	
	let mut hands = vec![];
	for &hand in &four_kind { hands.push( Hand::FourOfAKind(hand) ) }
	for &hand in &three_kind { hands.push( Hand::ThreeOfAKind(hand) ) }
	
	if pairs.len() == 2 {
		hands.push(Hand::TwoPairs( max(pairs[0], pairs[1]) ) );
		pairs.drain(); // no two Pair entries in addition to one TwoPair entry
	};
	if pairs.len() == 1 && three_kind.len() == 1 {
		hands.push( Hand::FullHouse(three_kind[0], pairs[0]) );
		pairs.drain(); // no extra single pair entry
	}
	for &hand in &pairs { hands.push( Hand::Pair(hand) ) }
	
	hands
}

fn find_straight_and_flush ( hand : &Vec<Card> ) -> Vec<Hand> {
	let mut hands = vec![];
	let mut is_straight = true;
	let mut is_flush = true;
	
	// straight
	for cards in hand.windows(2) {
	if let [card, following_card] = cards {
		if card.val() + 1 != following_card.val() { is_straight = false; }
	}}
	
	// flush
	let suit = hand[0].suit;
	for card in hand.iter() {
		if card.suit != suit { is_flush = false; }
	}
	
	// push hands
	let val = hand[4].val();
	if is_flush && is_straight {
		if val == Face::Ace.val() { hands.push( Hand::RoyalFlush ) }
		else { hands.push( Hand::StraightFlush(val) ) }
	} else if is_straight {
		hands.push( Hand::Straight(val) ) 
	} else if is_flush {
		hands.push( Hand::Flush(val) )
	}
	hands	
}

fn find_hands ( cards_str : &str ) -> Vec<Hand> {
	let cards = parse_cards(cards_str);
	let mut hands = vec![];
	
	hands.push_all( &find_same_kinds_and_fullhouse(&cards)[..] );
	hands.push_all( &find_straight_and_flush( &cards ) );
	
	if let Some(hand) = find_high_card(&cards, &hands) {
		hands.push( hand );
	}
	// reverse sort, high to low
	hands.sort_by(|a,b| b.cmp(a));	
	hands
}

fn main() {
	let path = Path::new(r"D:\Code\Project Euler\054_Poker_hands\target\p054_poker.txt");
	let mut file_handle = match File::open(&path) {
                Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
		Ok(file) => file,
	};
	let mut file = io::BufReader::new(file_handle);
	
	let mut player1_wins = 0;
	for line in file.lines() {
		let line = line.unwrap();
		let hand1 = find_hands(&line[..15]);
		let hand2 = find_hands(&line[15..]);
		// hands are sorted and ordered
		if hand1 > hand2 { player1_wins += 1 };
	}
	println!("{}", player1_wins);	
}

#[bench]
fn entire_program( b: &mut test::Bencher) {
	b.iter(|| main() );
}

#[bench]
fn without_file_io( b: &mut test::Bencher) {
	let path = Path::new(r"D:\Code\Project Euler\054_Poker_hands\target\p054_poker.txt");
	let mut file_handle = match File::open(&path) {
                Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
		Ok(file) => file,
	};
	let mut string = String::new();
	file_handle.read_to_string(&mut string);
	b.iter(|| {
		let mut player1_wins = 0;
		for line in string.lines() {
			let hand1 = find_hands(&line[..15]);
			let hand2 = find_hands(&line[15..]);
			// hands are sorted
			if hand1 > hand2 { player1_wins += 1 };
		}
	})
}