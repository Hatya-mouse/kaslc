use crate::runner::compiler::ptr_utils::alloc_buf_for_type;

pub fn alloc_and_write_each<T: Sized, E, F>(count: usize, mut get_value: F) -> Result<*mut (), E>
where
    F: FnMut(usize) -> Result<T, E>,
{
    let ptr = alloc_buf_for_type::<T>(count);
    for i in 0..count {
        let value = get_value(i)?;
        unsafe {
            ptr.add(i).write(value);
        }
    }
    Ok(ptr as *mut ())
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
