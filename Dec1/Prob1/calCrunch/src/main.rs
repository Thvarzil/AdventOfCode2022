use std::env;
use std::fs;
use std::str::Split;

fn main() {

    //hardcoding filepath
    let file_path = "kcalReport.txt";

    //storing contents to file to a very long string
    let contents: &str = &fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    //Converting string to slice, each item being one elf
    let elves: Split<&str> = contents.split("\n\n");

    //creating array to contain elf totals, elfNum is there to be able to identify which elf we are adding up.
    //Likely there's a better way to do this but... was focusing on slice/array iteration
    let mut elfNum=0;
    let mut elfTotals: [i64;250]=[0;250];
    for elf in elves{
        let foodItems: Split<&str> = elf.split("\n");
        for snack in foodItems{
            let snackNum = snack.parse::<i64>().unwrap();
            elfTotals[elfNum]=elfTotals[elfNum]+snackNum;
        }

        elfNum=elfNum+1;
    }
    
    //Answer to part one - the elf with the most calories
    println!("Top Elf: {}", sort(elfTotals)[249]);

    //Answer to part two - total of top three elves
    let sortedElfTotals = sort(elfTotals);
    let topElves = &sortedElfTotals[247..250]; 
    let mut topTotal = 0;
    for elf in topElves{
        topTotal = topTotal+elf;
    }

    println!("Total of Top Elves: {}", topTotal);
}

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