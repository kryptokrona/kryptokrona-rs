// Copyright (c) 2019-2024, The Kryptokrona Project
//
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification, are
// permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice, this list of
//    conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice, this list
//    of conditions and the following disclaimer in the documentation and/or other
//    materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors may be
//    used to endorse or promote products derived from this software without specific
//    prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY
// EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL
// THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF
// THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

pub mod ascii;
pub mod thor;
pub mod wallet_backend;

pub use thor::Thor;
pub use wallet_backend::WalletBackend;

extern crate libc;

use libc::{c_char, c_void, size_t};
use std::mem::transmute;
use std::sync::Arc;
use std::thread;

mod ffi;

pub const FAST_HASH_LENGTH: usize = 32;
pub const SLOW_HASH_LENGTH: usize = 32;

pub fn fast_hash(data: &[u8]) -> [u8; FAST_HASH_LENGTH] {
    use ffi::cn_fast_hash;

    debug_assert!(FAST_HASH_LENGTH == ffi::HASH_SIZE);

    let output = &mut [0u8; FAST_HASH_LENGTH];
    unsafe {
        cn_fast_hash(
            data.as_ptr() as *const c_void,
            data.len() as size_t,
            transmute::<*mut u8, *mut c_char>(output.as_mut_ptr()),
        )
    }

    *output
}

pub fn slow_hash(data: &[u8]) -> [u8; SLOW_HASH_LENGTH] {
    use ffi::cn_slow_hash;

    debug_assert!(SLOW_HASH_LENGTH == ffi::HASH_SIZE);

    let data_arc = Arc::new(data.to_owned());

    let mut output = vec![0u8; SLOW_HASH_LENGTH];

    // Spawn a thread to perform the hash calculation
    let child = thread::Builder::new()
        .stack_size(4194304)
        .spawn(move || {
            let data_ref = &*data_arc;

            unsafe {
                cn_slow_hash(
                    data_ref.as_ptr() as *const _ as *const c_void,
                    data_ref.len() as size_t,
                    output.as_mut_ptr() as *mut _ as *mut c_char,
                )
            }

            output
        })
        .unwrap();

    let hash = child.join().unwrap();
    let mut result = [0u8; SLOW_HASH_LENGTH];
    result.copy_from_slice(&hash[..SLOW_HASH_LENGTH]);

    result
}

#[cfg(test)]
pub mod tests {
    use super::*;

    struct TestVector {
        expected: &'static [u8],
        input: &'static [u8],
    }

