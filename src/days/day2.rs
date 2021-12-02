pub fn solve(input: &str) {
    let list: Vec<&str> = input
    .split("\n")
    .collect();
    part1(list);
    part2();
}

fn split_command(command: &str) -> (&str, i32){
    let splitted = command.split(" ").collect::<Vec<&str>>();
    let value = splitted[1].parse::<i32>().unwrap();
    (splitted[0],value)
}

fn move_command(command: &str, x: i32, y: i32) -> (i32,i32) {
        let res = split_command(command);
        let com = res.0;
        let value: i32 = res.1;
        match com {
             i if i == "forward" =>  (x + value,y),
             i if i == "down" => (x,y + value),
             i if i == "up" => (x, y - value),
             _ => (x,y)
        }
}

fn part1(list: Vec<&str>) {
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for i in list{
           let res = move_command(i, x, y);
           x = res.0;
           y = res.1;
    }
    println!("{} ",x*y);
}

fn part2() {
    println!("Not implemented");
}
