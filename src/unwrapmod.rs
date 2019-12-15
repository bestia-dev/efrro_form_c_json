// unwrapmod.rs
//! unwrap utils

#[inline]
pub fn required<T>(o: Option<T>) -> T {
    use std::process;
    match o {
        Some(t) => t,
        None => process::abort(),
    }
}

#[inline]
pub fn unwrap_option_abort<T>(o: Option<T>) -> T {
    use std::process;
    match o {
        Some(t) => t,
        None => process::abort(),
    }
}

#[inline]
pub fn unwrap_result_abort<T, E>(o: Result<T, E>) -> T {
    use std::process;
    match o {
        Ok(t) => t,
        Err(_e) => process::abort(),
    }
}
