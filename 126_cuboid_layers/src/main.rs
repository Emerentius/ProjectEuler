#![feature(nll)]
#[macro_use]
extern crate itertools;
#[derive(Copy, Clone)]
struct Cell {
    last_visit: u8,
    active: bool,
}

const SIZE: usize = 100;
fn idx(x: usize, y: usize, z: usize) -> usize {
    x * SIZE * SIZE + y * SIZE + z
}

fn pos(p: usize) -> isize {
    p as isize - SIZE as isize / 2
}

fn extend_cuboid([x, y, z]: [usize; 3], grid: &mut Vec<Cell>, round: u8) -> usize {
    //let mid = grid.len()/2;
    let mut n_new_cubes = 0;
    let mid = grid.len() /2;
    let cell = &mut grid[idx(x, y, z)];

    //println!("{}, {}, {}", pos(x), pos(y) ,pos(z));
    if cell.active && cell.last_visit < round {
        //println!("INSIDE {}, {}, {}", pos(x), pos(y) ,pos(z));
        cell.last_visit = round;
        for &(_x, _y, _z) in [
            (x-1, y, z),
            (x+1, y, z),
            (x, y-1, z),
            (x, y+1, z),
            (x, y, z-1),
            (x, y, z+1),
        ].iter()
        {
            n_new_cubes += extend_cuboid([_x, _y, _z], grid, round);

            let neighb_cell = &mut grid[idx(_x, _y, _z)];
            if !neighb_cell.active {
                //println!("NEW {}, {}, {}", pos(_x), pos(_y) ,pos(_z));
                n_new_cubes += 1;
                neighb_cell.active = true;
                neighb_cell.last_visit = round;
            }
        }
    }
    //println!("{}", n_new_cubes);
    n_new_cubes
}

fn build_cuboid(dimensions: [usize; 3]) -> Vec<Cell> {
    let mid = SIZE / 2;
    let [x, y, z] = dimensions;
    let rg = |dim: usize| {
        let dim = dim as isize;
        (-dim /2 .. (dim + 1)/2)
            .map(|offset| (mid as isize + offset) as usize)
    };
    //println!("{}, {}, {}", x, y, z);
    let mut grid = vec![Cell { last_visit: 0, active: false }; SIZE.pow(3)];
    for _x in rg(x) {
        for _y in rg(y) {
            for _z in rg(z) {
                //println!("build {}, {}, {}", pos(_x), pos(_y) ,pos(_z));
                let cell = &mut grid[idx(_x, _y, _z)];
                cell.active = true;

            }
        }
    }
    grid
}

fn build_cuboids(dimensions: [usize; 3], cuboid_occurences: &mut Vec<u16>) {
    let mut grid = build_cuboid(dimensions);
    let mid = SIZE / 2;
    let [x, y, z] = dimensions;

    let mut n_cuboids = x * y * z;
    let mut round = 1;
    while n_cuboids <= 1000 {
        n_cuboids = extend_cuboid([mid; 3], &mut grid, round);
        println!("{}", n_cuboids);
        cuboid_occurences[n_cuboids] += 1;
        let cell = &mut grid[idx(mid, mid, mid)];
        round += 1;
    }

}

fn main() {
    let mut cuboid_occurences = vec![0; 1_000_000];
    if true {
        build_cuboids([1, 3, 5], &mut cuboid_occurences);
    } else {
        for x in 1..1000 {
            println!("{}", x);
            for y in x..1000 {
                for z in y..1000 {
                    build_cuboids([x, y, z], &mut cuboid_occurences);
                }
            }
        }
        let solution = cuboid_occurences.iter().position(|&count| count == 1000).unwrap();
        println!("{}", solution);

        let print = |n| println!("C({}) = {}", n, cuboid_occurences[n]);
        print(22);
        print(46);
        print(78);
        print(118);
        print(154);

    }

}
