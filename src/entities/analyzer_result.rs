use crate::utils::helpers::extract_day_part;
use crate::entities::DailyAnalyzerResult;

pub struct AnalyzerResult {
    pub daily_results: Vec<DailyAnalyzerResult>,
    total_requests_number: u32,
    total_2xx_requests_number: u32,
    total_3xx_requests_number: u32,
    total_4xx_requests_number: u32,
    total_5xx_requests_number: u32,
    total_error_rate: f32,
}

impl AnalyzerResult {
    pub fn new() -> Self {
        Self {
            daily_results: Vec::new(),
            total_requests_number: 0,
            total_2xx_requests_number: 0,
            total_3xx_requests_number: 0,
            total_4xx_requests_number: 0,
            total_5xx_requests_number: 0,
            total_error_rate: 0.0,
        }
    }

    pub fn add_daily_result(&mut self, daily_result: DailyAnalyzerResult) {
        self.daily_results.push(daily_result);
    }

    pub fn calculate_total_requests_number(&mut self) {
        self.total_requests_number = self
            .daily_results
            .iter()
            .map(|daily_result| daily_result.daily_requests_number)
            .sum();
    }

    pub fn calculate_total_2xx_requests_number(&mut self) {
        self.total_2xx_requests_number = self
            .daily_results
            .iter()
            .map(|daily_result| daily_result.daily_2xx_requests_number)
            .sum();
    }

    pub fn calculate_total_3xx_requests_number(&mut self) {
        self.total_3xx_requests_number = self
            .daily_results
            .iter()
            .map(|daily_result| daily_result.daily_3xx_requests_number)
            .sum();
    }

    pub fn calculate_total_4xx_requests_number(&mut self) {
        self.total_4xx_requests_number = self
            .daily_results
            .iter()
            .map(|daily_result| daily_result.daily_4xx_requests_number)
            .sum();
    }

    pub fn calculate_total_5xx_requests_number(&mut self) {
        self.total_5xx_requests_number = self
            .daily_results
            .iter()
            .map(|daily_result| daily_result.daily_5xx_requests_number)
            .sum();
    }

    pub fn calculate_total_error_rate(&mut self) {
        self.total_error_rate =
            (self.total_4xx_requests_number as f32 + self.total_5xx_requests_number as f32) / self.total_requests_number as f32;
    }

        pub fn get_daily_result_if_day_exists(&mut self, date: &str) -> Option<&mut DailyAnalyzerResult> {
        let day_part = extract_day_part(date);
        self.daily_results.iter_mut().find(|daily_result| daily_result.date == day_part)
    }

    pub fn calculate_everything(&mut self) {
        self.calculate_total_requests_number();
        self.calculate_total_2xx_requests_number();
        self.calculate_total_3xx_requests_number();
        self.calculate_total_4xx_requests_number();
        self.calculate_total_5xx_requests_number();
        self.calculate_total_error_rate();
    }

    pub fn print_everything(&self) {
        println!("{}", "=".repeat(50));
        println!("📊 LOG ANALYSIS SUMMARY");
        println!("{}", "=".repeat(50));

        println!("\n📈 OVERALL STATISTICS:");
        println!("  • Total Requests: {}", self.total_requests_number);
        println!("  • 2xx Responses:  {}", self.total_2xx_requests_number);
        println!("  • 3xx Responses:  {}", self.total_3xx_requests_number);
        println!("  • 4xx Responses:  {}", self.total_4xx_requests_number);
        println!("  • 5xx Responses:  {}", self.total_5xx_requests_number);
        println!("  • Error Rate:     {:.2}%", self.total_error_rate * 100.0);

        println!("\n📅 DAILY BREAKDOWN:");
        if self.daily_results.is_empty() {
            println!("  No daily results available");
        } else {
            for (i, daily_result) in self.daily_results.iter().enumerate() {
                println!("  Day {}: {}", i + 1, daily_result.date);
                println!(
                    "    └─ Requests: {} | 2xx: {} | 3xx: {} | 4xx: {} | 5xx: {} | Error Rate: {:.2}%",
                    daily_result.daily_requests_number,
                    daily_result.daily_2xx_requests_number,
                    daily_result.daily_3xx_requests_number,
                    daily_result.daily_4xx_requests_number,
                    daily_result.daily_5xx_requests_number,
                    daily_result.daily_error_rate * 100.0
                );
            }
        }

        println!("\n{}", "=".repeat(50));
    }
}