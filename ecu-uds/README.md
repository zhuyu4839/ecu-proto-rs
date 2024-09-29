# A UDS PROTOCOL for ECU

[![Latest version](https://img.shields.io/crates/v/ecu-uds.svg)](https://crates.io/crates/ecu-uds)
[![Documentation](https://docs.rs/bleasy/badge.svg)](https://docs.rs/ecu-uds)
![LGPL](https://img.shields.io/badge/license-LGPL-green.svg)
![MIT](https://img.shields.io/badge/license-MIT-yellow.svg)

## Overview

**ecu-uds** is dedicated to implementing ISO 14229. ISO-TP (ISO 14229) is a UDS protocol used in automotive communication.

### Prerequisites

- Rust 1.70 or higher
- Cargo (included with Rust)

### Adding to Your Project

To use **ecu-uds** in your Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
ecu-uds = { version="lastest-version" }
```

### supported service
  - note:
    - the service marked with `✅` is completed.
    - the service marked with `⭕` is partially completed.
    - The service marked with `❌` is not implemented.

  *  SessionCtrl = 0x10,         // ✅
  *  ECUReset = 0x11,            // ✅
  *  ClearDiagnosticInfo = 0x14, // ✅
  *  ReadDTCInfo = 0x19,         // ⭕
  *  ReadDID = 0x22,             // ✅
  *  ReadMemByAddr = 0x23,       // ✅
  *  ReadScalingDID = 0x24,      // ✅
  *  SecurityAccess = 0x27,      // ✅
  *  CommunicationCtrl = 0x28,   // ✅
  *  Authentication = 0x29,      // ✅
  *  ReadDataByPeriodId = 0x2A,  // ✅
  *  DynamicalDefineDID = 0x2C,  // ✅
  *  WriteDID = 0x2E,            // ✅
  *  IOCtrl = 0x2F,              // ✅
  *  RoutineCtrl = 0x31,         // ✅
  *  RequestDownload = 0x34,     // ✅
  *  RequestUpload = 0x35,       // ✅
  *  TransferData = 0x36,        // ✅
  *  RequestTransferExit = 0x37, // ✅
  *  RequestFileTransfer = 0x38, // ✅
  *  WriteMemByAddr = 0x3D,      // ✅
  *  TesterPresent = 0x3E,       // ✅
  *  AccessTimingParam = 0x83,   // ✅
  *  SecuredDataTrans = 0x84,    // ✅
  *  CtrlDTCSetting = 0x85,      // ✅
  *  ResponseOnEvent = 0x86,     // ❌
  *  LinkCtrl = 0x87,            // ✅

## Contributing

We're always looking for users who have thoughts on how to make `ecu-uds` better, or users with
interesting use cases.

Of course, we're also happy to accept code contributions for outstanding feature requests!