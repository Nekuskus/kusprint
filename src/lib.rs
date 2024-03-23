#[macro_export]
macro_rules! expr_print {
    // The pattern for a single `eval`
    ($e:expr) => {
        {
            print!("{}", $e);
        }
    };

    // Decompose multiple `eval`s recursively
    ($e:expr, $($es:expr),+) => {{
        expr_print! { $e }
        expr_print! { $($es),+ }
    }};
}