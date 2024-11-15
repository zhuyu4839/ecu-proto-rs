# DoCAN Protocol

[![Latest version](https://img.shields.io/crates/v/docan.svg)](https://crates.io/crates/docan)
[![Documentation](https://docs.rs/bleasy/badge.svg)](https://docs.rs/iso15765-2)
![LGPL](https://img.shields.io/badge/license-LGPL-green.svg)
![MIT](https://img.shields.io/badge/license-MIT-yellow.svg)

## Overview

Diagnostic Communication over Controller Area Network (DoCAN) is a specialized protocol used primarily in automotive and industrial settings.

The driver must implement the CanDriver trait defined in [`rs-can`](https://crates.io/crates/rs-can).

##### The Server example

```rust
fn main() -> anyhow::Result<()> {
    let driver = YourDriver;
    let mut adapter = IsoTpAdapter::new(driver);

    let mut server = DoCanServer::new(adapter.clone(), CHANNEL, Address {
        tx_id: 0x7E8,
        rx_id: 0x7E0,
        fid: 0x7DF,
    });

    adapter.start(100);

    let msg = YourFrame;
    server.adapter().sender().send(msg)?;

    server.service_forever(100)?;
    
    server.service_stop()?;
    
    Ok(())
}
```

#### The client example
```rust
fn main() -> anyhow::Result<()> {
    let driver = YourDriver;
    let mut adapter = IsoTpAdapter::new(driver);

    let mut client = DoCanClient::new(adapter.clone(), None);
    client.init_channel(CHANNEL, Address {
        tx_id: 0x7E0,
        rx_id: 0x7E8,
        fid: 0x7DF,
    })?;

    adapter.start(100);

    let msg = YourFrame;
    client.adapter().sender().send(msg)?;
    
    client.session_ctrl(CHANNEL, SessionType::Default, true, AddressType::Functional)?;
    client.session_ctrl(CHANNEL, SessionType::Extended, false, AddressType::Physical)?;
    
    Ok(())
}
```

### Prerequisites

- Rust 1.70 or higher
- Cargo (included with Rust)

## Contributing

We're always looking for users who have thoughts on how to make `docan` better, or users with
interesting use cases.  Of course, we're also happy to accept code contributions for outstanding
feature requests!

