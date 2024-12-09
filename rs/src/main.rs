use memmap2::{MmapMut, MmapOptions};
use std::fs::OpenOptions;
use std::io;

fn main() -> io::Result<()> {
    // Path to the existing shared memory file
    let shm_path = "/tmp/shared_memory_example"; // Adjust path as necessary

    // Open the file for memory mapping (only open if it already exists)
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(shm_path)
        .expect("Failed to open shared memory: Ensure the shared memory file exists");

    // Map the existing shared memory
    let mut mmap = unsafe { MmapOptions::new().len(1024).map_mut(&file) }
        .expect("Failed to map memory");

    // Print the current content in the shared memory
    println!("Array before modification: {:?}", &mmap[0..5]);

    // Modify the mapped memory (e.g., change some data)
    mmap[0..5].copy_from_slice(&[100, 101, 102, 103, 104]);

    // Synchronize changes to ensure they are flushed
    mmap.flush().expect("Failed to flush memory");

    println!("Modified shared memory");
    Ok(())
}