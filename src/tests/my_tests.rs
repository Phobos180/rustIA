pub fn add_five(num: u32) -> u32 {
    num + 5
}

#[cfg(test)]
mod tests {
    // Import tests from the main test file
    use super::*;

    #[test]
    fn add_five_test() {
        let x: u32 = 100;
        let y: u32 = add_five(x);
        assert_eq!(y, 105);
    }
}