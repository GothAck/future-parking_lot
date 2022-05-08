// Copyright 2018 Marco Napetti
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
#![deny(warnings)]
#![deny(missing_docs)]

//! # future-parking_lot
//!
//! A simple Future implementation for parking_lot

/// parking_lot::Mutex Future implementation
pub mod mutex;
/// parking_lot::RwLock Future implementation
pub mod rwlock;

//Re-export `parking_lot` to avoid version mismatch
pub use parking_lot;

pub(crate) fn map_atomic_bool_compare_exchange_result(result: Result<bool, bool>) -> bool {
    match result {
        Ok(value) => value,
        Err(value) => value,
    }
}
