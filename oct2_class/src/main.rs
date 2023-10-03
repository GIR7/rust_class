
//function type - result type and return type

//unit type: return type
fn return_unit_type(){
    // can't have semicolon here, semicolon discard the unit return type
}

fn main() ->() {
    // let var_unit =  return_unit_type();
    let mut res = double(2);
    res = double(res);
    let res = res+1;
    assert_eq!(9,res);
    
    adds();
    println!("{res}");
    func_for();
}

fn match_pattern(x:u16){

    //matches: pattern match
    matches!(x, 2|4|8); // returns true if the x matches 2 or 4 or 8

    //ex on match keyword
    // let x = 7;
    // match x {
    //     2=>x,
        

    // }
}

fn double (x:u16) -> u16{
    x+x // no semicolon here - implicitly returns '()' as its body
    // semicolon is statement operator
}

//{block is the value} block's value is the last expression inside the {}
//returns a u16
fn adds()->u16{
    let res = double(2);
    let res = double(res);
    // bool type:
    let _var_bool = true;    
    res+1 //return value, no semicolon
}

#[test]
fn test_double(){
    assert_eq!(4,double(2));
}
#[test]
fn test_add(){
    assert_eq!(9,adds());
}

fn func_for(){
    let mut x = 7u64;
    println!("Starts loop: 0~5");
    for i in 0..5{
        x = x+i;
        println!("i is :{i} , x is {x}");
    }

    const N: usize = 3;// created at complier time
    //let create var at the run time
}