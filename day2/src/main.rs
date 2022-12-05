use std::path::Iter;

fn main() {
    let elves = lines();
    //let elves = ["A Y", "B X", "C Z"].iter();
    day1(elves);
    //day2(elves)
}

pub fn lines() ->impl Iterator<Item = &'static str> {
    include_str!("../data/day2.txt").lines()
}

fn day1(elves: impl Iterator<Item = &'static str>) {
    let mut total_score: i32 = 0;
    for i in elves {
        match i { 
            //X = rock, Y = paper, Z = scissors
            //rock
            "A X" => total_score += 1 + 3,
            "A Y" => total_score += 2 + 6,
            "A Z" => total_score += 3 + 0,
            //paper
            "B X" => total_score += 1 + 0,
            "B Y" => total_score += 2 + 3,
            "B Z" => total_score += 3 + 6,
            //scissors
            "C X" => total_score += 1 + 6,
            "C Y" => total_score += 2 + 0,
            "C Z" => total_score += 3 + 3,
            _ => println!("No input!")
        }
    }
    println!("Your total score is: {}", total_score)
}

fn day2(elves: impl Iterator<Item = &'static str>) {
    let mut total_score: i32 = 0;
    for i in elves {
        match i {
            //X = lose, Y = draw, Z = win
            //rock
            "A X" => total_score += 3 + 0,
            "A Y" => total_score += 1 + 3,
            "A Z" => total_score += 2 + 6,
            //paper
            "B X" => total_score += 1 + 0,
            "B Y" => total_score += 2 + 3,
            "B Z" => total_score += 3 + 6,
            //scissors
            "C X" => total_score += 2 + 0,
            "C Y" => total_score += 3 + 3,
            "C Z" => total_score += 1 + 6,
            _ => println!("No input!")
        }
    }
    println!("Your total score is: {}", total_score)
}