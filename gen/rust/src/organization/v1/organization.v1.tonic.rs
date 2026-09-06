// @generated
/// Generated client implementations.
pub mod organization_service_client {
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
    pub struct OrganizationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OrganizationServiceClient<tonic::transport::Channel> {
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
    impl<T> OrganizationServiceClient<T>
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
        ) -> OrganizationServiceClient<InterceptedService<T, F>>
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
            OrganizationServiceClient::new(InterceptedService::new(inner, interceptor))
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
        pub async fn create_organization(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateOrganizationRequest>,
        ) -> std::result::Result<tonic::Response<super::CreateOrganizationResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/organization.v1.OrganizationService/CreateOrganization",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "organization.v1.OrganizationService",
                "CreateOrganization",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_organization(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrganizationRequest>,
        ) -> std::result::Result<tonic::Response<super::GetOrganizationResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/organization.v1.OrganizationService/GetOrganization",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "organization.v1.OrganizationService",
                "GetOrganization",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn change_organization_name(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeOrganizationNameRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ChangeOrganizationNameResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/organization.v1.OrganizationService/ChangeOrganizationName",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "organization.v1.OrganizationService",
                "ChangeOrganizationName",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn transfer_superadmin(
            &mut self,
            request: impl tonic::IntoRequest<super::TransferSuperadminRequest>,
        ) -> std::result::Result<tonic::Response<super::TransferSuperadminResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/organization.v1.OrganizationService/TransferSuperadmin",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "organization.v1.OrganizationService",
                "TransferSuperadmin",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn issue_invitation(
            &mut self,
            request: impl tonic::IntoRequest<super::IssueInvitationRequest>,
        ) -> std::result::Result<tonic::Response<super::IssueInvitationResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/organization.v1.OrganizationService/IssueInvitation",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "organization.v1.OrganizationService",
                "IssueInvitation",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn claim_invitation(
            &mut self,
            request: impl tonic::IntoRequest<super::ClaimInvitationRequest>,
        ) -> std::result::Result<tonic::Response<super::ClaimInvitationResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/organization.v1.OrganizationService/ClaimInvitation",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "organization.v1.OrganizationService",
                "ClaimInvitation",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn revoke_invitation(
            &mut self,
            request: impl tonic::IntoRequest<super::RevokeInvitationRequest>,
        ) -> std::result::Result<tonic::Response<super::RevokeInvitationResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/organization.v1.OrganizationService/RevokeInvitation",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "organization.v1.OrganizationService",
                "RevokeInvitation",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn force_join(
            &mut self,
            request: impl tonic::IntoRequest<super::ForceJoinRequest>,
        ) -> std::result::Result<tonic::Response<super::ForceJoinResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/organization.v1.OrganizationService/ForceJoin",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "organization.v1.OrganizationService",
                "ForceJoin",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn mark_organization(
            &mut self,
            request: impl tonic::IntoRequest<super::MarkOrganizationRequest>,
        ) -> std::result::Result<tonic::Response<super::MarkOrganizationResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/organization.v1.OrganizationService/MarkOrganization",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "organization.v1.OrganizationService",
                "MarkOrganization",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get_platform_organization(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPlatformOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPlatformOrganizationResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/organization.v1.OrganizationService/GetPlatformOrganization",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "organization.v1.OrganizationService",
                "GetPlatformOrganization",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn prove_platformship(
            &mut self,
            request: impl tonic::IntoRequest<super::ProvePlatformshipRequest>,
        ) -> std::result::Result<tonic::Response<super::ProvePlatformshipResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/organization.v1.OrganizationService/ProvePlatformship",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "organization.v1.OrganizationService",
                "ProvePlatformship",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn evict_member(
            &mut self,
            request: impl tonic::IntoRequest<super::EvictMemberRequest>,
        ) -> std::result::Result<tonic::Response<super::EvictMemberResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic_prost::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/organization.v1.OrganizationService/EvictMember",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "organization.v1.OrganizationService",
                "EvictMember",
            ));
            self.inner.unary(req, path, codec).await
        }
        pub async fn leave(
            &mut self,
            request: impl tonic::IntoRequest<super::LeaveRequest>,
        ) -> std::result::Result<tonic::Response<super::LeaveResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::unknown(format!("Service was not ready: {}", e.into()))
            })?;
            let codec = tonic_prost::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/organization.v1.OrganizationService/Leave");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "organization.v1.OrganizationService",
                "Leave",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod organization_service_server {
    #![allow(
        unused_variables,
        dead_code,
        missing_docs,
        clippy::wildcard_imports,
        clippy::let_unit_value
    )]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with OrganizationServiceServer.

    pub trait OrganizationService: std::marker::Send + std::marker::Sync + 'static {
        async fn create_organization(
            &self,
            request: tonic::Request<super::CreateOrganizationRequest>,
        ) -> std::result::Result<tonic::Response<super::CreateOrganizationResponse>, tonic::Status>;
        async fn get_organization(
            &self,
            request: tonic::Request<super::GetOrganizationRequest>,
        ) -> std::result::Result<tonic::Response<super::GetOrganizationResponse>, tonic::Status>;
        async fn change_organization_name(
            &self,
            request: tonic::Request<super::ChangeOrganizationNameRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ChangeOrganizationNameResponse>,
            tonic::Status,
        >;
        async fn transfer_superadmin(
            &self,
            request: tonic::Request<super::TransferSuperadminRequest>,
        ) -> std::result::Result<tonic::Response<super::TransferSuperadminResponse>, tonic::Status>;
        async fn issue_invitation(
            &self,
            request: tonic::Request<super::IssueInvitationRequest>,
        ) -> std::result::Result<tonic::Response<super::IssueInvitationResponse>, tonic::Status>;
        async fn claim_invitation(
            &self,
            request: tonic::Request<super::ClaimInvitationRequest>,
        ) -> std::result::Result<tonic::Response<super::ClaimInvitationResponse>, tonic::Status>;
        async fn revoke_invitation(
            &self,
            request: tonic::Request<super::RevokeInvitationRequest>,
        ) -> std::result::Result<tonic::Response<super::RevokeInvitationResponse>, tonic::Status>;
        async fn force_join(
            &self,
            request: tonic::Request<super::ForceJoinRequest>,
        ) -> std::result::Result<tonic::Response<super::ForceJoinResponse>, tonic::Status>;
        async fn mark_organization(
            &self,
            request: tonic::Request<super::MarkOrganizationRequest>,
        ) -> std::result::Result<tonic::Response<super::MarkOrganizationResponse>, tonic::Status>;
        async fn get_platform_organization(
            &self,
            request: tonic::Request<super::GetPlatformOrganizationRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetPlatformOrganizationResponse>,
            tonic::Status,
        >;
        async fn prove_platformship(
            &self,
            request: tonic::Request<super::ProvePlatformshipRequest>,
        ) -> std::result::Result<tonic::Response<super::ProvePlatformshipResponse>, tonic::Status>;
        async fn evict_member(
            &self,
            request: tonic::Request<super::EvictMemberRequest>,
        ) -> std::result::Result<tonic::Response<super::EvictMemberResponse>, tonic::Status>;
        async fn leave(
            &self,
            request: tonic::Request<super::LeaveRequest>,
        ) -> std::result::Result<tonic::Response<super::LeaveResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct OrganizationServiceServer<T> {
        inner: Arc<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    impl<T> OrganizationServiceServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for OrganizationServiceServer<T>
    where
        T: OrganizationService,
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
                "/organization.v1.OrganizationService/CreateOrganization" => {
                    #[allow(non_camel_case_types)]
                    struct CreateOrganizationSvc<T: OrganizationService>(pub Arc<T>);
                    impl<T: OrganizationService>
                        tonic::server::UnaryService<super::CreateOrganizationRequest>
                        for CreateOrganizationSvc<T>
                    {
                        type Response = super::CreateOrganizationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateOrganizationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrganizationService>::create_organization(&inner, request)
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
                        let method = CreateOrganizationSvc(inner);
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
                "/organization.v1.OrganizationService/GetOrganization" => {
                    #[allow(non_camel_case_types)]
                    struct GetOrganizationSvc<T: OrganizationService>(pub Arc<T>);
                    impl<T: OrganizationService>
                        tonic::server::UnaryService<super::GetOrganizationRequest>
                        for GetOrganizationSvc<T>
                    {
                        type Response = super::GetOrganizationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetOrganizationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrganizationService>::get_organization(&inner, request).await
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
                        let method = GetOrganizationSvc(inner);
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
                "/organization.v1.OrganizationService/ChangeOrganizationName" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeOrganizationNameSvc<T: OrganizationService>(pub Arc<T>);
                    impl<T: OrganizationService>
                        tonic::server::UnaryService<super::ChangeOrganizationNameRequest>
                        for ChangeOrganizationNameSvc<T>
                    {
                        type Response = super::ChangeOrganizationNameResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChangeOrganizationNameRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrganizationService>::change_organization_name(
                                    &inner, request,
                                )
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
                        let method = ChangeOrganizationNameSvc(inner);
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
                "/organization.v1.OrganizationService/TransferSuperadmin" => {
                    #[allow(non_camel_case_types)]
                    struct TransferSuperadminSvc<T: OrganizationService>(pub Arc<T>);
                    impl<T: OrganizationService>
                        tonic::server::UnaryService<super::TransferSuperadminRequest>
                        for TransferSuperadminSvc<T>
                    {
                        type Response = super::TransferSuperadminResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TransferSuperadminRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrganizationService>::transfer_superadmin(&inner, request)
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
                        let method = TransferSuperadminSvc(inner);
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
                "/organization.v1.OrganizationService/IssueInvitation" => {
                    #[allow(non_camel_case_types)]
                    struct IssueInvitationSvc<T: OrganizationService>(pub Arc<T>);
                    impl<T: OrganizationService>
                        tonic::server::UnaryService<super::IssueInvitationRequest>
                        for IssueInvitationSvc<T>
                    {
                        type Response = super::IssueInvitationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::IssueInvitationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrganizationService>::issue_invitation(&inner, request).await
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
                        let method = IssueInvitationSvc(inner);
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
                "/organization.v1.OrganizationService/ClaimInvitation" => {
                    #[allow(non_camel_case_types)]
                    struct ClaimInvitationSvc<T: OrganizationService>(pub Arc<T>);
                    impl<T: OrganizationService>
                        tonic::server::UnaryService<super::ClaimInvitationRequest>
                        for ClaimInvitationSvc<T>
                    {
                        type Response = super::ClaimInvitationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ClaimInvitationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrganizationService>::claim_invitation(&inner, request).await
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
                        let method = ClaimInvitationSvc(inner);
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
                "/organization.v1.OrganizationService/RevokeInvitation" => {
                    #[allow(non_camel_case_types)]
                    struct RevokeInvitationSvc<T: OrganizationService>(pub Arc<T>);
                    impl<T: OrganizationService>
                        tonic::server::UnaryService<super::RevokeInvitationRequest>
                        for RevokeInvitationSvc<T>
                    {
                        type Response = super::RevokeInvitationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RevokeInvitationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrganizationService>::revoke_invitation(&inner, request).await
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
                        let method = RevokeInvitationSvc(inner);
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
                "/organization.v1.OrganizationService/ForceJoin" => {
                    #[allow(non_camel_case_types)]
                    struct ForceJoinSvc<T: OrganizationService>(pub Arc<T>);
                    impl<T: OrganizationService>
                        tonic::server::UnaryService<super::ForceJoinRequest> for ForceJoinSvc<T>
                    {
                        type Response = super::ForceJoinResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ForceJoinRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrganizationService>::force_join(&inner, request).await
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
                        let method = ForceJoinSvc(inner);
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
                "/organization.v1.OrganizationService/MarkOrganization" => {
                    #[allow(non_camel_case_types)]
                    struct MarkOrganizationSvc<T: OrganizationService>(pub Arc<T>);
                    impl<T: OrganizationService>
                        tonic::server::UnaryService<super::MarkOrganizationRequest>
                        for MarkOrganizationSvc<T>
                    {
                        type Response = super::MarkOrganizationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MarkOrganizationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrganizationService>::mark_organization(&inner, request).await
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
                        let method = MarkOrganizationSvc(inner);
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
                "/organization.v1.OrganizationService/GetPlatformOrganization" => {
                    #[allow(non_camel_case_types)]
                    struct GetPlatformOrganizationSvc<T: OrganizationService>(pub Arc<T>);
                    impl<T: OrganizationService>
                        tonic::server::UnaryService<super::GetPlatformOrganizationRequest>
                        for GetPlatformOrganizationSvc<T>
                    {
                        type Response = super::GetPlatformOrganizationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPlatformOrganizationRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrganizationService>::get_platform_organization(
                                    &inner, request,
                                )
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
                        let method = GetPlatformOrganizationSvc(inner);
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
                "/organization.v1.OrganizationService/ProvePlatformship" => {
                    #[allow(non_camel_case_types)]
                    struct ProvePlatformshipSvc<T: OrganizationService>(pub Arc<T>);
                    impl<T: OrganizationService>
                        tonic::server::UnaryService<super::ProvePlatformshipRequest>
                        for ProvePlatformshipSvc<T>
                    {
                        type Response = super::ProvePlatformshipResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProvePlatformshipRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrganizationService>::prove_platformship(&inner, request)
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
                        let method = ProvePlatformshipSvc(inner);
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
                "/organization.v1.OrganizationService/EvictMember" => {
                    #[allow(non_camel_case_types)]
                    struct EvictMemberSvc<T: OrganizationService>(pub Arc<T>);
                    impl<T: OrganizationService>
                        tonic::server::UnaryService<super::EvictMemberRequest>
                        for EvictMemberSvc<T>
                    {
                        type Response = super::EvictMemberResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::EvictMemberRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrganizationService>::evict_member(&inner, request).await
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
                        let method = EvictMemberSvc(inner);
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
                "/organization.v1.OrganizationService/Leave" => {
                    #[allow(non_camel_case_types)]
                    struct LeaveSvc<T: OrganizationService>(pub Arc<T>);
                    impl<T: OrganizationService> tonic::server::UnaryService<super::LeaveRequest> for LeaveSvc<T> {
                        type Response = super::LeaveResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LeaveRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as OrganizationService>::leave(&inner, request).await
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
                        let method = LeaveSvc(inner);
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
    impl<T> Clone for OrganizationServiceServer<T> {
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
    pub const SERVICE_NAME: &str = "organization.v1.OrganizationService";
    impl<T> tonic::server::NamedService for OrganizationServiceServer<T> {
        const NAME: &'static str = SERVICE_NAME;
    }
}
