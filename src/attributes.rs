use crate::{SQLPOINTER, SQLULEN, SQLUSMALLINT};
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

/// Statement attributes for `SQLSetStmtAttr`
#[repr(i32)]
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SqlStatementAttribute {
    SQL_ATTR_CURSOR_SCROLLABLE = -1,
    SQL_ATTR_CURSOR_SENSITIVITY = -2,

    SQL_ATTR_QUERY_TIMEOUT = 0,
    SQL_ATTR_MAX_ROWS = 1,
    SQL_ATTR_NOSCAN = 2,
    SQL_ATTR_MAX_LENGTH = 3,
    SQL_ATTR_ASYNC_ENABLE = 4,
    SQL_ATTR_ROW_BIND_TYPE = 5,
    SQL_ATTR_CURSOR_TYPE = 6,
    SQL_ATTR_CONCURRENCY = 7,
    SQL_ATTR_KEYSET_SIZE = 8,

    SQL_ATTR_SIMULATE_CURSOR = 10,
    SQL_ATTR_RETRIEVE_DATA = 11,
    SQL_ATTR_USE_BOOKMARKS = 12,
    SQL_ATTR_ROW_NUMBER = 14,
    SQL_ATTR_ENABLE_AUTO_IPD = 15,
    SQL_ATTR_FETCH_BOOKMARK_PTR = 16,

    // Attributes which correspond to descriptor header fields
    SQL_ATTR_PARAM_BIND_OFFSET_PTR = 17,
    SQL_ATTR_PARAM_BIND_TYPE = 18,
    SQL_ATTR_PARAM_OPERATION_PTR = 19,
    SQL_ATTR_PARAM_STATUS_PTR = 20,
    SQL_ATTR_PARAMS_PROCESSED_PTR = 21,
    SQL_ATTR_PARAMSET_SIZE = 22,
    SQL_ATTR_ROW_BIND_OFFSET_PTR = 23,
    SQL_ATTR_ROW_OPERATION_PTR = 24,
    SQL_ATTR_ROW_STATUS_PTR = 25,
    SQL_ATTR_ROWS_FETCHED_PTR = 26,
    SQL_ATTR_ROW_ARRAY_SIZE = 27,

    #[cfg(feature = "odbc_version_3_80")]
    SQL_ATTR_ASYNC_STMT_EVENT = 29,

    // Only callable by the Driver Manager
    //#[cfg(feature = "odbc_version_3_80")]
    //SQL_ATTR_ASYNC_STMT_PCALLBACK = ?,
    //#[cfg(feature = "odbc_version_3_80")]
    //SQL_ATTR_ASYNC_STMT_PCONTEXT = ?,

    // TODO: These are special
    SQL_ATTR_APP_ROW_DESC = 10010,
    SQL_ATTR_APP_PARAM_DESC = 10011,
    SQL_ATTR_IMP_ROW_DESC = 10012,
    SQL_ATTR_IMP_PARAM_DESC = 10013,
    SQL_ATTR_METADATA_ID = 10014,
}
pub use self::SqlStatementAttribute::*;

/// Possible values for each row of parameter values after a call to `SQLExecute` or
/// `SQLExecDirect`
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum SQL_PARAM_STATUS {
    SQL_PARAM_SUCCESS = 0,
    SQL_PARAM_DIAG_UNAVAILABLE = 1,
    SQL_PARAM_ERROR = 5,
    SQL_PARAM_SUCCESS_WITH_INFO = 6,
    SQL_PARAM_UNUSED = 7,
}
pub use SQL_PARAM_STATUS::*;

impl From<SQL_PARAM_STATUS> for SQLPOINTER {
    fn from(source: SQL_PARAM_STATUS) -> SQLPOINTER {
        source as SQLUSMALLINT as SQLPOINTER
    }
}

