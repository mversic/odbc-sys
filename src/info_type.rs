/// Information requested by SQLGetInfo
#[repr(u16)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum InfoType {
    MaxDriverConnectinos = 0,
    MaxConcurrentActivities = 1,
    DataSourceName = 2,
    // FetchDirection = 8, Deprecated in ODBC 3
    ServerName = 13,
    SearchPatternEscape = 14,
    DbmsName = 17,
    DbmsVer = 18,
    AccessibleTables = 19,
    AccessibleProcedures = 20,
    CursorCommitBehaviour = 23,
    DataSourceReadOnly = 25,
    DefaultTxnIsolation = 26,
    IdentifierCase = 28,
    IdentifierQuoteChar = 29,
    MaxColumnNameLen = 30,
    MaxCursorNameLen = 31,
    MaxSchemaNameLen = 32,
    MaxCatalogNameLen = 34,
    MaxTableNameLen = 35,
    // ScrollConcurrency = 43, deprecated in ODBC 3
    TransactionCapable = 46,
    UserName = 47,
    TransactionIsolationProtocol = 72,
    Integrity = 73,
    GetDataExtensions = 81,
    NullCollation = 85,
    AlterTable = 86,
    OrderByColumnsInSelect = 90,
    SpecialCharacters = 94,
    MaxColumnsInGroupBy = 97,
    MaxColumnsInIndex = 98,
    MaxColumnsInOrderBy = 99,
    MaxColumnsInSelect = 100,
    MaxColumnsInTable = 101,
    MaxIndexSize = 102,
    MaxRowSize = 104,
    MaxStatementLen = 105,
    MaxTablesInSelect = 106,
    MaxUserNameLen = 107,
    OuterJoinCapabilities = 115,
    XopenCliYear = 10000,
    CursorSensitivity = 10001,
    DescribeParameter = 10002,
    CatalogName = 10003,
    CollationSeq = 10004,
    MaxIdentifierLen = 10005,
    AsyncMode = 10021,
    MaxAsyncConcurrentStatements = 10022,
    AsyncDbcFunctions = 10023,
    DriverAwarePoolingSupported = 10024,
    AsyncNotification = 10025,
}