    const SLOW_HASH_TEST_VECTORS: &'static [TestVector] = &[
        TestVector {
            expected: &[
                0x2f, 0x8e, 0x3d, 0xf4, 0x0b, 0xd1, 0x1f, 0x9a, 0xc9, 0x0c, 0x74, 0x3c, 0xa8, 0xe3,
                0x2b, 0xb3, 0x91, 0xda, 0x4f, 0xb9, 0x86, 0x12, 0xaa, 0x3b, 0x6c, 0xdc, 0x63, 0x9e,
                0xe0, 0x0b, 0x31, 0xf5,
            ],
            input: &[
                0x64, 0x65, 0x20, 0x6f, 0x6d, 0x6e, 0x69, 0x62, 0x75, 0x73, 0x20, 0x64, 0x75, 0x62,
                0x69, 0x74, 0x61, 0x6e, 0x64, 0x75, 0x6d,
            ],
        },
        TestVector {
            expected: &[
                0x72, 0x2f, 0xa8, 0xcc, 0xd5, 0x94, 0xd4, 0x0e, 0x4a, 0x41, 0xf3, 0x82, 0x27, 0x34,
                0x30, 0x4c, 0x8d, 0x5e, 0xff, 0x7e, 0x1b, 0x52, 0x84, 0x08, 0xe2, 0x22, 0x9d, 0xa3,
                0x8b, 0xa5, 0x53, 0xc4,
            ],
            input: &[
                0x61, 0x62, 0x75, 0x6e, 0x64, 0x61, 0x6e, 0x73, 0x20, 0x63, 0x61, 0x75, 0x74, 0x65,
                0x6c, 0x61, 0x20, 0x6e, 0x6f, 0x6e, 0x20, 0x6e, 0x6f, 0x63, 0x65, 0x74,
            ],
        },
        TestVector {
            expected: &[
                0xbb, 0xec, 0x2c, 0xac, 0xf6, 0x98, 0x66, 0xa8, 0xe7, 0x40, 0x38, 0x0f, 0xe7, 0xb8,
                0x18, 0xfc, 0x78, 0xf8, 0x57, 0x12, 0x21, 0x74, 0x2d, 0x72, 0x9d, 0x9d, 0x02, 0xd7,
                0xf8, 0x98, 0x9b, 0x87,
            ],
            input: &[
                0x63, 0x61, 0x76, 0x65, 0x61, 0x74, 0x20, 0x65, 0x6d, 0x70, 0x74, 0x6f, 0x72,
            ],
        },
        TestVector {
            expected: &[
                0xb1, 0x25, 0x7d, 0xe4, 0xef, 0xc5, 0xce, 0x28, 0xc6, 0xb4, 0x0c, 0xeb, 0x1c, 0x6c,
                0x8f, 0x81, 0x2a, 0x64, 0x63, 0x4e, 0xb3, 0xe8, 0x1c, 0x52, 0x20, 0xbe, 0xe9, 0xb2,
                0xb7, 0x6a, 0x6f, 0x05,
            ],
            input: &[
                0x65, 0x78, 0x20, 0x6e, 0x69, 0x68, 0x69, 0x6c, 0x6f, 0x20, 0x6e, 0x69, 0x68, 0x69,
                0x6c, 0x20, 0x66, 0x69, 0x74,
            ],
        },
    ];

    const FAST_HASH_TEST_VECTORS: &'static [TestVector] = &[TestVector {
        expected: &[
            0xc8, 0x8c, 0xe9, 0x78, 0x3b, 0x4f, 0x11, 0x19, 0x0d, 0x7b, 0x9c, 0x17, 0xa6, 0x9c,
            0x1c, 0x52, 0x20, 0x0f, 0x9f, 0xaa, 0xee, 0x8e, 0x98, 0xdd, 0x07, 0xe6, 0x81, 0x11,
            0x75, 0x17, 0x71, 0x39,
        ],
        // mainnet's genesis tx
        input: &[
            0x01, 0x3c, 0x01, 0xff, 0x00, 0x01, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x03, 0x02,
            0x9b, 0x2e, 0x4c, 0x02, 0x81, 0xc0, 0xb0, 0x2e, 0x7c, 0x53, 0x29, 0x1a, 0x94, 0xd1,
            0xd0, 0xcb, 0xff, 0x88, 0x83, 0xf8, 0x02, 0x4f, 0x51, 0x42, 0xee, 0x49, 0x4f, 0xfb,
            0xbd, 0x08, 0x80, 0x71, 0x21, 0x01, 0x77, 0x67, 0xaa, 0xfc, 0xde, 0x9b, 0xe0, 0x0d,
            0xcf, 0xd0, 0x98, 0x71, 0x5e, 0xbc, 0xf7, 0xf4, 0x10, 0xda, 0xeb, 0xc5, 0x82, 0xfd,
            0xa6, 0x9d, 0x24, 0xa2, 0x8e, 0x9d, 0x0b, 0xc8, 0x90, 0xd1,
        ],
    }];

    // TODO: doesn't work on Ubuntu 22.04, but works on macos _all_ and Ubuntu 20.04
    #[test]
    fn slow_hash_test_vector() {
        for vector in SLOW_HASH_TEST_VECTORS.iter() {
            let hash = slow_hash(vector.input);
            assert_eq!(&hash, vector.expected);
        }
    }

    #[test]
    fn fast_hash_test_vector() {
        for vector in FAST_HASH_TEST_VECTORS.iter() {
            let hash = fast_hash(vector.input);
            assert_eq!(&hash, vector.expected);
        }
    }
}
