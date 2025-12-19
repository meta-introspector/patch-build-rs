#[macro_export]
macro_rules! fixme {
    ($($tt:tt)*) => {
        compile_error!(concat!("FIXME: ", stringify!($($tt)*)));
    };
}
