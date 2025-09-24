use crate::transport::Transport;

pub trait Device {
    fn new(transport: Box<dyn Transport>, w: f64) -> Self
    where
        Self: Sized;
    fn is_on(&self) -> bool;
    fn get_value(&self) -> f64;

    fn get_name(&self) -> String;

    fn on(&mut self);

    fn off(&mut self);
}
