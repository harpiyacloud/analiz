#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct User {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub email: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub username: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub password: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub enabled: bool,
    #[prost(string, tag = "6")]
    pub first_name: ::prost::alloc::string::String,
    #[prost(string, optional, tag = "7")]
    pub middle_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "8")]
    pub last_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "9")]
    pub full_name: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub language: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub time_zone: ::prost::alloc::string::String,
    #[prost(bool, tag = "12")]
    pub send_welcome_email: bool,
    #[prost(bool, tag = "13")]
    pub unsubscribed: bool,
    #[prost(string, optional, tag = "14")]
    pub user_image: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "15")]
    pub gender: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "16")]
    pub birth_date: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "17")]
    pub phone: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "18")]
    pub location: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "19")]
    pub bio: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "20")]
    pub mobile_no: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "21")]
    pub banner_image: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "22")]
    pub new_password: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, tag = "23")]
    pub logout_all_sessions: bool,
    #[prost(string, optional, tag = "24")]
    pub reset_password_key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "25")]
    pub last_reset_password_key_generated_on: ::core::option::Option<i64>,
    #[prost(int64, optional, tag = "26")]
    pub last_password_reset_date: ::core::option::Option<i64>,
    #[prost(string, optional, tag = "27")]
    pub email_signature: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, tag = "28")]
    pub simultaneous_sessions: bool,
    #[prost(string, optional, tag = "29")]
    pub last_ip: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "30")]
    pub last_login: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int32, optional, tag = "31")]
    pub login_after: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "32")]
    pub login_before: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "33")]
    pub user_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(int64, optional, tag = "34")]
    pub last_active: ::core::option::Option<i64>,
    #[prost(bool, tag = "35")]
    pub bypass_restrict_ip_check_if_2fa_enabled: bool,
    #[prost(string, optional, tag = "36")]
    pub last_known_versions: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "37")]
    pub api_key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "38")]
    pub api_secret: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, tag = "1001")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag = "1002")]
    pub modified_by: ::prost::alloc::string::String,
    #[prost(int64, tag = "1003")]
    pub creation: i64,
    #[prost(int64, tag = "1004")]
    pub modified: i64,
    #[prost(int64, tag = "1005")]
    pub deleted: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserFilter {
    #[prost(string, repeated, tag = "1")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "2")]
    pub username: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub email: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "4")]
    pub first_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "5")]
    pub last_name: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "101")]
    pub delete_time_range: ::core::option::Option<super::public::BetweenInt64>,
    #[prost(message, optional, tag = "102")]
    pub create_time_range: ::core::option::Option<super::public::BetweenInt64>,
    #[prost(message, optional, tag = "103")]
    pub update_time_range: ::core::option::Option<super::public::BetweenInt64>,
    #[prost(string, repeated, tag = "104")]
    pub owners: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "105")]
    pub modifiers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration = "super::public::BooleanScope", tag = "500")]
    pub deleted: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUserParams {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::public::Header>,
    #[prost(message, optional, tag = "2")]
    pub pager: ::core::option::Option<super::public::Pager>,
    #[prost(message, repeated, tag = "3")]
    pub sorts: ::prost::alloc::vec::Vec<super::public::Sort>,
    #[prost(message, optional, tag = "4")]
    pub filter: ::core::option::Option<UserFilter>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListUserReply {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::public::ReplyHeader>,
    #[prost(message, optional, tag = "2")]
    pub pager: ::core::option::Option<super::public::Pager>,
    #[prost(message, repeated, tag = "3")]
    pub data: ::prost::alloc::vec::Vec<User>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperateUserParams {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::public::Header>,
    #[prost(enumeration = "super::public::Method", tag = "2")]
    pub method: i32,
    #[prost(message, optional, tag = "3")]
    pub data: ::core::option::Option<User>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperateUserReply {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::public::ReplyHeader>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<User>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchOperateUserParams {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::public::Header>,
    #[prost(enumeration = "super::public::Method", tag = "2")]
    pub method: i32,
    #[prost(message, repeated, tag = "3")]
    pub data: ::prost::alloc::vec::Vec<User>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchOperateUserReply {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::public::ReplyHeader>,
}
/// Generated server implementations.
pub mod user_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with UserServiceServer.
    #[async_trait]
    pub trait UserService: Send + Sync + 'static {
        async fn list_user(
            &self,
            request: tonic::Request<super::ListUserParams>,
        ) -> std::result::Result<tonic::Response<super::ListUserReply>, tonic::Status>;
        async fn operate_user(
            &self,
            request: tonic::Request<super::OperateUserParams>,
        ) -> std::result::Result<
            tonic::Response<super::OperateUserReply>,
            tonic::Status,
        >;
        async fn batch_operate_user(
            &self,
            request: tonic::Request<super::BatchOperateUserParams>,
        ) -> std::result::Result<
            tonic::Response<super::BatchOperateUserReply>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct UserServiceServer<T: UserService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: UserService> UserServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for UserServiceServer<T>
    where
        T: UserService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/user_service.UserService/ListUser" => {
                    #[allow(non_camel_case_types)]
                    struct ListUserSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::ListUserParams>
                    for ListUserSvc<T> {
                        type Response = super::ListUserReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListUserParams>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::list_user(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/user_service.UserService/OperateUser" => {
                    #[allow(non_camel_case_types)]
                    struct OperateUserSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::OperateUserParams>
                    for OperateUserSvc<T> {
                        type Response = super::OperateUserReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OperateUserParams>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::operate_user(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = OperateUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/user_service.UserService/BatchOperateUser" => {
                    #[allow(non_camel_case_types)]
                    struct BatchOperateUserSvc<T: UserService>(pub Arc<T>);
                    impl<
                        T: UserService,
                    > tonic::server::UnaryService<super::BatchOperateUserParams>
                    for BatchOperateUserSvc<T> {
                        type Response = super::BatchOperateUserReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchOperateUserParams>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as UserService>::batch_operate_user(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = BatchOperateUserSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: UserService> Clone for UserServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: UserService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: UserService> tonic::server::NamedService for UserServiceServer<T> {
        const NAME: &'static str = "user_service.UserService";
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthToken {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(int32, tag = "2")]
    pub user_id: i32,
    #[prost(string, tag = "3")]
    pub access_token: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub refresh_token: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub issued_time: i64,
    #[prost(int64, tag = "6")]
    pub expire_time: i64,
    #[prost(int64, tag = "1001")]
    pub deleted: i64,
    #[prost(int64, tag = "1002")]
    pub creation: i64,
    #[prost(int64, tag = "1003")]
    pub modified: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AuthTokenFilter {
    #[prost(int32, repeated, tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, tag = "2")]
    pub user_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(string, repeated, tag = "3")]
    pub access_tokens: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "4")]
    pub refresh_tokens: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "21")]
    pub issued_time_range: ::core::option::Option<super::public::BetweenInt64>,
    #[prost(message, optional, tag = "22")]
    pub expire_time_range: ::core::option::Option<super::public::BetweenInt64>,
    #[prost(message, optional, tag = "101")]
    pub delete_time_range: ::core::option::Option<super::public::BetweenInt64>,
    #[prost(message, optional, tag = "102")]
    pub create_time_range: ::core::option::Option<super::public::BetweenInt64>,
    #[prost(message, optional, tag = "103")]
    pub update_time_range: ::core::option::Option<super::public::BetweenInt64>,
    #[prost(enumeration = "super::public::BooleanScope", tag = "500")]
    pub deleted: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAuthTokenParams {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::public::Header>,
    #[prost(message, optional, tag = "2")]
    pub pager: ::core::option::Option<super::public::Pager>,
    #[prost(message, repeated, tag = "3")]
    pub sorts: ::prost::alloc::vec::Vec<super::public::Sort>,
    #[prost(message, optional, tag = "4")]
    pub filter: ::core::option::Option<AuthTokenFilter>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAuthTokenReply {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::public::ReplyHeader>,
    #[prost(message, optional, tag = "2")]
    pub pager: ::core::option::Option<super::public::Pager>,
    #[prost(message, repeated, tag = "3")]
    pub data: ::prost::alloc::vec::Vec<AuthToken>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperateAuthTokenParams {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::public::Header>,
    #[prost(enumeration = "super::public::Method", tag = "2")]
    pub method: i32,
    #[prost(message, optional, tag = "3")]
    pub data: ::core::option::Option<AuthToken>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperateAuthTokenReply {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::public::ReplyHeader>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<AuthToken>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchOperateAuthTokenParams {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::public::Header>,
    #[prost(enumeration = "super::public::Method", tag = "2")]
    pub method: i32,
    #[prost(message, repeated, tag = "3")]
    pub data: ::prost::alloc::vec::Vec<AuthToken>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchOperateAuthTokenReply {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::public::ReplyHeader>,
}
