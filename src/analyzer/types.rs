#[derive(Debug)]
pub struct AnalyzerResult {
    pub daily_results: Vec<DailyAnalyzerResult>,
    pub total_requests_number: u32,
    pub total_2xx_requests_number: u32,
    pub total_3xx_requests_number: u32,
    pub total_4xx_requests_number: u32,
    pub total_5xx_requests_number: u32,
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
            self.total_4xx_requests_number as f32 / self.total_requests_number as f32;
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

#[derive(Debug)]
pub struct DailyAnalyzerResult {
    pub date: String,
    pub daily_requests_number: u32,
    pub daily_2xx_requests_number: u32,
    pub daily_3xx_requests_number: u32,
    pub daily_4xx_requests_number: u32,
    pub daily_5xx_requests_number: u32,
    daily_error_rate: f32,
}

impl DailyAnalyzerResult {
    pub fn new(date: String) -> Self {
        Self {
            date,
            daily_requests_number: 0,
            daily_2xx_requests_number: 0,
            daily_3xx_requests_number: 0,
            daily_4xx_requests_number: 0,
            daily_5xx_requests_number: 0,
            daily_error_rate: 0.0,
        }
    }

    pub fn add_daily_requests_number(&mut self, requests_number: u32) {
        self.daily_requests_number += requests_number;
    }

    pub fn add_daily_2xx_requests_number(&mut self, requests_number: u32) {
        self.daily_2xx_requests_number += requests_number;
    }

    pub fn add_daily_3xx_requests_number(&mut self, requests_number: u32) {
        self.daily_3xx_requests_number += requests_number;
    }

    pub fn add_daily_4xx_requests_number(&mut self, requests_number: u32) {
        self.daily_4xx_requests_number += requests_number;
    }

    pub fn add_daily_5xx_requests_number(&mut self, requests_number: u32) {
        self.daily_5xx_requests_number += requests_number;
    }

    pub fn calculate_daily_error_rate(&mut self) {
        self.daily_error_rate =
            self.daily_4xx_requests_number as f32 / self.daily_requests_number as f32;
    }
}
