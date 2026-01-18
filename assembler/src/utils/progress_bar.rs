use std::fmt::Display;

pub struct ProgressBar {
    pub total: usize,
    pub current: usize,
    width: usize,
}

impl ProgressBar {
    pub fn new(total: usize, width: usize) -> ProgressBar {
        ProgressBar {
            current: 0,
            total: total,
            width: width,
        }
    }
    pub fn advance(&mut self) {
        self.current += 1;
    }
    pub fn advance_by(&mut self, value: usize) {
        self.current += value;
    }
    pub fn set_current(&mut self, value: usize) {
        self.current += value;
    }
    pub fn display(&self) -> String {
        return self.display_extra("", "");
    }

    pub fn display_extra(&self, after: &str, before: &str) -> String {
        let mut display_string = String::new();
        display_string.push(' ');
        display_string.push_str(before);
        display_string.push('[');
        let percent: f64 = (self.current as f64) / (self.total as f64);

        let full: usize = (percent * (self.width as f64)) as usize;
        for i in 0..self.width {
            let ch = if i < full { '#' } else { ' ' };
            display_string.push(ch);
        }
        display_string.push(']');
        display_string.push_str(&format!(" {}/{}", self.current, self.total));
        display_string.push_str(after);

        display_string.push('\r');
        return display_string;
    }
}
impl Display for ProgressBar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display())
    }
}
