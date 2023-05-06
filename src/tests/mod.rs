#[cfg(test)]
mod tests {

    use crate::dynamix::*;

    fn try_run_script(path: &str) {
        match run_file(path) {
            Ok((result, error)) => {
                assert!(result as u32 == 0 && error.is_empty())
            }
            Err(..) => panic!("Failed to open file: /{path}"),
        }
    }

    #[test]
    fn script() {
        try_run_script("examples/script.dyn");
    }
}
