use iso13400_2::{LogicAddress, Message, Payload, RoutingActiveType, Version};
use iso13400_2::request::{AliveCheck, Diagnostic, DiagnosticPowerMode, EntityStatus, RoutingActive, VehicleID, VehicleIDWithEID, VehicleIDWithVIN};

#[test]
fn test_vehicle_id() -> anyhow::Result<()> {
    let source = hex::decode("02FD0001\
    00000000")?;

    let payload = VehicleID;
    let msg = Message::try_from(source.as_ref())?;
    assert_eq!(msg.version, Version::ISO13400_2_2012);
    match &msg.payload {
        Payload::VehicleIdentificationRequest(v) => assert_eq!(*v, payload),
        _ => panic!("Wrong payload type"),
    }

    let data: Vec<_> = msg.into();
    assert_eq!(data, source);

    Ok(())
}

#[test]
fn test_vehicle_id_with_eid() -> anyhow::Result<()> {
    let source = hex::decode("02FD0002\
    00000006\
    110011001100")?;

    let payload = VehicleIDWithEID::new([0x11, 0x00, 0x11, 0x00, 0x11, 0x00,]);
    let msg = Message::try_from(source.as_ref())?;
    assert_eq!(msg.version, Version::ISO13400_2_2012);
    match &msg.payload {
        Payload::VehicleIdentificationRequestWithEID(v) => assert_eq!(*v, payload),
        _ => panic!("Wrong payload type"),
    }

    let data: Vec<_> = msg.into();
    assert_eq!(data, source);

    Ok(())
}

#[test]
fn test_vehicle_id_with_vin() -> anyhow::Result<()> {
    let source = hex::decode("02FD0003\
    00000011\
    2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D2D")?;

    let payload = VehicleIDWithVIN::new("-".repeat(17))?;
    let msg = Message::try_from(source.as_ref())?;
    assert_eq!(msg.version, Version::ISO13400_2_2012);
    match &msg.payload {
        Payload::VehicleIdentificationRequestWithVIN(v) => assert_eq!(*v, payload),
        _ => panic!("Wrong payload type"),
    }

    let data: Vec<_> = msg.into();
    assert_eq!(data, source);

    Ok(())
}

#[test]
fn test_entity_status() -> anyhow::Result<()> {
    let source = hex::decode("02FD4001\
    00000000")?;

    let payload = EntityStatus;
    let msg = Message::try_from(source.as_ref())?;
    assert_eq!(msg.version, Version::ISO13400_2_2012);
    match &msg.payload {
        Payload::EntityStatusRequest(v) => assert_eq!(*v, payload),
        _ => panic!("Wrong payload type"),
    }

    let data: Vec<_> = msg.into();
    assert_eq!(data, source);

    Ok(())
}

#[test]
fn test_diag_power_mode() -> anyhow::Result<()> {
    let source = hex::decode("02FD4003\
    00000000")?;

    let payload = DiagnosticPowerMode;
    let msg = Message::try_from(source.as_ref())?;
    assert_eq!(msg.version, Version::ISO13400_2_2012);
    match &msg.payload {
        Payload::DiagnosticPowerModeRequest(v) => assert_eq!(*v, payload),
        _ => panic!("Wrong payload type"),
    }

    let data: Vec<_> = msg.into();
    assert_eq!(data, source);

    Ok(())
}

#[test]
fn test_routing_activation() -> anyhow::Result<()> {
    let source = hex::decode("02FD0005\
    0000000B\
    0E00\
    E0\
    00000000\
    00000000")?;

    let payload = RoutingActive::new(
        LogicAddress::Client(0x0E00),
        RoutingActiveType::CentralSecurity,
        Some(0x00)
    );
    let msg = Message::try_from(source.as_ref())?;
    assert_eq!(msg.version, Version::ISO13400_2_2012);
    match &msg.payload {
        Payload::RoutingActivationRequest(v) => assert_eq!(*v, payload),
        _ => panic!("Wrong payload type"),
    }

    let data: Vec<_> = msg.into();
    assert_eq!(data, source);

    Ok(())
}

#[test]
fn test_alive_check() -> anyhow::Result<()> {
    let source = hex::decode("02FD0007\
    00000000")?;

    let payload = AliveCheck;
    let msg = Message::try_from(source.as_ref())?;
    assert_eq!(msg.version, Version::ISO13400_2_2012);
    match &msg.payload {
        Payload::AliveCheckRequest(v) => assert_eq!(*v, payload),
        _ => panic!("Wrong payload type"),
    }

    let data: Vec<_> = msg.into();
    assert_eq!(data, source);

    Ok(())
}

#[test]
fn test_diag() -> anyhow::Result<()> {
    let source = hex::decode("02FD8001\
    00000007\
    0E00\
    0DFF\
    021001")?;

    let payload = Diagnostic::new(
        LogicAddress::Client(0x0E00),
        LogicAddress::VMSpecific(0x0DFF),
        vec![0x02, 0x10, 0x01]
    );
    let msg = Message::try_from(source.as_ref())?;
    assert_eq!(msg.version, Version::ISO13400_2_2012);
    match &msg.payload {
        Payload::Diagnostic(v) => assert_eq!(*v, payload),
        _ => panic!("Wrong payload type"),
    }

    let data: Vec<_> = msg.into();
    assert_eq!(data, source);

    Ok(())
}
