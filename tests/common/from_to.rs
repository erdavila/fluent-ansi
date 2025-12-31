#[cfg(test)]
macro_rules! assert_from_to {
    ($to_method:ident, $to_type:ty; $from_value:expr, $to_value:expr) => {{
        let result = $from_value.$to_method();
        assert_eq!(result, $to_value);

        let result: $to_type = From::from($from_value);
        assert_eq!(result, $to_value);
    }};
}
#[cfg(test)]
pub(crate) use assert_from_to;
