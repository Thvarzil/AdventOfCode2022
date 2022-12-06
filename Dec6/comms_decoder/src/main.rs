use std::fs;

fn main() {
    //grab file contents
    let file_path = "./data_buffer.txt";
   
    let contents: &str = &fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut last_four:Vec<char> = Vec::new();
    let mut last_fourteen:Vec<char> = Vec::new();
    let mut part_1_result = 0;
    let mut part_2_result = 0;
    let mut done = false;
    let mut done_2 = false;

    for cha in contents.chars(){
        last_four.push(cha);
        last_fourteen.push(cha);
        if !done{part_1_result += 1;}
        if !done_2{part_2_result += 1;}

        if !done {
            done = check_packet(&last_four,4);
        }
        done_2 = check_packet(&last_fourteen,14);

        if done && done_2 {break;}
        if last_four.len()==4{last_four.remove(0);}
        if last_fourteen.len()==14{last_fourteen.remove(0);}
    }

    println!("Part 1: {} -> Part 2: {}",part_1_result,part_2_result);
}

fn check_packet(packet:&Vec<char>,length: usize)->bool{
    
    if packet.len()==length{
        for i in 0..length{
            for j in 0..length{
                if i != j && packet[i] == packet[j]{
                    return false;
                }
            }
        }
        return true;
    }
    else{
        return false;}
}

