pub trait Subscriber {
    fn on_event(&mut self);
}

impl<F> Subscriber for F
where
    F: Fn(),
{
    fn on_event(&mut self) {
        self()
    }
}
#[derive(Default)]
pub struct LoggingSubscriber;

impl Subscriber for LoggingSubscriber {
    fn on_event(&mut self) {
        println!("Device added")
    }
}
