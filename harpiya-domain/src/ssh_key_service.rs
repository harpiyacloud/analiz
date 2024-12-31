#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SshKey {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub public_key: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub private_key: ::prost::alloc::string::String,
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
pub struct SshKeyFilter {
    #[prost(string, repeated, tag = "1")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
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
pub struct ListSshKeyParams {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::public::Header>,
    #[prost(message, optional, tag = "2")]
    pub pager: ::core::option::Option<super::public::Pager>,
    #[prost(message, repeated, tag = "3")]
    pub sorts: ::prost::alloc::vec::Vec<super::public::Sort>,
    #[prost(message, optional, tag = "4")]
    pub filter: ::core::option::Option<SshKeyFilter>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSshKeyReply {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::public::ReplyHeader>,
    #[prost(message, optional, tag = "2")]
    pub pager: ::core::option::Option<super::public::Pager>,
    #[prost(message, repeated, tag = "3")]
    pub data: ::prost::alloc::vec::Vec<SshKey>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperateSshKeyReply {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::public::ReplyHeader>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<SshKey>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperateSshKeyParams {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::public::Header>,
    #[prost(enumeration = "super::public::Method", tag = "2")]
    pub method: i32,
    #[prost(message, optional, tag = "3")]
    pub data: ::core::option::Option<SshKey>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchOperateSshKeyParams {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::public::Header>,
    #[prost(enumeration = "super::public::Method", tag = "2")]
    pub method: i32,
    #[prost(message, repeated, tag = "3")]
    pub data: ::prost::alloc::vec::Vec<SshKey>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchOperateSshKeyReply {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::public::ReplyHeader>,
}
/// Generated server implementations.
pub mod ssh_key_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with SshKeyServiceServer.
    #[async_trait]
    pub trait SshKeyService: Send + Sync + 'static {
        async fn list_ssh_key(
            &self,
            request: tonic::Request<super::ListSshKeyParams>,
        ) -> std::result::Result<tonic::Response<super::ListSshKeyReply>, tonic::Status>;
        async fn operate_ssh_key(
            &self,
            request: tonic::Request<super::OperateSshKeyParams>,
        ) -> std::result::Result<
            tonic::Response<super::OperateSshKeyReply>,
            tonic::Status,
        >;
        async fn batch_operate_ssh_key(
            &self,
            request: tonic::Request<super::BatchOperateSshKeyParams>,
        ) -> std::result::Result<
            tonic::Response<super::BatchOperateSshKeyReply>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct SshKeyServiceServer<T: SshKeyService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: SshKeyService> SshKeyServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SshKeyServiceServer<T>
    where
        T: SshKeyService,
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
                "/ssh_key_service.SSHKeyService/ListSSHKey" => {
                    #[allow(non_camel_case_types)]
                    struct ListSSHKeySvc<T: SshKeyService>(pub Arc<T>);
                    impl<
                        T: SshKeyService,
                    > tonic::server::UnaryService<super::ListSshKeyParams>
                    for ListSSHKeySvc<T> {
                        type Response = super::ListSshKeyReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListSshKeyParams>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SshKeyService>::list_ssh_key(&inner, request).await
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
                        let method = ListSSHKeySvc(inner);
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
                "/ssh_key_service.SSHKeyService/OperateSSHKey" => {
                    #[allow(non_camel_case_types)]
                    struct OperateSSHKeySvc<T: SshKeyService>(pub Arc<T>);
                    impl<
                        T: SshKeyService,
                    > tonic::server::UnaryService<super::OperateSshKeyParams>
                    for OperateSSHKeySvc<T> {
                        type Response = super::OperateSshKeyReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::OperateSshKeyParams>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SshKeyService>::operate_ssh_key(&inner, request).await
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
                        let method = OperateSSHKeySvc(inner);
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
                "/ssh_key_service.SSHKeyService/BatchOperateSSHKey" => {
                    #[allow(non_camel_case_types)]
                    struct BatchOperateSSHKeySvc<T: SshKeyService>(pub Arc<T>);
                    impl<
                        T: SshKeyService,
                    > tonic::server::UnaryService<super::BatchOperateSshKeyParams>
                    for BatchOperateSSHKeySvc<T> {
                        type Response = super::BatchOperateSshKeyReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchOperateSshKeyParams>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as SshKeyService>::batch_operate_ssh_key(&inner, request)
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
                        let method = BatchOperateSSHKeySvc(inner);
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
    impl<T: SshKeyService> Clone for SshKeyServiceServer<T> {
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
    impl<T: SshKeyService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: SshKeyService> tonic::server::NamedService for SshKeyServiceServer<T> {
        const NAME: &'static str = "ssh_key_service.SSHKeyService";
    }
}
