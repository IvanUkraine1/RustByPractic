use itertools::Itertools;

fn find_variables() {
    let nums: Vec<i32> = (1..=8).collect();


    let valid_permutations = nums
        .into_iter()
        .permutations(8)
        .filter(|perm| is_valid_solution(&perm))
        .collect::<Vec<_>>();

    for perm in valid_permutations {
        let perm = perm.iter().copied().collect::<Vec<i32>>();
        print_solution(perm[0], perm[1], perm[2], perm[3], perm[4], perm[5], perm[6], perm[7]);
    }
}

fn is_valid_solution(perm: &[i32]) -> bool {
    let [m, u, x, a, s, l, o, n] = perm else { return false };

    let muxa = m * 1000 + u * 100 + x * 10 + a;
    let slon = s * 1000 + l * 100 + o * 10 + n;

    muxa == slon * x
}

fn print_solution(m: i32, u: i32, x: i32, a: i32, s: i32, l: i32, o: i32, n: i32) {
    println!("{:4}{:4}{:4}{:4}", m, u, x, a);
    println!("{}{:8}{:4}", x, " ", a);
    println!("{:-<6}", "-");
    println!("{:4}{:4}{:4}{:4}", s, l, o, n);
}

#[test]
fn main() {
    find_variables();
}