/// Possible values containing row status values after a call to `SQLFetch` or `SQLFetchScroll`
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum SQL_ROW_STATUS {
    SQL_ROW_SUCCESS = 0,
    SQL_ROW_DELETED = 1,
    SQL_ROW_UPDATED = 2,
    SQL_ROW_NOROW = 3,
    SQL_ROW_ADDED = 4,
    SQL_ROW_ERROR = 5,
    SQL_ROW_SUCCESS_WITH_INFO = 6,
}
pub use SQL_ROW_STATUS::*;

impl From<SQL_ROW_STATUS> for SQLPOINTER {
    fn from(source: SQL_ROW_STATUS) -> SQLPOINTER {
        source as SQLUSMALLINT as SQLPOINTER
    }
}

/// Possible values used to ignore a parameter during execution of an SQL statement
/// `SQLExecDirect`
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum SQL_PARAM_OPERATION {
    SQL_PARAM_PROCEED = 0,
    SQL_PARAM_IGNORE = 1,
}
pub use SQL_PARAM_OPERATION::*;

impl From<SQL_PARAM_OPERATION> for SQLPOINTER {
    fn from(source: SQL_PARAM_OPERATION) -> SQLPOINTER {
        source as SQLUSMALLINT as SQLPOINTER
    }
}

/// Possible values used to ignore a row during a bulk operation using `SQLSetPos`
#[repr(u16)]
#[allow(non_camel_case_types)]
pub enum SQL_ROW_OPERATION {
    SQL_ROW_PROCEED = 0,
    SQL_ROW_IGNORE = 1,
}
pub use SQL_ROW_OPERATION::*;

impl From<SQL_ROW_OPERATION> for SQLPOINTER {
    fn from(source: SQL_ROW_OPERATION) -> SQLPOINTER {
        source as SQLUSMALLINT as SQLPOINTER
    }
}

/// Possible values for `SQL_ATTR_ASYNC_ENABLE` attribute set with `SQLSetStmtAttr` to
/// define whether a function called with the specified statement is executed asynchronously
#[cfg_attr(target_pointer_width = "64", repr(u64))]
#[cfg_attr(target_pointer_width = "32", repr(u32))]
#[allow(non_camel_case_types)]
pub enum SQL_ATTR_ASYNC_ENABLE {
    SQL_ASYNC_ENABLE_OFF = 0,
    SQL_ASYNC_ENABLE_ON = 1,
}
pub use SQL_ATTR_ASYNC_ENABLE::*;

/// Default value for `SQL_ATTR_ASYNC_ENABLE`
pub const SQL_ASYNC_ENABLE_DEFAULT: SQL_ATTR_ASYNC_ENABLE =
    SQL_ATTR_ASYNC_ENABLE::SQL_ASYNC_ENABLE_OFF;

impl From<SQL_ATTR_ASYNC_ENABLE> for SQLPOINTER {
    fn from(source: SQL_ATTR_ASYNC_ENABLE) -> SQLPOINTER {
        source as SQLULEN as SQLPOINTER
    }
}

/// Possible values for `SQL_ATTR_CONCURRENCY` attribute set with `SQLSetStmtAttr` to
/// define cursor concurrency
#[cfg_attr(target_pointer_width = "64", repr(u64))]
#[cfg_attr(target_pointer_width = "32", repr(u32))]
#[allow(non_camel_case_types)]
pub enum SQL_ATTR_CONCURRENCY {
    SQL_CONCUR_READ_ONLY = 1,
    SQL_CONCUR_LOCK = 2,
    SQL_CONCUR_ROWVER = 3,
    SQL_CONCUR_VALUES = 4,
}
pub use SQL_ATTR_CONCURRENCY::*;

/// Default value for `SQL_ATTR_CONCURRENCY`
pub const SQL_CONCUR_DEFAULT: SQL_ATTR_CONCURRENCY = SQL_ATTR_CONCURRENCY::SQL_CONCUR_READ_ONLY;

