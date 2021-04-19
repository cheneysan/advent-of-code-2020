use common::read_ints;

fn main() {
    let numbers = read_ints("./data/day1.txt");

    for (i1, n1) in numbers.iter().enumerate() {
        for n2 in &numbers[i1 + 1..] {
            if n1 + n2 == 2020 {
                println!("Part 1 is {}", n1 * n2);
                break;
            }
        }
    }

    for (i1, n1) in numbers.iter().enumerate() {
        for (i2, n2) in numbers[i1 + 1..].iter().enumerate() {
            for n3 in &numbers[i2 + 1..] {
                if n1 + n2 + n3 == 2020 {
                    println!("Part 2 is {}", n1 * n2 * n3);
                    break;
                }
            }
        }
    }

}