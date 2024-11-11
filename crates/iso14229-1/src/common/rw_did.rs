//! Commons of Service 22|2E


use crate::{error::UdsError, Service, utils, Configuration};

/// Table C.1 — DID data-parameter definitions
#[repr(u16)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum DataIdentifier {
    VehicleManufacturerSpecific(u16),
    NetworkConfigurationDataForTractorTrailerApplication(u16),
    IdentificationOptionVehicleManufacturerSpecific(u16),
    BootSoftwareIdentification = 0xF180,
    ApplicationSoftwareIdentification = 0xF181,
    ApplicationDataIdentification = 0xF182,
    BootSoftwareFingerprint = 0xF183,
    ApplicationSoftwareFingerprint = 0xF184,
    ApplicationDataFingerprint = 0xF185,
    ActiveDiagnosticSession = 0xF186,
    VehicleManufacturerSparePartNumber = 0xF187,
    VehicleManufacturerECUSoftwareNumber = 0xF188,
    VehicleManufacturerECUSoftwareVersionNumber = 0xF189,
    SystemSupplierIdentifier = 0xF18A,
    ECUManufacturingDate = 0xF18B,
    ECUSerialNumber = 0xF18C,
    SupportedFunctionalUnits = 0xF18D,
    VehicleManufacturerKitAssemblyPartNumber = 0xF18E,
    ISOSAEReservedStandardized = 0xF18F,
    VIN = 0xF190,
    VehicleManufacturerECUHardwareNumber = 0xF191,
    SystemSupplierECUHardwareNumber = 0xF192,
    SystemSupplierECUHardwareVersionNumber = 0xF193,
    SystemSupplierECUSoftwareNumber = 0xF194,
    SystemSupplierECUSoftwareVersionNumber = 0xF195,
    ExhaustRegulationOrTypeApprovalNumber = 0xF196,
    SystemNameOrEngineType = 0xF197,
    RepairShopCodeOrTesterSerialNumber = 0xF198,
    ProgrammingDate = 0xF199,
    CalibrationRepairShopCodeOrCalibrationEquipmentSerialNumber = 0xF19A,
    CalibrationDate = 0xF19B,
    CalibrationEquipmentSoftwareNumber = 0xF19C,
    ECUInstallationDate = 0xF19D,
    ODXFile = 0xF19E,
    Entity = 0xF19F,
    IdentificationOptionSystemSupplierSpecific(u16),
    Periodic(u16),
    DynamicallyDefined(u16),
    OBD(u16),
    OBDMonitor(u16),
    OBDInfoType(u16),
    Tachograph(u16),
    AirbagDeployment(u16),
    NumberOfEDRDevices = 0xFA10,
    EDRIdentification = 0xFA11,
    EDRDeviceAddressInformation = 0xFA12,
    EDREntries(u16),
    SafetySystem(u16),
    SystemSupplierSpecific(u16),
    UDSVersion = 0xFF00,
    Reserved(u16),
    // ReservedForISO15765-5 = 0xFF01,
}

impl From<u16> for DataIdentifier {
    fn from(value: u16) -> Self {
        match value {
            0x0100..=0xA5FF |
            0xA800..=0xACFF |
            0xB000..=0xB1FF |
            0xC000..=0xC2FF |
            0xCF00..=0xEFFF |
            0xF010..=0xF0FF => Self::VehicleManufacturerSpecific(value),
            0xF000..=0xF00F => Self::NetworkConfigurationDataForTractorTrailerApplication(value),
            0xF100..=0xF17F |
            0xF1A0..=0xF1EF => Self::IdentificationOptionVehicleManufacturerSpecific(value),
            0xF180 => Self::BootSoftwareIdentification,
            0xF181 => Self::ApplicationSoftwareIdentification,
            0xF182 => Self::ApplicationDataIdentification,
            0xF183 => Self::BootSoftwareFingerprint,
            0xF184 => Self::ApplicationSoftwareFingerprint,
            0xF185 => Self::ApplicationDataFingerprint,
            0xF186 => Self::ActiveDiagnosticSession,
            0xF187 => Self::VehicleManufacturerSparePartNumber,
            0xF188 => Self::VehicleManufacturerECUSoftwareNumber,
            0xF189 => Self::VehicleManufacturerECUSoftwareVersionNumber,
            0xF18A => Self::SystemSupplierIdentifier,
            0xF18B => Self::ECUManufacturingDate,
            0xF18C => Self::ECUSerialNumber,
            0xF18D => Self::SupportedFunctionalUnits,
            0xF18E => Self::VehicleManufacturerKitAssemblyPartNumber,
            0xF18F => Self::ISOSAEReservedStandardized,
            0xF190 => Self::VIN,
            0xF191 => Self::VehicleManufacturerECUHardwareNumber,
            0xF192 => Self::SystemSupplierECUHardwareNumber,
            0xF193 => Self::SystemSupplierECUHardwareVersionNumber,
            0xF194 => Self::SystemSupplierECUSoftwareNumber,
            0xF195 => Self::SystemSupplierECUSoftwareVersionNumber,
            0xF196 => Self::ExhaustRegulationOrTypeApprovalNumber,
            0xF197 => Self::SystemNameOrEngineType,
            0xF198 => Self::RepairShopCodeOrTesterSerialNumber,
            0xF199 => Self::ProgrammingDate,
            0xF19A => Self::CalibrationRepairShopCodeOrCalibrationEquipmentSerialNumber,
            0xF19B => Self::CalibrationDate,
            0xF19C => Self::CalibrationEquipmentSoftwareNumber,
            0xF19D => Self::ECUInstallationDate,
            0xF19E => Self::ODXFile,
            0xF19F => Self::Entity,
            0xF1F0..=0xF1FF => Self::IdentificationOptionSystemSupplierSpecific(value),
            0xF200..=0xF2FF => Self::Periodic(value),
            0xF300..=0xF3FF => Self::DynamicallyDefined(value),
            0xF400..=0xF5FF |
            0xF700..=0xF7FF => Self::OBD(value),
            0xF600..=0xF6FF => Self::OBDMonitor(value),
            0xF800..=0xF8FF => Self::OBDInfoType(value),
            0xF900..=0xF9FF => Self::Tachograph(value),
            0xFA00..=0xFA0F => Self::AirbagDeployment(value),
            0xFA10 => Self::NumberOfEDRDevices,
            0xFA11 => Self::EDRIdentification,
            0xFA12 => Self::EDRDeviceAddressInformation,
            0xFA13..=0xFA18 => Self::EDREntries(value),
            0xFA19..=0xFAFF => Self::SafetySystem(value),
            0xFD00..=0xFEFF => Self::SystemSupplierSpecific(value),
            0xFF00 => Self::UDSVersion,
            _ => Self::Reserved(value),
        }
    }
}

