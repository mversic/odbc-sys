use std::os::raw::c_void;

/// Governs behaviour of EnvironmentAttribute
#[repr(i32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EnvironmentAttribute {
    SQL_ATTR_ODBC_VERSION = 200,
    SQL_ATTR_CONNECTION_POOLING = 201,
    SQL_ATTR_CP_MATCH = 202,
    // For private driver manager
    SQL_ATTR_APPLICATION_KEY = 203,
    SQL_ATTR_OUTPUT_NTS = 10001,
}
pub use EnvironmentAttribute::*;

/// ODBC verions
///
/// Used in conjunction with `SQL_ATTR_ODBC_VERSION` and `SQLSetEnvAttr` to declare the ODBC
/// version used by the application.
#[allow(non_camel_case_types)]
#[repr(i32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OdbcVersion {
    SQL_OV_ODBC2 = 2,
    SQL_OV_ODBC3 = 3,
    #[cfg(feature = "odbc_version_3_80")]
    SQL_OV_ODBC3_80 = 380,
    #[cfg(feature = "odbc_version_4")]
    SQL_OV_ODBC4 = 400,
}
pub use OdbcVersion::*;

impl From<OdbcVersion> for *mut c_void {
    fn from(source: OdbcVersion) -> *mut c_void {
        source as i32 as *mut c_void
    }
}

/// Connection attributes for `SQLSetConnectAttr`
#[repr(i32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SqlConnectionAttribute {
    SQL_ATTR_ASYNC_ENABLE = 4,
    SQL_ATTR_ACCESS_MODE = 101,
    SQL_ATTR_AUTOCOMMIT = 102,
    SQL_ATTR_LOGIN_TIMEOUT = 103,
    SQL_ATTR_TRACE = 104,
    SQL_ATTR_TRACEFILE = 105,
    SQL_ATTR_TRANSLATE_LIB = 106,
    SQL_ATTR_TRANSLATE_OPTION = 107,
    SQL_ATTR_TXN_ISOLATION = 108,
    SQL_ATTR_CURRENT_CATALOG = 109,
    SQL_ATTR_ODBC_CURSORS = 110,
    SQL_ATTR_QUIET_MODE = 111,
    SQL_ATTR_PACKET_SIZE = 112,
    SQL_ATTR_CONNECTION_TIMEOUT = 113,
    SQL_ATTR_DISCONNECT_BEHAVIOR = 114,
    #[cfg(feature = "odbc_version_3_80")]
    SQL_ATTR_ASYNC_DBC_FUNCTIONS_ENABLE = 117,

//    #[cfg(feature = "odbc_version_3_80")]
//    SQL_ATTR_ASYNC_DBC_PCALLBACK = x,
//    #[cfg(feature = "odbc_version_3_80")]
//    SQL_ATTR_ASYNC_DBC_PCONTEXT = y,
//    #[cfg(feature = "odbc_version_3_80")]
//    SQL_ATTR_DBC_INFO_TOKEN = z,

    SQL_ATTR_ASYNC_DBC_EVENT = 119,
    SQL_ATTR_ENLIST_IN_DTC = 1207,
    SQL_ATTR_ENLIST_IN_XA = 1208,
    SQL_ATTR_CONNECTION_DEAD = 1209,
    SQL_ATTR_AUTO_IPD = 10001,
    SQL_ATTR_METADATA_ID = 10014,
}
pub use self::SqlConnectionAttribute::*;

