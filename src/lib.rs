use nn::os::FiberFunction;
use nn::os::FiberType;
use nnsdk as nn;
use std::boxed::Box;

fn construct_fiber(function: FiberFunction ) -> FiberType {
    let mut fiber = nn::os::FiberType {
        status: 0,
        is_aligned: false,
        function: function,
        args: std::ptr::null_mut(),
        unk1: std::ptr::null_mut(),
        stack: std::ptr::null_mut(),
        stack_size: 0,
        context: [0u8;208]
    };

    let stack = Box::new([0u8; 0x10000]);
    let stack = Box::leak(stack);
    unsafe { nn::os::InitializeFiber(&mut fiber, function, std::ptr::null_mut(), stack.as_ptr() as *mut skyline::libc::c_void, 0x10000, 0) };
    return fiber;
}

extern "C" fn test(args: *const u8) -> *const FiberType {
    panic!("fn ran");
    let fiber = Box::new(construct_fiber(test2));
    let fiber = Box::leak(fiber);
    return fiber;
}

extern "C" fn test2(args: *const u8) -> *const FiberType {
    println!("fn2 ran");
    let fiber = Box::new(construct_fiber(test));
    let fiber = Box::leak(fiber);
    return fiber;
}

#[skyline::main(name = "fiber-bindings-test")]
pub fn main() {
    let mut fiber = construct_fiber(test);
    unsafe { nn::os::SwitchToFiber(&mut fiber);}
    println!("Hello from skyline plugin");
}
