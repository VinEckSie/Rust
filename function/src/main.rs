fn main() {
    println!("Hello, world!");

    saturn_temp(500,'C');

    let x = getfiveplusone(5);
    println!("X is equal to {x}"); //print test
}

fn saturn_temp(temp: i32, unit: char){
    println!("The highest temperature on Saturn is: {temp} {unit}");
}

//function to test to change the return value as a statement
fn getfiveplusone(x: i32) -> i32 {
    x + 1
}
