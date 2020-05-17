use rand;

fn main() {
    println!("Hello, world!");
    let input_size = 16;
    // create a random input integer array (could be generic in the end)
    let mut input_vector: Vec<i32> = Vec::with_capacity(input_size);

    for _ in 0..input_vector.capacity() {
        input_vector.push(rand::random());
    };

    // recursive sorting

    // validate the answer
    for num in &input_vector {
        print!("{} ", num);
    }
    println!();
}
