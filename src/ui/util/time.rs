use chrono::{format::{DelayedFormat, StrftimeItems}, DateTime, Local, NaiveDateTime, TimeZone};

pub fn render_local(datetime: NaiveDateTime) -> String {
    format_local_date_time(convert_to_local(datetime)).to_string()
}

pub fn convert_to_local(datetime: NaiveDateTime) -> DateTime<Local> {
    let local = chrono::Local::now();
    let local_tz = local.timezone();
    local_tz.from_utc_datetime(&datetime)
}

/// Accepts a string represting the current time locally, and will convert it to a naive date time suitable for storing.
/// 
/// his should be isomorphic with render_local
pub fn parse_str_as_naive_to_utc(input: &str) -> Result<NaiveDateTime, Box<dyn std::error::Error>> {
    let local = chrono::Local::now();
    let local_tz = local.timezone();
    // If this is parsed, then it needs to be offset in _reverse_.
    let local = NaiveDateTime::parse_from_str(input, MINUTE_RESOLUTION_FMT)?;
    
    Ok(local_tz.from_local_datetime(&local).map(|x| x.naive_utc()).unwrap())
}

pub fn format_local_date_time(local_datetime: DateTime<Local>) -> DelayedFormat<StrftimeItems<'static>> {
    local_datetime.format(MINUTE_RESOLUTION_FMT)
}
pub const MINUTE_RESOLUTION_FMT: &'static str = "%Y-%m-%d %H:%M";

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn local_is_isomorphic() {
        let input = "2024-02-04 17:52";
        let parsed = dbg!(parse_str_as_naive_to_utc(input).expect("should parse 1"));
        let rendered = render_local(parsed);
        assert_eq!(input, rendered, "should be equal")

    }
}