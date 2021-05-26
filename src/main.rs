use std::ffi::c_void;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// The function pointer that C will call; converts the `void*` to the real closure and calls it
unsafe extern "C" fn call_shim(arg: *mut c_void) {
    let closure: &mut &mut dyn FnMut() = &mut *(arg as *mut &mut dyn FnMut());
    closure()
}

/// The function pointer that C will call; converts the `void*` to the real closure and calls it
unsafe extern "C" fn call_shim_boxed(arg: *mut c_void) {
    let mut closure: Box<Box<dyn FnMut()>> = Box::from_raw(arg as *mut _);
    closure()
}

/// Wrapper around c_call that accepts a closure.
unsafe fn c_call_with_closure<F>(mut callback: F)
where
    F: FnMut(),
{
    let mut cb: &mut dyn FnMut() = &mut callback;
    let cb = &mut cb;
    c_call(Some(call_shim), cb as *mut _ as *mut c_void);
}

/// Wrapper around c_deferred_call that accepts a closure.
unsafe fn c_deferred_call_with_closure<F>(callback: F)
where
    F: FnMut(),
{
    let cb: Box<Box<dyn FnMut()>> = Box::new(Box::new(callback));
    c_deferred_call(Some(call_shim_boxed), Box::into_raw(cb) as *mut _);
}

fn main() {
    unsafe {
        // Immediate call
        c_call_with_closure(|| {
            println!("This closure is called immediately");
        });

        // Stored function pointer
        c_deferred_call_with_closure(|| {
            println!("This closure is deferred until c_do_deferred_call");
        });
        println!("Not called yet");
        c_do_deferred_call();
    }
}
