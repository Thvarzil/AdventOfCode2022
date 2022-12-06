use std::fs;

fn main() {
    //grab file contents
    let file_path = "./data_buffer.txt";
   
    let contents: &str = &fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut last_three:Vec<char> = Vec::new();
    let mut part_1_result = 0;
    

    for cha in contents.chars(){
        let mut done = true;
        part_1_result += 1;

        if last_three.len()>0{
            let mut i = 0;
            for item in &last_three{
                i+=1;
                if item == &cha {
                    
                    for _j in 0..i{
                        if last_three.len()>0{last_three.remove(0);}
                        for item in &last_three{print!("{}",item);}
                println!("{}", cha);
                    }

                    done=false;
                    break;
                }

                
            }
            
            
            if done && last_three.len()>=3{
                println!("{}", part_1_result);
                for item in &last_three{print!("{}",item);}
                println!("{}", cha);
                break;
            }

            if last_three.len()>0{last_three.remove(0);}
            
        }

        last_three.push(cha);
        
    }
}
