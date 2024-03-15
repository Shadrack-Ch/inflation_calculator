
pub struct InflationCalculator {
    cpi_year1: f64,
    cpi_year2: f64,
}

impl InflationCalculator {
    pub fn new(cpi_year1: f64, cpi_year2: f64) -> Self {
        InflationCalculator { cpi_year1, cpi_year2 }
    }

    pub fn adjust_amount(&self, original_amount: f64) -> f64 {
        original_amount * (self.cpi_year2 / self.cpi_year1)
    }
}
