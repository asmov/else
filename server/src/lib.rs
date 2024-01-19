pub mod model;

#[macro_export]
macro_rules! s {
    ($s:literal) => { String::from($s) };
    ($s:ident) => { $s.to_string() };
}
