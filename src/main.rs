use std::fs;

fn read_data() {
    let contents = fs::read_to_string("src/orders.txt")
        .expect("Should have been able to read the file");

    let mut ord_ids: Vec<&str> = vec![];
    let mut dates: Vec<&str> = vec![];
    let mut emails: Vec<&str> = vec![];
    let mut options: Vec<&str> = vec![];
    let mut costs: Vec<f32> = vec![];
    let mut ratings: Vec<i64> = vec![];

    let lines = contents.split("\n");
    for line in lines {
        let data = line.split(",").collect::<Vec<&str>>();
        ord_ids.push(data[0]);
        dates.push(data[1]);
        emails.push(data[2]);
        options.push(data[3]);
        costs.push(data[4].parse::<f32>().unwrap());
        ratings.push(data[5].parse::<i64>().unwrap());
    }
}

fn main() {
    read_data();
}