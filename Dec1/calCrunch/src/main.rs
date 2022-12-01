use std::env;
use std::fs;
use std::str::Split;

fn main() {
    let file_path = "kcalReport.txt";
    
    

    let contents: &str = &fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let elves: Split<&str> = contents.split("\n\n");
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

fn sort<A, T>(mut array: A) -> A
where
    A: AsMut<[T]>,
    T: Ord,
{
    let slice = array.as_mut();
    slice.sort();

    array
}