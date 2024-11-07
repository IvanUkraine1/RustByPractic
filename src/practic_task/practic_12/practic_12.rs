use rand::Rng;

fn generate_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> Option<(i32, usize)> {
    data.windows(2)
        .enumerate()
        .map(|(i, pair)| (pair[0] + pair[1], i))
        .min_by_key(|&(sum, _)| sum)
}

fn print_result(data: &[i32], min_sum: i32, index: usize) {
    println!(
        "indexes: {}",
        (0..data.len())
            .map(|i| format!("{:>2}.", i))
            .collect::<Vec<_>>()
            .join(" ")
    );
    println!(
        "data:    [{}]",
        data.iter()
            .map(|&x| format!("{:>2}", x))
            .collect::<Vec<_>>()
            .join(", ")
    );

    let pointer_line: String = (0..data.len())
        .map(|i| match i {
            _ if i == index => " \\__",
            _ if i == index + 1 => " __/",
            _ => "    ",
        })
        .collect();

    println!("indexes: {}", pointer_line);
    println!(
        "min adjacent sum = {} + {} = {} at indexes: {}, {}",
        data[index], data[index + 1], min_sum, index, index + 1
    );
}

#[test]
fn main() {
    let vec = generate_random_vector(20);
    match min_adjacent_sum(&vec) {
        Some((min_sum, index)) => print_result(&vec, min_sum, index),
        None => println!("Невистачає елементів щоб знайти мінімальну пару."),
    }
}