// Copyright 2022 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//! Implementation of the Syslog trait as a wrapper around Window's events

use crate::syslog::{Error, Facility, Log, Syslog};
use crate::RawDescriptor;

pub struct PlatformSyslog {}

impl Syslog for PlatformSyslog {
    fn new(
        _proc_name: String,
        _facility: Facility,
    ) -> Result<(Option<Box<dyn Log + Send>>, Option<RawDescriptor>), Error> {
        Ok((None, None))
    }
}
