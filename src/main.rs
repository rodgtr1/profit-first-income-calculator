use std::io;

const TAXPER: f64 = 0.30;
const OWNERPER: f64 = 0.55;
const PROFITPER: f64 = 0.05;
const OPEXPER: f64 = 0.10;

fn main() {
    println!("Input your total.");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let num: f64 = input.trim().parse().unwrap();

    let taxes: f64 = num * TAXPER;
    let owner = num * OWNERPER;
    let profit = num * PROFITPER;
    let opex = num * OPEXPER;

    println!(
        "Taxes is {:.2}\n Owners pay is {:.2}\n Profit is {:.2}\n and Operating expenses is {:.2}",
        taxes, owner, profit, opex
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equals_100() {
        let result = TAXPER + OWNERPER + PROFITPER + OPEXPER;
        let formatted = f64::trunc(result  * 100.0) / 100.0;
        assert_eq!(formatted, 1.00);
    }
}