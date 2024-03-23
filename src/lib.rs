#[macro_export]
macro_rules! expr_print {
    ($e:expr) => {
        {
            print!("{}", $e);
        }
    };

    ($e:expr, $($es:expr),+) => {{
        expr_print! { $e }
        expr_print! { $($es),+ }
    }};
}

#[macro_export]
macro_rules! expr_print_delim {
    ($delim:expr, $e:expr, $($es:expr),+) => {{
        expr_print! { $e }
        expr_print! { $($delim, $es),+ }
    }};
}

