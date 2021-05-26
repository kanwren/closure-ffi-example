use std::ffi::c_void;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Wrapper around c_call that accepts a closure.
unsafe fn c_call_with_closure<F>(callback: F)
where
    F: FnOnce(),
{
    // The function pointer that C will call; converts the `void*` to the real closure and calls it
    unsafe extern "C" fn call_shim<F>(arg: *mut c_void) where F: FnOnce() {
        let closure: &mut Option<F> = &mut *(arg as *mut Option<F>);
        let closure = closure.take().unwrap();
        closure()
    }

    let f: unsafe extern "C" fn(*mut c_void) = call_shim::<F>;
    let mut closure = Some(callback);
    c_call(Some(f), &mut closure as *mut _ as *mut c_void);
}

/// Wrapper around c_deferred_call that accepts a closure.
unsafe fn c_deferred_call_with_closure<F>(callback: F)
where
    F: FnOnce(),
{
    // The function pointer that C will call; converts the `void*` to the real closure and calls it
    unsafe extern "C" fn call_shim_boxed<F>(arg: *mut c_void) where F: FnOnce() {
        let mut closure: Box<Option<F>> = Box::from_raw(arg as *mut _);
        let closure = closure.take().unwrap();
        closure()
    }

    let f: unsafe extern "C" fn(*mut c_void) = call_shim_boxed::<F>;
    let closure = Box::new(Some(callback));
    c_deferred_call(Some(f), Box::into_raw(closure) as *mut _);
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
