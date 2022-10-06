#![feature(test)]
use std::collections::BTreeSet;

type Vertex = usize;

fn main() {
    let adjacency_matrix_string = include_str!("p107_network.txt");

    let mut edges = adjacency_matrix_string.lines().enumerate().flat_map(|(i, line)|
            line.split(',').enumerate().skip(i).flat_map(move |(j, weight)|
                weight.parse::<u32>().map(|w| (w, i, j))
            )
        ).collect::<Vec<_>>();

    /*
    let mut edges = vec![];
    for (i, line) in adjacency_matrix_string.lines().enumerate() {
        for (j, weight) in line.split(',').enumerate()
            .skip(i)
            .flat_map(|(j, weight)| weight.parse::<u32>().map(|w| (j, w)) )
        {
            edges.push((weight, i, j));
        }
    }
    */
    edges.sort();
    let total_unoptimised_weight: u32 = edges.iter().map(|&(weight, _, _)| weight).sum();

    let mut networks: Vec<BTreeSet<Vertex>> = vec![];
    let mut total_weight = 0;
    for (weight, vert1, vert2) in edges {
        let tree1_pos;
        let tree2_pos;
        {
            // find networks that contain at least one of the vertices of the edge
            // 2 at most
            let mut iter = networks.iter()
                .enumerate()
                .filter(|&(_, ref net)| net.contains(&vert1) || net.contains(&vert2))
                .map(|(i, _)| i);
            tree1_pos = iter.next();
            tree2_pos = iter.next();
        }

        match (tree1_pos, tree2_pos) {
            // edge connects two previously disjoint networks
            // combine networks into one
            (Some(tree1_idx), Some(tree2_idx)) => {
                let mut tree2 = networks.swap_remove(tree2_idx);
                networks[tree1_idx].append(&mut tree2);
            },
            // edge connects new vertex to known network
            // extend network
            (Some(tree1_idx), None) => {
                let network = &mut networks[tree1_idx];
                if network.contains(&vert1) && network.contains(&vert2) {
                    continue
                }
                network.insert(vert1);
                network.insert(vert2);
            }
            // edge connects two vertices that belong to no known network
            // create new network
            (None, None) => {
                let new_network = [vert1, vert2].iter().cloned().collect();
                networks.push(new_network);

            },
            _ => unreachable!(),
        }
        total_weight += weight // re-invert to get correct value
    }
    println!("{}", total_unoptimised_weight - total_weight);
}

mod bench {
    extern crate test;
    #[bench]
    fn bench(b: &mut test::Bencher) {
        b.iter(|| ::main())
    }
}
