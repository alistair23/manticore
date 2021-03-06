// Copyright lowRISC contributors.
// Licensed under the Apache License, Version 2.0, see LICENSE for details.
// SPDX-License-Identifier: Apache-2.0

// NOTE: This file is autogenerated by:
// $ util/new_protocol_target.py --target-templates deserialize serialize_fuzz_safe -- device_info::DeviceInfoResponse<'static>

#![no_main]

use libfuzzer_sys::fuzz_target;

use manticore::protocol::FuzzSafe;
use manticore::protocol::wire::ToWire;
use manticore::protocol::device_info::DeviceInfoResponse;

fuzz_target!(|data: <DeviceInfoResponse<'static> as FuzzSafe>::Safe| {
    let mut out = [0u8; 1024];
    let data = data.as_ref();
    let _ = data.to_wire(&mut &mut out[..]);
});

