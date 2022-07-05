use crate::types::utils::{CiString255Type, CiString50Type};

use crate::types::enumerations::DataTransferStatus;

/// DataTransfer.req PDU sent either by the Central System
/// to the Charge Point or vice versa.
pub struct DataTransferReq<'a> {
    /// This identifies the Vendor specific implementation.
    vendor_id: CiString255Type,
    /// Additional identification field.
    message_id: Option<CiString50Type>,
    /// Data without specified length or format.
    data: Option<&'a [u8]>,
}

/// DataTransfer.conf PDU sent by the Charge Point to the
/// Central System or vice versa in response to a DataTransfer.req PDU.
pub struct DataTransferConf<'a> {
    /// This indicates the success or failure of the data transfer.
    status: DataTransferStatus,
    /// Data in response to request.
    data: Option<&'a [u8]>,
}
