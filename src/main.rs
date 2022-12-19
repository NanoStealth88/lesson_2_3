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

}
