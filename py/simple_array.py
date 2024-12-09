import numpy as np
import time
from multiprocessing import shared_memory

# Create a numpy array
data = np.array([1, 2, 6, 4, 5], dtype=np.int64)

# Create a shared memory block
shm = shared_memory.SharedMemory(create=True, size=data.nbytes, name="/tmp/shared_memory_example")

# Create a numpy array backed by shared memory
shared_array = np.ndarray(data.shape, dtype=data.dtype, buffer=shm.buf)
shared_array[:] = data[:]  # Copy the data into the shared memory

# Print the initial array
print("Original array:", shared_array)

print("Waiting for rust to modify the array..")
# while shm.buf[-1] != 1:
#     time.sleep(0.1)

time.sleep(5)
print(shared_array)


# # Simulate accessing the shared memory in another part of the code
# try:
#     existing_shm = shared_memory.SharedMemory(name="shared_memory_example")
#     existing_array = np.ndarray(data.shape, dtype=data.dtype, buffer=existing_shm.buf)

#     # Modify the shared array
#     existing_array[:] = [10, 20, 30, 40, 50]
#     print("Modified array:", existing_array)

#     # Clean up the shared memory (close in both instances)
#     existing_shm.close()
# finally:
#     # Ensure the shared memory is unlinked to release resources
shm.close()
shm.unlink()