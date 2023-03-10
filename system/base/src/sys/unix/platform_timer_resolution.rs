// Copyright 2022 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use crate::{EnabledHighResTimer, Result};

/// Noop struct on unix.
/// On windows, restores the platform timer resolution to its original value on Drop.
pub struct UnixSetTimerResolution {}
impl EnabledHighResTimer for UnixSetTimerResolution {}

pub fn enable_high_res_timers() -> Result<Box<dyn EnabledHighResTimer>> {
    Ok(Box::new(UnixSetTimerResolution {}))
}
