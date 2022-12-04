use std::fs;

fn main() {
    //grab file contents
    let file_path = "./elf_assignments.txt";
   
    let contents: &str = &fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    //split into team strings of x-y,a-b format
    let team_list = contents.split("\n");
    //initialize result variable and vector to hold each team's assignments
    let mut inefficient_teams = 0;
    let mut assignments: Vec<_> = Vec::new();//created globally to reduce memory reallocation
    //look at each team
    for team in team_list{
        //split team into two elves of x-y format
        let elves = team.split(",");
        //Look at each elf to split into i32 format range caps
        for elf in elves{
            for value in elf.split("-"){
                //push assignment values into vector
                assignments.push(value.parse::<i32>().unwrap());
            }
        }
        //vector should look like elf1low, elf1high, elf2low, elf2high. comparing accordingly
        if (assignments[0]<=assignments[2] && assignments[1]>=assignments[3])||(assignments[0]>=assignments[2] && assignments[1]<=assignments[3]){
            inefficient_teams = inefficient_teams+1; 
        }
        //empty assignments so we are set for next team.
        assignments.clear()
    }

    println!("Number of inefficient teams: {}", inefficient_teams);
}
