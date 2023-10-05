fn main() {
   let mut v = Vec::new();
   v.resize(10000,0);
   v[5] = 125;
   let s = sum(v);
   println!("{s}");
}

fn sum(v: Vec<u64>) -> u64{
    let mut total = 0;
    for x in v{
        total += x;
    }
    total
}

struct Point{
    x: u64,
    y:u64,
}