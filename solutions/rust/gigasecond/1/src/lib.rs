use time::{PrimitiveDateTime as DateTime, Duration};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    //todo!("What time is a gigasecond later than {start}");
    let giga_second = Duration::seconds(1_000_000_000);
    start + giga_second
}
