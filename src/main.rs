use rand;
use std::env;
use std::process;

fn merge(left: &[i32], right: &[i32], v: &mut [i32]) {
    let mut left_i = 0;
    let mut right_i = 0;
    let mut index = 0;

    while left_i < left.len() && right_i < right.len() {
        if left[left_i] <= right[right_i] {
            v[index] = left[left_i];
            left_i += 1;
        } else {
            v[index] = right[right_i];
            right_i += 1;
        }
        index += 1;
    }

    if left_i < left.len() {
        v[index..].copy_from_slice(&left[left_i..]);
    } else {
        v[index..].copy_from_slice(&right[right_i..]);
    }
}

fn mergesort(v: &mut [i32]) {
    let mid = v.len() / 2;
    if mid == 0 {
        return;
    }
    mergesort(&mut v[..mid]);
    mergesort(&mut v[mid..]);

    let mut ret = v.to_vec();
    merge(&v[..mid], &v[mid..], &mut ret[..]);
    v.copy_from_slice(&ret);
}

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
    mergesort(&mut input_vector[..]);

    // validate the answer
    assert_eq!(input_vector, valid_vector);
    println!("pass");
}
