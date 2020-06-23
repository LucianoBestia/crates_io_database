// qvs20_error_mod

// unwrap_or_error

#[macro_export]
macro_rules! unwrap_result_or_error(
    ($result:expr, $err:expr) => (
        match $result{
            Ok(p) => p,
            Err(e) =>  return $err,
        };
    );
);

#[macro_export]
macro_rules! unwrap_option_or_error(
    ($option:expr, $err:expr) => (
        match $option{
            Some(p) => p,
            None =>  return $err,
        };
    );
);
