fn main() {
    let mut elves = lines();
    let mut biggest_elf: i32 = 0;
    let mut current_elf: i32 = 0;
    let mut all_elves: Vec<i32> = vec![];
    while let Some(i)= elves.next() {
        if i == "" {
            println!("The total size of this elf is {} calories", current_elf);
            if current_elf > biggest_elf {
                biggest_elf = current_elf;
            }
            all_elves.push(current_elf);
            current_elf = 0;
        } else { 
            current_elf += i.parse::<i32>().unwrap();
            //println!("current elf size is: {}", current_elf)
        }
    }
    println!("The biggest elf is: {} calories", biggest_elf);
    all_elves.sort();
    let sorted_elves: i32 = all_elves.iter().rev().take(3).sum();
    println!("The top 3 elves are the carrying {} calories", sorted_elves)
}

pub fn lines() ->impl Iterator<Item = &'static str> {
    include_str!("../data/day1.txt").lines()
}
