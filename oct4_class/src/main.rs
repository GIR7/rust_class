//strings and aggerate



fn main() {
   let mut v = Vec::new();
   // vec is resizeable 
   //increase the size of the Vec to 10,000 elements, with the new elements set to 0.
   v.resize(10000,0);//mem is on hte heap
   v[5] = 125;
   let s = sum(v);
//    println!("{}",s.x+s.y);
    //1000 elements value - 0
    // let mut arr = [0;1_000];//mem is living on the stack
    // arr[5] = 77; //crates a copy
    // let res = sum(arr);
    // println!("Sum is {}", res);
    // arr[7] = 55; 
    // let res = sum(arr);
    // println!("Sum is {}", res);
}


fn sum(v: Vec<u64>) -> u64{
    
}
// fn sum(v: [u64;1_000]) -> u64{
//     let mut total = 0;
//   //  for i in 0..v.len() {//the loop variable `i` is only used to index `v`
//         //onsider using an iterator
//     //     for <item> in &v 
//     for x in v{
//         total += x;
//     }
//     total 
// }


// fn sum(v: Vec<u64>) -> Point{
//     let mut total = 0;
//     for x in v{
//         total += x;
//     }
//     Point{x:total, y:total}
// }

struct Point{
    x: u64,
    y: u64,
}
