# closure-ffi-example

An example of how to pass closures through FFI to C functions. The principle is
that if you have a C function that accepts a function pointer like a `void
(*)(void *)`, and an argument like a `void *`, then you can convert a reference
to the closure to a `void *` and pass in a shim function pointer that will
convert it back and call it.

Note that we need to use a double-reference `&mut &mut dyn FnMut()` here, as a
`&dyn FnMut()` is a fat pointer containing a vtable pointer. The same principle
applies with a `Box<Box<dyn FnMut()>>`.

Also note that if the closure is stored anywhere else and then invoked later,
the reference would be invalid. In this case, we use a `Box<Box<dyn FnMut()>>`;
see the `c_deferred_call`/`c_do_deferred_call` example.

## Build Instructions

```
# Compile only the C parts (make the shared library)
$ make cparts

# Compile everything (C + Rust)
$ make

# Run the program
$ make run

# Remove all build output
$ make clean
```

Requires make, a C and Rust toolchain, and libclang (for bindgen). You may need
to set `LIBCLANG_PATH` to the location of the libclang installation.

Nix is provided to set up a dev shell with the required dependencies and
environment variables.

