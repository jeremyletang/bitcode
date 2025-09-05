use crate::convert::{impl_convert, ConvertFrom};
use crate::int::ranged_int;
use chrono::{DateTime, Utc};

ranged_int!(Nanoseconds, i64, 0, i64::MAX);

type ChronoDateTimeEncode = i64;
type ChronoDateTimeDecode = Nanoseconds;

impl_convert!(DateTime<Utc>, ChronoDateTimeEncode, ChronoDateTimeDecode);

impl ConvertFrom<&DateTime<Utc>> for ChronoDateTimeEncode {
    #[inline(always)]
    fn convert_from(value: &DateTime<Utc>) -> Self {
        value.timestamp_nanos_opt().unwrap()
    }
}

impl ConvertFrom<ChronoDateTimeDecode> for DateTime<Utc> {
    #[inline(always)]
    fn convert_from(value: ChronoDateTimeDecode) -> Self {
        DateTime::<Utc>::from_timestamp_nanos(value.0.into_inner())
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, SecondsFormat, Utc};

    #[test]
    fn test() {
        assert_eq!(
            crate::decode::<DateTime<Utc>>(&crate::encode(&DateTime::<Utc>::from_timestamp_nanos(
                1757083455517000000
            )))
            .unwrap()
            .to_rfc3339_opts(SecondsFormat::Millis, true),
            "2025-09-05T14:44:15.517Z",
        );
    }
}
