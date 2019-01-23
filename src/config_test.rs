use super::*;

#[cfg(test)]
mod find_config_file_tests {
    use super::*;

    #[test]
    fn returns_error_on_io_error() {
        let exp_error = "Fatal error encountered while looking for existing config";
        let file_exists = |_path: &str| { Err(()) };
        let act = find_config_file("", file_exists);
        assert_eq!(act, Err(String::from(exp_error)));
    }
}


