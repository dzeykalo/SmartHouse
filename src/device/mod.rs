use crate::transport::Transport;

pub trait Device {
    fn new(transport: Box<dyn Transport + Send>) -> Self
    where
        Self: Sized;
    fn get_value(&self) -> f64;

    fn get_name(&self) -> String;
    
    fn get_state(&self) -> String;

    fn on(&mut self);

    fn off(&mut self);
}
