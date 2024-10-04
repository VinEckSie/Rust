fn main() {
//Rust is a statically typed language, it must know the types of all variables at compile time
    //If we don’t add the : u32 type annotation : error, bc it needs more information from us to know which type we want to use
    let _altitude: u32 = "42".parse().expect("Not a number!"); 

//scalar types
    //A scalar type represents a single value. 
    //Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
    //signed types start with i, and unsigned u (only positive)

    //Length	Signed	Unsigned
    //8-bit	    i8	    u8
    //16-bit	i16	    u16
    //32-bit	i32	    u32
    //64-bit	i64	    u64
    //128-bit	i128	u128
    //arch	    isize	usize
    
    // Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive
    // For instance, for an i8 number, we can store numbers from  -128 to 127
    // Unsigned variants can store numbers from 0 to 2n - 1
    // For instance, for an u8 number, we can store numbers from  0 to 255
    // isize and usize types depend on the architecture of the computer your program is running on
    //integer type default is i32

    //Two ways of creating integer 
    let _temp: u32 = 32_000; //increase readiness
    let _temp = 50u32; //used for numeric literals in a general way

    //integer overflow : try to put 256 in an u8 var (255 max)
    //debug mode > rust panic
    //release mode > rust perform complement wrapping : 256 becomes 0, 257 becomes 1. and so on
        //This behaviour is considered as an error : must handle with the following : 
            //Wrap all mode in wappring_* methods
            //Return None with checked_* methods
            //Return bool and the value with overflowing_* methods
            //Saturate at the maximum or minimum value with saturating_* methods

    //Two primitive floating points : f32 f64
    

    }