fn main() {
    let mut x = 5;
    let y = &mut x; 
    *y = 10; 
    let z = &x; 
    println!("{}", *z); //This will print 10

    // Following code will result in an error
    //let a = &mut x; 
    //*a = 10;
}