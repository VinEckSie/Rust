fn main() {
    //SAME CMDS whatever the OS
    //cargo new project_name
        // top level is just for readme file, config file, license infos
        //generate a .gitignore file
        //and a rs source file in a src repo
        //and a toml (tom obvious minimal language) Cargo configuration format file
            //[pachage] : infos needed to compile the program
            //[dependencies] packages of code needed to run the program (crates)

    //cargo check 
        //faster than build because no exe created : check if your code compile
    //cargo build 
        //create an executable in target/debug : debug because the default build is a debug build)
        //running build the first time will create a cargo.lock at the top level to keep track of exact versions of dependencies
    //./target/debug/filename to run the program
    //cargo run
        //!MOST DEVELOPPER use this instead of build cmd

    //cargo build --release
        //compile with optimizations when the project is ready for release)
        //will create a target/release dir
        //code will run faster but compile time will take more time, thats why wh

    println!("Hello, world!");
}
