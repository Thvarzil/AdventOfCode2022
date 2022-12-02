use std::env;
use std::fs;
use std::str::Split;

fn main() {
    let file_path = "./src/kcalReport.txt";
    
    

    let contents: &str = &fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let elves: Split<&str> = contents.split("\n\n");
    let mut elf_num=0;
    let mut elf_totals: [i64;250]=[0;250];
    for elf in elves{
        let food_items: Split<&str> = elf.split("\n");
        for snack in food_items{
            let snack_num = snack.parse::<i64>().unwrap();
            elf_totals[elf_num]=elf_totals[elf_num]+snack_num;
        }

        elf_num=elf_num+1;
    }
    
    elf_totals.sort();
    elf_totals.reverse();
    //Answer to part one - the elf with the most calories
    println!("Top Elf: {}", elf_totals[0]);

    //Answer to part two - total of top three elves
    let top_elves = &elf_totals[0..3]; 
    let mut top_total = 0;
    for elf in top_elves{
        top_total = top_total+elf;
    }

    println!("Total of Top Elves: {}", top_total);
}

