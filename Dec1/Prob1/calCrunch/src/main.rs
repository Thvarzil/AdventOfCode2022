use std::env;
use std::fs;
use std::str::Split;
use std::vec::Vec;

fn main() {

    //hardcoding filepath
    let file_path = "kcalReport.txt";

    //storing contents to file to a very long string
    let contents: &str = &fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    //Converting string to slice, each item being one elf
    let elves: Split<&str> = contents.split("\n\n");

    //creating array to contain elf totals, elf_num is there to be able to identify which elf we are adding up.
    //Likely there's a better way to do this but... was focusing on slice/array iteration
    let mut elf_num=0;
    let mut elf_totals = Vec::new();
    for elf in elves{
        let food_items: Split<&str> = elf.split("\n");
        let elf_total = 0;
        for snack in food_items{
            let snack_num = snack.parse::<i64>().unwrap();
            elf_total = elf_total+snack_num;
        }
        elf_totals.push(elf_total);
        elf_num=elf_num+1;
    }

    let mut elf_totals_sortable = elf_totals.as_slice();

    //sort descending
    elf_totals_sortable.sort().reverse();
    
    //Answer to part one - the elf with the most calories
    println!("Top Elf: {}", elf_totals_sortable[0]);

    //Answer to part two - total of top three elves
    let top_elves = elf_totals_sortable[0..3]; 
    let mut top_total = 0;
    for elf in top_elves{
        top_total = top_total+elf;
    }

    println!("Total of Top Elves: {}", top_total);
}

/* comment out to use rust native functionality
//simple sorting method
fn sort<A, T>(mut array: A) -> A
where
    A: AsMut<[T]>,
    T: Ord,
{
    let slice = array.as_mut();
    slice.sort();

    array
}