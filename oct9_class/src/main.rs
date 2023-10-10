//standard memory management :
/*stack: consist of stack frames, function called on one stack frame, then keep stores local vars
all mem gets freed once func returned
*/
//heap: stors the mem that lives outside of the stack frame
//carbage collector / refrence counting system

//Rust memory management 
/* Complier 

 */
fn main() {
    //Vec allocate on the heap
   let mut v = Vec::new(); 
   v.push(1);

   sum(v);//sum has the ownership of the v, freed after this line
}

fn sum(vec:Vec<u8>){//vec of bytes
    let res = 0;
    for i in vec{//bytes in the vec are copy types, iterator makes copy of vec
        res+=i;
    }
    println!("{}" ,res);// res is on the stack

    //vec gets freed at the end of this func
}