impl From<SQL_ATTR_CONCURRENCY> for SQLPOINTER {
    fn from(source: SQL_ATTR_CONCURRENCY) -> SQLPOINTER {
        source as SQLULEN as SQLPOINTER
    }
}

/// Possible values for `SQL_ATTR_CURSOR_SCROLLABLE` attribute set with `SQLSetStmtAttr` to
/// define the level of support the application requires
#[cfg_attr(target_pointer_width = "64", repr(u64))]
#[cfg_attr(target_pointer_width = "32", repr(u32))]
#[allow(non_camel_case_types)]
pub enum SQL_ATTR_CURSOR_SCROLLABLE {
    SQL_NONSCROLLABLE = 0,
    SQL_SCROLLABLE = 1,
}
pub use SQL_ATTR_CURSOR_SCROLLABLE::*;

impl From<SQL_ATTR_CURSOR_SCROLLABLE> for SQLPOINTER {
    fn from(source: SQL_ATTR_CURSOR_SCROLLABLE) -> SQLPOINTER {
        source as SQLULEN as SQLPOINTER
    }
}

/// Possible values for `SQL_ATTR_CURSOR_SENSITIVITY` attribute set with `SQLSetStmtAttr` to
/// define whether cursors on the statement handle make visible changes made to result set by
/// another cursor
#[cfg_attr(target_pointer_width = "64", repr(u64))]
#[cfg_attr(target_pointer_width = "32", repr(u32))]
#[allow(non_camel_case_types)]
pub enum SQL_ATTR_CURSOR_SENSITIVITY {
    SQL_UNSPECIFIED = 0,
    SQL_INSENSITIVE = 1,
    SQL_SENSITIVE = 2,
}
pub use SQL_ATTR_CURSOR_SENSITIVITY::*;

impl From<SQL_ATTR_CURSOR_SENSITIVITY> for SQLPOINTER {
    fn from(source: SQL_ATTR_CURSOR_SENSITIVITY) -> SQLPOINTER {
        source as SQLULEN as SQLPOINTER
    }
}

/// Possible values for `SQL_ATTR_CURSOR_TYPE` attribute set with `SQLSetStmtAttr` to
/// define the cursor type
#[cfg_attr(target_pointer_width = "64", repr(u64))]
#[cfg_attr(target_pointer_width = "32", repr(u32))]
#[allow(non_camel_case_types)]
pub enum SQL_ATTR_CURSOR_TYPE {
    SQL_CURSOR_FORWARD_ONLY = 0,
    SQL_CURSOR_KEYSET_DRIVEN = 1,
    SQL_CURSOR_DYNAMIC = 2,
    SQL_CURSOR_STATIC = 3,
}
pub use SQL_ATTR_CURSOR_TYPE::*;

/// Default value(0) for `SQL_ATTR_CURSOR_TYPE`
pub const SQL_CURSOR_TYPE_DEFAULT: SQL_ATTR_CURSOR_TYPE =
    SQL_ATTR_CURSOR_TYPE::SQL_CURSOR_FORWARD_ONLY;

impl From<SQL_ATTR_CURSOR_TYPE> for SQLPOINTER {
    fn from(source: SQL_ATTR_CURSOR_TYPE) -> SQLPOINTER {
        source as SQLULEN as SQLPOINTER
    }
}

/// Possible values for `SQL_ATTR_NOSCAN` attribute set with `SQLSetStmtAttr` to define
/// whether the driver should scan SQL strings for escape sequences
#[cfg_attr(target_pointer_width = "64", repr(u64))]
#[cfg_attr(target_pointer_width = "32", repr(u32))]
#[allow(non_camel_case_types)]
pub enum SQL_ATTR_NOSCAN {
    SQL_NOSCAN_OFF = 0,
    SQL_NOSCAN_ON = 1,
}
pub use SQL_ATTR_NOSCAN::*;

/// Default value for `SQL_ATTR_NOSCAN`
pub const SQL_NOSCAN_DEFAULT: SQL_ATTR_NOSCAN = SQL_ATTR_NOSCAN::SQL_NOSCAN_OFF;

