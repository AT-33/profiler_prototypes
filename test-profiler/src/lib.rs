use std::time::Instant;
use std::fmt::{Display, Formatter, Result};

#[derive(Copy, Clone)]
pub enum EventType {
    DurationBegin,
    DurationEnd,
}

pub static mut PROFILER_DATA: Vec<EventDesc> = Vec::new();
pub static mut EVENTS: Vec<(u128, &'static str, EventType)> = Vec::new();

pub static mut START_TIME: Option<Instant> = None;

impl Display for EventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            EventType::DurationBegin => f.write_str("B"),
            EventType::DurationEnd => f.write_str("E"),
        }
    }
}

pub struct EventDesc<'a> {
    name: &'a str,
    category: &'a str,
    event_type: EventType,
    timestamp: u128,
    process_id: u32,
    thread_id: u64,
    args: Vec<(&'static str, String)>,
}

impl EventDesc<'_> {
    pub fn new(name: &str, timestamp: u128, event_type: EventType) -> EventDesc {
        EventDesc {
            name,
            category: "default",
            event_type,
            timestamp,
            process_id: std::process::id(),
            thread_id: 0,
            args: vec![],
        }
    }
}

impl<'a> Display for EventDesc<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut s = String::new();
        s += "\t{\n";
        s += format!("\t\t\"name\": \"{}\",\n", self.name).as_str();
        s += format!("\t\t\"cat\": \"{}\",\n", self.category).as_str();
        s += format!("\t\t\"ph\": \"{}\",\n", self.event_type).as_str();
        s += format!("\t\t\"ts\": {},\n", self.timestamp).as_str();
        s += format!("\t\t\"pid\": {},\n", self.process_id).as_str();
        s += format!("\t\t\"tid\": {},\n", self.thread_id).as_str();

        s += "\t\t\"args\": {\n";
        for (i, (key, value)) in self.args.iter().enumerate() {
            s += format!("\t\t\t\"{}\": \"{}\"", key, value).as_str();
            s += if i == self.args.len() - 1 { "\n" } else { ",\n" };
        }
        s += "\t\t}\n\t}";

        f.write_str(s.as_str())
    }
}

unsafe fn prepare_json() {
    PROFILER_DATA.clear();
    for &(timestamp, name, event_type) in &EVENTS {
        let start_event = EventDesc::new(name, timestamp, event_type);
        PROFILER_DATA.push(start_event);
    }
}

pub fn get_trace_json() -> String {
    unsafe {
        prepare_json();
        let mut result = String::new();
        result += "[";
        for event in PROFILER_DATA.iter().take(PROFILER_DATA.len() - 1) {
            result += format!("{},", event).as_str();
        }
        if let Some(last) = PROFILER_DATA.last() {
            result += format!("{}", last).as_str();
        }
        result += "]";
        result
    }
}

#[inline]
pub fn init_profiler() {
    unsafe {
        START_TIME = Some(Instant::now());
    }
}

#[inline]
pub fn profile_event(name: &'static str, begin: bool) {
    unsafe {
        let timestamp = START_TIME
            .expect("Profiler must be initialized before calling profile_event()")
            .elapsed()
            .as_micros();
        let event_type = if begin {
            EventType::DurationBegin
        } else {
            EventType::DurationEnd
        };
        EVENTS.push((timestamp, name, event_type));
    }
}
