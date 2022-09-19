#[macro_export]
macro_rules! error {
    (@reason $r:expr) => {
        eprintln!("Error: {}", $r)
    };
    (@error $e:expr) => {
        return Err($e.to_string())
    };
    (r: $r:expr) => {
        {
            error!(@reason $r);

            error!(@error "Something went wrong. Please try again later!")
        }
    };
    (e: $e:expr $(, r: $r:expr )?) => {
        $( error!(@reason $r); )?

        error!(@error $e)
    }
}
