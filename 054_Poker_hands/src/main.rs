#![feature(test)]
extern crate test;

use std::cmp::{Ord, Ordering, max, min};
use Suit::*;
use Face::*;
use Hand::*;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Suit { Spade, Club, Diamond, Heart, }

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
enum FaceF {
	Two     = 2,
	Three   = 3,
	Four    = 4,
	Five    = 5,
	Six     = 6,
	Seven   = 7,
	Eight   = 8,
	Nine    = 9,
	Ten     = 10,
	Jack    = 11,
	Queen   = 12,
	King    = 13,
	Ace     = 14,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
enum Face { Pip(u8), Jack, Queen, King, Ace, }

impl Face {
	fn val(&self) -> u8 {
		match *self {
			Pip(n) => n,
			Jack => 11,
			Queen => 12,
			King => 13,
			Ace => 14,
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
enum Hand { // Hand(highest_value_in_hand)
	HighCard(u8),
	Pair(u8),
	TwoPairs(u8,u8), // higher, lower
	ThreeOfAKind(u8),
	Straight(u8),
	Flush(u8),
	FullHouse(u8,u8), // threes, twos
	FourOfAKind(u8),
	StraightFlush(u8),
	RoyalFlush, //just StraightFlush(max)
}

fn parse_single_card ( card_str : &str ) -> Card {
	let mut chars = card_str.chars();
	let face = match chars.next().unwrap() {
		ch @ '2'...'9' => Pip(ch.to_digit(10).unwrap() as u8),
		'T' => Pip(10),
		'A' => Ace,
		'Q' => Queen,
		'K' => King,
		'J' => Jack,
		_   => unreachable!(),
	};
	let suit = match chars.next().unwrap() {
		'S' => Spade,
		'H' => Heart,
		'D' => Diamond,
		'C' => Club,
		_ => unreachable!(),
	};
	Card{ suit : suit, face : face }
}

fn parse_cards ( cards_str : &str ) -> Vec<Card> {
	let mut cards = vec![];
	for card in cards_str.trim().split(' ') {
		cards.push( parse_single_card(card) );
	}
	cards.sort();
	cards
}

fn find_high_card ( cards: &Vec<Card>, hands : &Vec<Hand> ) -> Option<Hand> {
	// find highest card that isn't part of some pattern
	// sorted cards, unsorted hands
	let mut taken = vec![]; // values that are already part of a hand
	for hand in hands {
		match *hand {
			Pair(n) | ThreeOfAKind(n) | FourOfAKind(n) | Flush(n)  => {
				taken.push(n);
			}
			TwoPairs(n,m) => {
				taken.extend(&[n,m])
			}
			Straight(_) | FullHouse(_,_) |
			StraightFlush(_) | RoyalFlush => {
				// All cards part of the hand already
				return None;
			},
			HighCard(_) => unreachable!(),
		};
	}

	let card = cards.iter().rev().find(|&c| !taken.contains( &(c.val() ) ) );
	let val = card.unwrap().val();
	Some( HighCard(val) )
}

fn find_same_kinds_and_fullhouse ( hand : &Vec<Card> ) -> Vec<Hand> {
	// pairs are also a 'same kind'
	let mut pairs = vec![];
	let mut four_kind = vec![];
	let mut three_kind = vec![];

	let vals_equal = |cards : &[Card], val : u8| {
		cards.iter().all(|&card| card.val() == val)
	};
	let not_taken = |val:u8, a:&Vec<_>, b: &Vec<_>| {
		!a.contains(&val) && !b.contains(&val)
	};

	for pairing_size in (2..4+1).rev() {
		for cards in hand.windows(pairing_size) {
			let value = cards[0].val();
			if not_taken(value, &pairs, &three_kind)
			&& vals_equal(cards, value) {
				match pairing_size {
					4 => four_kind.push(value),
					3 => three_kind.push(value),
					2 => pairs.push(value),
					_ => unreachable!(),
				}
			}
		}
	}

	let mut hands = vec![];

	// draining to prevent double dipping
	if pairs.len() == 1 && three_kind.len() == 1 {
		hands.push( FullHouse(three_kind[0], pairs[0]) );
		pairs.clear();
		three_kind.clear();
	} else if pairs.len() == 2 {
		let high_val = max(pairs[0], pairs[1]);
		let low_val = min(pairs[0], pairs[1]);
		hands.push( TwoPairs( high_val, low_val ) );
		pairs.clear();
	};

	for &val in &four_kind  { hands.push( FourOfAKind(val) ) }
	for &val in &three_kind { hands.push( ThreeOfAKind(val) ) }
	for &val in &pairs      { hands.push( Pair(val) ) }

	hands
}

fn find_straight_and_flush ( hand : &Vec<Card> ) -> Vec<Hand> {
	let mut hands = vec![];

	let is_flush = hand.iter().all(|&card| card.suit != hand[0].suit);
	let is_straight = hand.iter().enumerate().all(|(i, &card)| {
		card.val() == hand[0].val() + i as u8
	});

	// push hands
	let val = hand[4].val();
	match (is_flush, is_straight) {
		(true, true) if val == Ace.val() => hands.push( RoyalFlush ),
		(true, true) => hands.push( StraightFlush(val) ),
		(false, true) => hands.push( Straight(val) ),
		(true, false) => hands.push( Flush(val) ),
		(false, false) => (),
	};
	hands
}

fn find_hands ( cards_str : &str ) -> Vec<Hand> {
	let cards = parse_cards(cards_str);
	let mut hands = vec![];
	hands.extend( &find_same_kinds_and_fullhouse(&cards)[..] );
	hands.extend( &find_straight_and_flush( &cards ) );

	if let Some(hand) = find_high_card(&cards, &hands) {
		hands.push( hand );
	}
	// reverse sort, high to low
	hands.sort_by(|a,b| b.cmp(a));
	hands
}

fn main() {
	let games = include_str!("p054_poker.txt");
	let mut player1_wins = 0;
	for line in games.lines() {
		let hand1 = find_hands(&line[..15]);
		let hand2 = find_hands(&line[15..]);
		// hands are sorted and ordered
		if hand1 > hand2 { player1_wins += 1 };
	}
	println!("{}", player1_wins);
	//let a: i32 = FaceF::Three as i32;
	//println!("{}", a as FaceF);
}

#[bench]
fn entire_program( b: &mut test::Bencher) {
	b.iter(|| main() );
}
