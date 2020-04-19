use std::env;
use std::fs;

struct Vertex {
    x: u32,
    y: u32,
    weight: u32,
}

enum Direction {
    North,
    South,
    West,
    East,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
	print_usage();
	std::process::exit(1);
    }
    let filename = &args[1];

    let contents = fs::read_to_string(filename)
	.expect("Something went wrong reading the file");

    let matrix: Vec<Vec<u32>> = contents.lines()
	.map(|x: &str| x.split(',').map(|x: &str| x.parse().unwrap()).collect())
	.collect();
    
    // println!("{:?}", matrix);

    let mut parent: Vec<u32> = Vec::new();
    parent.resize(matrix.len() * matrix.len(), 2000000000);

    let mut estimate: Vec<u32> = Vec::new();
    estimate.resize(matrix.len() * matrix.len(), 2000000000); // Inf
    estimate[0] = 0;

    let mut adjacency_list: Vec<Vec<(u32, u32)>> = Vec::new();

    // println!("{:?}", parent);

    // create adjacency-list
    for y in 0..matrix.len() {
	for x in 0..matrix.len() {
	    let idx = y * matrix.len() + x;

	    let mut neighbors: Vec<(u32, u32)> = Vec::new();
	    neighbors.resize(4,(2000000000,2000000000));

	    // north
	    if y != 0 {
		let idx = (y as u32 - 1) * matrix.len() as u32 + (x as u32);
		let weight: u32 = matrix[y-1][x];
		neighbors[Direction::North as usize] = (idx, weight);
	    }

	    // south
	    if y != matrix.len()-1 {
		let idx = (y as u32 + 1) * matrix.len() as u32 + (x as u32);
		let weight: u32 = matrix[y+1][x];
		neighbors[Direction::South as usize] = (idx, weight);
	    }

	    // west
	    if x != 0 {
		let idx = y as u32 * matrix.len() as u32 + (x as u32-1);
		let weight: u32 = matrix[y][x-1];
		neighbors[Direction::West as usize] = (idx, weight);
	    }

	    // east
	    if x != matrix.len()-1 {
		let idx: u32 = y as u32 * matrix.len() as u32 + (x as u32+1);
		let weight: u32 = matrix[y][x+1];
		neighbors[Direction::East as usize] = (idx, weight);	
	    }

	    adjacency_list.push(neighbors);
	}
    }

    // // println!("{:?}", adjacency_list);

    // dijkstra
    let mut S: Vec<u32> = Vec::new();
    let mut Q: Vec<u32> = (0u32..matrix.len() as u32 *matrix.len() as u32).collect::<Vec<u32>>();

    // // println!("{:?}", Q);

    while Q.len() > 0 {
	//// println!("Estimate: {:?}", estimate);  
	let u: u32 = extract_min(&mut Q, &estimate);
	//// println!("{:?}", u);    
	for v in adjacency_list[u as usize].iter() {
	    // println!("neighbor idx is {}, with weight {}", v.0, v.1);
	    if v.0 != 2000000000 {
		relax(u, v.1, v.0, &mut estimate, &mut parent);
	    }
	}
    }

    // // println!("{:?}", parent);
    // // println!("{:?}", estimate);  
    
    let flattened_matrix: Vec<u32> = matrix
	.iter()
	.flat_map(|array| array.iter())
	.cloned()
	.collect();
    let mut explorer: u32 = matrix.len() as u32 * matrix.len() as u32 - 1;

    let mut sum: u32 = 0;

    while parent[explorer as usize] != 2000000000 {
	// println!("Path: {}", flattened_matrix[explorer as usize]);
	sum = sum + flattened_matrix[explorer as usize];
	explorer = parent[explorer as usize];
    }

    sum = sum + flattened_matrix[0];

    println!("Minimal path sum is: {}", sum);

    
    
    
}

fn extract_min(q: &mut Vec<u32>, estimate: &Vec<u32>) -> u32 {
    let mut real_min_idx: u32 = 2000000000;
    let mut min_idx: u32 = 2000000000;
    let mut min_estimate: u32 = 2000000000;
    for i in 0..q.len() {
	let idx = q[i as usize];
	if estimate[idx as usize] < min_estimate {
	    real_min_idx = i as u32;
	    min_idx = idx as u32;
	    min_estimate = estimate[idx as usize];
	}
    }

    if min_idx == 2000000000 {
	eprintln!("Did not find any minima in extract min, exiting");
	std::process::exit(1);
    }
    
    q.remove(real_min_idx as usize);

    // println!("returning min idx {}", min_idx);
    return min_idx;
    
}

fn relax(u_idx: u32, v_w: u32, v_idx: u32, estimate: &mut Vec<u32>, parent: &mut Vec<u32>) {
    // println!("RELAXING!");
    // println!("v_idx={} v_idx_estimate={} u_idx={} u_idx_estimate={}, v_w={}", v_idx, estimate[v_idx as usize], u_idx, estimate[u_idx as usize], v_w);
    if estimate[v_idx as usize] > estimate[u_idx as usize] + v_w {
	// println!("Found better route!");
	estimate[v_idx as usize] = estimate[u_idx as usize] + v_w;
	parent[v_idx as usize] = u_idx;
    }
    // println!("DONE RELAXING!");
}

fn print_usage() {
    eprintln!("Usage: ./minimal-path-sum input.txt");
}
