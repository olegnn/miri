static mut PTR: *mut u8 = 0 as *mut _;

fn fun1(x: &mut u8) {
    unsafe {
        PTR = x;
    }
}

fn fun2() {
    // Now we use a pointer we are not allowed to use
    let _x = unsafe { *PTR }; //~ ERROR does not exist on the stack
}

fn main() {
    let mut val = 0;
    let val = &mut val;
    fun1(val);
    *val = 2; // this invalidates any raw ptrs `fun1` might have created.
    fun2(); // if they now use a raw ptr they break our reference
}
