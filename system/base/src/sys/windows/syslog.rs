// Copyright 2022 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//! Facilities for sending log message to syslog.
//!
//! Every function exported by this module is thread-safe. Each function will silently fail until
//! `syslog::init()` is called and returns `Ok`.
//!
//! # Examples
//!
//! ```
//! use base::{error, self, warn};
//!
//! if let Err(e) = base::syslog::init() {
//!     println!("failed to initiailize syslog: {}", e);
//!     return;
//! }
//! warn!("this is your {} warning", "final");
//! error!("something went horribly wrong: {}", "out of RAMs");
//! ```

// On windows RawDescriptor is !Sync + !Send, but also on windows we don't do anything with them
unsafe impl Sync for crate::syslog::State {}
unsafe impl Send for crate::syslog::State {}
pub use super::win::syslog::PlatformSyslog;
