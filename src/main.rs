
use std::io::{self};

fn main() {
    println!("Enter the number of days in a year on new planet:");
    let mut input = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut input).unwrap();
    
    let results:(i32,f32);
    match input.trim().parse() {
        Ok(x) => {
            results = calculate_leap_year(x);
            println!("A leap year should occur every {} years, which results in an error of {} days per century",results.0,results.1);
        }
        Err(_e) => eprintln!("Please enter a decimal number. {} is not a decimal number.", input)
    };
}

fn calculate_leap_year(days_in_year: f32) -> (i32,f32){
    //We are looking to find a leap year system that leads to an average of less than one day error per century
    
    let regular_year_length = days_in_year.round() as i32;
    let mut years_between_leap_years = 2;
    let mut error:f32;

    let mut century_length = (regular_year_length*100+50) as f32;
    error = century_length-days_in_year*100.0;
    

    for i in 3..100{
        century_length = (regular_year_length*100+100/i) as f32;
        let discrepancy = century_length-days_in_year*100.0;
        if discrepancy.abs()<error {
            error = discrepancy;
            years_between_leap_years = i;
        }
    }


    (years_between_leap_years,error)
}