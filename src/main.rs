use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, TimeZone};
use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() -> std::io::Result<()> {
    #[derive(Deserialize, Debug, Serialize)]
    struct Event{
        pub timestamp: String,
        pub event_type: String,
        pub channel: Option<String>,
        pub channel_id: Option<String>,
        pub voice_state_count:  Option<String>
    }
// tofu 889670458096623626
// lol 1178356375697510561
// 907194413489475584
// kor 1278397091558719588
    let file = File::open("input.json")?;
    let output_file = File::create("output.json")?;
    let reader = BufReader::new(file);

    let start_time = Utc.ymd(2024, 9, 1).and_hms(11, 0, 0);
    let end_time = Utc.ymd(2024, 9, 1).and_hms(14, 0, 0);
    
    let mut json_array = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if let Ok(event) = serde_json::from_str::<Event>(&line) {
                let is_heartbeat = event.event_type.contains("heartbeat");
                let timestamp_str = &event.timestamp.trim_matches('"');
                if let Ok(timestamp) = DateTime::parse_from_rfc3339(timestamp_str) {
                    if timestamp >= start_time && timestamp <= end_time && !is_heartbeat{
                        json_array.push(event);
                    }
                }
            
        }
    }
    json_array.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    serde_json::to_writer_pretty(output_file, &json_array)?;
    Ok(())
}
