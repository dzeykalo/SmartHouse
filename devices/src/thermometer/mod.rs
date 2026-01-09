use crate::device::Device;

#[derive(Debug, Default)]
pub struct Thermometer {
    temperature: f64,
    state: String,
}

impl Device for Thermometer {
    fn new(t: f64) -> Self {
        Self {
            temperature: t,
            state: "OFF".to_string(),
        }
    }

    fn get_value(&self) -> f64 {
        match self.get_state().as_str() {
            "ON" => self.temperature,
            _ => 0.0,
        }
    }

    fn get_name(&self) -> String {
        String::from("Thermometer")
    }

    fn get_state(&self) -> String {
        self.state.clone()
    }

    fn on(&mut self) {
        self.state = "ON".to_string();
    }

    fn off(&mut self) {
        self.state = "OFF".to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermometer_get_value() {
        let thermometer = Thermometer::new(0.0f64);
        assert_eq!(thermometer.get_value(), 25.0);
    }

    #[test]
    fn test_thermometer_off() {
        let mut thermometer = Thermometer::new(0.0f64);
        thermometer.off();
        assert_eq!(thermometer.get_value(), 0.0);
    }
}
