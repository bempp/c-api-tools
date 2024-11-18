use std::fmt::Display;

pub use c_api_tools::cfuncs;
pub use c_api_tools::concretise_types;
pub use c_api_tools::eval_with_concrete_type;

#[cfuncs(name = "my_wrapper", create, free, unwrap)]
pub struct MyWrapper;

pub struct MyStruct<T: num::Float> {
    pub a: T,
}

impl<T: num::Float> MyStruct<T> {
    pub fn new(a: T) -> Self {
        Self { a }
    }
}

pub fn set_float<T: num::Float>(a: &mut MyStruct<T>, num: *const std::ffi::c_void) {
    let num = unsafe { *(num as *const T) };
    a.a = num;
}

#[concretise_types(
    gen_type(name = "dtype", replace_with = ["f32", "f64"]),
    field(arg = 0, name = "wrap", wrapper = "MyWrapper", is_mut, replace_with = ["MyStruct<{{dtype}}>"]),
)]
pub fn test_func<T: num::Float + Display>(spam: &MyStruct<T>) {
    println!("{}", spam.a);
}

// pub fn print_float<T: num::Float + std::fmt::Display>(a: &MyStruct<T>) {
//     println!("a: {}", a.a);
// }

// #[no_mangle]
// pub unsafe extern "C" fn c_set_float(ptr: *mut MyWrapper, num: *const std::ffi::c_void) {
//     pub fn set_float<T: num::Float>(a: &mut MyStruct<T>, num: *const std::ffi::c_void) {
//         let num = unsafe { *(num as *const T) };
//         a.a = num;
//     }

//     if let Some(ptr) = my_wrapper_unwrap(ptr)
//         .unwrap()
//         .downcast_mut::<MyStruct<f32>>()
//     {
//         set_float(ptr, num)
//     }
// }

// #[no_mangle]
// pub unsafe extern "C" fn c_float_print(ptr: *mut MyWrapper) {
//     pub fn print_float<T: num::Float + std::fmt::Display>(a: &MyStruct<T>) {
//         println!("a: {}", a.a);
//     }
//     if let Some(a) = my_wrapper_unwrap(ptr)
//         .unwrap()
//         .downcast_mut::<MyStruct<f32>>()
//     {
//         print_float(a)
//     }
// }

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_fun() {}
}
