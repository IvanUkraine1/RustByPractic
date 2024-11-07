#[test]
fn main() {
    let digits = [1, 2, 3, 4, 5, 6, 7, 8];

    for &m in &digits {
        for &u in &digits {
            if u == m { continue; }
            for &x in &digits {
                if x == m || x == u { continue; }
                for &a in &digits {
                    if a == m || a == u || a == x { continue; }
                    for &s in &digits {
                        if s == m || s == u || s == x || s == a { continue; }
                        for &l in &digits {
                            if l == m || l == u || l == x || l == a || l == s { continue; }
                            for &o in &digits {
                                if o == m || o == u || o == x || o == a || o == s || o == l { continue; }
                                for &n in &digits {
                                    if n == m || n == u || n == x || n == a || n == s || n == l || n == o { continue; }

                                    let muxa = m * 1000 + u * 100 + x * 10 + a;
                                    let slon = s * 1000 + l * 100 + o * 10 + n;

                                    if muxa * a == slon * x {
                                        println!("  {}", muxa);
                                        println!("{}        {}", x, a);
                                        println!("  ------");
                                        println!("   {}", slon);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}