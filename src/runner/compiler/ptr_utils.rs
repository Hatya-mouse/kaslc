use kasl::scope_manager::BlueprintItem;
use std::alloc::{Layout, alloc, dealloc};

// --- OUTPUT & INPUT BUFFER ALLOCATION ---

pub(super) fn get_buffer_blueprint_ptr(
    items: Vec<&BlueprintItem>,
    buffer_size: usize,
) -> Vec<*mut ()> {
    let mut ptrs: Vec<*mut ()> = Vec::with_capacity(items.len());
    for item in items {
        let layout =
            Layout::from_size_align(item.actual_size * buffer_size, item.align as usize).unwrap();
        unsafe {
            let ptr = alloc(layout);
            if ptr.is_null() {
                panic!("Failed to allocate memory for blueprint item");
            }
            ptrs.push(ptr as *mut ());
        }
    }
    ptrs
}

pub(super) fn deallocate_buffer_blueprint_ptr(
    items: Vec<&BlueprintItem>,
    ptrs: Vec<*mut ()>,
    buffer_size: usize,
) {
    unsafe {
        for (item, ptr) in items.iter().zip(ptrs) {
            let layout =
                Layout::from_size_align(item.actual_size * buffer_size, item.align as usize)
                    .unwrap();
            if !ptr.is_null() {
                dealloc(ptr as *mut u8, layout);
            }
        }
    }
}

// --- STATE ALLOCATION ---

pub(super) fn get_blueprint_ptr(items: Vec<&BlueprintItem>) -> Vec<*mut ()> {
    let mut ptrs: Vec<*mut ()> = Vec::with_capacity(items.len());
    for item in items {
        let layout = Layout::from_size_align(item.actual_size, item.align as usize).unwrap();

        unsafe {
            let ptr: *mut u8 = alloc(layout);

            if ptr.is_null() {
                panic!("Failed to allocate memory for blueprint item");
            }

            let void_ptr = ptr as *mut ();
            ptrs.push(void_ptr);
        }
    }
    ptrs
}

pub(super) fn deallocate_blueprint_ptr(items: Vec<&BlueprintItem>, ptrs: Vec<*mut ()>) {
    for (item, ptr) in items.iter().zip(ptrs) {
        let layout = Layout::from_size_align(item.actual_size, item.align as usize).unwrap();

        if !ptr.is_null() {
            unsafe {
                dealloc(ptr as *mut u8, layout);
            }
        }
    }
}

pub fn alloc_buf_for_type<T: Sized>(capacity: usize) -> *mut T {
    let layout = Layout::array::<T>(capacity).unwrap();
    unsafe { alloc(layout) as *mut T }
}