impl From<SQL_ATTR_NOSCAN> for SQLPOINTER {
    fn from(source: SQL_ATTR_NOSCAN) -> SQLPOINTER {
        source as SQLULEN as SQLPOINTER
    }
}

/// Possible values for `SQL_ATTR_PARAM_BIND_TYPE` attribute set with `SQLSetStmtAttr` to define
/// the binding orientation to be used for dynamic parameters
#[cfg_attr(target_pointer_width = "64", repr(u64))]
#[cfg_attr(target_pointer_width = "32", repr(u32))]
#[allow(non_camel_case_types)]
pub enum SQL_ATTR_PARAM_BIND_TYPE {
    SQL_PARAM_BIND_BY_COLUMN = 0,
}
pub use SQL_ATTR_PARAM_BIND_TYPE::*;

/// Default value for `SQL_ATTR_PARAM_BIND_TYPE`
pub const SQL_PARAM_BIND_TYPE_DEFAULT: SQL_ATTR_PARAM_BIND_TYPE =
    SQL_ATTR_PARAM_BIND_TYPE::SQL_PARAM_BIND_BY_COLUMN;

impl From<SQL_ATTR_PARAM_BIND_TYPE> for SQLPOINTER {
    fn from(source: SQL_ATTR_PARAM_BIND_TYPE) -> SQLPOINTER {
        source as SQLULEN as SQLPOINTER
    }
}

/// Possible values for `SQL_ATTR_RETRIEVE_DATA` attribute set with `SQLSetStmtAttr`
#[cfg_attr(target_pointer_width = "64", repr(u64))]
#[cfg_attr(target_pointer_width = "32", repr(u32))]
#[allow(non_camel_case_types)]
pub enum SQL_ATTR_RETRIEVE_DATA {
    SQL_RD_OFF = 0,
    SQL_RD_ON = 1,
}
pub use SQL_ATTR_RETRIEVE_DATA::*;

/// Default value for `SQL_ATTR_RETRIEVE_DATA`
pub const SQL_RD_DEFAULT: SQL_ATTR_RETRIEVE_DATA = SQL_ATTR_RETRIEVE_DATA::SQL_RD_ON;

impl From<SQL_ATTR_RETRIEVE_DATA> for SQLPOINTER {
    fn from(source: SQL_ATTR_RETRIEVE_DATA) -> SQLPOINTER {
        source as SQLULEN as SQLPOINTER
    }
}

/// Possible values for `SQL_ATTR_ROW_BIND_TYPE` attribute set with `SQLSetStmtAttr` to define the
/// binding orientation to be used when `SQLFetch` or `SQLFetchScroll` is called
#[cfg_attr(target_pointer_width = "64", repr(u64))]
#[cfg_attr(target_pointer_width = "32", repr(u32))]
#[allow(non_camel_case_types)]
pub enum SQL_ATTR_ROW_BIND_TYPE {
    SQL_BIND_BY_COLUMN = 0,
    // Row-wise binding is selected by setting the value to the length of astructure or an instance
    // of a buffer into which resultcolumns will be bound
}
pub use SQL_ATTR_ROW_BIND_TYPE::*;

/// Default value for `SQL_ATTR_RETRIEVE_DATA`
pub const SQL_BIND_TYPE_DEFAULT: SQL_ATTR_ROW_BIND_TYPE =
    SQL_ATTR_ROW_BIND_TYPE::SQL_BIND_BY_COLUMN;

impl From<SQL_ATTR_ROW_BIND_TYPE> for SQLPOINTER {
    fn from(source: SQL_ATTR_ROW_BIND_TYPE) -> SQLPOINTER {
        source as SQLULEN as SQLPOINTER
    }
}

