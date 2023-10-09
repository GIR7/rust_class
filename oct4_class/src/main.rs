//strings and aggerate

struct Point{
    x: u64,
    y: u64,
}

enum Pointlike{
    Point_1D(u64),
    Point_2D(Point),
}

fn main() {
    let c = 'y';
    println!("char c is  {}", c);

    //uni code: 0x1f490.
    //from_u32: converts u32 to char, it returns Option<char>
    match char::from_u32(0x1f490){
        Some(v) => println!(" {}", v),
        None => println!(" Nah"),
    };
    println!(" {}", '💐');
    println!(" {:x}", '💐' as u32);

    // let mut v = Vec::new();
    // v.resize(10000,0);
    // v[5] = 125;
    // //how do be sure which type gets back? 
    // //- pattern match
    // let s = match sum(v){
    //     Pointlike::Point_1D(x) =>x,
    //     Pointlike::Point_2D(p) =>p.x+p.y,
    // };
    // println!("Sum is {}", s);

    // let mut v = Vec::new();
    // v.resize(10000,0);
    // v[5] = 125;
    // let s = sum(v);
    // println!("{}",s.x+s.y);

//    let mut v = Vec::new();// creates mem on the heap
//    // vec is resizeable 
//    //increase the size of the Vec to 10,000 elements, with the new elements set to 0.
//    v.resize(10000,0);//mem is on hte heap
//    v[5] = 125;
//    let res = sum(v);// mem are gone at the end of the sum() funciton, CAN'T (access the mem)do v[7] after tis line
//    println!("Sum is {}", res);



    //1000 elements value - 0
    // let mut arr = [0;1_000];//mem is living on the stack
    // arr[5] = 77; //crates a copy
    // let res = sum(arr);
    // println!("Sum is {}", res);
    // arr[7] = 55; 
    // let res = sum(arr);
    // println!("Sum is {}", res);
}

//enum
fn sum(v: Vec<u64>) -> Pointlike{
    let mut total = 0;
    for x in v{
        total += x;
    }
    Pointlike::Point_1D(total)
}


//struct
// fn sum(v: Vec<u64>) -> Point{
//     let mut total = 0;
//     for x in v{
//         total += x;
//     }
//     Point{x:total, y:total}
// }

//Vec - <T>
// fn sum(v: Vec<u64>) -> u64{
//     let mut t = 0;
//     for x in v{
//         t += x;
//     }
//     t
// }

//array
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





