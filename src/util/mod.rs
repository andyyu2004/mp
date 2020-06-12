pub fn join_display<T>(xs: &[T], sep: &str) -> String
where
    T: std::fmt::Display,
{
    xs.iter()
        .map(|x| format!("{}", x))
        .collect::<Vec<_>>()
        .join(sep)
}
