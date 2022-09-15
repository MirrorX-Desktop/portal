#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(descriptor_pool = "crate::DESCRIPTOR_POOL", message_name = "message.RegisterRequest")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterRequest {
    #[prost(sfixed64, optional, tag="1")]
    pub device_id: ::core::option::Option<i64>,
    #[prost(string, tag="2")]
    pub device_finger_print: ::prost::alloc::string::String,
}
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(descriptor_pool = "crate::DESCRIPTOR_POOL", message_name = "message.RegisterResponse")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterResponse {
    #[prost(sfixed64, tag="1")]
    pub device_id: i64,
    #[prost(sfixed64, tag="2")]
    pub expire: i64,
}
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(descriptor_pool = "crate::DESCRIPTOR_POOL", message_name = "message.HeartbeatRequest")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeartbeatRequest {
    #[prost(sfixed64, tag="1")]
    pub device_id: i64,
    #[prost(fixed32, tag="2")]
    pub timestamp: u32,
}
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(descriptor_pool = "crate::DESCRIPTOR_POOL", message_name = "message.HeartbeatResponse")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HeartbeatResponse {
    #[prost(fixed32, tag="1")]
    pub timestamp: u32,
}
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(descriptor_pool = "crate::DESCRIPTOR_POOL", message_name = "message.VisitRequest")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VisitRequest {
    #[prost(string, tag="1")]
    pub domain: ::prost::alloc::string::String,
    #[prost(sfixed64, tag="2")]
    pub active_device_id: i64,
    #[prost(sfixed64, tag="3")]
    pub passive_device_id: i64,
    #[prost(enumeration="ResourceType", tag="4")]
    pub resource_type: i32,
}
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(descriptor_pool = "crate::DESCRIPTOR_POOL", message_name = "message.VisitResponse")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VisitResponse {
    #[prost(string, tag="1")]
    pub domain: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub allow: bool,
}
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(descriptor_pool = "crate::DESCRIPTOR_POOL", message_name = "message.VisitReplyRequest")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VisitReplyRequest {
    #[prost(string, tag="1")]
    pub domain: ::prost::alloc::string::String,
    #[prost(sfixed64, tag="2")]
    pub active_device_id: i64,
    #[prost(sfixed64, tag="3")]
    pub passive_device_id: i64,
    #[prost(bool, tag="4")]
    pub allow: bool,
}
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(descriptor_pool = "crate::DESCRIPTOR_POOL", message_name = "message.VisitReplyResponse")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VisitReplyResponse {
}
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(descriptor_pool = "crate::DESCRIPTOR_POOL", message_name = "message.KeyExchangeRequest")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyExchangeRequest {
    #[prost(string, tag="1")]
    pub domain: ::prost::alloc::string::String,
    #[prost(sfixed64, tag="2")]
    pub active_device_id: i64,
    #[prost(sfixed64, tag="3")]
    pub passive_device_id: i64,
    #[prost(bytes="vec", tag="4")]
    pub password_salt: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="5")]
    pub secret: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="6")]
    pub secret_nonce: ::prost::alloc::vec::Vec<u8>,
}
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(descriptor_pool = "crate::DESCRIPTOR_POOL", message_name = "message.KeyExchangeResponse")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyExchangeResponse {
    #[prost(string, tag="1")]
    pub domain: ::prost::alloc::string::String,
    #[prost(sfixed64, tag="2")]
    pub active_device_id: i64,
    #[prost(sfixed64, tag="3")]
    pub passive_device_id: i64,
    #[prost(message, optional, tag="4")]
    pub key_exchange_result: ::core::option::Option<KeyExchangeResult>,
}
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(descriptor_pool = "crate::DESCRIPTOR_POOL", message_name = "message.KeyExchangeResult")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyExchangeResult {
    #[prost(oneof="key_exchange_result::InnerKeyExchangeResult", tags="1, 2")]
    pub inner_key_exchange_result: ::core::option::Option<key_exchange_result::InnerKeyExchangeResult>,
}
/// Nested message and enum types in `KeyExchangeResult`.
pub mod key_exchange_result {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum InnerKeyExchangeResult {
        #[prost(enumeration="super::KeyExchangeReplyError", tag="1")]
        Error(i32),
        #[prost(bytes, tag="2")]
        Secret(::prost::alloc::vec::Vec<u8>),
    }
}
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(descriptor_pool = "crate::DESCRIPTOR_POOL", message_name = "message.KeyExchangeReplyRequest")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyExchangeReplyRequest {
    #[prost(string, tag="1")]
    pub domain: ::prost::alloc::string::String,
    #[prost(sfixed64, tag="2")]
    pub active_device_id: i64,
    #[prost(sfixed64, tag="3")]
    pub passive_device_id: i64,
    #[prost(message, optional, tag="4")]
    pub key_exchange_result: ::core::option::Option<KeyExchangeResult>,
}
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(descriptor_pool = "crate::DESCRIPTOR_POOL", message_name = "message.KeyExchangeReplyResponse")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyExchangeReplyResponse {
}
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(descriptor_pool = "crate::DESCRIPTOR_POOL", message_name = "message.KeyExchangeActiveDeviceSecret")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyExchangeActiveDeviceSecret {
    #[prost(bytes="vec", tag="1")]
    pub exchange_reply_public_key_n: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub exchange_reply_public_key_e: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub active_exchange_public_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="4")]
    pub active_exchange_nonce: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag="5")]
    pub visit_credentials: ::prost::alloc::string::String,
}
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(descriptor_pool = "crate::DESCRIPTOR_POOL", message_name = "message.KeyExchangePassiveDeviceSecret")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyExchangePassiveDeviceSecret {
    #[prost(bytes="vec", tag="1")]
    pub passive_exchange_public_key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="2")]
    pub passive_exchange_nonce: ::prost::alloc::vec::Vec<u8>,
}
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(descriptor_pool = "crate::DESCRIPTOR_POOL", message_name = "message.SubscribeRequest")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeRequest {
    #[prost(sfixed64, tag="1")]
    pub device_id: i64,
    #[prost(string, tag="2")]
    pub device_finger_print: ::prost::alloc::string::String,
}
#[derive(::prost_reflect::ReflectMessage)]
#[prost_reflect(descriptor_pool = "crate::DESCRIPTOR_POOL", message_name = "message.PublishMessage")]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishMessage {
    #[prost(oneof="publish_message::InnerPublishMessage", tags="1, 2")]
    pub inner_publish_message: ::core::option::Option<publish_message::InnerPublishMessage>,
}
/// Nested message and enum types in `PublishMessage`.
pub mod publish_message {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum InnerPublishMessage {
        #[prost(message, tag="1")]
        VisitRequest(super::VisitRequest),
        #[prost(message, tag="2")]
        KeyExchangeRequest(super::KeyExchangeRequest),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ResourceType {
    Desktop = 0,
    Files = 1,
}
impl ResourceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ResourceType::Desktop => "RESOURCE_TYPE_DESKTOP",
            ResourceType::Files => "RESOURCE_TYPE_FILES",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum KeyExchangeReplyError {
    Internal = 0,
    InvalidArgs = 1,
    InvalidPassword = 2,
}
impl KeyExchangeReplyError {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            KeyExchangeReplyError::Internal => "KEY_EXCHANGE_REPLY_ERROR_INTERNAL",
            KeyExchangeReplyError::InvalidArgs => "KEY_EXCHANGE_REPLY_ERROR_INVALID_ARGS",
            KeyExchangeReplyError::InvalidPassword => "KEY_EXCHANGE_REPLY_ERROR_INVALID_PASSWORD",
        }
    }
}
