pub trait Reportable {
    fn generate_report(&self) -> String;
}

pub trait Report {
    fn report(&self) -> String;
}

#[derive(Default)]
pub struct Reporter {
    entries: Vec<String>,
}

impl Reporter {
    pub fn new() -> Self {
        println!("{0} Printing report {0}", "=".repeat(30));
        Self {
            entries: Vec::new(),
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add<T: Report>(mut self, item: &T) -> Self {
        self.entries.push(item.report());
        self
    }

    pub fn report(self) {
        for entry in &self.entries {
            println!("{}", entry);
        }
        println!("{0} End  of  report {0}", "=".repeat(30));
    }
}
