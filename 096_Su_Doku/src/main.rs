#![feature(test)]
extern crate test;

use std::fmt::Display;
use std::ops::{Index, IndexMut};

const N: usize = 9;

impl Display for Grid {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		for row in 0..9 {
			for col_off in (0..3).map(|z| 3*z) {
				try!(
					write!(f, "{}{}{} ",
						self[(row, 0+col_off)],
						self[(row, 1+col_off)],
						self[(row, 2+col_off)]
					)
				)
			}
			try!( write!(f, "\n") );
			if (row + 1) % 3 == 0 { try!( write!(f, "\n") ) };
		}
		Ok(())
	}
}

fn is_dup_free<T>(iter: T) -> bool
	where T: Iterator<Item=u32>
{
	let mut num_met = [false;9];
	for num in iter.filter(|&num| num != 0) {
		if num_met[num as usize -1] { return false }
		num_met[num as usize -1] = true;
	}
	true
}

struct Grid {
	grid: [u32; 81]
}

impl Index<usize> for Grid {
	type Output = u32;

	fn index(&self, idx: usize) -> &Self::Output {
		&self.grid[idx]
	}
}

impl IndexMut<usize> for Grid {
	fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
		&mut self.grid[idx]
	}
}

impl Index<(usize, usize)> for Grid {
	type Output = u32;

	fn index(&self, idx: (usize, usize)) -> &Self::Output {
		&self.grid[idx.0 * N + idx.1] // row * 9 + column
	}
}

impl IndexMut<(usize,usize)> for Grid {
	fn index_mut(&mut self, idx: (usize, usize)) -> &mut Self::Output {
		&mut self.grid[idx.0 * N + idx.1] // row * 9 + column
	}
}

impl Grid {
	fn new(nums: &[u32]) -> Option<Grid> {
		if nums.len() != 81 { return None }

		let mut grid = Grid { grid: [0; 81] };
		for (cell, &num) in grid.grid.iter_mut().zip(nums.iter()) {
			*cell = num;
		}
		Some(grid)
	}

	fn row_valid(&self, n: usize) -> bool {
		is_dup_free(
			(0..9).map(move |col| self[(n, col)])
		)
	}

	fn col_valid(&self, n: usize) -> bool {
		is_dup_free(
			(0..9).map(move |row| self[(row, n)])
		)
	}

	fn zone_valid(&self, n: usize) -> bool {
		is_dup_free(
			[(0,0), (0,1), (0,2),
		 	 (1,0), (1,1), (1,2),
		 	 (2,0), (2,1), (2,2)]
			 	.into_iter()
			 	.map(move |&(row, col)| self[(row + n/3*3, col + (n % 3)*3)])
		)
	}

	fn change_valid(&self, cell_nr: usize) -> bool {
		// cell_nr = 0..81
		let row = cell_nr / 9;
		let col = cell_nr - row*9;
		let zone = row / 3 * 3 + col / 3;
		self.row_valid(row) && self.col_valid(col) && self.zone_valid(zone)
	}
}

fn sudoku_solver(grid: &mut Grid, cell_nr: usize) -> bool {
	if cell_nr == 81 { return true } // base case
	if grid[cell_nr] != 0 { return sudoku_solver(grid, cell_nr+1) }

	for i in 1..9+1 {
		grid[cell_nr] = i;
		if grid.change_valid(cell_nr) && sudoku_solver(grid, cell_nr+1) {
			return true // base case hand through
		}
	}
	grid[cell_nr] = 0; // return to previous state
	false
}

fn sudoku_solver2(mut grid: Grid, cell_nr: usize) -> Option<Grid> {
	if cell_nr == 81 { return Some(grid) } // base case
	if grid[cell_nr] != 0 { return sudoku_solver2(grid, cell_nr+1) }

	for i in 1..9+1 {
		grid[cell_nr] = i;
		if grid.change_valid(cell_nr) {
			let gr2 = sudoku_solver2( Grid::new(&grid.grid).unwrap(), cell_nr+1);
			if gr2.is_some() { return gr2 } // base case hand through
		}
	}
	//grid[cell_nr] = 0; // return to previous state
	None
}

fn main() {
	let mut cache = vec![];
	let sudokus_string = include_str!("p096_sudoku.txt");
	let mut lines_iter = sudokus_string.lines()
		.filter(|line| !line.starts_with("Grid"));


	let mut sudokus = vec![];
	for _ in 1..50+1 {
		for num in lines_iter.by_ref()
			.take(9)
			.flat_map(|line| line.chars())
			.map(|ch| ch.to_digit(10).unwrap()  )
		{
			cache.push(num);
		}
		sudokus.push(
			sudoku_solver2(
				Grid::new(&cache).unwrap(),
				0
			).unwrap()
		);
		cache.clear();
	}

	let mut sum = 0;
	for sudoku in &sudokus {
		//sudoku_solver(sudoku, 0);
		sum += sudoku[0] * 100 + sudoku[1] * 10 + sudoku[2] ;
	}

	println!("{}", sum);
}

#[bench]
fn bench(b: &mut test::Bencher) {
	b.iter(|| main())
}
