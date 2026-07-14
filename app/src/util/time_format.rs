use std::ops::Sub;
use std::time::Duration as StdDuration;

use chrono::{DateTime, Datelike, Duration, Local, Utc};

use crate::i18n::t;

/// 日期时间显示格式。格式模板由 locale 资源提供，以便日期结构和时间制式一起本地化。
#[derive(Clone, Copy)]
pub enum LocalizedDateTimeFormat {
    Date,
    MonthDay,
    MonthDayYear,
    MonthDayTime,
    WeekdayDateTime,
    ShortDateTime,
    ShortNumericDateTime,
}

impl LocalizedDateTimeFormat {
    const fn translation_key(self) -> &'static str {
        match self {
            Self::Date => "common_extra.time.formats.date",
            Self::MonthDay => "common_extra.time.formats.month_day",
            Self::MonthDayYear => "common_extra.time.formats.month_day_year",
            Self::MonthDayTime => "common_extra.time.formats.month_day_time",
            Self::WeekdayDateTime => "common_extra.time.formats.weekday_date_time",
            Self::ShortDateTime => "common_extra.time.formats.short_date_time",
            Self::ShortNumericDateTime => "common_extra.time.formats.short_numeric_date_time",
        }
    }
}

/// 按当前界面语言格式化本地时区的日期时间。
pub fn format_localized_datetime(
    datetime: DateTime<Local>,
    format: LocalizedDateTimeFormat,
) -> String {
    datetime
        .format(t!(format.translation_key()).as_ref())
        .to_string()
}

/// 按当前界面语言格式化日期范围。
pub fn format_localized_date_range(start: DateTime<Local>, end: DateTime<Local>) -> String {
    let (start_format, end_format) = if start.year() == end.year() {
        (
            LocalizedDateTimeFormat::MonthDay,
            LocalizedDateTimeFormat::MonthDayYear,
        )
    } else {
        (
            LocalizedDateTimeFormat::MonthDayYear,
            LocalizedDateTimeFormat::MonthDayYear,
        )
    };
    t!(
        "common_extra.time.formats.range",
        start = format_localized_datetime(start, start_format),
        end = format_localized_datetime(end, end_format)
    )
    .to_string()
}

// Some conversion ratios for time units.
const SEC_TO_MS: f64 = 1000.;
const MIN_TO_MS: f64 = 60. * SEC_TO_MS;
const HOUR_TO_MS: f64 = 60. * MIN_TO_MS;
const DAY_TO_MS: f64 = 24. * HOUR_TO_MS;
const WEEK_TO_MS: f64 = 7. * DAY_TO_MS;
const MONTH_TO_MS: f64 = 30.44 * DAY_TO_MS;
const YEAR_TO_MS: f64 = 365.25 * DAY_TO_MS;

enum ApproxDurationUnit {
    Year,
    Month,
    Week,
    Day,
    Hour,
}

/// Subtract a given DateTime from now and format the duration is a concise, approximated,
/// human-readable form. e.g. "just now"
pub fn format_approx_duration_from_now(datetime: DateTime<Local>) -> String {
    human_readable_approx_duration(Local::now().sub(datetime), false)
}

/// Subtract a given DateTime from now and format the duration is a concise, approximated,
/// human-readable form. e.g. "Just now"
pub fn format_approx_duration_from_now_sentence_case(datetime: DateTime<Local>) -> String {
    human_readable_approx_duration(Local::now().sub(datetime), true)
}

/// Takes a time in UTC and determines roughly how long ago it occurred.
pub fn format_approx_duration_from_now_utc(datetime: DateTime<Utc>) -> String {
    human_readable_approx_duration(Utc::now().sub(datetime), false)
}

/// Format a duration into a human-readable string, e.g. "3.14 sec".
/// Compared to [`human_readable_approx_duration`], this method is for higher-precision, smaller
/// values.
pub fn human_readable_precise_duration(duration: Duration) -> String {
    let ms = duration.num_milliseconds() as f64;
    let weeks = ms / WEEK_TO_MS;
    if weeks >= 1. {
        return t!("common_extra.time.precise.over_one_week").to_string();
    }
    let days = ms / DAY_TO_MS;
    if days >= 1. {
        return t!(
            "common_extra.time.precise.days",
            count = format_sigfigs(days, 3)
        )
        .to_string();
    }
    let hours = ms / HOUR_TO_MS;
    if hours >= 1. {
        return t!(
            "common_extra.time.precise.hours",
            count = format_sigfigs(hours, 3)
        )
        .to_string();
    }
    let minutes = ms / MIN_TO_MS;
    if minutes >= 1. {
        return t!(
            "common_extra.time.precise.minutes",
            count = format_sigfigs(minutes, 3)
        )
        .to_string();
    }
    let seconds = ms / SEC_TO_MS;
    if seconds >= 1. {
        return t!(
            "common_extra.time.precise.seconds",
            count = format_sigfigs(seconds, 3)
        )
        .to_string();
    }
    t!(
        "common_extra.time.precise.milliseconds",
        count = duration.num_milliseconds()
    )
    .to_string()
}

