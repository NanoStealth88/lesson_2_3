fn main() {
    
    //----------------------------------------
    //      Initializing Multiple Variables
    //----------------------------------------

    let (first_number, second_number) = (250, 380.93);

    println!("The first number is {} and the second number is {}", first_number, second_number);

    //-----------------------------------------
    //      Readability of large number
    //-----------------------------------------

    let large_number: i32 = 1_000_000;      //underscore helps in readability of the number and doesn't affect its value, never knew this before

    println!("The value of the variable \'large_number\' is {}", large_number);

    //-----------------------------------------
    //      Integer Overflow
    //-----------------------------------------

    //let overflow_number:u8 = 256;           //cannot use this because it is greater than the max of 255.

    //-----------------------------------------
    //      Decimal numbers in other formats
    //-----------------------------------------

    let x = 255; 
    println!("The value of the variable x in hexidecimal is {:o} and in octal is {:X} and in binary {:b}", x,x,x);

    //------------------------------------------
    //      Snake Case convention
    //-----------------------------------------

    let number = 45;        //throws a warning to use snake case for variable names...i.e.  snakeCase
    
    //-------------------------------------------------
    //      Operations on number of different formats
    //------------------------------------------------

    let n1 = 14;
    let n2 = 15.6;
    let n3 = n1 as f64 + n2;

    println!("{}", n3);

    //-------------------------------------------------
    //      Shadowing - using or updating a variable
    //          with the same name which has been 
    //          previously used or declared in the program
    //      Rust community and literature says the first 
    //          variable is being shadowed by the second
    //--------------------------------------------------

    //let s = 5;
    //let s = 5*5;
    //println!("The value of the variable s = {}",s);


    /* 
    let s = 32;
    println!("The value of the variable s = {} is currently an integer",s);

    let s = 'A';
    println!("The value of the variable s = {} is currently char",s);

    let s = 64.5;
    println!("The value of the variable s = {} is currently a float",s);
    
    */

    // scope of variables in {}
    let mut s = 65;
    {
        //let s = 60;     //using this let command makes the variable inside the scope a new variable and only holds for the scope of this area
        s = 60;              //without using the let command it will change the outer variable
        println!("The value of the variable s inside the inner scop is {}", s);
    }
    println!("The value of the variable s outside the inner scop is {}", s);


    //----------------------------------
    //      Constants
    //      cannot use keyword mut - they are always immmutable and cannot be changed to mutable
    //      declare using the const keyword and the type must be annotated - it is not inferred   :u32 :float, etc
    //----------------------------------

    const MAX_SALARY:u32 = 100_000;
    println!("The value of the constant is {}", MAX_SALARY);

}
