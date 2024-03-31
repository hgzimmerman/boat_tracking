use chrono::{format::{DelayedFormat, StrftimeItems}, DateTime, Local, NaiveDateTime, TimeZone};

pub fn render_local(datetime: NaiveDateTime) -> String {
    format_local_date_time(convert_to_local(datetime)).to_string()
}

pub fn convert_to_local(datetime: NaiveDateTime) -> DateTime<Local> {
    let local = chrono::Local::now();
    let local_tz = local.timezone();
    local_tz.from_utc_datetime(&datetime)
}

pub fn format_local_date_time(local_datetime: DateTime<Local>) -> DelayedFormat<StrftimeItems<'static>> {
    local_datetime.format(MINUTE_RESOLUTION_FMT)
}
const MINUTE_RESOLUTION_FMT: &'static str = "%Y-%m-%d %H:%M";