use std::fs;

fn main() {
     //grab file contents
     let file_path = "./crates.txt";
   
     let contents: &str = &fs::read_to_string(file_path)
         .expect("Should have been able to read the file");

    //splitting 
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut crates: Vec<&str> = Vec::new();
    let mut instructions: Vec<&str> = Vec::new();

    //separating into a variable to hold crates and a variable to hold instructions
    for line in lines{
        if line.contains("["){
            crates.push(line);
        }
        else if line.contains("m") {
            instructions.push(line);
        }
    }

    

    //confirming data is being separated as expected
    for line in crates{
        println!("{}", line);
    }
    println!("------------------------------");
    for line in instructions{
        println!("{}", line);
    }
}
