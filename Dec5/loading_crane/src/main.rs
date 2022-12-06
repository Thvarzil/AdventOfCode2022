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
    //parsing to just the data needed
    let mut parsed_lines:Vec< Vec<char> > = Vec::new();
    for line in crates{
       let attempt= line.chars();
       let mut parsed_line: Vec<char> = Vec::new();
       for (i,item) in attempt.enumerate(){
            if i%4==1{
                parsed_line.push(item);
            }
        }
       parsed_lines.push(parsed_line);
    }

    //Rotation oclock
    let mut stacks:Vec<Vec<char>>=vec![Vec::new();9];

    for i in 1..9{
        for (j,item) in parsed_lines[parsed_lines.len()-i].iter().enumerate(){
            
            stacks[j].push(*item);
        }
    }

    for stack in stacks{
        for item in stack{
            print!("{}",item)
        }
        println!();
    }

    //parsing instructions

    let mut parsed_instructions:Vec<Vec<i32>> = Vec::new();

    for line in instructions{
        let mut instruction:Vec<i32> = Vec::new();
        for (j,word) in line.split(" ").enumerate(){
            if j%2==1 {
                instruction.push(word.parse().unwrap());
            }
        }
        parsed_instructions.push(instruction);
    }
}

fn move_crate<T>(vector:Vec<Vec<T>>,origin:i32,target:i32,iterations:i32){

}
