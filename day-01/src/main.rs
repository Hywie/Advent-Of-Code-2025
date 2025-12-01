use std::fs;

fn main() {
    println!("Reading File");

    let contents = fs::read_to_string("src/input.txt")
        .expect("input txt file should be in the same directory as main src file");

    let contents = contents.trim().split("\n");

    let mut current: i32 = 50;
    let mut result: u32 = 0;

    for instruction in contents {
        let instruction = instruction.trim();
        if instruction.trim().is_empty() {
            continue;
        }

        let direction = &instruction[0..1];
        let count = &instruction[1..].parse::<i32>().unwrap();

        if direction == "L" {
            current -= count % 100;
        } else if direction == "R" {
            current += count % 100;
        }

        // correct cycle round (not clever to work out below -99)
        if current > 99 {
            current = current - 100;
        } else if current < 0 {
            current = 100 - (current * -1);
        }

        result = if current == 0 { result + 1 } else { result };

        println!(
            "Direction: {}, Count: {}, Current: {}, Result:{}",
            direction, count, current, result
        );
    }

    println!("Result is: {}", result.to_string());
}
