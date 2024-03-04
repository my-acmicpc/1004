use std::io;

#[inline]
fn square(n: i32) -> i32 {
    n * n
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let test_case_count = line.trim().parse::<usize>().unwrap();
    for _ in 0..test_case_count {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.trim().split(' ').flat_map(&str::parse::<i32>);
        let x1 = iter.next().unwrap();
        let y1 = iter.next().unwrap();
        let x2 = iter.next().unwrap();
        let y2 = iter.next().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let circle_count = line.trim().parse::<usize>().unwrap();
        let mut result = 0;
        for _ in 0..circle_count {
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.trim().split(' ').flat_map(&str::parse::<i32>);
            let x = iter.next().unwrap();
            let y = iter.next().unwrap();
            let r = iter.next().unwrap();
            let r_squared = square(r);
            let distance1_squared = square(x1 - x) + square(y1 - y);
            let distance2_squared = square(x2 - x) + square(y2 - y);
            let in1 = distance1_squared < r_squared;
            let in2 = distance2_squared < r_squared;
            if in1 != in2 {
                result += 1
            }
        }
        println!("{}", result);
    }
}
