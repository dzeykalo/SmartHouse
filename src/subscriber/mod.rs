
pub trait Subscriber {
    fn on_event(&mut self);
}

#[derive(Default)]
pub struct LoggingSubscriber;

impl Subscriber for LoggingSubscriber {
    fn on_event(&mut self) {
        println!("Device added")
    }
}