// Commons of Service 14

// use crate::error::Error;
// use crate::utils;
//
// #[derive(Debug, Clone)]
// pub struct ClearDiagnostic {
//     group: utils::U24,
//     #[cfg(any(feature = "std2020"))]
//     mem_sel: Option<u8>, // Standard 2020 added
// }
//
// impl ClearDiagnostic {
//     #[cfg(any(feature = "std2020"))]
//     pub fn new(
//         group: utils::U24,
//         mem_sel: Option<u8>,
//     ) -> Self {
//         Self { group, mem_sel }
//     }
//
//     #[cfg(not(any(feature = "std2020")))]
//     pub fn new(
//         group: utils::U24,
//     ) -> Self {
//         Self { group }
//     }
//
//     pub fn group(&self) -> u32 {
//         self.group.0
//     }
//
//     #[cfg(not(any(feature = "std2020")))]
//     pub fn memory_selection(&self) -> Option<u8> {
//         self.mem_sel
//     }
// }
//
// impl<'a> TryFrom<&'a [u8]> for ClearDiagnostic {
//     type Error = Error;
//
//     #[cfg(any(feature = "std2020"))]
//     fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
//         let data_len = data.len();
//         utils::data_length_check(data_len, 3, false)?;
//         let mut offset = 0;
//         let group = utils::U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
//         offset += 3;
//
//         let mem_selection = if data_len > offset {
//             Some(data[offset])
//         }
//         else {
//             None
//         };
//
//         Ok(Self::new(group, mem_selection))
//     }
//
//     #[cfg(not(any(feature = "std2020")))]
//     fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
//         let data_len = data.len();
//         utils::data_length_check(data_len, 3)?;
//         let mut offset = 0;
//
//         let group = utils::U24::from_be_bytes([0, data[offset], data[offset + 1], data[offset + 2]]);
//
//         Ok(Self::new(group))
//     }
// }
//
// impl Into<Vec<u8>> for ClearDiagnostic {
//     fn into(self) -> Vec<u8> {
//         let mut result: Vec<_> = self.group.into();
//         #[cfg(any(feature = "std2020"))]
//         if let Some(v) = self.mem_sel {
//             result.push(v);
//         }
//
//         result
//     }
// }
