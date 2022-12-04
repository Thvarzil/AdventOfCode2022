use std::fs;

fn main() {
    let file_path = "./rucksacks.txt";
   
    let contents: &str = &fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut total_prio = 0;
    let mut duplicates: Vec<char> = Vec::new();
    let rucksack_list = contents.split("\n");
    let mut badge_total = 0;
    let mut current_group: Vec<_> = Vec::new();

    for rucksack in rucksack_list{
        let mut item_names: Vec<_> = rucksack.chars().collect();
        let front_pocket: Vec<_> = item_names[..item_names.len()/2].to_vec();
        let back_pocket: Vec<_> = item_names[item_names.len()/2..item_names.len()].to_vec();

        for item in front_pocket{
            let mut done = false;
            for item_back in &back_pocket{
                
                if item==*item_back {
                    total_prio = total_prio+item_to_priority(item);
                    done=true;
                    break;
                }
                
            }
            if done {break;}
        }

        current_group.push(rucksack);
        if current_group.len()==3{
            let items: Vec<_> = rucksack.chars().collect();

            for item in items{
                if current_group[0].find(item) != None && current_group[1].find(item) != None {
                    badge_total = badge_total + item_to_priority(item);
                    break;
                }
            }

            current_group.clear();
        }
    }

    println!("Solution to Part 1: {}", total_prio);
    println!("Solution to Part 2: {}", badge_total);

}

fn item_to_priority(item: char)->u32{
    let ascii_val = item as u32;
    if ascii_val>96 {return ascii_val-96;}
    else {return ascii_val-38;}
}