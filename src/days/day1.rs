use crate::helpers;

pub fn solve(input: &str) {
    let lines: Vec<i32> = helpers::file::to_list(&input)
    .iter()
    .map(|x| x.parse().unwrap())
    .collect();
    part1(&lines);
    part2(&lines);
}

fn part1(list: &Vec<i32>) {
    let mut sum = 0;
    for i in list.windows(2) {
        if i[0] < i[1]{
            sum += 1;
        }
    }
    println!("{}",sum);
}

fn part2(list: &Vec<i32>) {
    let mut new_list: Vec<i32>= Vec::new();
    for i in list.windows(3) {
        let sum:i32 = i.iter().sum();
        new_list.push(sum);
    }
    part1(&new_list)
}
