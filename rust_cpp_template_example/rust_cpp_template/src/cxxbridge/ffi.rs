#[cxx::bridge]
mod ffi {
    extern "C" {
        type OperationsInt;

        fn new_operations_int(value: i32) -> *mut OperationsInt;
        fn operations_int_add(self: *mut OperationsInt, other: i32) -> i32;
        fn operations_int_subtract(self: *mut OperationsInt, other: i32) -> i32;
        fn delete_operations_int(self: *mut OperationsInt);
    }
}

pub struct Operations {
    ptr: *mut ffi::OperationsInt,
}

impl Operations {
    pub fn new(value: i32) -> Self {
        let ptr = unsafe { ffi::new_operations_int(value) };
        Operations { ptr }
    }

    pub fn add(&self, other: i32) -> i32 {
        unsafe { ffi::operations_int_add(self.ptr, other) }
    }

    pub fn subtract(&self, other: i32) -> i32 {
        unsafe { ffi::operations_int_subtract(self.ptr, other) }
    }
}

impl Drop for Operations {
    fn drop(&mut self) {
        unsafe { ffi::delete_operations_int(self.ptr) }
    }
}
