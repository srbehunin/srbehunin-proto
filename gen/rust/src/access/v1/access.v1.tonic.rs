// @generated
/// Generated client implementations.
pub mod account_service_client {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value
    )]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct AccountServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AccountServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AccountServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::Body>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + std::marker::Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + std::marker::Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AccountServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                    http::Request<tonic::body::Body>,
                    Response = http::Response<
                        <T as tonic::client::GrpcService<tonic::body::Body>>::ResponseBody,
                    >,
                >,
            <T as tonic::codegen::Service<http::Request<tonic::body::Body>>>::Error:
                Into<StdError> + std::marker::Send + std::marker::Sync,
        {
            AccountServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn create_account(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAccountRequest>,
        ) -> std::result::Result<tonic::Response<super::CreateAccountResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic_prost::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/access.v1.AccountService/CreateAccount");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("access.v1.AccountService", "CreateAccount"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn verify_password(
            &mut self,
            request: impl tonic::IntoRequest<super::VerifyPasswordRequest>,
        ) -> std::result::Result<tonic::Response<super::VerifyPasswordResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic_prost::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/access.v1.AccountService/VerifyPassword");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "access.v1.AccountService",
                "VerifyPassword",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn logout(
            &mut self,
            request: impl tonic::IntoRequest<super::LogoutRequest>,
        ) -> std::result::Result<tonic::Response<super::LogoutResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/access.v1.AccountService/Logout");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("access.v1.AccountService", "Logout"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn validate_session(
            &mut self,
            request: impl tonic::IntoRequest<super::ValidateSessionRequest>,
        ) -> std::result::Result<tonic::Response<super::ValidateSessionResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic_prost::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/access.v1.AccountService/ValidateSession");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "access.v1.AccountService",
                "ValidateSession",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_account(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccountRequest>,
        ) -> std::result::Result<tonic::Response<super::GetAccountResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/access.v1.AccountService/GetAccount");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("access.v1.AccountService", "GetAccount"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn add_credential(
            &mut self,
            request: impl tonic::IntoRequest<super::AddCredentialRequest>,
        ) -> std::result::Result<tonic::Response<super::AddCredentialResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic_prost::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/access.v1.AccountService/AddCredential");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("access.v1.AccountService", "AddCredential"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove_credential(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveCredentialRequest>,
        ) -> std::result::Result<tonic::Response<super::RemoveCredentialResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic_prost::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/access.v1.AccountService/RemoveCredential");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "access.v1.AccountService",
                "RemoveCredential",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn change_name(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeNameRequest>,
        ) -> std::result::Result<tonic::Response<super::ChangeNameResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/access.v1.AccountService/ChangeName");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("access.v1.AccountService", "ChangeName"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn request_password_reset(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestPasswordResetRequest>,
        ) -> std::result::Result<tonic::Response<super::RequestPasswordResetResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/access.v1.AccountService/RequestPasswordReset",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "access.v1.AccountService",
                "RequestPasswordReset",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn reset_password(
            &mut self,
            request: impl tonic::IntoRequest<super::ResetPasswordRequest>,
        ) -> std::result::Result<tonic::Response<super::ResetPasswordResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic_prost::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/access.v1.AccountService/ResetPassword");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("access.v1.AccountService", "ResetPassword"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn force_logout(
            &mut self,
            request: impl tonic::IntoRequest<super::ForceLogoutRequest>,
        ) -> std::result::Result<tonic::Response<super::ForceLogoutResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic_prost::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/access.v1.AccountService/ForceLogout");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("access.v1.AccountService", "ForceLogout"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn force_reset_password(
            &mut self,
            request: impl tonic::IntoRequest<super::ForceResetPasswordRequest>,
        ) -> std::result::Result<tonic::Response<super::ForceResetPasswordResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/access.v1.AccountService/ForceResetPassword",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "access.v1.AccountService",
                "ForceResetPassword",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn lock_account(
            &mut self,
            request: impl tonic::IntoRequest<super::LockAccountRequest>,
        ) -> std::result::Result<tonic::Response<super::LockAccountResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic_prost::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/access.v1.AccountService/LockAccount");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("access.v1.AccountService", "LockAccount"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn unlock_account(
            &mut self,
            request: impl tonic::IntoRequest<super::UnlockAccountRequest>,
        ) -> std::result::Result<tonic::Response<super::UnlockAccountResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic_prost::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/access.v1.AccountService/UnlockAccount");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("access.v1.AccountService", "UnlockAccount"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod account_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with AccountServiceServer.

    pub trait AccountService: std::marker::Send + std::marker::Sync + 'static {
        async fn create_account(
            &self,
            request: tonic::Request<super::CreateAccountRequest>,
        ) -> std::result::Result<tonic::Response<super::CreateAccountResponse>, tonic::Status>;
        async fn verify_password(
            &self,
            request: tonic::Request<super::VerifyPasswordRequest>,
        ) -> std::result::Result<tonic::Response<super::VerifyPasswordResponse>, tonic::Status>;
        async fn logout(
            &self,
            request: tonic::Request<super::LogoutRequest>,
        ) -> std::result::Result<tonic::Response<super::LogoutResponse>, tonic::Status>;
        async fn validate_session(
            &self,
            request: tonic::Request<super::ValidateSessionRequest>,
        ) -> std::result::Result<tonic::Response<super::ValidateSessionResponse>, tonic::Status>;
        async fn get_account(
            &self,
            request: tonic::Request<super::GetAccountRequest>,
        ) -> std::result::Result<tonic::Response<super::GetAccountResponse>, tonic::Status>;
        async fn add_credential(
            &self,
            request: tonic::Request<super::AddCredentialRequest>,
        ) -> std::result::Result<tonic::Response<super::AddCredentialResponse>, tonic::Status>;
        async fn remove_credential(
            &self,
            request: tonic::Request<super::RemoveCredentialRequest>,
        ) -> std::result::Result<tonic::Response<super::RemoveCredentialResponse>, tonic::Status>;
        async fn change_name(
            &self,
            request: tonic::Request<super::ChangeNameRequest>,
        ) -> std::result::Result<tonic::Response<super::ChangeNameResponse>, tonic::Status>;
        async fn request_password_reset(
            &self,
            request: tonic::Request<super::RequestPasswordResetRequest>,
        ) -> std::result::Result<tonic::Response<super::RequestPasswordResetResponse>, tonic::Status>;
        async fn reset_password(
            &self,
            request: tonic::Request<super::ResetPasswordRequest>,
        ) -> std::result::Result<tonic::Response<super::ResetPasswordResponse>, tonic::Status>;
        async fn force_logout(
            &self,
            request: tonic::Request<super::ForceLogoutRequest>,
        ) -> std::result::Result<tonic::Response<super::ForceLogoutResponse>, tonic::Status>;
        async fn force_reset_password(
            &self,
            request: tonic::Request<super::ForceResetPasswordRequest>,
        ) -> std::result::Result<tonic::Response<super::ForceResetPasswordResponse>, tonic::Status>;
        async fn lock_account(
            &self,
            request: tonic::Request<super::LockAccountRequest>,
        ) -> std::result::Result<tonic::Response<super::LockAccountResponse>, tonic::Status>;
        async fn unlock_account(
            &self,
            request: tonic::Request<super::UnlockAccountRequest>,
        ) -> std::result::Result<tonic::Response<super::UnlockAccountResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct AccountServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> AccountServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AccountServiceServer<T>
    where
        T: AccountService,
        B: Body + std::marker::Send + 'static,
        B::Error: Into<StdError> + std::marker::Send + 'static,
    {
        type Response = http::Response<tonic::body::Body>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            match req.uri().path() {
                "/access.v1.AccountService/CreateAccount" => {
                    #[allow(non_camel_case_types)]
                    struct CreateAccountSvc<T: AccountService>(pub Arc<T>);
                    impl<T: AccountService> tonic::server::UnaryService<super::CreateAccountRequest>
                        for CreateAccountSvc<T>
                    {
                        type Response = super::CreateAccountResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateAccountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AccountService>::create_account(&inner, request).await
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
                        let method = CreateAccountSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/access.v1.AccountService/VerifyPassword" => {
                    #[allow(non_camel_case_types)]
                    struct VerifyPasswordSvc<T: AccountService>(pub Arc<T>);
                    impl<T: AccountService>
                        tonic::server::UnaryService<super::VerifyPasswordRequest>
                        for VerifyPasswordSvc<T>
                    {
                        type Response = super::VerifyPasswordResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::VerifyPasswordRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AccountService>::verify_password(&inner, request).await
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
                        let method = VerifyPasswordSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/access.v1.AccountService/Logout" => {
                    #[allow(non_camel_case_types)]
                    struct LogoutSvc<T: AccountService>(pub Arc<T>);
                    impl<T: AccountService> tonic::server::UnaryService<super::LogoutRequest> for LogoutSvc<T> {
                        type Response = super::LogoutResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LogoutRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut =
                                async move { <T as AccountService>::logout(&inner, request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let method = LogoutSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/access.v1.AccountService/ValidateSession" => {
                    #[allow(non_camel_case_types)]
                    struct ValidateSessionSvc<T: AccountService>(pub Arc<T>);
                    impl<T: AccountService>
                        tonic::server::UnaryService<super::ValidateSessionRequest>
                        for ValidateSessionSvc<T>
                    {
                        type Response = super::ValidateSessionResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ValidateSessionRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AccountService>::validate_session(&inner, request).await
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
                        let method = ValidateSessionSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/access.v1.AccountService/GetAccount" => {
                    #[allow(non_camel_case_types)]
                    struct GetAccountSvc<T: AccountService>(pub Arc<T>);
                    impl<T: AccountService> tonic::server::UnaryService<super::GetAccountRequest> for GetAccountSvc<T> {
                        type Response = super::GetAccountResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAccountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AccountService>::get_account(&inner, request).await
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
                        let method = GetAccountSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/access.v1.AccountService/AddCredential" => {
                    #[allow(non_camel_case_types)]
                    struct AddCredentialSvc<T: AccountService>(pub Arc<T>);
                    impl<T: AccountService> tonic::server::UnaryService<super::AddCredentialRequest>
                        for AddCredentialSvc<T>
                    {
                        type Response = super::AddCredentialResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddCredentialRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AccountService>::add_credential(&inner, request).await
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
                        let method = AddCredentialSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/access.v1.AccountService/RemoveCredential" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveCredentialSvc<T: AccountService>(pub Arc<T>);
                    impl<T: AccountService>
                        tonic::server::UnaryService<super::RemoveCredentialRequest>
                        for RemoveCredentialSvc<T>
                    {
                        type Response = super::RemoveCredentialResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveCredentialRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AccountService>::remove_credential(&inner, request).await
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
                        let method = RemoveCredentialSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/access.v1.AccountService/ChangeName" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeNameSvc<T: AccountService>(pub Arc<T>);
                    impl<T: AccountService> tonic::server::UnaryService<super::ChangeNameRequest> for ChangeNameSvc<T> {
                        type Response = super::ChangeNameResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChangeNameRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AccountService>::change_name(&inner, request).await
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
                        let method = ChangeNameSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/access.v1.AccountService/RequestPasswordReset" => {
                    #[allow(non_camel_case_types)]
                    struct RequestPasswordResetSvc<T: AccountService>(pub Arc<T>);
                    impl<T: AccountService>
                        tonic::server::UnaryService<super::RequestPasswordResetRequest>
                        for RequestPasswordResetSvc<T>
                    {
                        type Response = super::RequestPasswordResetResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestPasswordResetRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AccountService>::request_password_reset(&inner, request).await
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
                        let method = RequestPasswordResetSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/access.v1.AccountService/ResetPassword" => {
                    #[allow(non_camel_case_types)]
                    struct ResetPasswordSvc<T: AccountService>(pub Arc<T>);
                    impl<T: AccountService> tonic::server::UnaryService<super::ResetPasswordRequest>
                        for ResetPasswordSvc<T>
                    {
                        type Response = super::ResetPasswordResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ResetPasswordRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AccountService>::reset_password(&inner, request).await
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
                        let method = ResetPasswordSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/access.v1.AccountService/ForceLogout" => {
                    #[allow(non_camel_case_types)]
                    struct ForceLogoutSvc<T: AccountService>(pub Arc<T>);
                    impl<T: AccountService> tonic::server::UnaryService<super::ForceLogoutRequest>
                        for ForceLogoutSvc<T>
                    {
                        type Response = super::ForceLogoutResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ForceLogoutRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AccountService>::force_logout(&inner, request).await
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
                        let method = ForceLogoutSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/access.v1.AccountService/ForceResetPassword" => {
                    #[allow(non_camel_case_types)]
                    struct ForceResetPasswordSvc<T: AccountService>(pub Arc<T>);
                    impl<T: AccountService>
                        tonic::server::UnaryService<super::ForceResetPasswordRequest>
                        for ForceResetPasswordSvc<T>
                    {
                        type Response = super::ForceResetPasswordResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ForceResetPasswordRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AccountService>::force_reset_password(&inner, request).await
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
                        let method = ForceResetPasswordSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/access.v1.AccountService/LockAccount" => {
                    #[allow(non_camel_case_types)]
                    struct LockAccountSvc<T: AccountService>(pub Arc<T>);
                    impl<T: AccountService> tonic::server::UnaryService<super::LockAccountRequest>
                        for LockAccountSvc<T>
                    {
                        type Response = super::LockAccountResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LockAccountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AccountService>::lock_account(&inner, request).await
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
                        let method = LockAccountSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                "/access.v1.AccountService/UnlockAccount" => {
                    #[allow(non_camel_case_types)]
                    struct UnlockAccountSvc<T: AccountService>(pub Arc<T>);
                    impl<T: AccountService> tonic::server::UnaryService<super::UnlockAccountRequest>
                        for UnlockAccountSvc<T>
                    {
                        type Response = super::UnlockAccountResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UnlockAccountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AccountService>::unlock_account(&inner, request).await
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
                        let method = UnlockAccountSvc(inner);
                        let codec = tonic_prost::ProstCodec::default();
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
                _ => Box::pin(async move {
                    let mut response = http::Response::new(tonic::body::Body::default());
                    let headers = response.headers_mut();
                    headers.insert(
                        tonic::Status::GRPC_STATUS,
                        (tonic::Code::Unimplemented as i32).into(),
                    );
                    headers.insert(
                        http::header::CONTENT_TYPE,
                        tonic::metadata::GRPC_CONTENT_TYPE,
                    );
                    Ok(response)
                }),
            }
        }
    }
    impl<T> Clone for AccountServiceServer<T> {
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
    /// Generated gRPC service name
    pub const SERVICE_NAME: &str = "access.v1.AccountService";
    impl<T> tonic::server::NamedService for AccountServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
