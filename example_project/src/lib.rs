//! Example project
#![cfg_attr(feature = "strict", deny(warnings), deny(unused_crate_dependencies))]
#![warn(missing_docs)]

use std::fmt::Display;

pub use c_api_tools::cfuncs;
pub use c_api_tools::concretise_types;
pub use c_api_tools::eval_with_concrete_type;

#[cfuncs(name = "my_wrapper", create, free, unwrap)]
pub struct MyWrapper;

pub struct MyStruct<T: num::Float, V: num::Float> {
    pub a: T,
    pub b: V,
}

impl<T: num::Float, V: num::Float> MyStruct<T, V> {
    pub fn new(a: T, b: V) -> Self {
        Self { a, b }
    }
}

#[concretise_types(
    gen_type(name = "dtype1", replace_with = ["f32", "f64"]),
    gen_type(name = "dtype2", replace_with = ["f32", "f64"]),
    field(arg = 0, name = "wrap", wrapper = "MyWrapper", replace_with = ["MyStruct<{{dtype1}}, {{dtype2}}>"]),
)]
pub fn test_func<T: num::Float + Display, V: num::Float + Display>(spam: &MyStruct<T, V>) {
    println!("{} {}", spam.a, spam.b);
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_fun() {
        let wrapper = my_wrapper_create();

        let unwrapped = unsafe { my_wrapper_unwrap(wrapper) }.unwrap();

        *unwrapped = Box::new(MyStruct::<f64, f32> { a: 5.0, b: 3.0 });

        unsafe { test_func(wrapper) };
        unsafe { my_wrapper_free(wrapper) };
    }
}
