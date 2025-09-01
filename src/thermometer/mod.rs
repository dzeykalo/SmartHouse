use crate::device::Device;

#[derive(Debug)]
pub struct Thermometer {
    temperature: f64,
}

impl Device for Thermometer {
    fn new(t: f64) -> Self {
        Self { temperature: t }
    }

    fn is_on(&self) -> bool {
        true
    }

    fn get_value(&self) -> f64 {
        self.temperature
    }

    fn get_name(&self) -> String {
        String::from("Thermometer")
    }

    fn on(&mut self) {}

    fn off(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermometer() {
        let thermometer = Thermometer::new(23.0);
        assert_eq!(thermometer.get_name(), "Thermometer");
        assert_eq!(thermometer.get_value(), 23.0);
        assert_eq!(thermometer.is_on(), true);
    }

    #[test]
    fn test_thermometer_turn_on_off() {
        let mut thermometer = Thermometer::new(25.0);
        thermometer.on();
        assert_eq!(thermometer.is_on(), true);

        thermometer.off();
        assert_eq!(thermometer.is_on(), true);
    }
}
