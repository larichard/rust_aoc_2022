use std::iter::zip;

fn main() {
    //let rucksacks = lines();
    part1();
    part2()
}

pub fn lines() ->impl Iterator<Item = &'static str> {
    include_str!("../data/day3.txt").lines()
}

fn part1() {
    let mut alphabet: Vec<char> = ('a'..='z').collect::<Vec<_>>();
    let mut upper_alphabet: Vec<char> = ('A'..='Z').collect::<Vec<_>>();
    alphabet.append(&mut upper_alphabet);
    let priorities: Vec<i32> = (1..=52).collect::<Vec<_>>();

    let dict = zip(alphabet, priorities);
    let rucksacks = lines();
    // let rucksacks = vec!["vJrwpWtwJgWrhcsFMMfFFhFp",
    //                                 "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
    //                                 "PmmdzqPrVvPwwTWBwg",
    //                                 "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
    //                                 "ttgJtRGJQctTZtZT",
    //                                 "CrZsJsPPZsGzwwsLwLmpwMDw"];
    let mut total_priorities: i32 = 0;
    for i in rucksacks {
        let split_num = i.len() / 2;
        let first_half = &i[0..split_num];
        let second_half = &i[split_num..];
        let mut found = '\0';
        for j in first_half.chars() {
            if second_half.contains(j) {
                found = j;
                break
            }
        }
        for i in dict.clone() {
            if i.0 == found {
                total_priorities += i.1
            }
        }
    }
    println!("Part 1: The sum of the priorities is {}", total_priorities)
}

#[derive(Debug)]
struct Triplet {
    first: &'static str,
    second: &'static str,
    third: &'static str
}

impl Triplet {
    fn new(first:&'static str, second:&'static str, third:&'static str) -> Triplet {
        Triplet { first: first, second: second, third: third }
    }
}

fn part2() {
    let mut total_priorities: i32 = 0;
    let mut alphabet: Vec<char> = ('a'..='z').collect::<Vec<_>>();
    let mut upper_alphabet: Vec<char> = ('A'..='Z').collect::<Vec<_>>();
    alphabet.append(&mut upper_alphabet);
    let priorities: Vec<i32> = (1..=52).collect::<Vec<_>>();
    let dict = zip(alphabet, priorities);

    let rucksacks: Vec<_> = lines().collect();
    let triplets: Vec<Triplet> = rucksacks.chunks_exact(3).map(|chunk| Triplet::new(chunk[0], chunk[1], chunk[2])).collect();
    
    for tri in triplets {
        for j in tri.first.chars() {
            if tri.second.contains(j) && tri.third.contains(j) {
                match dict.clone().find(|x| x.0 == j) {
                    Some(found) => total_priorities += found.1,
                    None => println!("No corresponding priority for {j}")
                }
            break
            }
        }
    }
    
    println!("Part 2: The sum of the priorities is {}", total_priorities)
}