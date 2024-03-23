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

#[macro_export]
macro_rules! expr_println {
    ($e:expr) => {
        {
            println!("{}", $e);
        }
    };

    ($e:expr, $($es:expr),+) => {{
        expr_print! { $e }
        expr_print! { $($es),+ }
        println!()
    }};
}

#[macro_export]
macro_rules! expr_println_delim {
    ($delim:expr, $e:expr, $($es:expr),+) => {{
        expr_print! { $e }
        expr_print! { $($delim, $es),+ }
        println!()
    }};
}

