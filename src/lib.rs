/// Returns a hexxy with a given size.
///
/// ```no_run
///  /\    \
/// /  \____\
/// \
///  \____
/// ```
pub fn hexxy(n: usize, underscores: bool) -> String {
    let horizontal = if underscores { "_" } else { "-" };
    let mut s = String::new();
    for i in 0..n {
        s.push_str(&" ".repeat(n - i - 1));
        s.push('/');
        s.push_str(&" ".repeat(i * 2));
        s.push('\\');
        s.push_str(&(if i == n - 1 { horizontal } else { " " }).repeat(n * 2));
        s.push('\\');
        s.push('\n');
    }
    for i in 0..n {
        s.push_str(&" ".repeat(i));
        s.push('\\');
        if i == n - 1 {
            s.push_str(&horizontal.repeat(n * 2));
        } else {
            s.push('\n');
        }
    }
    return s;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hexxy() {
        assert_eq!(
            hexxy(1, false),
            concat! {
            r"/\--\", "\n",
            r"\--"}
        );
        assert_eq!(
            hexxy(1, true),
            concat! {
            r"/\__\", "\n",
            r"\__"}
        );

        assert_eq!(
            hexxy(2, true),
            concat! {
            r" /\    \", "\n",
            r"/  \____\", "\n",
            r"\", "\n",
            r" \____"}
        );
        assert_eq!(
            hexxy(3, true),
            concat! {
            r"  /\      \", "\n",
            r" /  \      \", "\n",
            r"/    \______\", "\n",
            r"\", "\n",
            r" \", "\n",
            r"  \______"}
        );
    }
}
