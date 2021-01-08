#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wp_test_init() {
        unsafe {
            wp_init(WpInitFlags_WP_INIT_ALL);
        }
        assert_eq!(2 + 2, 4);
    }
}