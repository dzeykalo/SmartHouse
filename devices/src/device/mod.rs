pub trait Device: Send + Sync {
    fn new(value: f64) -> Self
    where
        Self: Sized;
    fn get_value(&self) -> f64;

    fn get_name(&self) -> String;

    fn get_state(&self) -> String;

    fn on(&mut self);

    fn off(&mut self);
}
