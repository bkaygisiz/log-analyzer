

pub struct DailyAnalyzerResult {
    pub date: String,
    pub daily_requests_number: u32,
    pub daily_2xx_requests_number: u32,
    pub daily_3xx_requests_number: u32,
    pub daily_4xx_requests_number: u32,
    pub daily_5xx_requests_number: u32,
    pub daily_error_rate: f32,
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
            (self.daily_4xx_requests_number as f32 + self.daily_5xx_requests_number as f32) / self.daily_requests_number as f32;
    }

    pub fn update_self(&mut self, status_code: &str) {
        self.add_daily_requests_number(1);
        if status_code.starts_with("2") {
            self.add_daily_2xx_requests_number(1);
        }
        if status_code.starts_with("3") {
            self.add_daily_3xx_requests_number(1);
        }
        if status_code.starts_with("4") {
            self.add_daily_4xx_requests_number(1);
        }
        if status_code.starts_with("5") {
            self.add_daily_5xx_requests_number(1);
        }
        self.calculate_daily_error_rate();
    }
}
