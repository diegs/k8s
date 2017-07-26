// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct ExtraValue {
    // message fields
    items: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ExtraValue {}

impl ExtraValue {
    pub fn new() -> ExtraValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExtraValue {
        static mut instance: ::protobuf::lazy::Lazy<ExtraValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExtraValue,
        };
        unsafe {
            instance.get(ExtraValue::new)
        }
    }

    // repeated string items = 1;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[::std::string::String] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.items
    }
}

impl ::protobuf::Message for ExtraValue {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.items)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.items {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.items {
            os.write_string(1, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ExtraValue {
    fn new() -> ExtraValue {
        ExtraValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExtraValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "items",
                    ExtraValue::get_items_for_reflect,
                    ExtraValue::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExtraValue>(
                    "ExtraValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExtraValue {
    fn clear(&mut self) {
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ExtraValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExtraValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TokenReview {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    spec: ::protobuf::SingularPtrField<TokenReviewSpec>,
    status: ::protobuf::SingularPtrField<TokenReviewStatus>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TokenReview {}

impl TokenReview {
    pub fn new() -> TokenReview {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TokenReview {
        static mut instance: ::protobuf::lazy::Lazy<TokenReview> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TokenReview,
        };
        unsafe {
            instance.get(TokenReview::new)
        }
    }

    // optional .k8s.io.apimachinery.pkg.apis.meta.v1.ObjectMeta metadata = 1;

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: super::generated::ObjectMeta) {
        self.metadata = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut super::generated::ObjectMeta {
        if self.metadata.is_none() {
            self.metadata.set_default();
        }
        self.metadata.as_mut().unwrap()
    }

    // Take field
    pub fn take_metadata(&mut self) -> super::generated::ObjectMeta {
        self.metadata.take().unwrap_or_else(|| super::generated::ObjectMeta::new())
    }

    pub fn get_metadata(&self) -> &super::generated::ObjectMeta {
        self.metadata.as_ref().unwrap_or_else(|| super::generated::ObjectMeta::default_instance())
    }

    fn get_metadata_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::ObjectMeta> {
        &self.metadata
    }

    fn mut_metadata_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::ObjectMeta> {
        &mut self.metadata
    }

    // optional .k8s.io.api.authentication.v1beta1.TokenReviewSpec spec = 2;

    pub fn clear_spec(&mut self) {
        self.spec.clear();
    }

    pub fn has_spec(&self) -> bool {
        self.spec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spec(&mut self, v: TokenReviewSpec) {
        self.spec = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_spec(&mut self) -> &mut TokenReviewSpec {
        if self.spec.is_none() {
            self.spec.set_default();
        }
        self.spec.as_mut().unwrap()
    }

    // Take field
    pub fn take_spec(&mut self) -> TokenReviewSpec {
        self.spec.take().unwrap_or_else(|| TokenReviewSpec::new())
    }

    pub fn get_spec(&self) -> &TokenReviewSpec {
        self.spec.as_ref().unwrap_or_else(|| TokenReviewSpec::default_instance())
    }

    fn get_spec_for_reflect(&self) -> &::protobuf::SingularPtrField<TokenReviewSpec> {
        &self.spec
    }

    fn mut_spec_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TokenReviewSpec> {
        &mut self.spec
    }

    // optional .k8s.io.api.authentication.v1beta1.TokenReviewStatus status = 3;

    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: TokenReviewStatus) {
        self.status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut TokenReviewStatus {
        if self.status.is_none() {
            self.status.set_default();
        }
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> TokenReviewStatus {
        self.status.take().unwrap_or_else(|| TokenReviewStatus::new())
    }

    pub fn get_status(&self) -> &TokenReviewStatus {
        self.status.as_ref().unwrap_or_else(|| TokenReviewStatus::default_instance())
    }

    fn get_status_for_reflect(&self) -> &::protobuf::SingularPtrField<TokenReviewStatus> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TokenReviewStatus> {
        &mut self.status
    }
}

impl ::protobuf::Message for TokenReview {
    fn is_initialized(&self) -> bool {
        for v in &self.metadata {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.spec {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.status {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.metadata)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.spec)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.status)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.metadata.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.spec.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.status.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.metadata.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.spec.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.status.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TokenReview {
    fn new() -> TokenReview {
        TokenReview::new()
    }

    fn descriptor_static(_: ::std::option::Option<TokenReview>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    TokenReview::get_metadata_for_reflect,
                    TokenReview::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TokenReviewSpec>>(
                    "spec",
                    TokenReview::get_spec_for_reflect,
                    TokenReview::mut_spec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TokenReviewStatus>>(
                    "status",
                    TokenReview::get_status_for_reflect,
                    TokenReview::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TokenReview>(
                    "TokenReview",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TokenReview {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_spec();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TokenReview {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TokenReview {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TokenReviewSpec {
    // message fields
    token: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TokenReviewSpec {}

impl TokenReviewSpec {
    pub fn new() -> TokenReviewSpec {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TokenReviewSpec {
        static mut instance: ::protobuf::lazy::Lazy<TokenReviewSpec> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TokenReviewSpec,
        };
        unsafe {
            instance.get(TokenReviewSpec::new)
        }
    }

    // optional string token = 1;

    pub fn clear_token(&mut self) {
        self.token.clear();
    }

    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    // Param is passed by value, moved
    pub fn set_token(&mut self, v: ::std::string::String) {
        self.token = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_token(&mut self) -> &mut ::std::string::String {
        if self.token.is_none() {
            self.token.set_default();
        }
        self.token.as_mut().unwrap()
    }

    // Take field
    pub fn take_token(&mut self) -> ::std::string::String {
        self.token.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_token(&self) -> &str {
        match self.token.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_token_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.token
    }

    fn mut_token_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.token
    }
}

impl ::protobuf::Message for TokenReviewSpec {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.token)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.token.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.token.as_ref() {
            os.write_string(1, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TokenReviewSpec {
    fn new() -> TokenReviewSpec {
        TokenReviewSpec::new()
    }

    fn descriptor_static(_: ::std::option::Option<TokenReviewSpec>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "token",
                    TokenReviewSpec::get_token_for_reflect,
                    TokenReviewSpec::mut_token_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TokenReviewSpec>(
                    "TokenReviewSpec",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TokenReviewSpec {
    fn clear(&mut self) {
        self.clear_token();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TokenReviewSpec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TokenReviewSpec {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TokenReviewStatus {
    // message fields
    authenticated: ::std::option::Option<bool>,
    user: ::protobuf::SingularPtrField<UserInfo>,
    error: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TokenReviewStatus {}

impl TokenReviewStatus {
    pub fn new() -> TokenReviewStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TokenReviewStatus {
        static mut instance: ::protobuf::lazy::Lazy<TokenReviewStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TokenReviewStatus,
        };
        unsafe {
            instance.get(TokenReviewStatus::new)
        }
    }

    // optional bool authenticated = 1;

    pub fn clear_authenticated(&mut self) {
        self.authenticated = ::std::option::Option::None;
    }

    pub fn has_authenticated(&self) -> bool {
        self.authenticated.is_some()
    }

    // Param is passed by value, moved
    pub fn set_authenticated(&mut self, v: bool) {
        self.authenticated = ::std::option::Option::Some(v);
    }

    pub fn get_authenticated(&self) -> bool {
        self.authenticated.unwrap_or(false)
    }

    fn get_authenticated_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.authenticated
    }

    fn mut_authenticated_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.authenticated
    }

    // optional .k8s.io.api.authentication.v1beta1.UserInfo user = 2;

    pub fn clear_user(&mut self) {
        self.user.clear();
    }

    pub fn has_user(&self) -> bool {
        self.user.is_some()
    }

    // Param is passed by value, moved
    pub fn set_user(&mut self, v: UserInfo) {
        self.user = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_user(&mut self) -> &mut UserInfo {
        if self.user.is_none() {
            self.user.set_default();
        }
        self.user.as_mut().unwrap()
    }

    // Take field
    pub fn take_user(&mut self) -> UserInfo {
        self.user.take().unwrap_or_else(|| UserInfo::new())
    }

    pub fn get_user(&self) -> &UserInfo {
        self.user.as_ref().unwrap_or_else(|| UserInfo::default_instance())
    }

    fn get_user_for_reflect(&self) -> &::protobuf::SingularPtrField<UserInfo> {
        &self.user
    }

    fn mut_user_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<UserInfo> {
        &mut self.user
    }

    // optional string error = 3;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: ::std::string::String) {
        self.error = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut ::std::string::String {
        if self.error.is_none() {
            self.error.set_default();
        }
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> ::std::string::String {
        self.error.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_error(&self) -> &str {
        match self.error.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.error
    }
}

impl ::protobuf::Message for TokenReviewStatus {
    fn is_initialized(&self) -> bool {
        for v in &self.user {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.authenticated = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.user)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.error)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.authenticated {
            my_size += 2;
        }
        if let Some(ref v) = self.user.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.error.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.authenticated {
            os.write_bool(1, v)?;
        }
        if let Some(ref v) = self.user.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.error.as_ref() {
            os.write_string(3, &v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TokenReviewStatus {
    fn new() -> TokenReviewStatus {
        TokenReviewStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<TokenReviewStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "authenticated",
                    TokenReviewStatus::get_authenticated_for_reflect,
                    TokenReviewStatus::mut_authenticated_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<UserInfo>>(
                    "user",
                    TokenReviewStatus::get_user_for_reflect,
                    TokenReviewStatus::mut_user_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "error",
                    TokenReviewStatus::get_error_for_reflect,
                    TokenReviewStatus::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TokenReviewStatus>(
                    "TokenReviewStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TokenReviewStatus {
    fn clear(&mut self) {
        self.clear_authenticated();
        self.clear_user();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TokenReviewStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TokenReviewStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UserInfo {
    // message fields
    username: ::protobuf::SingularField<::std::string::String>,
    uid: ::protobuf::SingularField<::std::string::String>,
    groups: ::protobuf::RepeatedField<::std::string::String>,
    pub extra: ::std::collections::HashMap<::std::string::String, ExtraValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UserInfo {}

impl UserInfo {
    pub fn new() -> UserInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UserInfo {
        static mut instance: ::protobuf::lazy::Lazy<UserInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UserInfo,
        };
        unsafe {
            instance.get(UserInfo::new)
        }
    }

    // optional string username = 1;

    pub fn clear_username(&mut self) {
        self.username.clear();
    }

    pub fn has_username(&self) -> bool {
        self.username.is_some()
    }

    // Param is passed by value, moved
    pub fn set_username(&mut self, v: ::std::string::String) {
        self.username = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_username(&mut self) -> &mut ::std::string::String {
        if self.username.is_none() {
            self.username.set_default();
        }
        self.username.as_mut().unwrap()
    }

    // Take field
    pub fn take_username(&mut self) -> ::std::string::String {
        self.username.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_username(&self) -> &str {
        match self.username.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_username_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.username
    }

    fn mut_username_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.username
    }

    // optional string uid = 2;

    pub fn clear_uid(&mut self) {
        self.uid.clear();
    }

    pub fn has_uid(&self) -> bool {
        self.uid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uid(&mut self, v: ::std::string::String) {
        self.uid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uid(&mut self) -> &mut ::std::string::String {
        if self.uid.is_none() {
            self.uid.set_default();
        }
        self.uid.as_mut().unwrap()
    }

    // Take field
    pub fn take_uid(&mut self) -> ::std::string::String {
        self.uid.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_uid(&self) -> &str {
        match self.uid.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_uid_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.uid
    }

    fn mut_uid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.uid
    }

    // repeated string groups = 3;

    pub fn clear_groups(&mut self) {
        self.groups.clear();
    }

    // Param is passed by value, moved
    pub fn set_groups(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.groups = v;
    }

    // Mutable pointer to the field.
    pub fn mut_groups(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.groups
    }

    // Take field
    pub fn take_groups(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.groups, ::protobuf::RepeatedField::new())
    }

    pub fn get_groups(&self) -> &[::std::string::String] {
        &self.groups
    }

    fn get_groups_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.groups
    }

    fn mut_groups_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.groups
    }

    // repeated .k8s.io.api.authentication.v1beta1.UserInfo.ExtraEntry extra = 4;

    pub fn clear_extra(&mut self) {
        self.extra.clear();
    }

    // Param is passed by value, moved
    pub fn set_extra(&mut self, v: ::std::collections::HashMap<::std::string::String, ExtraValue>) {
        self.extra = v;
    }

    // Mutable pointer to the field.
    pub fn mut_extra(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ExtraValue> {
        &mut self.extra
    }

    // Take field
    pub fn take_extra(&mut self) -> ::std::collections::HashMap<::std::string::String, ExtraValue> {
        ::std::mem::replace(&mut self.extra, ::std::collections::HashMap::new())
    }

    pub fn get_extra(&self) -> &::std::collections::HashMap<::std::string::String, ExtraValue> {
        &self.extra
    }

    fn get_extra_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, ExtraValue> {
        &self.extra
    }

    fn mut_extra_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ExtraValue> {
        &mut self.extra
    }
}

impl ::protobuf::Message for UserInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.username)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.uid)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.groups)?;
                },
                4 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<ExtraValue>>(wire_type, is, &mut self.extra)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.username.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.uid.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        for value in &self.groups {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<ExtraValue>>(4, &self.extra);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.username.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.uid.as_ref() {
            os.write_string(2, &v)?;
        }
        for v in &self.groups {
            os.write_string(3, &v)?;
        };
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<ExtraValue>>(4, &self.extra, os)?;
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for UserInfo {
    fn new() -> UserInfo {
        UserInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<UserInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "username",
                    UserInfo::get_username_for_reflect,
                    UserInfo::mut_username_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "uid",
                    UserInfo::get_uid_for_reflect,
                    UserInfo::mut_uid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "groups",
                    UserInfo::get_groups_for_reflect,
                    UserInfo::mut_groups_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<ExtraValue>>(
                    "extra",
                    UserInfo::get_extra_for_reflect,
                    UserInfo::mut_extra_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UserInfo>(
                    "UserInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UserInfo {
    fn clear(&mut self) {
        self.clear_username();
        self.clear_uid();
        self.clear_groups();
        self.clear_extra();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UserInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UserInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n1k8s.io/api/authentication/v1beta1/generated.proto\x12!k8s.io.api.auth\
    entication.v1beta1\x1a4k8s.io/apimachinery/pkg/apis/meta/v1/generated.pr\
    oto\x1a/k8s.io/apimachinery/pkg/runtime/generated.proto\x1a6k8s.io/apima\
    chinery/pkg/runtime/schema/generated.proto\x1a3k8s.io/apimachinery/pkg/u\
    til/intstr/generated.proto\"\"\n\nExtraValue\x12\x14\n\x05items\x18\x01\
    \x20\x03(\tR\x05items\"\xf1\x01\n\x0bTokenReview\x12L\n\x08metadata\x18\
    \x01\x20\x01(\x0b20.k8s.io.apimachinery.pkg.apis.meta.v1.ObjectMetaR\x08\
    metadata\x12F\n\x04spec\x18\x02\x20\x01(\x0b22.k8s.io.api.authentication\
    .v1beta1.TokenReviewSpecR\x04spec\x12L\n\x06status\x18\x03\x20\x01(\x0b2\
    4.k8s.io.api.authentication.v1beta1.TokenReviewStatusR\x06status\"'\n\
    \x0fTokenReviewSpec\x12\x14\n\x05token\x18\x01\x20\x01(\tR\x05token\"\
    \x90\x01\n\x11TokenReviewStatus\x12$\n\rauthenticated\x18\x01\x20\x01(\
    \x08R\rauthenticated\x12?\n\x04user\x18\x02\x20\x01(\x0b2+.k8s.io.api.au\
    thentication.v1beta1.UserInfoR\x04user\x12\x14\n\x05error\x18\x03\x20\
    \x01(\tR\x05error\"\x87\x02\n\x08UserInfo\x12\x1a\n\x08username\x18\x01\
    \x20\x01(\tR\x08username\x12\x10\n\x03uid\x18\x02\x20\x01(\tR\x03uid\x12\
    \x16\n\x06groups\x18\x03\x20\x03(\tR\x06groups\x12L\n\x05extra\x18\x04\
    \x20\x03(\x0b26.k8s.io.api.authentication.v1beta1.UserInfo.ExtraEntryR\
    \x05extra\x1ag\n\nExtraEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\
    \x12C\n\x05value\x18\x02\x20\x01(\x0b2-.k8s.io.api.authentication.v1beta\
    1.ExtraValueR\x05value:\x028\x01B\tZ\x07v1beta1\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
