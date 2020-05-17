use rand;
use std::env;
use std::process;

mod mergesort;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: {:?} <input_size>", args[0]);
        process::exit(-1);
    }

    let input_size: usize = args[1].trim().parse().expect("please type a number");

    // create a random input integer array (could be generic in the end)
    let mut input_vector: Vec<i32> = Vec::with_capacity(input_size);

    for _ in 0..input_vector.capacity() {
        input_vector.push(rand::random());
    }

    let mut valid_vector = input_vector.clone();
    valid_vector.sort();

    // recursive sorting
    mergesort::mergesort(&mut input_vector[..]);

    // validate the answer
    assert_eq!(input_vector, valid_vector);
    println!("pass");
}
