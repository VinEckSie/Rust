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


    //CONSTANTS
        //always immutable, so cannot use mut keywords
        //use UPPER CASE and underscores
        //can be set only to a constant expression
        const ORBITAL_PERIOD_IN_DAYS: u32 = 6000;
        println!("\n The duration of one orbit on Venus is {ORBITAL_PERIOD_IN_DAYS} days.");
}