impl Into<u16> for DataIdentifier {
    fn into(self) -> u16 {
        match self {
            Self::BootSoftwareIdentification => 0xF180,
            Self::ApplicationSoftwareIdentification => 0xF181,
            Self::ApplicationDataIdentification => 0xF182,
            Self::BootSoftwareFingerprint => 0xF183,
            Self::ApplicationSoftwareFingerprint => 0xF184,
            Self::ApplicationDataFingerprint => 0xF185,
            Self::ActiveDiagnosticSession => 0xF186,
            Self::VehicleManufacturerSparePartNumber => 0xF187,
            Self::VehicleManufacturerECUSoftwareNumber => 0xF188,
            Self::VehicleManufacturerECUSoftwareVersionNumber => 0xF189,
            Self::SystemSupplierIdentifier => 0xF18A,
            Self::ECUManufacturingDate => 0xF18B,
            Self::ECUSerialNumber => 0xF18C,
            Self::SupportedFunctionalUnits => 0xF18D,
            Self::VehicleManufacturerKitAssemblyPartNumber => 0xF18E,
            Self::ISOSAEReservedStandardized => 0xF18F,
            Self::VIN => 0xF190,
            Self::VehicleManufacturerECUHardwareNumber => 0xF191,
            Self::SystemSupplierECUHardwareNumber => 0xF192,
            Self::SystemSupplierECUHardwareVersionNumber => 0xF193,
            Self::SystemSupplierECUSoftwareNumber => 0xF194,
            Self::SystemSupplierECUSoftwareVersionNumber => 0xF195,
            Self::ExhaustRegulationOrTypeApprovalNumber => 0xF196,
            Self::SystemNameOrEngineType => 0xF197,
            Self::RepairShopCodeOrTesterSerialNumber => 0xF198,
            Self::ProgrammingDate => 0xF199,
            Self::CalibrationRepairShopCodeOrCalibrationEquipmentSerialNumber => 0xF19A,
            Self::CalibrationDate => 0xF19B,
            Self::CalibrationEquipmentSoftwareNumber => 0xF19C,
            Self::ECUInstallationDate => 0xF19D,
            Self::ODXFile => 0xF19E,
            Self::Entity => 0xF19F,
            Self::VehicleManufacturerSpecific(v) |
            Self::NetworkConfigurationDataForTractorTrailerApplication(v) |
            Self::IdentificationOptionVehicleManufacturerSpecific(v) |
            Self::IdentificationOptionSystemSupplierSpecific(v) |
            Self::Periodic(v) |
            Self::DynamicallyDefined(v) |
            Self::OBD(v) |
            Self::OBDMonitor(v) |
            Self::OBDInfoType(v) |
            Self::Tachograph(v) |
            Self::AirbagDeployment(v) |
            Self::EDREntries(v) |
            Self::SafetySystem(v) |
            Self::SystemSupplierSpecific(v) => v,
            Self::NumberOfEDRDevices => 0xFA10,
            Self::EDRIdentification => 0xFA11,
            Self::EDRDeviceAddressInformation => 0xFA12,
            Self::UDSVersion => 0xFF00,
            Self::Reserved(v) => v,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DIDData {
    pub did: DataIdentifier,
    pub data: Vec<u8>,
}

impl DIDData {
    pub fn new(
        did: DataIdentifier,
        data: Vec<u8>,
        cfg: &Configuration,
    ) -> Result<Self, UdsError> {
        let &did_len = cfg.did_cfg.get(&did)
            .ok_or(UdsError::DidNotSupported(did))?;
        utils::data_length_check(data.len(), did_len, true)?;

        Ok(Self { did, data })
    }
}

// impl<'a> TryFrom<&'a [u8]> for DIDData {
//     type Error = Error;
//     fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
//         let data_len = data.len();
//         utils::data_length_check(data_len, 2, false)?;
//
//         let mut offset = 0;
//         let did = DataIdentifier::from(
//             u16::from_be_bytes([data[offset], data[offset + 1]])
//         );
//         offset += 2;
//
//         Ok(Self { did, data: data[offset..].to_vec() })
//     }
// }

impl Into<Vec<u8>> for DIDData {
    fn into(mut self) -> Vec<u8> {
        let did: u16 = self.did.into();
        let mut result = did.to_be_bytes().to_vec();
        result.append(&mut self.data);

        result
    }
}


