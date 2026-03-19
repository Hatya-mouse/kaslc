use crate::runner::compiler::ptr_utils::alloc_buf_for_type;

pub fn alloc_and_write_each<T: Sized, F>(count: usize, mut get_value: F) -> *mut ()
where
    F: FnMut(usize) -> T,
{
    let ptr = alloc_buf_for_type::<T>(count);
    for i in 0..count {
        let value = get_value(i);
        unsafe {
            ptr.add(i).write(value);
        }
    }
    ptr as *mut ()
}

pub fn alloc_and_spread<T: Sized + Copy>(count: usize, value: T) -> *mut () {
    let ptr = alloc_buf_for_type::<T>(count);
    for i in 0..count {
        unsafe {
            ptr.add(i).write(value);
        }
    }
    ptr as *mut ()
}
