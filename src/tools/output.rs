use notify_debouncer_full::DebouncedEvent;

pub fn print_event(event: &DebouncedEvent) {
    println!("|{:?}|{:?}|", event.paths, event.kind);
}

pub fn print_events(events: Vec<DebouncedEvent>) {
    events.iter().for_each(|event| print_event(&event));
}

pub fn print_sep_line() {
    println!("------------------------------------------------------------------------------------------------------");
}
