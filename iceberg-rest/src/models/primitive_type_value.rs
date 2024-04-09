// // ToDo: Do not use for API
// #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
// #[serde(tag = "type", rename_all = "kebab-case")]
// pub enum PrimitiveTypeValue {
//     BooleanTypeValue(bool),
//     IntegerTypeValue(i32),
//     LongTypeValue(i64),
//     FloatTypeValue(f32),
//     DoubleTypeValue(f64),
//     /// Decimal type values are serialized as strings. Decimals with a positive scale serialize as numeric plain text, while decimals with a negative scale use scientific notation and the exponent will be equal to the negated scale. For instance, a decimal with a positive scale is '123.4500', with zero scale is '2', and with a negative scale is '2E+20'
//     DecimalTypeValue(String),
//     StringTypeValue(String),
//     /// UUID type values are serialized as a 36-character lowercase string in standard UUID format as specified by RFC-4122
//     UuidTypeValue(uuid::Uuid),
//     /// Date type values follow the 'YYYY-MM-DD' ISO-8601 standard date format
//     DateTypeValue(String),
//     /// Time type values follow the 'HH:MM:SS.ssssss' ISO-8601 format with microsecond precision
//     TimeTypeValue(String),
//     /// Timestamp type values follow the 'YYYY-MM-DDTHH:MM:SS.ssssss' ISO-8601 format with microsecond precision
//     TimestampTypeValue(String),
//     /// TimestampTz type values follow the 'YYYY-MM-DDTHH:MM:SS.ssssss+00:00' ISO-8601 format with microsecond precision, and a timezone offset (+00:00 for UTC)
//     TimestampTzTypeValue(String),
//     /// Timestamp_ns type values follow the 'YYYY-MM-DDTHH:MM:SS.sssssssss' ISO-8601 format with nanosecond precision
//     TimestampNanoTypeValue(String),
//     /// Timestamp_ns type values follow the 'YYYY-MM-DDTHH:MM:SS.sssssssss+00:00' ISO-8601 format with nanosecond precision, and a timezone offset (+00:00 for UTC)
//     TimestampTzNanoTypeValue(String),
//     /// Fixed length type values are stored and serialized as an uppercase hexadecimal string preserving the fixed length
//     FixedTypeValue(String),
//     /// Binary type values are stored and serialized as an uppercase hexadecimal string
//     BinaryTypeValue(String),
// }
