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
pub struct CertificateSigningRequest {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    spec: ::protobuf::SingularPtrField<CertificateSigningRequestSpec>,
    status: ::protobuf::SingularPtrField<CertificateSigningRequestStatus>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CertificateSigningRequest {}

impl CertificateSigningRequest {
    pub fn new() -> CertificateSigningRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CertificateSigningRequest {
        static mut instance: ::protobuf::lazy::Lazy<CertificateSigningRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CertificateSigningRequest,
        };
        unsafe {
            instance.get(CertificateSigningRequest::new)
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

    // optional .k8s.io.api.certificates.v1beta1.CertificateSigningRequestSpec spec = 2;

    pub fn clear_spec(&mut self) {
        self.spec.clear();
    }

    pub fn has_spec(&self) -> bool {
        self.spec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spec(&mut self, v: CertificateSigningRequestSpec) {
        self.spec = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_spec(&mut self) -> &mut CertificateSigningRequestSpec {
        if self.spec.is_none() {
            self.spec.set_default();
        }
        self.spec.as_mut().unwrap()
    }

    // Take field
    pub fn take_spec(&mut self) -> CertificateSigningRequestSpec {
        self.spec.take().unwrap_or_else(|| CertificateSigningRequestSpec::new())
    }

    pub fn get_spec(&self) -> &CertificateSigningRequestSpec {
        self.spec.as_ref().unwrap_or_else(|| CertificateSigningRequestSpec::default_instance())
    }

    fn get_spec_for_reflect(&self) -> &::protobuf::SingularPtrField<CertificateSigningRequestSpec> {
        &self.spec
    }

    fn mut_spec_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CertificateSigningRequestSpec> {
        &mut self.spec
    }

    // optional .k8s.io.api.certificates.v1beta1.CertificateSigningRequestStatus status = 3;

    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: CertificateSigningRequestStatus) {
        self.status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut CertificateSigningRequestStatus {
        if self.status.is_none() {
            self.status.set_default();
        }
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> CertificateSigningRequestStatus {
        self.status.take().unwrap_or_else(|| CertificateSigningRequestStatus::new())
    }

    pub fn get_status(&self) -> &CertificateSigningRequestStatus {
        self.status.as_ref().unwrap_or_else(|| CertificateSigningRequestStatus::default_instance())
    }

    fn get_status_for_reflect(&self) -> &::protobuf::SingularPtrField<CertificateSigningRequestStatus> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CertificateSigningRequestStatus> {
        &mut self.status
    }
}

impl ::protobuf::Message for CertificateSigningRequest {
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

impl ::protobuf::MessageStatic for CertificateSigningRequest {
    fn new() -> CertificateSigningRequest {
        CertificateSigningRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CertificateSigningRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    CertificateSigningRequest::get_metadata_for_reflect,
                    CertificateSigningRequest::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CertificateSigningRequestSpec>>(
                    "spec",
                    CertificateSigningRequest::get_spec_for_reflect,
                    CertificateSigningRequest::mut_spec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CertificateSigningRequestStatus>>(
                    "status",
                    CertificateSigningRequest::get_status_for_reflect,
                    CertificateSigningRequest::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CertificateSigningRequest>(
                    "CertificateSigningRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CertificateSigningRequest {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_spec();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CertificateSigningRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CertificateSigningRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CertificateSigningRequestCondition {
    // message fields
    field_type: ::protobuf::SingularField<::std::string::String>,
    reason: ::protobuf::SingularField<::std::string::String>,
    message: ::protobuf::SingularField<::std::string::String>,
    lastUpdateTime: ::protobuf::SingularPtrField<super::generated::Time>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CertificateSigningRequestCondition {}

impl CertificateSigningRequestCondition {
    pub fn new() -> CertificateSigningRequestCondition {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CertificateSigningRequestCondition {
        static mut instance: ::protobuf::lazy::Lazy<CertificateSigningRequestCondition> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CertificateSigningRequestCondition,
        };
        unsafe {
            instance.get(CertificateSigningRequestCondition::new)
        }
    }

    // optional string type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type.clear();
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ::std::string::String) {
        self.field_type = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_field_type(&mut self) -> &mut ::std::string::String {
        if self.field_type.is_none() {
            self.field_type.set_default();
        }
        self.field_type.as_mut().unwrap()
    }

    // Take field
    pub fn take_field_type(&mut self) -> ::std::string::String {
        self.field_type.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_field_type(&self) -> &str {
        match self.field_type.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_field_type_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.field_type
    }

    // optional string reason = 2;

    pub fn clear_reason(&mut self) {
        self.reason.clear();
    }

    pub fn has_reason(&self) -> bool {
        self.reason.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reason(&mut self, v: ::std::string::String) {
        self.reason = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reason(&mut self) -> &mut ::std::string::String {
        if self.reason.is_none() {
            self.reason.set_default();
        }
        self.reason.as_mut().unwrap()
    }

    // Take field
    pub fn take_reason(&mut self) -> ::std::string::String {
        self.reason.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_reason(&self) -> &str {
        match self.reason.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_reason_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.reason
    }

    fn mut_reason_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.reason
    }

    // optional string message = 3;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        if self.message.is_none() {
            self.message.set_default();
        }
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        self.message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message(&self) -> &str {
        match self.message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_message_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.message
    }

    // optional .k8s.io.apimachinery.pkg.apis.meta.v1.Time lastUpdateTime = 4;

    pub fn clear_lastUpdateTime(&mut self) {
        self.lastUpdateTime.clear();
    }

    pub fn has_lastUpdateTime(&self) -> bool {
        self.lastUpdateTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastUpdateTime(&mut self, v: super::generated::Time) {
        self.lastUpdateTime = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lastUpdateTime(&mut self) -> &mut super::generated::Time {
        if self.lastUpdateTime.is_none() {
            self.lastUpdateTime.set_default();
        }
        self.lastUpdateTime.as_mut().unwrap()
    }

    // Take field
    pub fn take_lastUpdateTime(&mut self) -> super::generated::Time {
        self.lastUpdateTime.take().unwrap_or_else(|| super::generated::Time::new())
    }

    pub fn get_lastUpdateTime(&self) -> &super::generated::Time {
        self.lastUpdateTime.as_ref().unwrap_or_else(|| super::generated::Time::default_instance())
    }

    fn get_lastUpdateTime_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::Time> {
        &self.lastUpdateTime
    }

    fn mut_lastUpdateTime_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::Time> {
        &mut self.lastUpdateTime
    }
}

impl ::protobuf::Message for CertificateSigningRequestCondition {
    fn is_initialized(&self) -> bool {
        for v in &self.lastUpdateTime {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.field_type)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.reason)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.message)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.lastUpdateTime)?;
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
        if let Some(ref v) = self.field_type.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.reason.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.message.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.lastUpdateTime.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.field_type.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.reason.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.message.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.lastUpdateTime.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CertificateSigningRequestCondition {
    fn new() -> CertificateSigningRequestCondition {
        CertificateSigningRequestCondition::new()
    }

    fn descriptor_static(_: ::std::option::Option<CertificateSigningRequestCondition>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    CertificateSigningRequestCondition::get_field_type_for_reflect,
                    CertificateSigningRequestCondition::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "reason",
                    CertificateSigningRequestCondition::get_reason_for_reflect,
                    CertificateSigningRequestCondition::mut_reason_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    CertificateSigningRequestCondition::get_message_for_reflect,
                    CertificateSigningRequestCondition::mut_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::Time>>(
                    "lastUpdateTime",
                    CertificateSigningRequestCondition::get_lastUpdateTime_for_reflect,
                    CertificateSigningRequestCondition::mut_lastUpdateTime_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CertificateSigningRequestCondition>(
                    "CertificateSigningRequestCondition",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CertificateSigningRequestCondition {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_reason();
        self.clear_message();
        self.clear_lastUpdateTime();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CertificateSigningRequestCondition {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CertificateSigningRequestCondition {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CertificateSigningRequestList {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ListMeta>,
    items: ::protobuf::RepeatedField<CertificateSigningRequest>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CertificateSigningRequestList {}

impl CertificateSigningRequestList {
    pub fn new() -> CertificateSigningRequestList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CertificateSigningRequestList {
        static mut instance: ::protobuf::lazy::Lazy<CertificateSigningRequestList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CertificateSigningRequestList,
        };
        unsafe {
            instance.get(CertificateSigningRequestList::new)
        }
    }

    // optional .k8s.io.apimachinery.pkg.apis.meta.v1.ListMeta metadata = 1;

    pub fn clear_metadata(&mut self) {
        self.metadata.clear();
    }

    pub fn has_metadata(&self) -> bool {
        self.metadata.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metadata(&mut self, v: super::generated::ListMeta) {
        self.metadata = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metadata(&mut self) -> &mut super::generated::ListMeta {
        if self.metadata.is_none() {
            self.metadata.set_default();
        }
        self.metadata.as_mut().unwrap()
    }

    // Take field
    pub fn take_metadata(&mut self) -> super::generated::ListMeta {
        self.metadata.take().unwrap_or_else(|| super::generated::ListMeta::new())
    }

    pub fn get_metadata(&self) -> &super::generated::ListMeta {
        self.metadata.as_ref().unwrap_or_else(|| super::generated::ListMeta::default_instance())
    }

    fn get_metadata_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::ListMeta> {
        &self.metadata
    }

    fn mut_metadata_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::ListMeta> {
        &mut self.metadata
    }

    // repeated .k8s.io.api.certificates.v1beta1.CertificateSigningRequest items = 2;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<CertificateSigningRequest>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<CertificateSigningRequest> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<CertificateSigningRequest> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[CertificateSigningRequest] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<CertificateSigningRequest> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CertificateSigningRequest> {
        &mut self.items
    }
}

impl ::protobuf::Message for CertificateSigningRequestList {
    fn is_initialized(&self) -> bool {
        for v in &self.metadata {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.items {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.items)?;
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
        for value in &self.items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
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
        for v in &self.items {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for CertificateSigningRequestList {
    fn new() -> CertificateSigningRequestList {
        CertificateSigningRequestList::new()
    }

    fn descriptor_static(_: ::std::option::Option<CertificateSigningRequestList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ListMeta>>(
                    "metadata",
                    CertificateSigningRequestList::get_metadata_for_reflect,
                    CertificateSigningRequestList::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CertificateSigningRequest>>(
                    "items",
                    CertificateSigningRequestList::get_items_for_reflect,
                    CertificateSigningRequestList::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CertificateSigningRequestList>(
                    "CertificateSigningRequestList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CertificateSigningRequestList {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CertificateSigningRequestList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CertificateSigningRequestList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CertificateSigningRequestSpec {
    // message fields
    request: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    keyUsage: ::protobuf::RepeatedField<::std::string::String>,
    username: ::protobuf::SingularField<::std::string::String>,
    uid: ::protobuf::SingularField<::std::string::String>,
    groups: ::protobuf::RepeatedField<::std::string::String>,
    pub extra: ::std::collections::HashMap<::std::string::String, ExtraValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CertificateSigningRequestSpec {}

impl CertificateSigningRequestSpec {
    pub fn new() -> CertificateSigningRequestSpec {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CertificateSigningRequestSpec {
        static mut instance: ::protobuf::lazy::Lazy<CertificateSigningRequestSpec> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CertificateSigningRequestSpec,
        };
        unsafe {
            instance.get(CertificateSigningRequestSpec::new)
        }
    }

    // optional bytes request = 1;

    pub fn clear_request(&mut self) {
        self.request.clear();
    }

    pub fn has_request(&self) -> bool {
        self.request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_request(&mut self, v: ::std::vec::Vec<u8>) {
        self.request = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_request(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.request.is_none() {
            self.request.set_default();
        }
        self.request.as_mut().unwrap()
    }

    // Take field
    pub fn take_request(&mut self) -> ::std::vec::Vec<u8> {
        self.request.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_request(&self) -> &[u8] {
        match self.request.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_request_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.request
    }

    fn mut_request_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.request
    }

    // repeated string keyUsage = 5;

    pub fn clear_keyUsage(&mut self) {
        self.keyUsage.clear();
    }

    // Param is passed by value, moved
    pub fn set_keyUsage(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.keyUsage = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keyUsage(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.keyUsage
    }

    // Take field
    pub fn take_keyUsage(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.keyUsage, ::protobuf::RepeatedField::new())
    }

    pub fn get_keyUsage(&self) -> &[::std::string::String] {
        &self.keyUsage
    }

    fn get_keyUsage_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.keyUsage
    }

    fn mut_keyUsage_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.keyUsage
    }

    // optional string username = 2;

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

    // optional string uid = 3;

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

    // repeated string groups = 4;

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

    // repeated .k8s.io.api.certificates.v1beta1.CertificateSigningRequestSpec.ExtraEntry extra = 6;

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

impl ::protobuf::Message for CertificateSigningRequestSpec {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.request)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.keyUsage)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.username)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.uid)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.groups)?;
                },
                6 => {
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
        if let Some(ref v) = self.request.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        for value in &self.keyUsage {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        if let Some(ref v) = self.username.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.uid.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        for value in &self.groups {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<ExtraValue>>(6, &self.extra);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.request.as_ref() {
            os.write_bytes(1, &v)?;
        }
        for v in &self.keyUsage {
            os.write_string(5, &v)?;
        };
        if let Some(ref v) = self.username.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.uid.as_ref() {
            os.write_string(3, &v)?;
        }
        for v in &self.groups {
            os.write_string(4, &v)?;
        };
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<ExtraValue>>(6, &self.extra, os)?;
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

impl ::protobuf::MessageStatic for CertificateSigningRequestSpec {
    fn new() -> CertificateSigningRequestSpec {
        CertificateSigningRequestSpec::new()
    }

    fn descriptor_static(_: ::std::option::Option<CertificateSigningRequestSpec>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "request",
                    CertificateSigningRequestSpec::get_request_for_reflect,
                    CertificateSigningRequestSpec::mut_request_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "keyUsage",
                    CertificateSigningRequestSpec::get_keyUsage_for_reflect,
                    CertificateSigningRequestSpec::mut_keyUsage_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "username",
                    CertificateSigningRequestSpec::get_username_for_reflect,
                    CertificateSigningRequestSpec::mut_username_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "uid",
                    CertificateSigningRequestSpec::get_uid_for_reflect,
                    CertificateSigningRequestSpec::mut_uid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "groups",
                    CertificateSigningRequestSpec::get_groups_for_reflect,
                    CertificateSigningRequestSpec::mut_groups_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<ExtraValue>>(
                    "extra",
                    CertificateSigningRequestSpec::get_extra_for_reflect,
                    CertificateSigningRequestSpec::mut_extra_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CertificateSigningRequestSpec>(
                    "CertificateSigningRequestSpec",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CertificateSigningRequestSpec {
    fn clear(&mut self) {
        self.clear_request();
        self.clear_keyUsage();
        self.clear_username();
        self.clear_uid();
        self.clear_groups();
        self.clear_extra();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CertificateSigningRequestSpec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CertificateSigningRequestSpec {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CertificateSigningRequestStatus {
    // message fields
    conditions: ::protobuf::RepeatedField<CertificateSigningRequestCondition>,
    certificate: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CertificateSigningRequestStatus {}

impl CertificateSigningRequestStatus {
    pub fn new() -> CertificateSigningRequestStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CertificateSigningRequestStatus {
        static mut instance: ::protobuf::lazy::Lazy<CertificateSigningRequestStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CertificateSigningRequestStatus,
        };
        unsafe {
            instance.get(CertificateSigningRequestStatus::new)
        }
    }

    // repeated .k8s.io.api.certificates.v1beta1.CertificateSigningRequestCondition conditions = 1;

    pub fn clear_conditions(&mut self) {
        self.conditions.clear();
    }

    // Param is passed by value, moved
    pub fn set_conditions(&mut self, v: ::protobuf::RepeatedField<CertificateSigningRequestCondition>) {
        self.conditions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_conditions(&mut self) -> &mut ::protobuf::RepeatedField<CertificateSigningRequestCondition> {
        &mut self.conditions
    }

    // Take field
    pub fn take_conditions(&mut self) -> ::protobuf::RepeatedField<CertificateSigningRequestCondition> {
        ::std::mem::replace(&mut self.conditions, ::protobuf::RepeatedField::new())
    }

    pub fn get_conditions(&self) -> &[CertificateSigningRequestCondition] {
        &self.conditions
    }

    fn get_conditions_for_reflect(&self) -> &::protobuf::RepeatedField<CertificateSigningRequestCondition> {
        &self.conditions
    }

    fn mut_conditions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CertificateSigningRequestCondition> {
        &mut self.conditions
    }

    // optional bytes certificate = 2;

    pub fn clear_certificate(&mut self) {
        self.certificate.clear();
    }

    pub fn has_certificate(&self) -> bool {
        self.certificate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_certificate(&mut self, v: ::std::vec::Vec<u8>) {
        self.certificate = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_certificate(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.certificate.is_none() {
            self.certificate.set_default();
        }
        self.certificate.as_mut().unwrap()
    }

    // Take field
    pub fn take_certificate(&mut self) -> ::std::vec::Vec<u8> {
        self.certificate.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_certificate(&self) -> &[u8] {
        match self.certificate.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_certificate_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.certificate
    }

    fn mut_certificate_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.certificate
    }
}

impl ::protobuf::Message for CertificateSigningRequestStatus {
    fn is_initialized(&self) -> bool {
        for v in &self.conditions {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.conditions)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.certificate)?;
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
        for value in &self.conditions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.certificate.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.conditions {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.certificate.as_ref() {
            os.write_bytes(2, &v)?;
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

impl ::protobuf::MessageStatic for CertificateSigningRequestStatus {
    fn new() -> CertificateSigningRequestStatus {
        CertificateSigningRequestStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<CertificateSigningRequestStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CertificateSigningRequestCondition>>(
                    "conditions",
                    CertificateSigningRequestStatus::get_conditions_for_reflect,
                    CertificateSigningRequestStatus::mut_conditions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "certificate",
                    CertificateSigningRequestStatus::get_certificate_for_reflect,
                    CertificateSigningRequestStatus::mut_certificate_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CertificateSigningRequestStatus>(
                    "CertificateSigningRequestStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CertificateSigningRequestStatus {
    fn clear(&mut self) {
        self.clear_conditions();
        self.clear_certificate();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CertificateSigningRequestStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CertificateSigningRequestStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

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

static file_descriptor_proto_data: &'static [u8] = b"\
    \n/k8s.io/api/certificates/v1beta1/generated.proto\x12\x1fk8s.io.api.cer\
    tificates.v1beta1\x1a4k8s.io/apimachinery/pkg/apis/meta/v1/generated.pro\
    to\x1a/k8s.io/apimachinery/pkg/runtime/generated.proto\x1a6k8s.io/apimac\
    hinery/pkg/runtime/schema/generated.proto\x1a3k8s.io/apimachinery/pkg/ut\
    il/intstr/generated.proto\"\x97\x02\n\x19CertificateSigningRequest\x12L\
    \n\x08metadata\x18\x01\x20\x01(\x0b20.k8s.io.apimachinery.pkg.apis.meta.\
    v1.ObjectMetaR\x08metadata\x12R\n\x04spec\x18\x02\x20\x01(\x0b2>.k8s.io.\
    api.certificates.v1beta1.CertificateSigningRequestSpecR\x04spec\x12X\n\
    \x06status\x18\x03\x20\x01(\x0b2@.k8s.io.api.certificates.v1beta1.Certif\
    icateSigningRequestStatusR\x06status\"\xbe\x01\n\"CertificateSigningRequ\
    estCondition\x12\x12\n\x04type\x18\x01\x20\x01(\tR\x04type\x12\x16\n\x06\
    reason\x18\x02\x20\x01(\tR\x06reason\x12\x18\n\x07message\x18\x03\x20\
    \x01(\tR\x07message\x12R\n\x0elastUpdateTime\x18\x04\x20\x01(\x0b2*.k8s.\
    io.apimachinery.pkg.apis.meta.v1.TimeR\x0elastUpdateTime\"\xbd\x01\n\x1d\
    CertificateSigningRequestList\x12J\n\x08metadata\x18\x01\x20\x01(\x0b2..\
    k8s.io.apimachinery.pkg.apis.meta.v1.ListMetaR\x08metadata\x12P\n\x05ite\
    ms\x18\x02\x20\x03(\x0b2:.k8s.io.api.certificates.v1beta1.CertificateSig\
    ningRequestR\x05items\"\xe3\x02\n\x1dCertificateSigningRequestSpec\x12\
    \x18\n\x07request\x18\x01\x20\x01(\x0cR\x07request\x12\x1a\n\x08keyUsage\
    \x18\x05\x20\x03(\tR\x08keyUsage\x12\x1a\n\x08username\x18\x02\x20\x01(\
    \tR\x08username\x12\x10\n\x03uid\x18\x03\x20\x01(\tR\x03uid\x12\x16\n\
    \x06groups\x18\x04\x20\x03(\tR\x06groups\x12_\n\x05extra\x18\x06\x20\x03\
    (\x0b2I.k8s.io.api.certificates.v1beta1.CertificateSigningRequestSpec.Ex\
    traEntryR\x05extra\x1ae\n\nExtraEntry\x12\x10\n\x03key\x18\x01\x20\x01(\
    \tR\x03key\x12A\n\x05value\x18\x02\x20\x01(\x0b2+.k8s.io.api.certificate\
    s.v1beta1.ExtraValueR\x05value:\x028\x01\"\xa8\x01\n\x1fCertificateSigni\
    ngRequestStatus\x12c\n\nconditions\x18\x01\x20\x03(\x0b2C.k8s.io.api.cer\
    tificates.v1beta1.CertificateSigningRequestConditionR\nconditions\x12\
    \x20\n\x0bcertificate\x18\x02\x20\x01(\x0cR\x0bcertificate\"\"\n\nExtraV\
    alue\x12\x14\n\x05items\x18\x01\x20\x03(\tR\x05itemsB\tZ\x07v1beta1\
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
