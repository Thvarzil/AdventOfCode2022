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

    let mut parsed_lines:Vec< Vec<char> > = Vec::new();
    for line in crates{
       let attempt= line.chars();
       let mut parsed_line: Vec<char> = Vec::new();
       for (i,item) in attempt.enumerate(){
            if i%4==1{
                parsed_line.push(item);
                print!("{}", item);
            }
       }
       
       println!("{}",parsed_line.len());

       parsed_lines.push(parsed_line);
       
    }
}
