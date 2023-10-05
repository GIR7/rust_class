fn main() {
   let mut v = Vec::new();
   v.resize(10000,0);
   v[5] = 125;
   let s = sum(v);
   println!("{}",s.x+s.y);
}

fn sum(v: Vec<u64>) -> Point{
    let mut total = 0;
    for x in v{
        total += x;
    }
    Point{x:total, y:total}
}

struct Point{
    x: u64,
    y: u64,
}
