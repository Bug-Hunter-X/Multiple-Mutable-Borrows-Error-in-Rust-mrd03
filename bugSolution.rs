fn main() {
    let mut x = 5;
    { //Scope for mutable reference y
        let y = &mut x; 
        *y = 10; 
    }

    let z = &x; 
    println!("{}", *z); //This will print 10
    
    //Multiple mutable reference in different scope works
    { 
        let a = &mut x; 
        *a = 12; 
    }

    println!("{}", x); //This will print 12
} 