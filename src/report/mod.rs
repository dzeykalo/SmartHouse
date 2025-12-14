pub trait Reportable {
    fn generate_report(&self) -> String;
}

pub trait Report {
    fn report(&self) -> String;
}

pub struct Reporter;

impl Reporter {
    pub fn new() -> Self {
        println!("=== Printing report ===");
        Self
    }
    
    pub fn add<T: Report>(self, item: &T) -> Self {
        println!("{}", item.report());
        self
    }

    pub fn report(self) {
        println!("=== End of report ===");
    }
}
