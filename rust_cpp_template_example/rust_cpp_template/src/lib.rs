mod cxxbridge;

pub use cxxbridge::ffi::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_operations() {
        let op = Operations::new(10);
        assert_eq!(op.add(5), 15);
        assert_eq!(op.subtract(3), 7);
    }
}
