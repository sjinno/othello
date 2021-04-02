#[macro_export]
macro_rules! flip {
    ($val:expr) => {{
        format!("{}", $val)
    }};
}
