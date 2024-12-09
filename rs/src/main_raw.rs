use std::ffi::CString;
use std::ptr;
use libc::{shm_open, mmap, PROT_READ, PROT_WRITE, MAP_SHARED, MAP_FAILED, O_RDWR};
fn main() {
    let shm_name = CString::new("/shared_memory_example").unwrap();

    // open shared memory
    let fd = unsafe{ shm_open(shm_name.as_ptr(), O_RDWR, 0)};
    if fd == -1{
        eprintln!("Filed to open the shared memory");
        return;
    }

    // Map the shared memory into the Rust process's address space
    let size = 5 * std::mem::size_of::<i64>() + 1; // 5 elements + 1-byte flag
    let ptr = unsafe { mmap(ptr::null_mut(), size, PROT_READ | PROT_WRITE, MAP_SHARED, fd, 0) };
    if ptr == MAP_FAILED {
        eprintln!("Failed to map shared memory");
        return;
    }

    // Cast the pointer to access the array
    let array = unsafe { std::slice::from_raw_parts_mut(ptr as *mut i64, 5) };
    println!("Array before modification: {:?}", array);

    // Modify the array
    for i in 0..array.len() {
        array[i] *= 2;  // Example modification: double each element
    }
    println!("Array after modification: {:?}", array);

    // Set the synchronization flag
    unsafe {
        let flag_ptr = (ptr as *mut u8).add(size - 1);
        *flag_ptr = 1;  // Set flag to indicate modification is done
    }
}
