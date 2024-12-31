#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ECode {
    /// Request successful
    Success = 0,
    /// ######### Client error #########
    BadRequest = 400000,
    /// auth token error
    ///
    /// The account is not logged in
    AuthTokenRequired = 400001,
    /// Invalid Token
    AuthTokenInvalid = 400002,
    /// Token does not exist
    AuthTokenNotFound = 400003,
    /// Token expired
    AuthTokenExpired = 400004,
    /// access error
    ///
    /// The requested resource does not exist
    AccessNotFound = 401001,
    /// No access permission
    AccessForbidden = 401002,
    /// The interface is disabled
    AccessDisabled = 401003,
    /// ######### Parameter error #########
    ///
    /// Parameter exception
    InvalidParams = 431000,
    /// user params error
    ///
    /// Parameter missing name
    UserParamsErrorNameRequired = 431001,
    /// Parameter is missing username
    UserParamsErrorUsernameRequired = 431002,
    /// Parameter missing password
    UserParamsErrorPasswordRequired = 431003,
    /// Username already exists
    UserParamsErrorUsernameExist = 431004,
    /// User does not exist
    UserParamsErrorUserNotFound = 431005,
    /// user login
    ///
    /// Invalid name
    UserParamsErrorNameInvalid = 431006,
    /// Invalid username
    UserParamsErrorUsernameInvalid = 431007,
    /// Invalid password
    UserParamsErrorPasswordInvalid = 431008,
    /// ######### Server error #########
    SystemInternalError = 500000,
    SystemErrorUnimplemented = 500001,
    /// public-service label error
    ///
    /// PUBLIC_SERVICE_ERROR_ListLabel
    PublicServiceErrorListLabel = 520001,
    /// PUBLIC_SERVICE_ERROR_OperateLabel
    PublicServiceErrorOperateLabel = 520002,
    /// PUBLIC_SERVICE_ERROR_BatchOperateLabel
    PublicServiceErrorBatchOperateLabel = 520003,
    /// public-service list enum error
    ///
    /// PUBLIC_SERVICE_ERROR_ListEnum
    PublicServiceErrorListEnum = 521001,
}
impl ECode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ECode::Success => "SUCCESS",
            ECode::BadRequest => "BAD_REQUEST",
            ECode::AuthTokenRequired => "AUTH_TOKEN_Required",
            ECode::AuthTokenInvalid => "AUTH_TOKEN_Invalid",
            ECode::AuthTokenNotFound => "AUTH_TOKEN_NotFound",
            ECode::AuthTokenExpired => "AUTH_TOKEN_Expired",
            ECode::AccessNotFound => "ACCESS_NotFound",
            ECode::AccessForbidden => "ACCESS_Forbidden",
            ECode::AccessDisabled => "ACCESS_Disabled",
            ECode::InvalidParams => "INVALID_PARAMS",
            ECode::UserParamsErrorNameRequired => "USER_PARAMS_ERROR_NameRequired",
            ECode::UserParamsErrorUsernameRequired => {
                "USER_PARAMS_ERROR_UsernameRequired"
            }
            ECode::UserParamsErrorPasswordRequired => {
                "USER_PARAMS_ERROR_PasswordRequired"
            }
            ECode::UserParamsErrorUsernameExist => "USER_PARAMS_ERROR_UsernameExist",
            ECode::UserParamsErrorUserNotFound => "USER_PARAMS_ERROR_UserNotFound",
            ECode::UserParamsErrorNameInvalid => "USER_PARAMS_ERROR_NameInvalid",
            ECode::UserParamsErrorUsernameInvalid => "USER_PARAMS_ERROR_UsernameInvalid",
            ECode::UserParamsErrorPasswordInvalid => "USER_PARAMS_ERROR_PasswordInvalid",
            ECode::SystemInternalError => "SYSTEM_INTERNAL_ERROR",
            ECode::SystemErrorUnimplemented => "SYSTEM_ERROR_Unimplemented",
            ECode::PublicServiceErrorListLabel => "PUBLIC_SERVICE_ERROR_ListLabel",
            ECode::PublicServiceErrorOperateLabel => "PUBLIC_SERVICE_ERROR_OperateLabel",
            ECode::PublicServiceErrorBatchOperateLabel => {
                "PUBLIC_SERVICE_ERROR_BatchOperateLabel"
            }
            ECode::PublicServiceErrorListEnum => "PUBLIC_SERVICE_ERROR_ListEnum",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SUCCESS" => Some(Self::Success),
            "BAD_REQUEST" => Some(Self::BadRequest),
            "AUTH_TOKEN_Required" => Some(Self::AuthTokenRequired),
            "AUTH_TOKEN_Invalid" => Some(Self::AuthTokenInvalid),
            "AUTH_TOKEN_NotFound" => Some(Self::AuthTokenNotFound),
            "AUTH_TOKEN_Expired" => Some(Self::AuthTokenExpired),
            "ACCESS_NotFound" => Some(Self::AccessNotFound),
            "ACCESS_Forbidden" => Some(Self::AccessForbidden),
            "ACCESS_Disabled" => Some(Self::AccessDisabled),
            "INVALID_PARAMS" => Some(Self::InvalidParams),
            "USER_PARAMS_ERROR_NameRequired" => Some(Self::UserParamsErrorNameRequired),
            "USER_PARAMS_ERROR_UsernameRequired" => {
                Some(Self::UserParamsErrorUsernameRequired)
            }
            "USER_PARAMS_ERROR_PasswordRequired" => {
                Some(Self::UserParamsErrorPasswordRequired)
            }
            "USER_PARAMS_ERROR_UsernameExist" => Some(Self::UserParamsErrorUsernameExist),
            "USER_PARAMS_ERROR_UserNotFound" => Some(Self::UserParamsErrorUserNotFound),
            "USER_PARAMS_ERROR_NameInvalid" => Some(Self::UserParamsErrorNameInvalid),
            "USER_PARAMS_ERROR_UsernameInvalid" => {
                Some(Self::UserParamsErrorUsernameInvalid)
            }
            "USER_PARAMS_ERROR_PasswordInvalid" => {
                Some(Self::UserParamsErrorPasswordInvalid)
            }
            "SYSTEM_INTERNAL_ERROR" => Some(Self::SystemInternalError),
            "SYSTEM_ERROR_Unimplemented" => Some(Self::SystemErrorUnimplemented),
            "PUBLIC_SERVICE_ERROR_ListLabel" => Some(Self::PublicServiceErrorListLabel),
            "PUBLIC_SERVICE_ERROR_OperateLabel" => {
                Some(Self::PublicServiceErrorOperateLabel)
            }
            "PUBLIC_SERVICE_ERROR_BatchOperateLabel" => {
                Some(Self::PublicServiceErrorBatchOperateLabel)
            }
            "PUBLIC_SERVICE_ERROR_ListEnum" => Some(Self::PublicServiceErrorListEnum),
            _ => None,
        }
    }
}
