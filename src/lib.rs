pub use auto::builders::*;
pub use auto::traits::*;
pub use auto::*;
pub use char_attributes::CharAttributes;

mod auto;
mod char_attributes;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
