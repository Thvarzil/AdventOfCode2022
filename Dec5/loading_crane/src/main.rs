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
    for line in &crates{
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
            if !item.is_whitespace(){
             stacks[j].push(*item);
            }
            
            
        }
    }

    //Clone stacks for part two
    let mut stacks_p2 = stacks.clone();

    //executing instructions

    

    for line in instructions{
        let mut instruction:Vec<usize> = Vec::new();
        for (j,word) in line.split(" ").enumerate(){
            if j%2==1 {
                instruction.push(word.parse().unwrap());
            }
        }
        
        move_crate(&mut stacks,instruction[2],instruction[1],instruction[0], true);
        move_crate(&mut stacks_p2,instruction[2],instruction[1],instruction[0], false);
    }
    println!();
    for stack in &stacks{
        for item in stack{
            print!("{}",item);
        }
        println!();
    }
    println!();
    for stack in &stacks_p2{
        for item in stack{
            print!("{}",item);
        }
        println!();
    }
}

fn move_crate(vector:&mut Vec<Vec<char>>,target:usize,origin:usize,iterations:usize,part_1:bool){
    let mut part_2 = String::from("");
    for _i in 0..iterations{
        
        if !vector[origin-1].is_empty(){
            let crane = vector[origin-1].pop().unwrap();
            if part_1{vector[target-1].push(crane);}
            else {part_2.push(crane)}
        }
    }

    if !part_1{
        let moved = part_2.chars();
        let mut moved_vec:Vec<char> = Vec::new();
        for cha in moved{
            moved_vec.push(cha);
        }
        moved_vec.reverse();

        for item in moved_vec{
            vector[target-1].push(item);
        }
    }
    
}
