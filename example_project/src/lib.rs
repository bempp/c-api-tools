pub use c_api_tools::cfuncs;
pub use c_api_tools::eval_with_concrete_type;

#[cfuncs(name = "my_wrapper", create)]
pub struct MyWrapper;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_c_wrapper() {
        hello_world();
    }
}
