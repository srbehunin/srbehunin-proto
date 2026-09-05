///Shorthand for `OwnedView<CreateAppRequestView<'static>>`.
pub type OwnedCreateAppRequestView = ::buffa::view::OwnedView<
    crate::proto::app::v1::__buffa::view::CreateAppRequestView<'static>,
>;
///Shorthand for `OwnedView<CreateAppResponseView<'static>>`.
pub type OwnedCreateAppResponseView = ::buffa::view::OwnedView<
    crate::proto::app::v1::__buffa::view::CreateAppResponseView<'static>,
>;
///Shorthand for `OwnedView<UpdateAppRequestView<'static>>`.
pub type OwnedUpdateAppRequestView = ::buffa::view::OwnedView<
    crate::proto::app::v1::__buffa::view::UpdateAppRequestView<'static>,
>;
///Shorthand for `OwnedView<UpdateAppResponseView<'static>>`.
pub type OwnedUpdateAppResponseView = ::buffa::view::OwnedView<
    crate::proto::app::v1::__buffa::view::UpdateAppResponseView<'static>,
>;
///Shorthand for `OwnedView<GetAppByIdRequestView<'static>>`.
pub type OwnedGetAppByIdRequestView = ::buffa::view::OwnedView<
    crate::proto::app::v1::__buffa::view::GetAppByIdRequestView<'static>,
>;
///Shorthand for `OwnedView<GetAppByIdResponseView<'static>>`.
pub type OwnedGetAppByIdResponseView = ::buffa::view::OwnedView<
    crate::proto::app::v1::__buffa::view::GetAppByIdResponseView<'static>,
>;
///Shorthand for `OwnedView<GetAppByHandleRequestView<'static>>`.
pub type OwnedGetAppByHandleRequestView = ::buffa::view::OwnedView<
    crate::proto::app::v1::__buffa::view::GetAppByHandleRequestView<'static>,
>;
///Shorthand for `OwnedView<GetAppByHandleResponseView<'static>>`.
pub type OwnedGetAppByHandleResponseView = ::buffa::view::OwnedView<
    crate::proto::app::v1::__buffa::view::GetAppByHandleResponseView<'static>,
>;
///Shorthand for `OwnedView<SetAppStatusRequestView<'static>>`.
pub type OwnedSetAppStatusRequestView = ::buffa::view::OwnedView<
    crate::proto::app::v1::__buffa::view::SetAppStatusRequestView<'static>,
>;
///Shorthand for `OwnedView<SetAppStatusResponseView<'static>>`.
pub type OwnedSetAppStatusResponseView = ::buffa::view::OwnedView<
    crate::proto::app::v1::__buffa::view::SetAppStatusResponseView<'static>,
>;
///Shorthand for `OwnedView<GetAllRequestView<'static>>`.
pub type OwnedGetAllRequestView = ::buffa::view::OwnedView<
    crate::proto::app::v1::__buffa::view::GetAllRequestView<'static>,
>;
///Shorthand for `OwnedView<GetAllResponseView<'static>>`.
pub type OwnedGetAllResponseView = ::buffa::view::OwnedView<
    crate::proto::app::v1::__buffa::view::GetAllResponseView<'static>,
>;
impl ::connectrpc::Encodable<crate::proto::app::v1::CreateAppResponse>
for crate::proto::app::v1::__buffa::view::CreateAppResponseView<'_> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(self, codec)
    }
}
impl ::connectrpc::Encodable<crate::proto::app::v1::CreateAppResponse>
for ::buffa::view::OwnedView<
    crate::proto::app::v1::__buffa::view::CreateAppResponseView<'static>,
> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(self.reborrow(), codec)
    }
    /// An `OwnedView` still holds the buffer it was decoded from, so
    /// its large fields can be handed to the response body by
    /// reference count instead of copied. The bare view impl above
    /// cannot do this: it has borrows but no buffer to name.
    fn encode_segments(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::connectrpc::EncodedBody, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body_segments(
            self.reborrow(),
            self.bytes(),
            codec,
        )
    }
}
impl ::connectrpc::Encodable<crate::proto::app::v1::UpdateAppResponse>
for crate::proto::app::v1::__buffa::view::UpdateAppResponseView<'_> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(self, codec)
    }
}
impl ::connectrpc::Encodable<crate::proto::app::v1::UpdateAppResponse>
for ::buffa::view::OwnedView<
    crate::proto::app::v1::__buffa::view::UpdateAppResponseView<'static>,
> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(self.reborrow(), codec)
    }
    /// An `OwnedView` still holds the buffer it was decoded from, so
    /// its large fields can be handed to the response body by
    /// reference count instead of copied. The bare view impl above
    /// cannot do this: it has borrows but no buffer to name.
    fn encode_segments(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::connectrpc::EncodedBody, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body_segments(
            self.reborrow(),
            self.bytes(),
            codec,
        )
    }
}
impl ::connectrpc::Encodable<crate::proto::app::v1::GetAppByIdResponse>
for crate::proto::app::v1::__buffa::view::GetAppByIdResponseView<'_> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(self, codec)
    }
}
impl ::connectrpc::Encodable<crate::proto::app::v1::GetAppByIdResponse>
for ::buffa::view::OwnedView<
    crate::proto::app::v1::__buffa::view::GetAppByIdResponseView<'static>,
> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(self.reborrow(), codec)
    }
    /// An `OwnedView` still holds the buffer it was decoded from, so
    /// its large fields can be handed to the response body by
    /// reference count instead of copied. The bare view impl above
    /// cannot do this: it has borrows but no buffer to name.
    fn encode_segments(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::connectrpc::EncodedBody, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body_segments(
            self.reborrow(),
            self.bytes(),
            codec,
        )
    }
}
impl ::connectrpc::Encodable<crate::proto::app::v1::GetAppByHandleResponse>
for crate::proto::app::v1::__buffa::view::GetAppByHandleResponseView<'_> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(self, codec)
    }
}
impl ::connectrpc::Encodable<crate::proto::app::v1::GetAppByHandleResponse>
for ::buffa::view::OwnedView<
    crate::proto::app::v1::__buffa::view::GetAppByHandleResponseView<'static>,
> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(self.reborrow(), codec)
    }
    /// An `OwnedView` still holds the buffer it was decoded from, so
    /// its large fields can be handed to the response body by
    /// reference count instead of copied. The bare view impl above
    /// cannot do this: it has borrows but no buffer to name.
    fn encode_segments(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::connectrpc::EncodedBody, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body_segments(
            self.reborrow(),
            self.bytes(),
            codec,
        )
    }
}
impl ::connectrpc::Encodable<crate::proto::app::v1::SetAppStatusResponse>
for crate::proto::app::v1::__buffa::view::SetAppStatusResponseView<'_> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(self, codec)
    }
}
impl ::connectrpc::Encodable<crate::proto::app::v1::SetAppStatusResponse>
for ::buffa::view::OwnedView<
    crate::proto::app::v1::__buffa::view::SetAppStatusResponseView<'static>,
> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(self.reborrow(), codec)
    }
    /// An `OwnedView` still holds the buffer it was decoded from, so
    /// its large fields can be handed to the response body by
    /// reference count instead of copied. The bare view impl above
    /// cannot do this: it has borrows but no buffer to name.
    fn encode_segments(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::connectrpc::EncodedBody, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body_segments(
            self.reborrow(),
            self.bytes(),
            codec,
        )
    }
}
impl ::connectrpc::Encodable<crate::proto::app::v1::GetAllResponse>
for crate::proto::app::v1::__buffa::view::GetAllResponseView<'_> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(self, codec)
    }
}
impl ::connectrpc::Encodable<crate::proto::app::v1::GetAllResponse>
for ::buffa::view::OwnedView<
    crate::proto::app::v1::__buffa::view::GetAllResponseView<'static>,
> {
    fn encode(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::buffa::bytes::Bytes, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body(self.reborrow(), codec)
    }
    /// An `OwnedView` still holds the buffer it was decoded from, so
    /// its large fields can be handed to the response body by
    /// reference count instead of copied. The bare view impl above
    /// cannot do this: it has borrows but no buffer to name.
    fn encode_segments(
        &self,
        codec: ::connectrpc::CodecFormat,
    ) -> ::std::result::Result<::connectrpc::EncodedBody, ::connectrpc::ConnectError> {
        ::connectrpc::__codegen::encode_view_body_segments(
            self.reborrow(),
            self.bytes(),
            codec,
        )
    }
}
/// Full service name for this service.
pub const APP_SERVICE_SERVICE_NAME: &str = "app.v1.AppService";
/// Static [`Spec`](::connectrpc::Spec) for the `CreateApp` RPC, as seen by the server; the generated client passes it with [`origin`](::connectrpc::Spec::origin) `Client` (compare across sides with [`Spec::same_method`](::connectrpc::Spec::same_method)).
pub const APP_SERVICE_CREATE_APP_SPEC: ::connectrpc::Spec = ::connectrpc::Spec::server(
        "/app.v1.AppService/CreateApp",
        ::connectrpc::StreamType::Unary,
    )
    .with_idempotency_level(::connectrpc::IdempotencyLevel::Unknown);
/// Static [`Spec`](::connectrpc::Spec) for the `UpdateApp` RPC, as seen by the server; the generated client passes it with [`origin`](::connectrpc::Spec::origin) `Client` (compare across sides with [`Spec::same_method`](::connectrpc::Spec::same_method)).
pub const APP_SERVICE_UPDATE_APP_SPEC: ::connectrpc::Spec = ::connectrpc::Spec::server(
        "/app.v1.AppService/UpdateApp",
        ::connectrpc::StreamType::Unary,
    )
    .with_idempotency_level(::connectrpc::IdempotencyLevel::Unknown);
/// Static [`Spec`](::connectrpc::Spec) for the `GetAppById` RPC, as seen by the server; the generated client passes it with [`origin`](::connectrpc::Spec::origin) `Client` (compare across sides with [`Spec::same_method`](::connectrpc::Spec::same_method)).
pub const APP_SERVICE_GET_APP_BY_ID_SPEC: ::connectrpc::Spec = ::connectrpc::Spec::server(
        "/app.v1.AppService/GetAppById",
        ::connectrpc::StreamType::Unary,
    )
    .with_idempotency_level(::connectrpc::IdempotencyLevel::Unknown);
/// Static [`Spec`](::connectrpc::Spec) for the `GetAppByHandle` RPC, as seen by the server; the generated client passes it with [`origin`](::connectrpc::Spec::origin) `Client` (compare across sides with [`Spec::same_method`](::connectrpc::Spec::same_method)).
pub const APP_SERVICE_GET_APP_BY_HANDLE_SPEC: ::connectrpc::Spec = ::connectrpc::Spec::server(
        "/app.v1.AppService/GetAppByHandle",
        ::connectrpc::StreamType::Unary,
    )
    .with_idempotency_level(::connectrpc::IdempotencyLevel::Unknown);
/// Static [`Spec`](::connectrpc::Spec) for the `SetAppStatus` RPC, as seen by the server; the generated client passes it with [`origin`](::connectrpc::Spec::origin) `Client` (compare across sides with [`Spec::same_method`](::connectrpc::Spec::same_method)).
pub const APP_SERVICE_SET_APP_STATUS_SPEC: ::connectrpc::Spec = ::connectrpc::Spec::server(
        "/app.v1.AppService/SetAppStatus",
        ::connectrpc::StreamType::Unary,
    )
    .with_idempotency_level(::connectrpc::IdempotencyLevel::Unknown);
/// Static [`Spec`](::connectrpc::Spec) for the `GetAll` RPC, as seen by the server; the generated client passes it with [`origin`](::connectrpc::Spec::origin) `Client` (compare across sides with [`Spec::same_method`](::connectrpc::Spec::same_method)).
pub const APP_SERVICE_GET_ALL_SPEC: ::connectrpc::Spec = ::connectrpc::Spec::server(
        "/app.v1.AppService/GetAll",
        ::connectrpc::StreamType::Unary,
    )
    .with_idempotency_level(::connectrpc::IdempotencyLevel::Unknown);
