/// `Calculator` is a structure that represents a simple calculator which can add two numbers.
///
/// # Example
///
/// ```
/// use add::calculator::Calculator;
///
/// let calculator = Calculator::new(1.0, 2.0);
/// assert_eq!(calculator.calc(), 3.0);
/// ``` 
pub struct Calculator {
    addend1: f64,
    addend2: f64,
}

impl Calculator {
    /// Constructs a new `Calculator`.
    ///
    /// # Arguments
    ///
    /// * `addend1` - The first number to add.
    /// * `addend2` - The second number to add.
    ///
    /// # Example
    ///
    /// ```
    /// use add::calculator::Calculator;
    ///
    /// let calculator = Calculator::new(1.0, 2.0);
    /// ```
    pub fn new(addend1: f64, addend2: f64) -> Self {
        Self {
            addend1,
            addend2,
        }
    }

    /// Adds the two numbers and returns the result.
    ///
    /// # Example
    ///
    /// ```
    /// use add::calculator::Calculator;
    ///
    /// let calculator = Calculator::new(1.0, 2.0);
    /// assert_eq!(calculator.calc(), 3.0);
    /// ``` 
    pub fn calc(&self) -> f64 {
        self.addend1 + self.addend2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculator_new() {
        let calculator = Calculator::new(1.0, 2.0);
        assert_eq!(calculator.addend1, 1.0);
        assert_eq!(calculator.addend2, 2.0);
    }

    #[test]
    fn test_calc() {
        let calculator = Calculator::new(1.0, 2.0);
        assert_eq!(calculator.calc(), 3.0);

        let calculator = Calculator::new(3.0, 3.0);
        assert_eq!(calculator.calc(), 6.0);
    }
}