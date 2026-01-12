
fn main() {
    let mut counter: usize = 0;

    let mut lesser_count: i64 = 0;
    let mut greater_count: i64 = 0;

    while counter <= 1_00_000 {
        let random_number: i64 = rand::random();

        if random_number.abs() >= i64::MAX / 2 {
            greater_count += 1;
            // greater_branch(random_number);
        } else {
            lesser_count += 1;
            // lesser_branch(random_number);
        }
        counter += 1
    }

    println!("lesser={lesser_count}");
    println!("greater={greater_count}");
}

fn greater_branch(n: i64) {
    println!("{n} is greater!");
}

fn lesser_branch(n: i64) {
    println!("{n} is lesser!");
}
