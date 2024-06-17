pub struct Calculator {
    addend1: f64,
    addend2: f64,
}

impl Calculator {
    pub fn new(addend1: f64, addend2: f64) -> Self {
        Self {
            addend1,
            addend2,
        }
    }

    pub fn calc(&self) -> f64 {
        self.addend1 + self.addend2
    }
}