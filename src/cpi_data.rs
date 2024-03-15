use reqwest;
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
struct ApiResponse {
    pages: i32,
    data: Vec<CpiData>,
}

#[derive(Deserialize)]
struct CpiData {
    date: String,
    value: Option<f64>,
}

pub async fn fetch_cpi_data(year: i32) -> Result<f64, Box<dyn Error>> {
    let url = format!("http://api.worldbank.org/v2/country/all/indicator/FP.CPI.TOTL?date={}&format=json", year);
    let response = reqwest::get(&url).await?;
    
    if response.status().is_success() {
        let api_response: Vec<ApiResponse> = response.json().await?;
        if let Some(first_page) = api_response.get(0) {
            if let Some(cpi_data) = first_page.data.get(0) {
                if let Some(cpi) = cpi_data.value {
                    Ok(cpi)
                } else {
                    Err("CPI data not available for this year".into())
                }
            } else {
                Err("No data found".into())
            }
        } else {
            Err("Invalid response format".into())
        }
    } else {
        Err("Failed to fetch CPI data".into())
    }
}
