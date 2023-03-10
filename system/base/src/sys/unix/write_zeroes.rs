// Copyright 2018 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::{cmp::min, fs::File, io, os::unix::fs::FileExt};

use super::{fallocate, FallocateMode};

pub(crate) fn file_punch_hole(file: &File, offset: u64, length: u64) -> io::Result<()> {
    fallocate(file, FallocateMode::PunchHole, true, offset, length as u64)
        .map_err(|e| io::Error::from_raw_os_error(e.errno()))
}

pub(crate) fn file_write_zeroes_at(file: &File, offset: u64, length: usize) -> io::Result<usize> {
    // Try to use fallocate() first.
    if fallocate(file, FallocateMode::ZeroRange, true, offset, length as u64).is_ok() {
        return Ok(length);
    }

    // fall back to write()
    // fallocate() failed; fall back to writing a buffer of zeroes
    // until we have written up to length.
    let buf_size = min(length, 0x10000);
    let buf = vec![0u8; buf_size];
    let mut nwritten: usize = 0;
    while nwritten < length {
        let remaining = length - nwritten;
        let write_size = min(remaining, buf_size);
        nwritten += file.write_at(&buf[0..write_size], offset + nwritten as u64)?;
    }
    Ok(length)
}
