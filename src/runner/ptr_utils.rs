use kasl::scope_manager::BlueprintItem;
use std::alloc::{Layout, alloc, dealloc};

pub(super) fn get_blueprint_ptr(items: &[BlueprintItem]) -> Vec<*mut ()> {
    let mut ptrs: Vec<*mut ()> = Vec::with_capacity(items.len());
    for item in items {
        let layout = Layout::from_size_align(item.size, item.align as usize).unwrap();

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

pub(super) fn deallocate_blueprint_ptr(items: &[BlueprintItem], ptrs: Vec<*mut ()>) {
    unsafe {
        for (item, ptr) in items.iter().zip(ptrs) {
            let layout = Layout::from_size_align(item.size, item.align as usize).unwrap();

            if !ptr.is_null() {
                dealloc(ptr as *mut u8, layout);
            }
        }
    }
}
