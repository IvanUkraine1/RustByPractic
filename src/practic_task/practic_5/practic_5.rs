#[test]
fn main() {
    const W: u32 = 25;
    const H: u32 = 10;
    let k = W as f32 / H as f32; // 2.5

    fn mul(a: u32, b: f32) -> u32 {
        (a as f32 * b) as u32
    }

    for y in 1..=H {
        for x in 1..=W {
            let c: char = match (x, y) {
                (_, 1 | H) => '-',
                (1 | W, _) => '|',
                _ if x == mul(y, k) => '\\',
                _ if x == W - mul(y, k) => '/',
                _ => ' ',
            };
            print!("{}", c);
        }
        println!();
    }
}