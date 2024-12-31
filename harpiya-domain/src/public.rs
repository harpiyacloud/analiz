#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    #[prost(string, tag = "1")]
    pub trace_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub source: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplyHeader {
    #[prost(string, tag = "1")]
    pub trace_id: ::prost::alloc::string::String,
    #[prost(enumeration = "super::ecode::ECode", tag = "2")]
    pub code: i32,
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pager {
    /// Page Number
    #[prost(uint64, tag = "1")]
    pub index: u64,
    /// Quantity
    #[prost(uint64, tag = "2")]
    pub size: u64,
    /// The total amount
    #[prost(uint64, tag = "3")]
    pub count: u64,
    /// Disable paging
    #[prost(bool, tag = "5")]
    pub disabled: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RangeI64 {
    #[prost(int64, tag = "1")]
    pub left: i64,
    #[prost(int64, tag = "2")]
    pub right: i64,
    #[prost(enumeration = "RangeScope", tag = "3")]
    pub scope: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BetweenInt64 {
    #[prost(enumeration = "RangeScope", tag = "1")]
    pub scope: i32,
    #[prost(int64, tag = "2")]
    pub left: i64,
    #[prost(int64, tag = "3")]
    pub right: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sort {
    #[prost(string, tag = "1")]
    pub field: ::prost::alloc::string::String,
    #[prost(enumeration = "SortDirection", tag = "2")]
    pub direction: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Uuid {
    #[prost(string, tag = "1")]
    pub value: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BooleanScope {
    /// DEFAULT
    BoolAll = 0,
    /// FALSE
    BoolFalse = 1,
    /// TRUE
    BoolTrue = 2,
}
impl BooleanScope {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BooleanScope::BoolAll => "BOOL_ALL",
            BooleanScope::BoolFalse => "BOOL_FALSE",
            BooleanScope::BoolTrue => "BOOL_TRUE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BOOL_ALL" => Some(Self::BoolAll),
            "BOOL_FALSE" => Some(Self::BoolFalse),
            "BOOL_TRUE" => Some(Self::BoolTrue),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RangeScope {
    RangeAll = 0,
    RangeLeft = 1,
    RangeRight = 2,
}
impl RangeScope {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RangeScope::RangeAll => "RANGE_ALL",
            RangeScope::RangeLeft => "RANGE_LEFT",
            RangeScope::RangeRight => "RANGE_RIGHT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RANGE_ALL" => Some(Self::RangeAll),
            "RANGE_LEFT" => Some(Self::RangeLeft),
            "RANGE_RIGHT" => Some(Self::RangeRight),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SortDirection {
    SortAsc = 0,
    SortDesc = 1,
}
impl SortDirection {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SortDirection::SortAsc => "SORT_ASC",
            SortDirection::SortDesc => "SORT_DESC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SORT_ASC" => Some(Self::SortAsc),
            "SORT_DESC" => Some(Self::SortDesc),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Method {
    Create = 0,
    Update = 1,
    Delete = 2,
    /// High-order method types
    Upsert = 101,
}
impl Method {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Method::Create => "CREATE",
            Method::Update => "UPDATE",
            Method::Delete => "DELETE",
            Method::Upsert => "UPSERT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CREATE" => Some(Self::Create),
            "UPDATE" => Some(Self::Update),
            "DELETE" => Some(Self::Delete),
            "UPSERT" => Some(Self::Upsert),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TaskStatus {
    Pending = 0,
    Running = 1,
    Success = 2,
    Failure = 4,
    Finally = 5,
}
impl TaskStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TaskStatus::Pending => "PENDING",
            TaskStatus::Running => "RUNNING",
            TaskStatus::Success => "SUCCESS",
            TaskStatus::Failure => "FAILURE",
            TaskStatus::Finally => "FINALLY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PENDING" => Some(Self::Pending),
            "RUNNING" => Some(Self::Running),
            "SUCCESS" => Some(Self::Success),
            "FAILURE" => Some(Self::Failure),
            "FINALLY" => Some(Self::Finally),
            _ => None,
        }
    }
}
