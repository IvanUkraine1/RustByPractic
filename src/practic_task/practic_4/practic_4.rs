#[test]
fn main() {
    let size = 11;
    let seredina = size / 2;

    for i in 0..size {
        let current = if i <= seredina { i } else { size - i - 1 };

        let spaces = " ".repeat(seredina - current);
        let stars = "*".repeat(2 * current + 1);

        println!("{}{}", spaces, stars);
    }
}