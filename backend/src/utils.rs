// backend/src/utils.rs

// External declaration of the SHA3-512 function implemented in assembly
#[no_mangle]
pub extern "C" fn sha3_512(input: *const u8, input_len: usize, output: *mut u8);

// Example of using the assembly SHA3-512 function
pub fn hash_data(data: &[u8]) -> [u8; 64] {
    let mut output: [u8; 64] = [0; 64]; // SHA3-512 produces a 64-byte output

    unsafe {
        sha3_512(data.as_ptr(), data.len(), output.as_mut_ptr());
    }

    output
}
