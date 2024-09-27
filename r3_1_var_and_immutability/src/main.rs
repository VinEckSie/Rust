
fn main() {
//IMMUTABILITY
    //by default, variable are immutable
    //safety and easy concurrency
    //ability to set up a var muttable with mut keyword

    // The starting temperature is set to 22°C and cannot be changed
    let initial_temperature = 22;
    //initial_temperature = 25; //generate immutability error if uncommented
    println!("The initial temperature of the module is: {initial_temperature}°C");

    // A change in the space environment has been detected, so the temperature needs to be adjusted
    let mut current_temperature = initial_temperature;  // Mutability is allowed here
    println!("The current temperature of the module is: {current_temperature}°C");

    // Adjust the temperature of the module
    current_temperature = 25;
    println!("The adjusted temperature of the module is: {current_temperature}°C");

//constants
    //always immutable, so cannot use mut keywords
    //use UPPER CASE and underscores
    //can be set only to a constant expression
    const ORBITAL_PERIOD_IN_DAYS: u32 = 19413120 / 86400;
    println!("\nThe duration of one orbit on Venus is {ORBITAL_PERIOD_IN_DAYS} days.");

//shadowing
    //we can shadow a variable with a second variable, by using the same name and the keyword let
    //the type can be different, we cannot use mut keyword
    //so, we can perform transformations but keep immutability of the var
    //it spares us from having to come up with different name
        
     // First, declare an immutable string variable
     let velocity = "50";
     println!("\nThe velocity of the spacecraft is: {} km/s", velocity);
 
     // Shadowing: Convert the string to an integer implicitly through shadowing
     let velocity = velocity.parse::<i32>().expect("Not a valid number");
     
     // Shadow again to convert the velocity from km/s to meters per second
     let velocity = velocity * 1000;
     println!("The velocity of the spacecraft in meters per second is: {} m/s", velocity);
 }
