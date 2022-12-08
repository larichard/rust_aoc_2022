fn main() {
    part1();
    part2()
}

fn part1() {
    let data = lines();
    //let data = ["2-4,6-8", "2-3,4-5", "2-8,3-7", "5-7,7-9","6-6,4-6","2-6,4-8"].iter();
    let mut fully_contains: i32 = 0;
    for i in data {
        let mut split = i.split(',');
        let split_sched = (split.next().unwrap(), split.next().unwrap());
        let mut first_sched = split_sched.0.split('-');
        let split_first_sched: (i32, i32) = (to_int(first_sched.next().unwrap()), to_int(first_sched.next().unwrap()));
        let mut second_sched = split_sched.1.split('-');
        let split_second_sched: (i32, i32) = (to_int(second_sched.next().unwrap()), to_int(second_sched.next().unwrap()));
        if (split_first_sched.0 <= split_second_sched.0 && split_first_sched.1 >= split_second_sched.1) ||
           (split_second_sched.0 <= split_first_sched.0 && split_second_sched.1 >= split_first_sched.1) {
            //println!("{:?}", split_first_sched);
            fully_contains += 1;
        }
    }
    println!("Part 1: {} schedules are fully contained by their partners", fully_contains)
}

fn to_int(x:&str) -> i32 {
    x.parse::<i32>().unwrap()
}

fn part2() {
    let data = lines();
    //let data = ["2-4,6-8", "2-3,4-5", "2-8,3-7", "5-7,7-9","6-6,4-6","2-6,4-8"].iter();
    let mut total_overlap: i32 = 0;
    for i in data {
        let mut split = i.split(',');
        let split_sched: (&str, &str) = (split.next().unwrap(), split.next().unwrap());
        let mut first_sched = split_sched.0.split('-');
        let split_first_sched: (i32, i32) = (to_int(first_sched.next().unwrap()), to_int(first_sched.next().unwrap()));
        let mut second_sched = split_sched.1.split('-');
        let split_second_sched: (i32, i32) = (to_int(second_sched.next().unwrap()), to_int(second_sched.next().unwrap()));
        // for j in split_first_sched.0..=split_first_sched.1 {
        //     if (split_second_sched.0..=split_second_sched.1).contains(&j) {
        //         total_overlap += 1;
        //         break
        //     }
        // }
        // if ((split_first_sched.0..=split_first_sched.1).contains(&split_second_sched.0) || (split_first_sched.0..=split_first_sched.1).contains(&split_second_sched.1)) ||
        //    ((split_second_sched.0..=split_second_sched.1).contains(&split_first_sched.0) || (split_second_sched.0..=split_second_sched.1).contains(&split_first_sched.1)) {
        //         total_overlap += 1;
        // }
        if (split_first_sched.0 >= split_second_sched.0 && split_first_sched.0 <= split_second_sched.1) ||
           (split_first_sched.1 <= split_second_sched.1 && split_first_sched.1 >= split_second_sched.0) || 
           (split_second_sched.0 >= split_first_sched.0 && split_second_sched.0 <= split_first_sched.1) ||
           (split_second_sched.1 <= split_first_sched.1 && split_second_sched.1 >= split_first_sched.0) {
            total_overlap += 1;
           }
    }
    println!("Part 2: {} schedules overlap at least once", total_overlap)
}

pub fn lines() -> impl Iterator<Item = &'static str> {
    include_str!("../data/day4.txt").lines()
}

