use std::vec::Vec;
use std::fs;
fn main() {
    let file_path = "./src/moves.txt";
    let mut score_p1 = 0;
    let mut score_p2 = 0;
    let contents: &str = &fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut battles = Vec::new();

    for battle in contents.split("\n"){
        let mut battle_vec = Vec::new();
        let battle_array = battle.split(" ");

        for elf_move in battle_array{
            battle_vec.push(elf_move);
        }

        let battle_tuple = (battle_vec[0], battle_vec[1]);
        
        battles.push(battle_tuple);
    } 


    for battle in battles{
        match battle.1{
            "X" => score_p1 = score_p1+1,
            "Y" => {score_p1 = score_p1+2;score_p2=score_p2+3;},
            "Z" => {score_p1 = score_p1+3;score_p2=score_p2+6;},
            &_  => println!("Something went horribly wrong"),
        }

        score_p1 = score_p1+fight(battle.0,battle.1);
        score_p2 = score_p2+fight2(battle.0,battle.1);
    }
     
    println!("Score for Part 1: {}",score_p1);
    println!("Score for Part 2: {}",score_p2);
    

}

fn fight(opp:&str, you:&str)->i32{
    let mut opp_power = 0;
    let mut you_power = 0;

    match opp{
        "A" => opp_power = 1,
        "B" => opp_power = 2,
        "C" => opp_power = 3,
        &_ => println!("Opp went horribly wrong"),
    }
    match you{
        "X" => you_power = 1,
        "Y" => you_power = 2,
        "Z" => you_power = 3,
        &_ => println!("You went horribly wrong"),
    }
    let result = you_power-opp_power;

    match result{
        0 => return 3,
        1 => return 6,
        -2 => return 6,
        _ => return 0,
    }

}

fn fight2(opp:&str, you:&str)->i32{
    let mut opp_power = 0;

    match opp{
        "A" => opp_power = 1,
        "B" => opp_power = 2,
        "C" => opp_power = 3,
        &_ => println!("Opp went horribly wrong"),
    }

    if (you=="X" && opp_power==1) {return 3;}
    else if you=="X" {return opp_power-1;}
    else if you=="Y" {return opp_power;}
    else if you=="Z" && opp_power==2 {return 3;}
    else {return (opp_power+1)%3;}

}