fn format_sigfigs(num: f64, sigfigs: usize) -> String {
    let a = num.abs();
    let precision = if a > 1. {
        let n = (1. + a.log10().floor()) as usize;
        sigfigs.saturating_sub(n)
    } else if a > 0. {
        let n = -(1. + a.log10().floor()) as usize;
        sigfigs + n
    } else {
        0
    };
    format!("{num:.precision$}")
}

/// Format an approximated duration into a human-readable string, e.g. "2 days ago".
/// Precision is limited to the most significant unit, i.e. 2 days and _n_ hours always displays
/// simply as "2 days ago".
pub fn human_readable_approx_duration(duration: Duration, sentence_case: bool) -> String {
    let ms = duration.num_milliseconds() as f64;
    let years = ms / YEAR_TO_MS;
    if years >= 1. {
        return truncated_quantity_with_unit(years, ApproxDurationUnit::Year);
    }
    let months = ms / MONTH_TO_MS;
    if months >= 1. {
        return truncated_quantity_with_unit(months, ApproxDurationUnit::Month);
    }
    let weeks = ms / WEEK_TO_MS;
    if weeks >= 1. {
        return truncated_quantity_with_unit(weeks, ApproxDurationUnit::Week);
    }
    let days = ms / DAY_TO_MS;
    if days >= 1. {
        return truncated_quantity_with_unit(days, ApproxDurationUnit::Day);
    }
    let hours = ms / HOUR_TO_MS;
    if hours >= 1. {
        return truncated_quantity_with_unit(hours, ApproxDurationUnit::Hour);
    }
    // Minutes and seconds are both abbreviated, so skip pluralization.
    let minutes = ms / MIN_TO_MS;
    if minutes >= 1. {
        return t!("common_extra.time.relative.minutes", count = minutes as i32).to_string();
    }
    if sentence_case {
        t!("common_extra.time.relative.just_now_sentence").to_string()
    } else {
        t!("common_extra.time.relative.just_now").to_string()
    }
}

/// Provided a value and a unit, this will format the quantity as an integer number with the
/// unit pluralized if the value is not 1.
fn truncated_quantity_with_unit(num: f64, unit: ApproxDurationUnit) -> String {
    let truncated_int = num as i32;
    match (unit, truncated_int == 1) {
        (ApproxDurationUnit::Year, true) => {
            t!("common_extra.time.relative.year_one", count = truncated_int).to_string()
        }
        (ApproxDurationUnit::Year, false) => {
            t!("common_extra.time.relative.years", count = truncated_int).to_string()
        }
        (ApproxDurationUnit::Month, true) => t!(
            "common_extra.time.relative.month_one",
            count = truncated_int
        )
        .to_string(),
        (ApproxDurationUnit::Month, false) => {
            t!("common_extra.time.relative.months", count = truncated_int).to_string()
        }
        (ApproxDurationUnit::Week, true) => {
            t!("common_extra.time.relative.week_one", count = truncated_int).to_string()
        }
        (ApproxDurationUnit::Week, false) => {
            t!("common_extra.time.relative.weeks", count = truncated_int).to_string()
        }
        (ApproxDurationUnit::Day, true) => {
            t!("common_extra.time.relative.day_one", count = truncated_int).to_string()
        }
        (ApproxDurationUnit::Day, false) => {
            t!("common_extra.time.relative.days", count = truncated_int).to_string()
        }
        (ApproxDurationUnit::Hour, true) => {
            t!("common_extra.time.relative.hour_one", count = truncated_int).to_string()
        }
        (ApproxDurationUnit::Hour, false) => {
            t!("common_extra.time.relative.hours", count = truncated_int).to_string()
        }
    }
}

/// Formats elapsed time as a whole-seconds string with proper singular/plural
/// (e.g. "1 second", "15 seconds").
pub fn format_elapsed_seconds(elapsed: StdDuration) -> String {
    let total_seconds = elapsed.as_secs();
    if total_seconds == 1 {
        t!(
            "common_extra.time.elapsed.second_one",
            count = total_seconds
        )
        .to_string()
    } else {
        t!("common_extra.time.elapsed.seconds", count = total_seconds).to_string()
    }
}

/// Formats a monotonic `Instant` as a human-readable relative timestamp.
/// (Uses `Instant` rather than wall-clock `DateTime` for elapsed-time display.)
pub fn format_elapsed_since(created_at: instant::Instant) -> String {
    let secs = created_at.elapsed().as_secs();

    if secs < 60 {
        t!("common_extra.time.relative.just_now_sentence").to_string()
    } else if secs < 3600 {
        let mins = secs / 60;
        if mins == 1 {
            t!("common_extra.time.relative.minute_one", count = mins).to_string()
        } else {
            t!("common_extra.time.relative.minutes_full", count = mins).to_string()
        }
    } else if secs < 86400 {
        let hours = secs / 3600;
        if hours == 1 {
            t!("common_extra.time.relative.hour_one", count = hours).to_string()
        } else {
            t!("common_extra.time.relative.hours", count = hours).to_string()
        }
    } else {
        let days = secs / 86400;
        if days == 1 {
            t!("common_extra.time.relative.day_one", count = days).to_string()
        } else {
            t!("common_extra.time.relative.days", count = days).to_string()
        }
    }
}

#[cfg(test)]
#[path = "time_format_tests.rs"]
mod tests;