/// Possible values for `SQL_ATTR_SIMULATE_CURSOR` attribute set with `SQLSetStmtAttr` to define
/// whether drivers that simulate positioned update and delete statements guarantee that such
/// statements affect only one single row.
#[cfg_attr(target_pointer_width = "64", repr(u64))]
#[cfg_attr(target_pointer_width = "32", repr(u32))]
#[allow(non_camel_case_types)]
pub enum SQL_ATTR_SIMULATE_CURSOR {
    SQL_SC_NON_UNIQUE = 0,
    SQL_SC_TRY_UNIQUE = 1,
    SQL_SC_UNIQUE = 2,
}
pub use SQL_ATTR_SIMULATE_CURSOR::*;

impl From<SQL_ATTR_SIMULATE_CURSOR> for SQLPOINTER {
    fn from(source: SQL_ATTR_SIMULATE_CURSOR) -> SQLPOINTER {
        source as SQLULEN as SQLPOINTER
    }
}

/// Possible values for `SQL_ATTR_USE_BOOKMARKS` attribute set with `SQLSetStmtAttr` to define
/// whether an application will use bookmarks with a cursor
#[cfg_attr(target_pointer_width = "64", repr(u64))]
#[cfg_attr(target_pointer_width = "32", repr(u32))]
#[allow(non_camel_case_types)]
pub enum SQL_ATTR_USE_BOOKMARKS {
    SQL_UB_OFF = 0,
    SQL_UB_ON = 1,
    SQL_UB_VARIABLE = 2,
}
pub use SQL_ATTR_USE_BOOKMARKS::*;

/// Default value for `SQL_ATTR_USE_BOOKMARKS`
pub const SQL_UB_DEFAULT: SQL_ATTR_USE_BOOKMARKS = SQL_ATTR_USE_BOOKMARKS::SQL_UB_OFF;

impl From<SQL_ATTR_USE_BOOKMARKS> for SQLPOINTER {
    fn from(source: SQL_ATTR_USE_BOOKMARKS) -> SQLPOINTER {
        source as SQLULEN as SQLPOINTER
    }
}

/// Default value for `SQL_ATTR_QUERY_TIMEOUT` which defines query timeout (no timeout)
pub const SQL_QUERY_TIMEOUT_DEFAULT: SQLULEN = 0;

/// Default value(0) for `SQL_ATTR_KEYSET_SIZE` attribute which defines the number of rows in the
/// keyset for a keyset-driven cursor
pub const SQL_KEYSET_SIZE_DEFAULT: SQLULEN = 0;

/// Default value(0) for `SQL_ATTR_MAX_LENGTH` attribute which defines the maximum amount of data
/// that the driver returns from a character of binary column
pub const SQL_MAX_LENGTH_DEFAULT: SQLULEN = 0;

/// Default value(0) for `SQL_ATTR_MAX_ROWS` attribute which defines the maximum number of rows to
/// return to the application for a SELECT statement
pub const SQL_MAX_ROWS_DEFAULT: SQLULEN = 0;

/// Null value for `SQL_ATTR_APP_PARAM_DESC` and `SQL_ATTR_APP_ROW_DESC` attribute
/// set with `SQLSetStmtAttr` to use null descriptor
pub const SQL_NULL_DESC: SQLULEN = 0;

///// Enable/disable null-terminated strings
/////
///// Possible values for `SQL_ATTR_OUTPUT_NTS` attribute set with `SQLSetEnvAttr` to
///// enable/disable null-terminated strings
//#[allow(non_camel_case_types)]
//#[repr(i32)]
//#[derive(Debug, PartialEq, Eq, Clone, Copy)]
//pub enum SQL_ATTR_OUTPUT_NTS {
//    SQL_FALSE = 0,
//    SQL_TRUE = 1,
//}
//pub use SQL_ATTR_OUTPUT_NTS::*;
//
//impl From<SQL_ATTR_OUTPUT_NTS> for SQLPOINTER {
//    fn from(source: SQL_ATTR_OUTPUT_NTS) -> SQLPOINTER {
//        source as i32 as SQLPOINTER
//    }
//}
