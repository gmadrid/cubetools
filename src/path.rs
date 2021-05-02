pub struct Path {
    path_string: String,
}

impl Path {
    pub fn new() -> Self {
        Path {
            path_string: Default::default(),
        }
    }

    pub fn output(&self) -> &str {
        &self.path_string.trim()
    }

    pub fn M(mut self, x: i32, y: i32) -> Self {
        self.add(&format!("M {} {} ", x, y));
        self
    }

    pub fn h(mut self, x: i32) -> Self {
        self.add(&format!("h {} ", x));
        self
    }

    pub fn v(mut self, y: i32) -> Self {
        self.add(&format!("v {} ", y));
        self
    }

    fn add(&mut self, str: &str) {
        self.path_string.push_str(str);
    }
}
