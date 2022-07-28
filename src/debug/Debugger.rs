pub struct Debugger {
    debug: bool,
}

impl Debugger {
    fn new() -> Debugger {
        Debugger { debug: false }
    }

    pub fn log(&self, message: &str) {
        if self.debug {
            println!("{}", message);
        }
    }

    pub fn log_error(&self, message: &str) {
        if self.debug {
            eprintln!("{}", message);
        }
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }
}
