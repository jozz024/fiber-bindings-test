use nnsdk as nn;
use nn::os::FiberFunction;
use nn::os::FiberType;
use std::boxed::Box;

fn construct_fiber(function: FiberFunction, args: *const u8 ) -> FiberType {
    // Constructing the nn::os::FiberType struct with default values (besides function, because it requires a `FiberFunction`).
    let mut fiber = nn::os::FiberType {
        status: 0,
        is_aligned: false,
        function,
        args: std::ptr::null_mut(),
        unk1: std::ptr::null_mut(),
        stack: std::ptr::null_mut(),
        stack_size: 0,
        context: [0u8;208]
    };

    // Setup the stack, 0x10000 should be more than enough for what we're doing.
    let stack = Box::new([0u8; 0x10000]);
    // Leak the stack, so we get a mutable reference.
    let stack = Box::leak(stack);
    // Run nn::os::InitializeFiber with the required arguments to setup the current fiber.
    unsafe { nn::os::InitializeFiber(&mut fiber, function, args as *mut skyline::libc::c_void, stack.as_ptr() as *mut skyline::libc::c_void, 0x10000, 0) };
    // Return an instance of the nn::os::FiberType struct.
    return fiber;
}

extern "C" fn test(args: *const u8) -> *const FiberType {
    println!("First test function running!");
    // Print the value of `args`, in this case it will be 111.
    dbg!(unsafe { *args });

    // We construct the fiber and then box it so we get a mutable reference
    // Set args to a null pointer because we dont use them for `test2`
    let fiber = Box::new(construct_fiber(test2, std::ptr::null()));
    let fiber = Box::leak(fiber);
    // Note: Switching to the next fiber is crucial if you return a fiber, or else the game will crash.
    unsafe{ nn::os::SwitchToFiber(fiber) }
    // Return &mut FiberType
    return fiber;
}

extern "C" fn test2(_args: *const u8) -> *const FiberType {
    println!("Second Test Function running!");
    // Return a null pointer to go back to executing the `main` function, or whatever function the fiber started in
    return std::ptr::null();
}

#[skyline::main(name = "fiber-bindings-test")]
pub fn main() {
    // Construct the arguments for the function `test`, which is just an unsigned integer.
    let args = Box::new(111);
    let args = Box::leak(args);

    let mut fiber = construct_fiber(test, args);
    unsafe { nn::os::SwitchToFiber(&mut fiber);}
}
