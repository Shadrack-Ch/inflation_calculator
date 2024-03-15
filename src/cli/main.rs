use std::env;
use inflation_calculator::cpi_data; // Assuming your library is named `inflation_calculator`
use inflation_calculator::calculator; // Calculator module

fn main() {
    let args: Vec<String> = env::args().collect();

    // Basic command line argument parsing
    // You'll likely want to use a more robust solution like `clap`
    if args.len() < 4 {
        eprintln!("Usage: inflation_calculator_cli <year1> <year2> <amount>");
        return;
    }

    let year1 = args[1].parse::<i32>().expect("Invalid input for year1");
    let year2 = args[2].parse::<i32>().expect("Invalid input for year2");
    let amount = args[3].parse::<f64>().expect("Invalid input for amount");

    // Fetch CPI data and calculate the adjusted amount
    // This is a placeholder - you'll need to implement async handling
    let cpi_year1 = cpi_data::fetch_cpi_data(year1).await.expect("Failed to fetch CPI data for year1");
    let cpi_year2 = cpi_data::fetch_cpi_data(year2).await.expect("Failed to fetch CPI data for year2");

    let adjusted_amount = calculator::adjust_for_inflation(amount, cpi_year1, cpi_year2);

    println!("Adjusted amount from {} to {}: {}", year1, year2, adjusted_amount);
}
