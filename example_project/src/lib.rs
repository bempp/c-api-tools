pub use c_api_tools::cfuncs;
pub use c_api_tools::eval_with_concrete_type;

#[cfuncs(name = "my_wrapper", create, free, unwrap)]
pub struct MyWrapper;