/// Server trait for AppService.
///
/// # Implementing handlers
///
/// Implement methods with plain `async fn`; the returned future satisfies
/// the `Send` bound automatically.
///
/// **Unary and server-streaming requests** arrive as
/// [`ServiceRequest<'_, Req>`](::connectrpc::ServiceRequest): a zero-copy
/// view of the request plus its body, valid for the duration of the call.
/// Fields are read directly (`request.name` is a `&str` into the decoded
/// buffer) and the borrow may be held across `.await` points. Anything
/// that must outlive the call — `tokio::spawn`, channels, server state,
/// or data captured by a returned response stream — takes owned data:
/// call `request.to_owned_message()` (or copy the specific fields)
/// first.
///
/// **Client-streaming and bidi requests** arrive as
/// [`InboundStream<Req>`](::connectrpc::InboundStream) — a
/// `ServiceStream` of [`StreamMessage`](::connectrpc::StreamMessage)s.
/// Each item owns its decoded buffer and is `Send + 'static`, so items
/// can be buffered or moved into spawned tasks; read fields zero-copy
/// through the generated accessor methods (`item.name()`) or `.view()`,
/// convert with `.to_owned_message()`, or yield an item back unchanged —
/// `StreamMessage<M>` implements `Encodable<M>`.
///
/// Request types resolved through `extern_path` (e.g. well-known types
/// from another crate) use the same wrappers; the crate that owns the
/// type must be generated with buffa ≥ 0.9.0 and views enabled so the
/// backing `HasMessageView` impl exists.
///
/// The `impl Encodable<Out>` return bound accepts the owned `Out`, the
/// generated `OutView<'_>` / `OwnedOutView`,
/// [`MaybeBorrowed`](::connectrpc::MaybeBorrowed), or
/// [`PreEncoded`](::connectrpc::PreEncoded) for handlers that encode a
/// non-`'static` view internally and pass the bytes across the handler
/// boundary. View bodies are not emitted for output types mapped via
/// `extern_path` (the impl would be an orphan); return owned for
/// WKT/extern outputs.
///
/// Server-streaming and bidi-streaming methods return
/// `ServiceStream<impl Encodable<Out> + Send + use<Self>>`. The
/// `use<Self>` precise-capturing clause excludes `&self`'s lifetime and
/// the request's lifetime (unary methods use `use<'a, Self>` and may
/// borrow from `&self`), so stream items must be `'static` and cannot
/// borrow from the request. To stream view-encoded data, encode each
/// item inside the stream body and yield
/// [`PreEncoded`](::connectrpc::PreEncoded) — see its `# Streaming
/// example` doc.
#[allow(clippy::type_complexity)]
pub trait AppService: Send + Sync + 'static {
    /// Handle the CreateApp RPC.
    ///
    /// `'a` lets the response body borrow from `&self` (e.g. server-resident state).
    ///
    /// `request` is borrowed from the request body and is valid for the
    /// duration of the call; message fields are read directly on it
    /// (zero-copy). The response cannot borrow from `request` — use
    /// `.to_owned_message()` (or copy the specific fields) for anything
    /// returned, stored, or moved into `tokio::spawn`.
    fn create_app<'a>(
        &'a self,
        ctx: ::connectrpc::RequestContext,
        request: ::connectrpc::ServiceRequest<
            '_,
            crate::proto::app::v1::CreateAppRequest,
        >,
    ) -> impl ::std::future::Future<
        Output = ::connectrpc::ServiceResult<
            impl ::connectrpc::Encodable<
                crate::proto::app::v1::CreateAppResponse,
            > + Send + use<'a, Self>,
        >,
    > + Send;
    /// Handle the UpdateApp RPC.
    ///
    /// `'a` lets the response body borrow from `&self` (e.g. server-resident state).
    ///
    /// `request` is borrowed from the request body and is valid for the
    /// duration of the call; message fields are read directly on it
    /// (zero-copy). The response cannot borrow from `request` — use
    /// `.to_owned_message()` (or copy the specific fields) for anything
    /// returned, stored, or moved into `tokio::spawn`.
    fn update_app<'a>(
        &'a self,
        ctx: ::connectrpc::RequestContext,
        request: ::connectrpc::ServiceRequest<
            '_,
            crate::proto::app::v1::UpdateAppRequest,
        >,
    ) -> impl ::std::future::Future<
        Output = ::connectrpc::ServiceResult<
            impl ::connectrpc::Encodable<
                crate::proto::app::v1::UpdateAppResponse,
            > + Send + use<'a, Self>,
        >,
    > + Send;
    /// Handle the GetAppById RPC.
    ///
    /// `'a` lets the response body borrow from `&self` (e.g. server-resident state).
    ///
    /// `request` is borrowed from the request body and is valid for the
    /// duration of the call; message fields are read directly on it
    /// (zero-copy). The response cannot borrow from `request` — use
    /// `.to_owned_message()` (or copy the specific fields) for anything
    /// returned, stored, or moved into `tokio::spawn`.
    fn get_app_by_id<'a>(
        &'a self,
        ctx: ::connectrpc::RequestContext,
        request: ::connectrpc::ServiceRequest<
            '_,
            crate::proto::app::v1::GetAppByIdRequest,
        >,
    ) -> impl ::std::future::Future<
        Output = ::connectrpc::ServiceResult<
            impl ::connectrpc::Encodable<
                crate::proto::app::v1::GetAppByIdResponse,
            > + Send + use<'a, Self>,
        >,
    > + Send;
    /// Handle the GetAppByHandle RPC.
    ///
    /// `'a` lets the response body borrow from `&self` (e.g. server-resident state).
    ///
    /// `request` is borrowed from the request body and is valid for the
    /// duration of the call; message fields are read directly on it
    /// (zero-copy). The response cannot borrow from `request` — use
    /// `.to_owned_message()` (or copy the specific fields) for anything
    /// returned, stored, or moved into `tokio::spawn`.
    fn get_app_by_handle<'a>(
        &'a self,
        ctx: ::connectrpc::RequestContext,
        request: ::connectrpc::ServiceRequest<
            '_,
            crate::proto::app::v1::GetAppByHandleRequest,
        >,
    ) -> impl ::std::future::Future<
        Output = ::connectrpc::ServiceResult<
            impl ::connectrpc::Encodable<
                crate::proto::app::v1::GetAppByHandleResponse,
            > + Send + use<'a, Self>,
        >,
    > + Send;
    /// Handle the SetAppStatus RPC.
    ///
    /// `'a` lets the response body borrow from `&self` (e.g. server-resident state).
    ///
    /// `request` is borrowed from the request body and is valid for the
    /// duration of the call; message fields are read directly on it
    /// (zero-copy). The response cannot borrow from `request` — use
    /// `.to_owned_message()` (or copy the specific fields) for anything
    /// returned, stored, or moved into `tokio::spawn`.
    fn set_app_status<'a>(
        &'a self,
        ctx: ::connectrpc::RequestContext,
        request: ::connectrpc::ServiceRequest<
            '_,
            crate::proto::app::v1::SetAppStatusRequest,
        >,
    ) -> impl ::std::future::Future<
        Output = ::connectrpc::ServiceResult<
            impl ::connectrpc::Encodable<
                crate::proto::app::v1::SetAppStatusResponse,
            > + Send + use<'a, Self>,
        >,
    > + Send;
    /// Handle the GetAll RPC.
    ///
    /// `'a` lets the response body borrow from `&self` (e.g. server-resident state).
    ///
    /// `request` is borrowed from the request body and is valid for the
    /// duration of the call; message fields are read directly on it
    /// (zero-copy). The response cannot borrow from `request` — use
    /// `.to_owned_message()` (or copy the specific fields) for anything
    /// returned, stored, or moved into `tokio::spawn`.
    fn get_all<'a>(
        &'a self,
        ctx: ::connectrpc::RequestContext,
        request: ::connectrpc::ServiceRequest<'_, crate::proto::app::v1::GetAllRequest>,
    ) -> impl ::std::future::Future<
        Output = ::connectrpc::ServiceResult<
            impl ::connectrpc::Encodable<
                crate::proto::app::v1::GetAllResponse,
            > + Send + use<'a, Self>,
        >,
    > + Send;
}
/// Extension trait for registering a service implementation with a Router.
///
/// This trait is automatically implemented for all types that implement the service trait.
/// Prefer [`Router::add_service`](::connectrpc::Router::add_service) for
/// top-down registration; `register` remains available for compatibility
/// and cases where the service-first call shape is more convenient.
///
/// # Example
///
/// ```rust,ignore
/// use std::sync::Arc;
///
/// let service = Arc::new(MyServiceImpl);
/// let router = service.register(Router::new());
/// ```
pub trait AppServiceExt: AppService {
    /// Register this service implementation with a Router.
    ///
    /// Takes ownership of the `Arc<Self>` and returns a new Router with
    /// this service's methods registered.
    fn register(
        self: ::std::sync::Arc<Self>,
        router: ::connectrpc::Router,
    ) -> ::connectrpc::Router;
}
impl<S: AppService> AppServiceExt for S {
    fn register(
        self: ::std::sync::Arc<Self>,
        router: ::connectrpc::Router,
    ) -> ::connectrpc::Router {
        router
            .route_view(
                APP_SERVICE_SERVICE_NAME,
                "CreateApp",
                {
                    let svc = ::std::sync::Arc::clone(&self);
                    ::connectrpc::view_handler_fn(move |
                        ctx,
                        req: ::buffa::view::OwnedView<
                            crate::proto::app::v1::__buffa::view::CreateAppRequestView<
                                'static,
                            >,
                        >,
                        format|
                    {
                        let svc = ::std::sync::Arc::clone(&svc);
                        async move {
                            let sreq = ::connectrpc::ServiceRequest::<
                                crate::proto::app::v1::CreateAppRequest,
                            >::from_parts(req.reborrow(), req.bytes());
                            svc.create_app(ctx, sreq)
                                .await?
                                .encode::<crate::proto::app::v1::CreateAppResponse>(format)
                        }
                    })
                },
            )
            .with_spec(APP_SERVICE_CREATE_APP_SPEC)
            .route_view(
                APP_SERVICE_SERVICE_NAME,
                "UpdateApp",
                {
                    let svc = ::std::sync::Arc::clone(&self);
                    ::connectrpc::view_handler_fn(move |
                        ctx,
                        req: ::buffa::view::OwnedView<
                            crate::proto::app::v1::__buffa::view::UpdateAppRequestView<
                                'static,
                            >,
                        >,
                        format|
                    {
                        let svc = ::std::sync::Arc::clone(&svc);
                        async move {
                            let sreq = ::connectrpc::ServiceRequest::<
                                crate::proto::app::v1::UpdateAppRequest,
                            >::from_parts(req.reborrow(), req.bytes());
                            svc.update_app(ctx, sreq)
                                .await?
                                .encode::<crate::proto::app::v1::UpdateAppResponse>(format)
                        }
                    })
                },
            )
            .with_spec(APP_SERVICE_UPDATE_APP_SPEC)
            .route_view(
                APP_SERVICE_SERVICE_NAME,
                "GetAppById",
                {
                    let svc = ::std::sync::Arc::clone(&self);
                    ::connectrpc::view_handler_fn(move |
                        ctx,
                        req: ::buffa::view::OwnedView<
                            crate::proto::app::v1::__buffa::view::GetAppByIdRequestView<
                                'static,
                            >,
                        >,
                        format|
                    {
                        let svc = ::std::sync::Arc::clone(&svc);
                        async move {
                            let sreq = ::connectrpc::ServiceRequest::<
                                crate::proto::app::v1::GetAppByIdRequest,
                            >::from_parts(req.reborrow(), req.bytes());
                            svc.get_app_by_id(ctx, sreq)
                                .await?
                                .encode::<crate::proto::app::v1::GetAppByIdResponse>(format)
                        }
                    })
                },
            )
            .with_spec(APP_SERVICE_GET_APP_BY_ID_SPEC)
            .route_view(
                APP_SERVICE_SERVICE_NAME,
                "GetAppByHandle",
                {
                    let svc = ::std::sync::Arc::clone(&self);
                    ::connectrpc::view_handler_fn(move |
                        ctx,
                        req: ::buffa::view::OwnedView<
                            crate::proto::app::v1::__buffa::view::GetAppByHandleRequestView<
                                'static,
                            >,
                        >,
                        format|
                    {
                        let svc = ::std::sync::Arc::clone(&svc);
                        async move {
                            let sreq = ::connectrpc::ServiceRequest::<
                                crate::proto::app::v1::GetAppByHandleRequest,
                            >::from_parts(req.reborrow(), req.bytes());
                            svc.get_app_by_handle(ctx, sreq)
                                .await?
                                .encode::<
                                    crate::proto::app::v1::GetAppByHandleResponse,
                                >(format)
                        }
                    })
                },
            )
            .with_spec(APP_SERVICE_GET_APP_BY_HANDLE_SPEC)
            .route_view(
                APP_SERVICE_SERVICE_NAME,
                "SetAppStatus",
                {
                    let svc = ::std::sync::Arc::clone(&self);
                    ::connectrpc::view_handler_fn(move |
                        ctx,
                        req: ::buffa::view::OwnedView<
                            crate::proto::app::v1::__buffa::view::SetAppStatusRequestView<
                                'static,
                            >,
                        >,
                        format|
                    {
                        let svc = ::std::sync::Arc::clone(&svc);
                        async move {
                            let sreq = ::connectrpc::ServiceRequest::<
                                crate::proto::app::v1::SetAppStatusRequest,
                            >::from_parts(req.reborrow(), req.bytes());
                            svc.set_app_status(ctx, sreq)
                                .await?
                                .encode::<
                                    crate::proto::app::v1::SetAppStatusResponse,
                                >(format)
                        }
                    })
                },
            )
            .with_spec(APP_SERVICE_SET_APP_STATUS_SPEC)
            .route_view(
                APP_SERVICE_SERVICE_NAME,
                "GetAll",
                {
                    let svc = ::std::sync::Arc::clone(&self);
                    ::connectrpc::view_handler_fn(move |
                        ctx,
                        req: ::buffa::view::OwnedView<
                            crate::proto::app::v1::__buffa::view::GetAllRequestView<
                                'static,
                            >,
                        >,
                        format|
                    {
                        let svc = ::std::sync::Arc::clone(&svc);
                        async move {
                            let sreq = ::connectrpc::ServiceRequest::<
                                crate::proto::app::v1::GetAllRequest,
                            >::from_parts(req.reborrow(), req.bytes());
                            svc.get_all(ctx, sreq)
                                .await?
                                .encode::<crate::proto::app::v1::GetAllResponse>(format)
                        }
                    })
                },
            )
            .with_spec(APP_SERVICE_GET_ALL_SPEC)
    }
}
/// Type-inference marker used by [`Router::add_service`](::connectrpc::Router::add_service).
#[doc(hidden)]
pub struct AppServiceRegisterMarker;
impl<S: AppService> ::connectrpc::ServiceRegister<AppServiceRegisterMarker>
for ::std::sync::Arc<S> {
    fn register_service(self, router: ::connectrpc::Router) -> ::connectrpc::Router {
        <S as AppServiceExt>::register(self, router)
    }
}
/// Monomorphic dispatcher for `AppService`.
///
/// Unlike `.register(Router)` which type-erases each method into an `Arc<dyn ErasedHandler>` stored in a `HashMap`, this struct dispatches via a compile-time `match` on method name: no vtable, no hash lookup.
///
/// # Example
///
/// ```rust,ignore
/// use connectrpc::ConnectRpcService;
///
/// let server = AppServiceServer::new(MyImpl);
/// let service = ConnectRpcService::new(server);
/// // hand `service` to axum/hyper as a fallback_service
/// ```
pub struct AppServiceServer<T> {
    inner: ::std::sync::Arc<T>,
}
impl<T: AppService> AppServiceServer<T> {
    /// Wrap a service implementation in a monomorphic dispatcher.
    pub fn new(service: T) -> Self {
        Self {
            inner: ::std::sync::Arc::new(service),
        }
    }
    /// Wrap an already-`Arc`'d service implementation.
    pub fn from_arc(inner: ::std::sync::Arc<T>) -> Self {
        Self { inner }
    }
}
impl<T> Clone for AppServiceServer<T> {
    fn clone(&self) -> Self {
        Self {
            inner: ::std::sync::Arc::clone(&self.inner),
        }
    }
}
impl<T: AppService> ::connectrpc::Dispatcher for AppServiceServer<T> {
    #[inline]
    fn lookup(
        &self,
        path: &str,
    ) -> Option<::connectrpc::dispatcher::codegen::MethodDescriptor> {
        let method = path.strip_prefix("app.v1.AppService/")?;
        match method {
            "CreateApp" => {
                Some(
                    ::connectrpc::dispatcher::codegen::MethodDescriptor::unary(false)
                        .with_spec(APP_SERVICE_CREATE_APP_SPEC),
                )
            }
            "UpdateApp" => {
                Some(
                    ::connectrpc::dispatcher::codegen::MethodDescriptor::unary(false)
                        .with_spec(APP_SERVICE_UPDATE_APP_SPEC),
                )
            }
            "GetAppById" => {
                Some(
                    ::connectrpc::dispatcher::codegen::MethodDescriptor::unary(false)
                        .with_spec(APP_SERVICE_GET_APP_BY_ID_SPEC),
                )
            }
            "GetAppByHandle" => {
                Some(
                    ::connectrpc::dispatcher::codegen::MethodDescriptor::unary(false)
                        .with_spec(APP_SERVICE_GET_APP_BY_HANDLE_SPEC),
                )
            }
            "SetAppStatus" => {
                Some(
                    ::connectrpc::dispatcher::codegen::MethodDescriptor::unary(false)
                        .with_spec(APP_SERVICE_SET_APP_STATUS_SPEC),
                )
            }
            "GetAll" => {
                Some(
                    ::connectrpc::dispatcher::codegen::MethodDescriptor::unary(false)
                        .with_spec(APP_SERVICE_GET_ALL_SPEC),
                )
            }
            _ => None,
        }
    }
    fn call_unary(
        &self,
        path: &str,
        ctx: ::connectrpc::RequestContext,
        request: ::connectrpc::Payload,
        format: ::connectrpc::CodecFormat,
    ) -> ::connectrpc::dispatcher::codegen::UnaryResult {
        let Some(method) = path.strip_prefix("app.v1.AppService/") else {
            return ::connectrpc::dispatcher::codegen::unimplemented_unary(path);
        };
        let _ = (&ctx, &request, &format);
        match method {
            "CreateApp" => {
                let svc = ::std::sync::Arc::clone(&self.inner);
                Box::pin(async move {
                    let body = ::connectrpc::dispatcher::codegen::request_proto_bytes::<
                        crate::proto::app::v1::CreateAppRequest,
                    >(request.encoded()?, format)?;
                    let req: crate::proto::app::v1::__buffa::view::CreateAppRequestView<
                        '_,
                    > = ::connectrpc::dispatcher::codegen::decode_borrowed_request_view(
                        &body,
                        ctx.decode_options(),
                    )?;
                    let req = ::connectrpc::ServiceRequest::<
                        crate::proto::app::v1::CreateAppRequest,
                    >::from_parts(&req, &body);
                    svc.create_app(ctx, req)
                        .await?
                        .encode::<crate::proto::app::v1::CreateAppResponse>(format)
                })
            }
            "UpdateApp" => {
                let svc = ::std::sync::Arc::clone(&self.inner);
                Box::pin(async move {
                    let body = ::connectrpc::dispatcher::codegen::request_proto_bytes::<
                        crate::proto::app::v1::UpdateAppRequest,
                    >(request.encoded()?, format)?;
                    let req: crate::proto::app::v1::__buffa::view::UpdateAppRequestView<
                        '_,
                    > = ::connectrpc::dispatcher::codegen::decode_borrowed_request_view(
                        &body,
                        ctx.decode_options(),
                    )?;
                    let req = ::connectrpc::ServiceRequest::<
                        crate::proto::app::v1::UpdateAppRequest,
                    >::from_parts(&req, &body);
                    svc.update_app(ctx, req)
                        .await?
                        .encode::<crate::proto::app::v1::UpdateAppResponse>(format)
                })
            }
            "GetAppById" => {
                let svc = ::std::sync::Arc::clone(&self.inner);
                Box::pin(async move {
                    let body = ::connectrpc::dispatcher::codegen::request_proto_bytes::<
                        crate::proto::app::v1::GetAppByIdRequest,
                    >(request.encoded()?, format)?;
                    let req: crate::proto::app::v1::__buffa::view::GetAppByIdRequestView<
                        '_,
                    > = ::connectrpc::dispatcher::codegen::decode_borrowed_request_view(
                        &body,
                        ctx.decode_options(),
                    )?;
                    let req = ::connectrpc::ServiceRequest::<
                        crate::proto::app::v1::GetAppByIdRequest,
                    >::from_parts(&req, &body);
                    svc.get_app_by_id(ctx, req)
                        .await?
                        .encode::<crate::proto::app::v1::GetAppByIdResponse>(format)
                })
            }
            "GetAppByHandle" => {
                let svc = ::std::sync::Arc::clone(&self.inner);
                Box::pin(async move {
                    let body = ::connectrpc::dispatcher::codegen::request_proto_bytes::<
                        crate::proto::app::v1::GetAppByHandleRequest,
                    >(request.encoded()?, format)?;
                    let req: crate::proto::app::v1::__buffa::view::GetAppByHandleRequestView<
                        '_,
                    > = ::connectrpc::dispatcher::codegen::decode_borrowed_request_view(
                        &body,
                        ctx.decode_options(),
                    )?;
                    let req = ::connectrpc::ServiceRequest::<
                        crate::proto::app::v1::GetAppByHandleRequest,
                    >::from_parts(&req, &body);
                    svc.get_app_by_handle(ctx, req)
                        .await?
                        .encode::<crate::proto::app::v1::GetAppByHandleResponse>(format)
                })
            }
            "SetAppStatus" => {
                let svc = ::std::sync::Arc::clone(&self.inner);
                Box::pin(async move {
                    let body = ::connectrpc::dispatcher::codegen::request_proto_bytes::<
                        crate::proto::app::v1::SetAppStatusRequest,
                    >(request.encoded()?, format)?;
                    let req: crate::proto::app::v1::__buffa::view::SetAppStatusRequestView<
                        '_,
                    > = ::connectrpc::dispatcher::codegen::decode_borrowed_request_view(
                        &body,
                        ctx.decode_options(),
                    )?;
                    let req = ::connectrpc::ServiceRequest::<
                        crate::proto::app::v1::SetAppStatusRequest,
                    >::from_parts(&req, &body);
                    svc.set_app_status(ctx, req)
                        .await?
                        .encode::<crate::proto::app::v1::SetAppStatusResponse>(format)
                })
            }
            "GetAll" => {
                let svc = ::std::sync::Arc::clone(&self.inner);
                Box::pin(async move {
                    let body = ::connectrpc::dispatcher::codegen::request_proto_bytes::<
                        crate::proto::app::v1::GetAllRequest,
                    >(request.encoded()?, format)?;
                    let req: crate::proto::app::v1::__buffa::view::GetAllRequestView<
                        '_,
                    > = ::connectrpc::dispatcher::codegen::decode_borrowed_request_view(
                        &body,
                        ctx.decode_options(),
                    )?;
                    let req = ::connectrpc::ServiceRequest::<
                        crate::proto::app::v1::GetAllRequest,
                    >::from_parts(&req, &body);
                    svc.get_all(ctx, req)
                        .await?
                        .encode::<crate::proto::app::v1::GetAllResponse>(format)
                })
            }
            _ => ::connectrpc::dispatcher::codegen::unimplemented_unary(path),
        }
    }
    fn call_server_streaming(
        &self,
        path: &str,
        ctx: ::connectrpc::RequestContext,
        request: ::buffa::bytes::Bytes,
        format: ::connectrpc::CodecFormat,
    ) -> ::connectrpc::dispatcher::codegen::StreamingResult {
        let Some(method) = path.strip_prefix("app.v1.AppService/") else {
            return ::connectrpc::dispatcher::codegen::unimplemented_streaming(path);
        };
        let _ = (&ctx, &request, &format);
        match method {
            _ => ::connectrpc::dispatcher::codegen::unimplemented_streaming(path),
        }
    }
    fn call_client_streaming(
        &self,
        path: &str,
        ctx: ::connectrpc::RequestContext,
        requests: ::connectrpc::dispatcher::codegen::RequestStream,
        format: ::connectrpc::CodecFormat,
    ) -> ::connectrpc::dispatcher::codegen::UnaryResult {
        let Some(method) = path.strip_prefix("app.v1.AppService/") else {
            return ::connectrpc::dispatcher::codegen::unimplemented_unary(path);
        };
        let _ = (&ctx, &requests, &format);
        match method {
            _ => ::connectrpc::dispatcher::codegen::unimplemented_unary(path),
        }
    }
    fn call_bidi_streaming(
        &self,
        path: &str,
        ctx: ::connectrpc::RequestContext,
        requests: ::connectrpc::dispatcher::codegen::RequestStream,
        format: ::connectrpc::CodecFormat,
    ) -> ::connectrpc::dispatcher::codegen::StreamingResult {
        let Some(method) = path.strip_prefix("app.v1.AppService/") else {
            return ::connectrpc::dispatcher::codegen::unimplemented_streaming(path);
        };
        let _ = (&ctx, &requests, &format);
        match method {
            _ => ::connectrpc::dispatcher::codegen::unimplemented_streaming(path),
        }
    }
}
/// Client for this service.
///
/// Generic over `T: ClientTransport`. For **gRPC** (HTTP/2), use
/// `Http2Connection` — it has honest `poll_ready` and composes with
/// `tower::balance` for multi-connection load balancing. For **Connect
/// over HTTP/1.1** (or unknown protocol), use `HttpClient`.
///
/// # Example (gRPC / HTTP/2)
///
/// ```rust,ignore
/// use connectrpc::client::{Http2Connection, ClientConfig};
/// use connectrpc::Protocol;
///
/// let uri: http::Uri = "http://localhost:8080".parse()?;
/// let conn = Http2Connection::connect_plaintext(uri.clone()).await?.shared(1024);
/// let config = ClientConfig::new(uri).with_protocol(Protocol::Grpc);
///
/// let client = AppServiceClient::new(conn, config);
/// let response = client.create_app(request).await?;
/// ```
///
/// # Example (Connect / HTTP/1.1 or ALPN)
///
/// ```rust,ignore
/// use connectrpc::client::{HttpClient, ClientConfig};
///
/// let http = HttpClient::plaintext();  // cleartext http:// only
/// let config = ClientConfig::new("http://localhost:8080".parse()?);
///
/// let client = AppServiceClient::new(http, config);
/// let response = client.create_app(request).await?;
/// ```
///
/// # Working with the response
///
/// Unary calls return [`UnaryResponse<OwnedView<FooView>>`](::connectrpc::client::UnaryResponse).
/// [`view()`](::connectrpc::client::UnaryResponse::view) borrows the response
/// message, so field access is zero-copy:
///
/// ```rust,ignore
/// let resp = client.create_app(request).await?;
/// let name: &str = resp.view().name;  // borrow into the response buffer
/// ```
///
/// If you need the owned struct (e.g. to store or pass by value), use
/// [`into_owned()`](::connectrpc::client::UnaryResponse::into_owned):
///
/// ```rust,ignore
/// let owned = client.create_app(request).await?.into_owned();
/// ```
///
/// [`into_view()`](::connectrpc::client::UnaryResponse::into_view) keeps the
/// zero-copy decoded body (an `OwnedView`) without copying; field access on it
/// goes through `.reborrow()`. Streaming responses yield one
/// [`StreamMessage`](::connectrpc::StreamMessage) per received message from
/// `.message().await` — read fields zero-copy through the generated accessor
/// methods (`msg.name()`) or `.view()`, or convert with `.to_owned_message()`.
#[derive(Clone)]
pub struct AppServiceClient<T> {
    transport: T,
    config: ::connectrpc::client::ClientConfig,
}
impl<T> AppServiceClient<T>
where
    T: ::connectrpc::client::ClientTransport,
    <T::ResponseBody as ::connectrpc::http_body::Body>::Error: ::std::fmt::Display,
{
    /// Create a new client with the given transport and configuration.
    pub fn new(transport: T, config: ::connectrpc::client::ClientConfig) -> Self {
        Self { transport, config }
    }
    /// Get the client configuration.
    pub fn config(&self) -> &::connectrpc::client::ClientConfig {
        &self.config
    }
    /// Get a mutable reference to the client configuration.
    pub fn config_mut(&mut self) -> &mut ::connectrpc::client::ClientConfig {
        &mut self.config
    }
    /// Call the CreateApp RPC. Sends a request to /app.v1.AppService/CreateApp.
    pub async fn create_app(
        &self,
        request: crate::proto::app::v1::CreateAppRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::proto::app::v1::__buffa::view::CreateAppResponseView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        self.create_app_with_options(
                request,
                ::connectrpc::client::CallOptions::default(),
            )
            .await
    }
    /// Call the CreateApp RPC with explicit per-call options. Options override [`ClientConfig`](::connectrpc::client::ClientConfig) defaults.
    pub async fn create_app_with_options(
        &self,
        request: crate::proto::app::v1::CreateAppRequest,
        options: ::connectrpc::client::CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::proto::app::v1::__buffa::view::CreateAppResponseView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        ::connectrpc::client::call_unary(
                &self.transport,
                &self.config,
                APP_SERVICE_CREATE_APP_SPEC
                    .with_origin(::connectrpc::SpecOrigin::Client),
                request,
                options,
            )
            .await
    }
    /// Call the UpdateApp RPC. Sends a request to /app.v1.AppService/UpdateApp.
    pub async fn update_app(
        &self,
        request: crate::proto::app::v1::UpdateAppRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::proto::app::v1::__buffa::view::UpdateAppResponseView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        self.update_app_with_options(
                request,
                ::connectrpc::client::CallOptions::default(),
            )
            .await
    }
    /// Call the UpdateApp RPC with explicit per-call options. Options override [`ClientConfig`](::connectrpc::client::ClientConfig) defaults.
    pub async fn update_app_with_options(
        &self,
        request: crate::proto::app::v1::UpdateAppRequest,
        options: ::connectrpc::client::CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::proto::app::v1::__buffa::view::UpdateAppResponseView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        ::connectrpc::client::call_unary(
                &self.transport,
                &self.config,
                APP_SERVICE_UPDATE_APP_SPEC
                    .with_origin(::connectrpc::SpecOrigin::Client),
                request,
                options,
            )
            .await
    }
    /// Call the GetAppById RPC. Sends a request to /app.v1.AppService/GetAppById.
    pub async fn get_app_by_id(
        &self,
        request: crate::proto::app::v1::GetAppByIdRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::proto::app::v1::__buffa::view::GetAppByIdResponseView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        self.get_app_by_id_with_options(
                request,
                ::connectrpc::client::CallOptions::default(),
            )
            .await
    }
    /// Call the GetAppById RPC with explicit per-call options. Options override [`ClientConfig`](::connectrpc::client::ClientConfig) defaults.
    pub async fn get_app_by_id_with_options(
        &self,
        request: crate::proto::app::v1::GetAppByIdRequest,
        options: ::connectrpc::client::CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::proto::app::v1::__buffa::view::GetAppByIdResponseView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        ::connectrpc::client::call_unary(
                &self.transport,
                &self.config,
                APP_SERVICE_GET_APP_BY_ID_SPEC
                    .with_origin(::connectrpc::SpecOrigin::Client),
                request,
                options,
            )
            .await
    }
    /// Call the GetAppByHandle RPC. Sends a request to /app.v1.AppService/GetAppByHandle.
    pub async fn get_app_by_handle(
        &self,
        request: crate::proto::app::v1::GetAppByHandleRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::proto::app::v1::__buffa::view::GetAppByHandleResponseView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        self.get_app_by_handle_with_options(
                request,
                ::connectrpc::client::CallOptions::default(),
            )
            .await
    }
    /// Call the GetAppByHandle RPC with explicit per-call options. Options override [`ClientConfig`](::connectrpc::client::ClientConfig) defaults.
    pub async fn get_app_by_handle_with_options(
        &self,
        request: crate::proto::app::v1::GetAppByHandleRequest,
        options: ::connectrpc::client::CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::proto::app::v1::__buffa::view::GetAppByHandleResponseView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        ::connectrpc::client::call_unary(
                &self.transport,
                &self.config,
                APP_SERVICE_GET_APP_BY_HANDLE_SPEC
                    .with_origin(::connectrpc::SpecOrigin::Client),
                request,
                options,
            )
            .await
    }
    /// Call the SetAppStatus RPC. Sends a request to /app.v1.AppService/SetAppStatus.
    pub async fn set_app_status(
        &self,
        request: crate::proto::app::v1::SetAppStatusRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::proto::app::v1::__buffa::view::SetAppStatusResponseView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        self.set_app_status_with_options(
                request,
                ::connectrpc::client::CallOptions::default(),
            )
            .await
    }
    /// Call the SetAppStatus RPC with explicit per-call options. Options override [`ClientConfig`](::connectrpc::client::ClientConfig) defaults.
    pub async fn set_app_status_with_options(
        &self,
        request: crate::proto::app::v1::SetAppStatusRequest,
        options: ::connectrpc::client::CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::proto::app::v1::__buffa::view::SetAppStatusResponseView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        ::connectrpc::client::call_unary(
                &self.transport,
                &self.config,
                APP_SERVICE_SET_APP_STATUS_SPEC
                    .with_origin(::connectrpc::SpecOrigin::Client),
                request,
                options,
            )
            .await
    }
    /// Call the GetAll RPC. Sends a request to /app.v1.AppService/GetAll.
    pub async fn get_all(
        &self,
        request: crate::proto::app::v1::GetAllRequest,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::proto::app::v1::__buffa::view::GetAllResponseView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        self.get_all_with_options(request, ::connectrpc::client::CallOptions::default())
            .await
    }
    /// Call the GetAll RPC with explicit per-call options. Options override [`ClientConfig`](::connectrpc::client::ClientConfig) defaults.
    pub async fn get_all_with_options(
        &self,
        request: crate::proto::app::v1::GetAllRequest,
        options: ::connectrpc::client::CallOptions,
    ) -> Result<
        ::connectrpc::client::UnaryResponse<
            ::buffa::view::OwnedView<
                crate::proto::app::v1::__buffa::view::GetAllResponseView<'static>,
            >,
        >,
        ::connectrpc::ConnectError,
    > {
        ::connectrpc::client::call_unary(
                &self.transport,
                &self.config,
                APP_SERVICE_GET_ALL_SPEC.with_origin(::connectrpc::SpecOrigin::Client),
                request,
                options,
            )
            .await
    }
}
