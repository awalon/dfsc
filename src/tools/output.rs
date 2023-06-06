use notify_debouncer_full::DebouncedEvent;
use std::fs::File;
use std::io::Write;
use std::ops::Add;

pub struct Output {
    file: Option<File>,
}

impl Output {
    pub fn new() -> Self {
        Self { file: None }
    }

    pub fn set_file(&mut self, file: String) {
        match File::create(file) {
            Ok(out) => self.file = Some(out),
            Err(e) => {
                eprintln!("+++ cannot create/open log file: {}", e.to_string());
                self.file = None
            }
        }
    }

    pub fn println(&self, line: String) {
        print!("{}\n", line);

        self.file
            .as_ref()
            .and_then(|mut out| out.write(line.add("\n").as_bytes()).ok());
    }

    pub fn print_header(&self) {
        let line = format!("|object|kind|");
        self.println(line)
    }

    pub fn print_event(&self, event: &DebouncedEvent) {
        let line = format!("|{:?}|{:?}|", event.paths, event.kind);
        self.println(line)
    }

    pub fn print_events(&self, events: Vec<DebouncedEvent>) {
        events.iter().for_each(|event| self.print_event(&event));
    }

    pub fn print_sep_line(&self) {
        let line = format!("------------------------------------------------------------------------------------------------------");
        self.println(line)
    }
}
