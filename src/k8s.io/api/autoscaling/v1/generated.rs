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
pub struct CrossVersionObjectReference {
    // message fields
    kind: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    apiVersion: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CrossVersionObjectReference {}

impl CrossVersionObjectReference {
    pub fn new() -> CrossVersionObjectReference {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CrossVersionObjectReference {
        static mut instance: ::protobuf::lazy::Lazy<CrossVersionObjectReference> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CrossVersionObjectReference,
        };
        unsafe {
            instance.get(CrossVersionObjectReference::new)
        }
    }

    // optional string kind = 1;

    pub fn clear_kind(&mut self) {
        self.kind.clear();
    }

    pub fn has_kind(&self) -> bool {
        self.kind.is_some()
    }

    // Param is passed by value, moved
    pub fn set_kind(&mut self, v: ::std::string::String) {
        self.kind = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_kind(&mut self) -> &mut ::std::string::String {
        if self.kind.is_none() {
            self.kind.set_default();
        }
        self.kind.as_mut().unwrap()
    }

    // Take field
    pub fn take_kind(&mut self) -> ::std::string::String {
        self.kind.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_kind(&self) -> &str {
        match self.kind.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_kind_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.kind
    }

    fn mut_kind_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.kind
    }

    // optional string name = 2;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional string apiVersion = 3;

    pub fn clear_apiVersion(&mut self) {
        self.apiVersion.clear();
    }

    pub fn has_apiVersion(&self) -> bool {
        self.apiVersion.is_some()
    }

    // Param is passed by value, moved
    pub fn set_apiVersion(&mut self, v: ::std::string::String) {
        self.apiVersion = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_apiVersion(&mut self) -> &mut ::std::string::String {
        if self.apiVersion.is_none() {
            self.apiVersion.set_default();
        }
        self.apiVersion.as_mut().unwrap()
    }

    // Take field
    pub fn take_apiVersion(&mut self) -> ::std::string::String {
        self.apiVersion.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_apiVersion(&self) -> &str {
        match self.apiVersion.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_apiVersion_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.apiVersion
    }

    fn mut_apiVersion_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.apiVersion
    }
}

impl ::protobuf::Message for CrossVersionObjectReference {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.kind)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.apiVersion)?;
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
        if let Some(ref v) = self.kind.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.apiVersion.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.kind.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.apiVersion.as_ref() {
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

impl ::protobuf::MessageStatic for CrossVersionObjectReference {
    fn new() -> CrossVersionObjectReference {
        CrossVersionObjectReference::new()
    }

    fn descriptor_static(_: ::std::option::Option<CrossVersionObjectReference>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "kind",
                    CrossVersionObjectReference::get_kind_for_reflect,
                    CrossVersionObjectReference::mut_kind_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CrossVersionObjectReference::get_name_for_reflect,
                    CrossVersionObjectReference::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "apiVersion",
                    CrossVersionObjectReference::get_apiVersion_for_reflect,
                    CrossVersionObjectReference::mut_apiVersion_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CrossVersionObjectReference>(
                    "CrossVersionObjectReference",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CrossVersionObjectReference {
    fn clear(&mut self) {
        self.clear_kind();
        self.clear_name();
        self.clear_apiVersion();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CrossVersionObjectReference {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CrossVersionObjectReference {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HorizontalPodAutoscaler {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    spec: ::protobuf::SingularPtrField<HorizontalPodAutoscalerSpec>,
    status: ::protobuf::SingularPtrField<HorizontalPodAutoscalerStatus>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HorizontalPodAutoscaler {}

impl HorizontalPodAutoscaler {
    pub fn new() -> HorizontalPodAutoscaler {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HorizontalPodAutoscaler {
        static mut instance: ::protobuf::lazy::Lazy<HorizontalPodAutoscaler> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HorizontalPodAutoscaler,
        };
        unsafe {
            instance.get(HorizontalPodAutoscaler::new)
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

    // optional .k8s.io.api.autoscaling.v1.HorizontalPodAutoscalerSpec spec = 2;

    pub fn clear_spec(&mut self) {
        self.spec.clear();
    }

    pub fn has_spec(&self) -> bool {
        self.spec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spec(&mut self, v: HorizontalPodAutoscalerSpec) {
        self.spec = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_spec(&mut self) -> &mut HorizontalPodAutoscalerSpec {
        if self.spec.is_none() {
            self.spec.set_default();
        }
        self.spec.as_mut().unwrap()
    }

    // Take field
    pub fn take_spec(&mut self) -> HorizontalPodAutoscalerSpec {
        self.spec.take().unwrap_or_else(|| HorizontalPodAutoscalerSpec::new())
    }

    pub fn get_spec(&self) -> &HorizontalPodAutoscalerSpec {
        self.spec.as_ref().unwrap_or_else(|| HorizontalPodAutoscalerSpec::default_instance())
    }

    fn get_spec_for_reflect(&self) -> &::protobuf::SingularPtrField<HorizontalPodAutoscalerSpec> {
        &self.spec
    }

    fn mut_spec_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<HorizontalPodAutoscalerSpec> {
        &mut self.spec
    }

    // optional .k8s.io.api.autoscaling.v1.HorizontalPodAutoscalerStatus status = 3;

    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: HorizontalPodAutoscalerStatus) {
        self.status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut HorizontalPodAutoscalerStatus {
        if self.status.is_none() {
            self.status.set_default();
        }
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> HorizontalPodAutoscalerStatus {
        self.status.take().unwrap_or_else(|| HorizontalPodAutoscalerStatus::new())
    }

    pub fn get_status(&self) -> &HorizontalPodAutoscalerStatus {
        self.status.as_ref().unwrap_or_else(|| HorizontalPodAutoscalerStatus::default_instance())
    }

    fn get_status_for_reflect(&self) -> &::protobuf::SingularPtrField<HorizontalPodAutoscalerStatus> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<HorizontalPodAutoscalerStatus> {
        &mut self.status
    }
}

impl ::protobuf::Message for HorizontalPodAutoscaler {
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

impl ::protobuf::MessageStatic for HorizontalPodAutoscaler {
    fn new() -> HorizontalPodAutoscaler {
        HorizontalPodAutoscaler::new()
    }

    fn descriptor_static(_: ::std::option::Option<HorizontalPodAutoscaler>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    HorizontalPodAutoscaler::get_metadata_for_reflect,
                    HorizontalPodAutoscaler::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<HorizontalPodAutoscalerSpec>>(
                    "spec",
                    HorizontalPodAutoscaler::get_spec_for_reflect,
                    HorizontalPodAutoscaler::mut_spec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<HorizontalPodAutoscalerStatus>>(
                    "status",
                    HorizontalPodAutoscaler::get_status_for_reflect,
                    HorizontalPodAutoscaler::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HorizontalPodAutoscaler>(
                    "HorizontalPodAutoscaler",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HorizontalPodAutoscaler {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_spec();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HorizontalPodAutoscaler {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HorizontalPodAutoscaler {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HorizontalPodAutoscalerCondition {
    // message fields
    field_type: ::protobuf::SingularField<::std::string::String>,
    status: ::protobuf::SingularField<::std::string::String>,
    lastTransitionTime: ::protobuf::SingularPtrField<super::generated::Time>,
    reason: ::protobuf::SingularField<::std::string::String>,
    message: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HorizontalPodAutoscalerCondition {}

impl HorizontalPodAutoscalerCondition {
    pub fn new() -> HorizontalPodAutoscalerCondition {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HorizontalPodAutoscalerCondition {
        static mut instance: ::protobuf::lazy::Lazy<HorizontalPodAutoscalerCondition> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HorizontalPodAutoscalerCondition,
        };
        unsafe {
            instance.get(HorizontalPodAutoscalerCondition::new)
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

    // optional string status = 2;

    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: ::std::string::String) {
        self.status = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut ::std::string::String {
        if self.status.is_none() {
            self.status.set_default();
        }
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> ::std::string::String {
        self.status.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_status(&self) -> &str {
        match self.status.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_status_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.status
    }

    // optional .k8s.io.apimachinery.pkg.apis.meta.v1.Time lastTransitionTime = 3;

    pub fn clear_lastTransitionTime(&mut self) {
        self.lastTransitionTime.clear();
    }

    pub fn has_lastTransitionTime(&self) -> bool {
        self.lastTransitionTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastTransitionTime(&mut self, v: super::generated::Time) {
        self.lastTransitionTime = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lastTransitionTime(&mut self) -> &mut super::generated::Time {
        if self.lastTransitionTime.is_none() {
            self.lastTransitionTime.set_default();
        }
        self.lastTransitionTime.as_mut().unwrap()
    }

    // Take field
    pub fn take_lastTransitionTime(&mut self) -> super::generated::Time {
        self.lastTransitionTime.take().unwrap_or_else(|| super::generated::Time::new())
    }

    pub fn get_lastTransitionTime(&self) -> &super::generated::Time {
        self.lastTransitionTime.as_ref().unwrap_or_else(|| super::generated::Time::default_instance())
    }

    fn get_lastTransitionTime_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::Time> {
        &self.lastTransitionTime
    }

    fn mut_lastTransitionTime_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::Time> {
        &mut self.lastTransitionTime
    }

    // optional string reason = 4;

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

    // optional string message = 5;

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
}

impl ::protobuf::Message for HorizontalPodAutoscalerCondition {
    fn is_initialized(&self) -> bool {
        for v in &self.lastTransitionTime {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.status)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.lastTransitionTime)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.reason)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.message)?;
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
        if let Some(ref v) = self.status.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.lastTransitionTime.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.reason.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(ref v) = self.message.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.field_type.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.status.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.lastTransitionTime.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.reason.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(ref v) = self.message.as_ref() {
            os.write_string(5, &v)?;
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

impl ::protobuf::MessageStatic for HorizontalPodAutoscalerCondition {
    fn new() -> HorizontalPodAutoscalerCondition {
        HorizontalPodAutoscalerCondition::new()
    }

    fn descriptor_static(_: ::std::option::Option<HorizontalPodAutoscalerCondition>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    HorizontalPodAutoscalerCondition::get_field_type_for_reflect,
                    HorizontalPodAutoscalerCondition::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "status",
                    HorizontalPodAutoscalerCondition::get_status_for_reflect,
                    HorizontalPodAutoscalerCondition::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::Time>>(
                    "lastTransitionTime",
                    HorizontalPodAutoscalerCondition::get_lastTransitionTime_for_reflect,
                    HorizontalPodAutoscalerCondition::mut_lastTransitionTime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "reason",
                    HorizontalPodAutoscalerCondition::get_reason_for_reflect,
                    HorizontalPodAutoscalerCondition::mut_reason_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    HorizontalPodAutoscalerCondition::get_message_for_reflect,
                    HorizontalPodAutoscalerCondition::mut_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HorizontalPodAutoscalerCondition>(
                    "HorizontalPodAutoscalerCondition",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HorizontalPodAutoscalerCondition {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_status();
        self.clear_lastTransitionTime();
        self.clear_reason();
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HorizontalPodAutoscalerCondition {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HorizontalPodAutoscalerCondition {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HorizontalPodAutoscalerList {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ListMeta>,
    items: ::protobuf::RepeatedField<HorizontalPodAutoscaler>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HorizontalPodAutoscalerList {}

impl HorizontalPodAutoscalerList {
    pub fn new() -> HorizontalPodAutoscalerList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HorizontalPodAutoscalerList {
        static mut instance: ::protobuf::lazy::Lazy<HorizontalPodAutoscalerList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HorizontalPodAutoscalerList,
        };
        unsafe {
            instance.get(HorizontalPodAutoscalerList::new)
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

    // repeated .k8s.io.api.autoscaling.v1.HorizontalPodAutoscaler items = 2;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<HorizontalPodAutoscaler>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<HorizontalPodAutoscaler> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<HorizontalPodAutoscaler> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[HorizontalPodAutoscaler] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<HorizontalPodAutoscaler> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<HorizontalPodAutoscaler> {
        &mut self.items
    }
}

impl ::protobuf::Message for HorizontalPodAutoscalerList {
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

impl ::protobuf::MessageStatic for HorizontalPodAutoscalerList {
    fn new() -> HorizontalPodAutoscalerList {
        HorizontalPodAutoscalerList::new()
    }

    fn descriptor_static(_: ::std::option::Option<HorizontalPodAutoscalerList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ListMeta>>(
                    "metadata",
                    HorizontalPodAutoscalerList::get_metadata_for_reflect,
                    HorizontalPodAutoscalerList::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<HorizontalPodAutoscaler>>(
                    "items",
                    HorizontalPodAutoscalerList::get_items_for_reflect,
                    HorizontalPodAutoscalerList::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HorizontalPodAutoscalerList>(
                    "HorizontalPodAutoscalerList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HorizontalPodAutoscalerList {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HorizontalPodAutoscalerList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HorizontalPodAutoscalerList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HorizontalPodAutoscalerSpec {
    // message fields
    scaleTargetRef: ::protobuf::SingularPtrField<CrossVersionObjectReference>,
    minReplicas: ::std::option::Option<i32>,
    maxReplicas: ::std::option::Option<i32>,
    targetCPUUtilizationPercentage: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HorizontalPodAutoscalerSpec {}

impl HorizontalPodAutoscalerSpec {
    pub fn new() -> HorizontalPodAutoscalerSpec {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HorizontalPodAutoscalerSpec {
        static mut instance: ::protobuf::lazy::Lazy<HorizontalPodAutoscalerSpec> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HorizontalPodAutoscalerSpec,
        };
        unsafe {
            instance.get(HorizontalPodAutoscalerSpec::new)
        }
    }

    // optional .k8s.io.api.autoscaling.v1.CrossVersionObjectReference scaleTargetRef = 1;

    pub fn clear_scaleTargetRef(&mut self) {
        self.scaleTargetRef.clear();
    }

    pub fn has_scaleTargetRef(&self) -> bool {
        self.scaleTargetRef.is_some()
    }

    // Param is passed by value, moved
    pub fn set_scaleTargetRef(&mut self, v: CrossVersionObjectReference) {
        self.scaleTargetRef = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_scaleTargetRef(&mut self) -> &mut CrossVersionObjectReference {
        if self.scaleTargetRef.is_none() {
            self.scaleTargetRef.set_default();
        }
        self.scaleTargetRef.as_mut().unwrap()
    }

    // Take field
    pub fn take_scaleTargetRef(&mut self) -> CrossVersionObjectReference {
        self.scaleTargetRef.take().unwrap_or_else(|| CrossVersionObjectReference::new())
    }

    pub fn get_scaleTargetRef(&self) -> &CrossVersionObjectReference {
        self.scaleTargetRef.as_ref().unwrap_or_else(|| CrossVersionObjectReference::default_instance())
    }

    fn get_scaleTargetRef_for_reflect(&self) -> &::protobuf::SingularPtrField<CrossVersionObjectReference> {
        &self.scaleTargetRef
    }

    fn mut_scaleTargetRef_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CrossVersionObjectReference> {
        &mut self.scaleTargetRef
    }

    // optional int32 minReplicas = 2;

    pub fn clear_minReplicas(&mut self) {
        self.minReplicas = ::std::option::Option::None;
    }

    pub fn has_minReplicas(&self) -> bool {
        self.minReplicas.is_some()
    }

    // Param is passed by value, moved
    pub fn set_minReplicas(&mut self, v: i32) {
        self.minReplicas = ::std::option::Option::Some(v);
    }

    pub fn get_minReplicas(&self) -> i32 {
        self.minReplicas.unwrap_or(0)
    }

    fn get_minReplicas_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.minReplicas
    }

    fn mut_minReplicas_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.minReplicas
    }

    // optional int32 maxReplicas = 3;

    pub fn clear_maxReplicas(&mut self) {
        self.maxReplicas = ::std::option::Option::None;
    }

    pub fn has_maxReplicas(&self) -> bool {
        self.maxReplicas.is_some()
    }

    // Param is passed by value, moved
    pub fn set_maxReplicas(&mut self, v: i32) {
        self.maxReplicas = ::std::option::Option::Some(v);
    }

    pub fn get_maxReplicas(&self) -> i32 {
        self.maxReplicas.unwrap_or(0)
    }

    fn get_maxReplicas_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.maxReplicas
    }

    fn mut_maxReplicas_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.maxReplicas
    }

    // optional int32 targetCPUUtilizationPercentage = 4;

    pub fn clear_targetCPUUtilizationPercentage(&mut self) {
        self.targetCPUUtilizationPercentage = ::std::option::Option::None;
    }

    pub fn has_targetCPUUtilizationPercentage(&self) -> bool {
        self.targetCPUUtilizationPercentage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_targetCPUUtilizationPercentage(&mut self, v: i32) {
        self.targetCPUUtilizationPercentage = ::std::option::Option::Some(v);
    }

    pub fn get_targetCPUUtilizationPercentage(&self) -> i32 {
        self.targetCPUUtilizationPercentage.unwrap_or(0)
    }

    fn get_targetCPUUtilizationPercentage_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.targetCPUUtilizationPercentage
    }

    fn mut_targetCPUUtilizationPercentage_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.targetCPUUtilizationPercentage
    }
}

impl ::protobuf::Message for HorizontalPodAutoscalerSpec {
    fn is_initialized(&self) -> bool {
        for v in &self.scaleTargetRef {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.scaleTargetRef)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.minReplicas = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.maxReplicas = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.targetCPUUtilizationPercentage = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.scaleTargetRef.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.minReplicas {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.maxReplicas {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.targetCPUUtilizationPercentage {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.scaleTargetRef.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.minReplicas {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.maxReplicas {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.targetCPUUtilizationPercentage {
            os.write_int32(4, v)?;
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

impl ::protobuf::MessageStatic for HorizontalPodAutoscalerSpec {
    fn new() -> HorizontalPodAutoscalerSpec {
        HorizontalPodAutoscalerSpec::new()
    }

    fn descriptor_static(_: ::std::option::Option<HorizontalPodAutoscalerSpec>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CrossVersionObjectReference>>(
                    "scaleTargetRef",
                    HorizontalPodAutoscalerSpec::get_scaleTargetRef_for_reflect,
                    HorizontalPodAutoscalerSpec::mut_scaleTargetRef_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "minReplicas",
                    HorizontalPodAutoscalerSpec::get_minReplicas_for_reflect,
                    HorizontalPodAutoscalerSpec::mut_minReplicas_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "maxReplicas",
                    HorizontalPodAutoscalerSpec::get_maxReplicas_for_reflect,
                    HorizontalPodAutoscalerSpec::mut_maxReplicas_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "targetCPUUtilizationPercentage",
                    HorizontalPodAutoscalerSpec::get_targetCPUUtilizationPercentage_for_reflect,
                    HorizontalPodAutoscalerSpec::mut_targetCPUUtilizationPercentage_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HorizontalPodAutoscalerSpec>(
                    "HorizontalPodAutoscalerSpec",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HorizontalPodAutoscalerSpec {
    fn clear(&mut self) {
        self.clear_scaleTargetRef();
        self.clear_minReplicas();
        self.clear_maxReplicas();
        self.clear_targetCPUUtilizationPercentage();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HorizontalPodAutoscalerSpec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HorizontalPodAutoscalerSpec {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HorizontalPodAutoscalerStatus {
    // message fields
    observedGeneration: ::std::option::Option<i64>,
    lastScaleTime: ::protobuf::SingularPtrField<super::generated::Time>,
    currentReplicas: ::std::option::Option<i32>,
    desiredReplicas: ::std::option::Option<i32>,
    currentCPUUtilizationPercentage: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HorizontalPodAutoscalerStatus {}

impl HorizontalPodAutoscalerStatus {
    pub fn new() -> HorizontalPodAutoscalerStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HorizontalPodAutoscalerStatus {
        static mut instance: ::protobuf::lazy::Lazy<HorizontalPodAutoscalerStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HorizontalPodAutoscalerStatus,
        };
        unsafe {
            instance.get(HorizontalPodAutoscalerStatus::new)
        }
    }

    // optional int64 observedGeneration = 1;

    pub fn clear_observedGeneration(&mut self) {
        self.observedGeneration = ::std::option::Option::None;
    }

    pub fn has_observedGeneration(&self) -> bool {
        self.observedGeneration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_observedGeneration(&mut self, v: i64) {
        self.observedGeneration = ::std::option::Option::Some(v);
    }

    pub fn get_observedGeneration(&self) -> i64 {
        self.observedGeneration.unwrap_or(0)
    }

    fn get_observedGeneration_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.observedGeneration
    }

    fn mut_observedGeneration_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.observedGeneration
    }

    // optional .k8s.io.apimachinery.pkg.apis.meta.v1.Time lastScaleTime = 2;

    pub fn clear_lastScaleTime(&mut self) {
        self.lastScaleTime.clear();
    }

    pub fn has_lastScaleTime(&self) -> bool {
        self.lastScaleTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastScaleTime(&mut self, v: super::generated::Time) {
        self.lastScaleTime = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lastScaleTime(&mut self) -> &mut super::generated::Time {
        if self.lastScaleTime.is_none() {
            self.lastScaleTime.set_default();
        }
        self.lastScaleTime.as_mut().unwrap()
    }

    // Take field
    pub fn take_lastScaleTime(&mut self) -> super::generated::Time {
        self.lastScaleTime.take().unwrap_or_else(|| super::generated::Time::new())
    }

    pub fn get_lastScaleTime(&self) -> &super::generated::Time {
        self.lastScaleTime.as_ref().unwrap_or_else(|| super::generated::Time::default_instance())
    }

    fn get_lastScaleTime_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::Time> {
        &self.lastScaleTime
    }

    fn mut_lastScaleTime_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::Time> {
        &mut self.lastScaleTime
    }

    // optional int32 currentReplicas = 3;

    pub fn clear_currentReplicas(&mut self) {
        self.currentReplicas = ::std::option::Option::None;
    }

    pub fn has_currentReplicas(&self) -> bool {
        self.currentReplicas.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currentReplicas(&mut self, v: i32) {
        self.currentReplicas = ::std::option::Option::Some(v);
    }

    pub fn get_currentReplicas(&self) -> i32 {
        self.currentReplicas.unwrap_or(0)
    }

    fn get_currentReplicas_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.currentReplicas
    }

    fn mut_currentReplicas_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.currentReplicas
    }

    // optional int32 desiredReplicas = 4;

    pub fn clear_desiredReplicas(&mut self) {
        self.desiredReplicas = ::std::option::Option::None;
    }

    pub fn has_desiredReplicas(&self) -> bool {
        self.desiredReplicas.is_some()
    }

    // Param is passed by value, moved
    pub fn set_desiredReplicas(&mut self, v: i32) {
        self.desiredReplicas = ::std::option::Option::Some(v);
    }

    pub fn get_desiredReplicas(&self) -> i32 {
        self.desiredReplicas.unwrap_or(0)
    }

    fn get_desiredReplicas_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.desiredReplicas
    }

    fn mut_desiredReplicas_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.desiredReplicas
    }

    // optional int32 currentCPUUtilizationPercentage = 5;

    pub fn clear_currentCPUUtilizationPercentage(&mut self) {
        self.currentCPUUtilizationPercentage = ::std::option::Option::None;
    }

    pub fn has_currentCPUUtilizationPercentage(&self) -> bool {
        self.currentCPUUtilizationPercentage.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currentCPUUtilizationPercentage(&mut self, v: i32) {
        self.currentCPUUtilizationPercentage = ::std::option::Option::Some(v);
    }

    pub fn get_currentCPUUtilizationPercentage(&self) -> i32 {
        self.currentCPUUtilizationPercentage.unwrap_or(0)
    }

    fn get_currentCPUUtilizationPercentage_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.currentCPUUtilizationPercentage
    }

    fn mut_currentCPUUtilizationPercentage_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.currentCPUUtilizationPercentage
    }
}

impl ::protobuf::Message for HorizontalPodAutoscalerStatus {
    fn is_initialized(&self) -> bool {
        for v in &self.lastScaleTime {
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
                    let tmp = is.read_int64()?;
                    self.observedGeneration = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.lastScaleTime)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.currentReplicas = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.desiredReplicas = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.currentCPUUtilizationPercentage = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.observedGeneration {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.lastScaleTime.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.currentReplicas {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.desiredReplicas {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.currentCPUUtilizationPercentage {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.observedGeneration {
            os.write_int64(1, v)?;
        }
        if let Some(ref v) = self.lastScaleTime.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.currentReplicas {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.desiredReplicas {
            os.write_int32(4, v)?;
        }
        if let Some(v) = self.currentCPUUtilizationPercentage {
            os.write_int32(5, v)?;
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

impl ::protobuf::MessageStatic for HorizontalPodAutoscalerStatus {
    fn new() -> HorizontalPodAutoscalerStatus {
        HorizontalPodAutoscalerStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<HorizontalPodAutoscalerStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "observedGeneration",
                    HorizontalPodAutoscalerStatus::get_observedGeneration_for_reflect,
                    HorizontalPodAutoscalerStatus::mut_observedGeneration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::Time>>(
                    "lastScaleTime",
                    HorizontalPodAutoscalerStatus::get_lastScaleTime_for_reflect,
                    HorizontalPodAutoscalerStatus::mut_lastScaleTime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "currentReplicas",
                    HorizontalPodAutoscalerStatus::get_currentReplicas_for_reflect,
                    HorizontalPodAutoscalerStatus::mut_currentReplicas_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "desiredReplicas",
                    HorizontalPodAutoscalerStatus::get_desiredReplicas_for_reflect,
                    HorizontalPodAutoscalerStatus::mut_desiredReplicas_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "currentCPUUtilizationPercentage",
                    HorizontalPodAutoscalerStatus::get_currentCPUUtilizationPercentage_for_reflect,
                    HorizontalPodAutoscalerStatus::mut_currentCPUUtilizationPercentage_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HorizontalPodAutoscalerStatus>(
                    "HorizontalPodAutoscalerStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HorizontalPodAutoscalerStatus {
    fn clear(&mut self) {
        self.clear_observedGeneration();
        self.clear_lastScaleTime();
        self.clear_currentReplicas();
        self.clear_desiredReplicas();
        self.clear_currentCPUUtilizationPercentage();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HorizontalPodAutoscalerStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HorizontalPodAutoscalerStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MetricSpec {
    // message fields
    field_type: ::protobuf::SingularField<::std::string::String>,
    object: ::protobuf::SingularPtrField<ObjectMetricSource>,
    pods: ::protobuf::SingularPtrField<PodsMetricSource>,
    resource: ::protobuf::SingularPtrField<ResourceMetricSource>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MetricSpec {}

impl MetricSpec {
    pub fn new() -> MetricSpec {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MetricSpec {
        static mut instance: ::protobuf::lazy::Lazy<MetricSpec> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MetricSpec,
        };
        unsafe {
            instance.get(MetricSpec::new)
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

    // optional .k8s.io.api.autoscaling.v1.ObjectMetricSource object = 2;

    pub fn clear_object(&mut self) {
        self.object.clear();
    }

    pub fn has_object(&self) -> bool {
        self.object.is_some()
    }

    // Param is passed by value, moved
    pub fn set_object(&mut self, v: ObjectMetricSource) {
        self.object = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_object(&mut self) -> &mut ObjectMetricSource {
        if self.object.is_none() {
            self.object.set_default();
        }
        self.object.as_mut().unwrap()
    }

    // Take field
    pub fn take_object(&mut self) -> ObjectMetricSource {
        self.object.take().unwrap_or_else(|| ObjectMetricSource::new())
    }

    pub fn get_object(&self) -> &ObjectMetricSource {
        self.object.as_ref().unwrap_or_else(|| ObjectMetricSource::default_instance())
    }

    fn get_object_for_reflect(&self) -> &::protobuf::SingularPtrField<ObjectMetricSource> {
        &self.object
    }

    fn mut_object_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ObjectMetricSource> {
        &mut self.object
    }

    // optional .k8s.io.api.autoscaling.v1.PodsMetricSource pods = 3;

    pub fn clear_pods(&mut self) {
        self.pods.clear();
    }

    pub fn has_pods(&self) -> bool {
        self.pods.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pods(&mut self, v: PodsMetricSource) {
        self.pods = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pods(&mut self) -> &mut PodsMetricSource {
        if self.pods.is_none() {
            self.pods.set_default();
        }
        self.pods.as_mut().unwrap()
    }

    // Take field
    pub fn take_pods(&mut self) -> PodsMetricSource {
        self.pods.take().unwrap_or_else(|| PodsMetricSource::new())
    }

    pub fn get_pods(&self) -> &PodsMetricSource {
        self.pods.as_ref().unwrap_or_else(|| PodsMetricSource::default_instance())
    }

    fn get_pods_for_reflect(&self) -> &::protobuf::SingularPtrField<PodsMetricSource> {
        &self.pods
    }

    fn mut_pods_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PodsMetricSource> {
        &mut self.pods
    }

    // optional .k8s.io.api.autoscaling.v1.ResourceMetricSource resource = 4;

    pub fn clear_resource(&mut self) {
        self.resource.clear();
    }

    pub fn has_resource(&self) -> bool {
        self.resource.is_some()
    }

    // Param is passed by value, moved
    pub fn set_resource(&mut self, v: ResourceMetricSource) {
        self.resource = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_resource(&mut self) -> &mut ResourceMetricSource {
        if self.resource.is_none() {
            self.resource.set_default();
        }
        self.resource.as_mut().unwrap()
    }

    // Take field
    pub fn take_resource(&mut self) -> ResourceMetricSource {
        self.resource.take().unwrap_or_else(|| ResourceMetricSource::new())
    }

    pub fn get_resource(&self) -> &ResourceMetricSource {
        self.resource.as_ref().unwrap_or_else(|| ResourceMetricSource::default_instance())
    }

    fn get_resource_for_reflect(&self) -> &::protobuf::SingularPtrField<ResourceMetricSource> {
        &self.resource
    }

    fn mut_resource_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResourceMetricSource> {
        &mut self.resource
    }
}

impl ::protobuf::Message for MetricSpec {
    fn is_initialized(&self) -> bool {
        for v in &self.object {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.pods {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.resource {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.object)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pods)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.resource)?;
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
        if let Some(ref v) = self.object.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.pods.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.resource.as_ref() {
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
        if let Some(ref v) = self.object.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.pods.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.resource.as_ref() {
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

impl ::protobuf::MessageStatic for MetricSpec {
    fn new() -> MetricSpec {
        MetricSpec::new()
    }

    fn descriptor_static(_: ::std::option::Option<MetricSpec>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    MetricSpec::get_field_type_for_reflect,
                    MetricSpec::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ObjectMetricSource>>(
                    "object",
                    MetricSpec::get_object_for_reflect,
                    MetricSpec::mut_object_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PodsMetricSource>>(
                    "pods",
                    MetricSpec::get_pods_for_reflect,
                    MetricSpec::mut_pods_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResourceMetricSource>>(
                    "resource",
                    MetricSpec::get_resource_for_reflect,
                    MetricSpec::mut_resource_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MetricSpec>(
                    "MetricSpec",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MetricSpec {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_object();
        self.clear_pods();
        self.clear_resource();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MetricSpec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MetricSpec {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MetricStatus {
    // message fields
    field_type: ::protobuf::SingularField<::std::string::String>,
    object: ::protobuf::SingularPtrField<ObjectMetricStatus>,
    pods: ::protobuf::SingularPtrField<PodsMetricStatus>,
    resource: ::protobuf::SingularPtrField<ResourceMetricStatus>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MetricStatus {}

impl MetricStatus {
    pub fn new() -> MetricStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MetricStatus {
        static mut instance: ::protobuf::lazy::Lazy<MetricStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MetricStatus,
        };
        unsafe {
            instance.get(MetricStatus::new)
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

    // optional .k8s.io.api.autoscaling.v1.ObjectMetricStatus object = 2;

    pub fn clear_object(&mut self) {
        self.object.clear();
    }

    pub fn has_object(&self) -> bool {
        self.object.is_some()
    }

    // Param is passed by value, moved
    pub fn set_object(&mut self, v: ObjectMetricStatus) {
        self.object = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_object(&mut self) -> &mut ObjectMetricStatus {
        if self.object.is_none() {
            self.object.set_default();
        }
        self.object.as_mut().unwrap()
    }

    // Take field
    pub fn take_object(&mut self) -> ObjectMetricStatus {
        self.object.take().unwrap_or_else(|| ObjectMetricStatus::new())
    }

    pub fn get_object(&self) -> &ObjectMetricStatus {
        self.object.as_ref().unwrap_or_else(|| ObjectMetricStatus::default_instance())
    }

    fn get_object_for_reflect(&self) -> &::protobuf::SingularPtrField<ObjectMetricStatus> {
        &self.object
    }

    fn mut_object_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ObjectMetricStatus> {
        &mut self.object
    }

    // optional .k8s.io.api.autoscaling.v1.PodsMetricStatus pods = 3;

    pub fn clear_pods(&mut self) {
        self.pods.clear();
    }

    pub fn has_pods(&self) -> bool {
        self.pods.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pods(&mut self, v: PodsMetricStatus) {
        self.pods = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_pods(&mut self) -> &mut PodsMetricStatus {
        if self.pods.is_none() {
            self.pods.set_default();
        }
        self.pods.as_mut().unwrap()
    }

    // Take field
    pub fn take_pods(&mut self) -> PodsMetricStatus {
        self.pods.take().unwrap_or_else(|| PodsMetricStatus::new())
    }

    pub fn get_pods(&self) -> &PodsMetricStatus {
        self.pods.as_ref().unwrap_or_else(|| PodsMetricStatus::default_instance())
    }

    fn get_pods_for_reflect(&self) -> &::protobuf::SingularPtrField<PodsMetricStatus> {
        &self.pods
    }

    fn mut_pods_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PodsMetricStatus> {
        &mut self.pods
    }

    // optional .k8s.io.api.autoscaling.v1.ResourceMetricStatus resource = 4;

    pub fn clear_resource(&mut self) {
        self.resource.clear();
    }

    pub fn has_resource(&self) -> bool {
        self.resource.is_some()
    }

    // Param is passed by value, moved
    pub fn set_resource(&mut self, v: ResourceMetricStatus) {
        self.resource = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_resource(&mut self) -> &mut ResourceMetricStatus {
        if self.resource.is_none() {
            self.resource.set_default();
        }
        self.resource.as_mut().unwrap()
    }

    // Take field
    pub fn take_resource(&mut self) -> ResourceMetricStatus {
        self.resource.take().unwrap_or_else(|| ResourceMetricStatus::new())
    }

    pub fn get_resource(&self) -> &ResourceMetricStatus {
        self.resource.as_ref().unwrap_or_else(|| ResourceMetricStatus::default_instance())
    }

    fn get_resource_for_reflect(&self) -> &::protobuf::SingularPtrField<ResourceMetricStatus> {
        &self.resource
    }

    fn mut_resource_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResourceMetricStatus> {
        &mut self.resource
    }
}

impl ::protobuf::Message for MetricStatus {
    fn is_initialized(&self) -> bool {
        for v in &self.object {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.pods {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.resource {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.object)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.pods)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.resource)?;
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
        if let Some(ref v) = self.object.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.pods.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.resource.as_ref() {
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
        if let Some(ref v) = self.object.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.pods.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.resource.as_ref() {
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

impl ::protobuf::MessageStatic for MetricStatus {
    fn new() -> MetricStatus {
        MetricStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<MetricStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    MetricStatus::get_field_type_for_reflect,
                    MetricStatus::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ObjectMetricStatus>>(
                    "object",
                    MetricStatus::get_object_for_reflect,
                    MetricStatus::mut_object_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PodsMetricStatus>>(
                    "pods",
                    MetricStatus::get_pods_for_reflect,
                    MetricStatus::mut_pods_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResourceMetricStatus>>(
                    "resource",
                    MetricStatus::get_resource_for_reflect,
                    MetricStatus::mut_resource_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MetricStatus>(
                    "MetricStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MetricStatus {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_object();
        self.clear_pods();
        self.clear_resource();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MetricStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MetricStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ObjectMetricSource {
    // message fields
    target: ::protobuf::SingularPtrField<CrossVersionObjectReference>,
    metricName: ::protobuf::SingularField<::std::string::String>,
    targetValue: ::protobuf::SingularPtrField<super::generated::Quantity>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ObjectMetricSource {}

impl ObjectMetricSource {
    pub fn new() -> ObjectMetricSource {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ObjectMetricSource {
        static mut instance: ::protobuf::lazy::Lazy<ObjectMetricSource> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ObjectMetricSource,
        };
        unsafe {
            instance.get(ObjectMetricSource::new)
        }
    }

    // optional .k8s.io.api.autoscaling.v1.CrossVersionObjectReference target = 1;

    pub fn clear_target(&mut self) {
        self.target.clear();
    }

    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target(&mut self, v: CrossVersionObjectReference) {
        self.target = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_target(&mut self) -> &mut CrossVersionObjectReference {
        if self.target.is_none() {
            self.target.set_default();
        }
        self.target.as_mut().unwrap()
    }

    // Take field
    pub fn take_target(&mut self) -> CrossVersionObjectReference {
        self.target.take().unwrap_or_else(|| CrossVersionObjectReference::new())
    }

    pub fn get_target(&self) -> &CrossVersionObjectReference {
        self.target.as_ref().unwrap_or_else(|| CrossVersionObjectReference::default_instance())
    }

    fn get_target_for_reflect(&self) -> &::protobuf::SingularPtrField<CrossVersionObjectReference> {
        &self.target
    }

    fn mut_target_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CrossVersionObjectReference> {
        &mut self.target
    }

    // optional string metricName = 2;

    pub fn clear_metricName(&mut self) {
        self.metricName.clear();
    }

    pub fn has_metricName(&self) -> bool {
        self.metricName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metricName(&mut self, v: ::std::string::String) {
        self.metricName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metricName(&mut self) -> &mut ::std::string::String {
        if self.metricName.is_none() {
            self.metricName.set_default();
        }
        self.metricName.as_mut().unwrap()
    }

    // Take field
    pub fn take_metricName(&mut self) -> ::std::string::String {
        self.metricName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_metricName(&self) -> &str {
        match self.metricName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_metricName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.metricName
    }

    fn mut_metricName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.metricName
    }

    // optional .k8s.io.apimachinery.pkg.api.resource.Quantity targetValue = 3;

    pub fn clear_targetValue(&mut self) {
        self.targetValue.clear();
    }

    pub fn has_targetValue(&self) -> bool {
        self.targetValue.is_some()
    }

    // Param is passed by value, moved
    pub fn set_targetValue(&mut self, v: super::generated::Quantity) {
        self.targetValue = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_targetValue(&mut self) -> &mut super::generated::Quantity {
        if self.targetValue.is_none() {
            self.targetValue.set_default();
        }
        self.targetValue.as_mut().unwrap()
    }

    // Take field
    pub fn take_targetValue(&mut self) -> super::generated::Quantity {
        self.targetValue.take().unwrap_or_else(|| super::generated::Quantity::new())
    }

    pub fn get_targetValue(&self) -> &super::generated::Quantity {
        self.targetValue.as_ref().unwrap_or_else(|| super::generated::Quantity::default_instance())
    }

    fn get_targetValue_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::Quantity> {
        &self.targetValue
    }

    fn mut_targetValue_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::Quantity> {
        &mut self.targetValue
    }
}

impl ::protobuf::Message for ObjectMetricSource {
    fn is_initialized(&self) -> bool {
        for v in &self.target {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.targetValue {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.target)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.metricName)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.targetValue)?;
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
        if let Some(ref v) = self.target.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.metricName.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.targetValue.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.target.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.metricName.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.targetValue.as_ref() {
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

impl ::protobuf::MessageStatic for ObjectMetricSource {
    fn new() -> ObjectMetricSource {
        ObjectMetricSource::new()
    }

    fn descriptor_static(_: ::std::option::Option<ObjectMetricSource>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CrossVersionObjectReference>>(
                    "target",
                    ObjectMetricSource::get_target_for_reflect,
                    ObjectMetricSource::mut_target_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "metricName",
                    ObjectMetricSource::get_metricName_for_reflect,
                    ObjectMetricSource::mut_metricName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::Quantity>>(
                    "targetValue",
                    ObjectMetricSource::get_targetValue_for_reflect,
                    ObjectMetricSource::mut_targetValue_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ObjectMetricSource>(
                    "ObjectMetricSource",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ObjectMetricSource {
    fn clear(&mut self) {
        self.clear_target();
        self.clear_metricName();
        self.clear_targetValue();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ObjectMetricSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ObjectMetricSource {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ObjectMetricStatus {
    // message fields
    target: ::protobuf::SingularPtrField<CrossVersionObjectReference>,
    metricName: ::protobuf::SingularField<::std::string::String>,
    currentValue: ::protobuf::SingularPtrField<super::generated::Quantity>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ObjectMetricStatus {}

impl ObjectMetricStatus {
    pub fn new() -> ObjectMetricStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ObjectMetricStatus {
        static mut instance: ::protobuf::lazy::Lazy<ObjectMetricStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ObjectMetricStatus,
        };
        unsafe {
            instance.get(ObjectMetricStatus::new)
        }
    }

    // optional .k8s.io.api.autoscaling.v1.CrossVersionObjectReference target = 1;

    pub fn clear_target(&mut self) {
        self.target.clear();
    }

    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }

    // Param is passed by value, moved
    pub fn set_target(&mut self, v: CrossVersionObjectReference) {
        self.target = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_target(&mut self) -> &mut CrossVersionObjectReference {
        if self.target.is_none() {
            self.target.set_default();
        }
        self.target.as_mut().unwrap()
    }

    // Take field
    pub fn take_target(&mut self) -> CrossVersionObjectReference {
        self.target.take().unwrap_or_else(|| CrossVersionObjectReference::new())
    }

    pub fn get_target(&self) -> &CrossVersionObjectReference {
        self.target.as_ref().unwrap_or_else(|| CrossVersionObjectReference::default_instance())
    }

    fn get_target_for_reflect(&self) -> &::protobuf::SingularPtrField<CrossVersionObjectReference> {
        &self.target
    }

    fn mut_target_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CrossVersionObjectReference> {
        &mut self.target
    }

    // optional string metricName = 2;

    pub fn clear_metricName(&mut self) {
        self.metricName.clear();
    }

    pub fn has_metricName(&self) -> bool {
        self.metricName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metricName(&mut self, v: ::std::string::String) {
        self.metricName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metricName(&mut self) -> &mut ::std::string::String {
        if self.metricName.is_none() {
            self.metricName.set_default();
        }
        self.metricName.as_mut().unwrap()
    }

    // Take field
    pub fn take_metricName(&mut self) -> ::std::string::String {
        self.metricName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_metricName(&self) -> &str {
        match self.metricName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_metricName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.metricName
    }

    fn mut_metricName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.metricName
    }

    // optional .k8s.io.apimachinery.pkg.api.resource.Quantity currentValue = 3;

    pub fn clear_currentValue(&mut self) {
        self.currentValue.clear();
    }

    pub fn has_currentValue(&self) -> bool {
        self.currentValue.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currentValue(&mut self, v: super::generated::Quantity) {
        self.currentValue = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_currentValue(&mut self) -> &mut super::generated::Quantity {
        if self.currentValue.is_none() {
            self.currentValue.set_default();
        }
        self.currentValue.as_mut().unwrap()
    }

    // Take field
    pub fn take_currentValue(&mut self) -> super::generated::Quantity {
        self.currentValue.take().unwrap_or_else(|| super::generated::Quantity::new())
    }

    pub fn get_currentValue(&self) -> &super::generated::Quantity {
        self.currentValue.as_ref().unwrap_or_else(|| super::generated::Quantity::default_instance())
    }

    fn get_currentValue_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::Quantity> {
        &self.currentValue
    }

    fn mut_currentValue_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::Quantity> {
        &mut self.currentValue
    }
}

impl ::protobuf::Message for ObjectMetricStatus {
    fn is_initialized(&self) -> bool {
        for v in &self.target {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.currentValue {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.target)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.metricName)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.currentValue)?;
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
        if let Some(ref v) = self.target.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.metricName.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.currentValue.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.target.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.metricName.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.currentValue.as_ref() {
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

impl ::protobuf::MessageStatic for ObjectMetricStatus {
    fn new() -> ObjectMetricStatus {
        ObjectMetricStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<ObjectMetricStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CrossVersionObjectReference>>(
                    "target",
                    ObjectMetricStatus::get_target_for_reflect,
                    ObjectMetricStatus::mut_target_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "metricName",
                    ObjectMetricStatus::get_metricName_for_reflect,
                    ObjectMetricStatus::mut_metricName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::Quantity>>(
                    "currentValue",
                    ObjectMetricStatus::get_currentValue_for_reflect,
                    ObjectMetricStatus::mut_currentValue_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ObjectMetricStatus>(
                    "ObjectMetricStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ObjectMetricStatus {
    fn clear(&mut self) {
        self.clear_target();
        self.clear_metricName();
        self.clear_currentValue();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ObjectMetricStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ObjectMetricStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PodsMetricSource {
    // message fields
    metricName: ::protobuf::SingularField<::std::string::String>,
    targetAverageValue: ::protobuf::SingularPtrField<super::generated::Quantity>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PodsMetricSource {}

impl PodsMetricSource {
    pub fn new() -> PodsMetricSource {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PodsMetricSource {
        static mut instance: ::protobuf::lazy::Lazy<PodsMetricSource> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PodsMetricSource,
        };
        unsafe {
            instance.get(PodsMetricSource::new)
        }
    }

    // optional string metricName = 1;

    pub fn clear_metricName(&mut self) {
        self.metricName.clear();
    }

    pub fn has_metricName(&self) -> bool {
        self.metricName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metricName(&mut self, v: ::std::string::String) {
        self.metricName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metricName(&mut self) -> &mut ::std::string::String {
        if self.metricName.is_none() {
            self.metricName.set_default();
        }
        self.metricName.as_mut().unwrap()
    }

    // Take field
    pub fn take_metricName(&mut self) -> ::std::string::String {
        self.metricName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_metricName(&self) -> &str {
        match self.metricName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_metricName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.metricName
    }

    fn mut_metricName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.metricName
    }

    // optional .k8s.io.apimachinery.pkg.api.resource.Quantity targetAverageValue = 2;

    pub fn clear_targetAverageValue(&mut self) {
        self.targetAverageValue.clear();
    }

    pub fn has_targetAverageValue(&self) -> bool {
        self.targetAverageValue.is_some()
    }

    // Param is passed by value, moved
    pub fn set_targetAverageValue(&mut self, v: super::generated::Quantity) {
        self.targetAverageValue = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_targetAverageValue(&mut self) -> &mut super::generated::Quantity {
        if self.targetAverageValue.is_none() {
            self.targetAverageValue.set_default();
        }
        self.targetAverageValue.as_mut().unwrap()
    }

    // Take field
    pub fn take_targetAverageValue(&mut self) -> super::generated::Quantity {
        self.targetAverageValue.take().unwrap_or_else(|| super::generated::Quantity::new())
    }

    pub fn get_targetAverageValue(&self) -> &super::generated::Quantity {
        self.targetAverageValue.as_ref().unwrap_or_else(|| super::generated::Quantity::default_instance())
    }

    fn get_targetAverageValue_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::Quantity> {
        &self.targetAverageValue
    }

    fn mut_targetAverageValue_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::Quantity> {
        &mut self.targetAverageValue
    }
}

impl ::protobuf::Message for PodsMetricSource {
    fn is_initialized(&self) -> bool {
        for v in &self.targetAverageValue {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.metricName)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.targetAverageValue)?;
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
        if let Some(ref v) = self.metricName.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.targetAverageValue.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.metricName.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.targetAverageValue.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for PodsMetricSource {
    fn new() -> PodsMetricSource {
        PodsMetricSource::new()
    }

    fn descriptor_static(_: ::std::option::Option<PodsMetricSource>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "metricName",
                    PodsMetricSource::get_metricName_for_reflect,
                    PodsMetricSource::mut_metricName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::Quantity>>(
                    "targetAverageValue",
                    PodsMetricSource::get_targetAverageValue_for_reflect,
                    PodsMetricSource::mut_targetAverageValue_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PodsMetricSource>(
                    "PodsMetricSource",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PodsMetricSource {
    fn clear(&mut self) {
        self.clear_metricName();
        self.clear_targetAverageValue();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PodsMetricSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PodsMetricSource {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PodsMetricStatus {
    // message fields
    metricName: ::protobuf::SingularField<::std::string::String>,
    currentAverageValue: ::protobuf::SingularPtrField<super::generated::Quantity>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PodsMetricStatus {}

impl PodsMetricStatus {
    pub fn new() -> PodsMetricStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PodsMetricStatus {
        static mut instance: ::protobuf::lazy::Lazy<PodsMetricStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PodsMetricStatus,
        };
        unsafe {
            instance.get(PodsMetricStatus::new)
        }
    }

    // optional string metricName = 1;

    pub fn clear_metricName(&mut self) {
        self.metricName.clear();
    }

    pub fn has_metricName(&self) -> bool {
        self.metricName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_metricName(&mut self, v: ::std::string::String) {
        self.metricName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_metricName(&mut self) -> &mut ::std::string::String {
        if self.metricName.is_none() {
            self.metricName.set_default();
        }
        self.metricName.as_mut().unwrap()
    }

    // Take field
    pub fn take_metricName(&mut self) -> ::std::string::String {
        self.metricName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_metricName(&self) -> &str {
        match self.metricName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_metricName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.metricName
    }

    fn mut_metricName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.metricName
    }

    // optional .k8s.io.apimachinery.pkg.api.resource.Quantity currentAverageValue = 2;

    pub fn clear_currentAverageValue(&mut self) {
        self.currentAverageValue.clear();
    }

    pub fn has_currentAverageValue(&self) -> bool {
        self.currentAverageValue.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currentAverageValue(&mut self, v: super::generated::Quantity) {
        self.currentAverageValue = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_currentAverageValue(&mut self) -> &mut super::generated::Quantity {
        if self.currentAverageValue.is_none() {
            self.currentAverageValue.set_default();
        }
        self.currentAverageValue.as_mut().unwrap()
    }

    // Take field
    pub fn take_currentAverageValue(&mut self) -> super::generated::Quantity {
        self.currentAverageValue.take().unwrap_or_else(|| super::generated::Quantity::new())
    }

    pub fn get_currentAverageValue(&self) -> &super::generated::Quantity {
        self.currentAverageValue.as_ref().unwrap_or_else(|| super::generated::Quantity::default_instance())
    }

    fn get_currentAverageValue_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::Quantity> {
        &self.currentAverageValue
    }

    fn mut_currentAverageValue_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::Quantity> {
        &mut self.currentAverageValue
    }
}

impl ::protobuf::Message for PodsMetricStatus {
    fn is_initialized(&self) -> bool {
        for v in &self.currentAverageValue {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.metricName)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.currentAverageValue)?;
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
        if let Some(ref v) = self.metricName.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.currentAverageValue.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.metricName.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.currentAverageValue.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for PodsMetricStatus {
    fn new() -> PodsMetricStatus {
        PodsMetricStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<PodsMetricStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "metricName",
                    PodsMetricStatus::get_metricName_for_reflect,
                    PodsMetricStatus::mut_metricName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::Quantity>>(
                    "currentAverageValue",
                    PodsMetricStatus::get_currentAverageValue_for_reflect,
                    PodsMetricStatus::mut_currentAverageValue_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PodsMetricStatus>(
                    "PodsMetricStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PodsMetricStatus {
    fn clear(&mut self) {
        self.clear_metricName();
        self.clear_currentAverageValue();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PodsMetricStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PodsMetricStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResourceMetricSource {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    targetAverageUtilization: ::std::option::Option<i32>,
    targetAverageValue: ::protobuf::SingularPtrField<super::generated::Quantity>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResourceMetricSource {}

impl ResourceMetricSource {
    pub fn new() -> ResourceMetricSource {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResourceMetricSource {
        static mut instance: ::protobuf::lazy::Lazy<ResourceMetricSource> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResourceMetricSource,
        };
        unsafe {
            instance.get(ResourceMetricSource::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional int32 targetAverageUtilization = 2;

    pub fn clear_targetAverageUtilization(&mut self) {
        self.targetAverageUtilization = ::std::option::Option::None;
    }

    pub fn has_targetAverageUtilization(&self) -> bool {
        self.targetAverageUtilization.is_some()
    }

    // Param is passed by value, moved
    pub fn set_targetAverageUtilization(&mut self, v: i32) {
        self.targetAverageUtilization = ::std::option::Option::Some(v);
    }

    pub fn get_targetAverageUtilization(&self) -> i32 {
        self.targetAverageUtilization.unwrap_or(0)
    }

    fn get_targetAverageUtilization_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.targetAverageUtilization
    }

    fn mut_targetAverageUtilization_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.targetAverageUtilization
    }

    // optional .k8s.io.apimachinery.pkg.api.resource.Quantity targetAverageValue = 3;

    pub fn clear_targetAverageValue(&mut self) {
        self.targetAverageValue.clear();
    }

    pub fn has_targetAverageValue(&self) -> bool {
        self.targetAverageValue.is_some()
    }

    // Param is passed by value, moved
    pub fn set_targetAverageValue(&mut self, v: super::generated::Quantity) {
        self.targetAverageValue = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_targetAverageValue(&mut self) -> &mut super::generated::Quantity {
        if self.targetAverageValue.is_none() {
            self.targetAverageValue.set_default();
        }
        self.targetAverageValue.as_mut().unwrap()
    }

    // Take field
    pub fn take_targetAverageValue(&mut self) -> super::generated::Quantity {
        self.targetAverageValue.take().unwrap_or_else(|| super::generated::Quantity::new())
    }

    pub fn get_targetAverageValue(&self) -> &super::generated::Quantity {
        self.targetAverageValue.as_ref().unwrap_or_else(|| super::generated::Quantity::default_instance())
    }

    fn get_targetAverageValue_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::Quantity> {
        &self.targetAverageValue
    }

    fn mut_targetAverageValue_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::Quantity> {
        &mut self.targetAverageValue
    }
}

impl ::protobuf::Message for ResourceMetricSource {
    fn is_initialized(&self) -> bool {
        for v in &self.targetAverageValue {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.targetAverageUtilization = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.targetAverageValue)?;
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.targetAverageUtilization {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.targetAverageValue.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.targetAverageUtilization {
            os.write_int32(2, v)?;
        }
        if let Some(ref v) = self.targetAverageValue.as_ref() {
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

impl ::protobuf::MessageStatic for ResourceMetricSource {
    fn new() -> ResourceMetricSource {
        ResourceMetricSource::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResourceMetricSource>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ResourceMetricSource::get_name_for_reflect,
                    ResourceMetricSource::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "targetAverageUtilization",
                    ResourceMetricSource::get_targetAverageUtilization_for_reflect,
                    ResourceMetricSource::mut_targetAverageUtilization_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::Quantity>>(
                    "targetAverageValue",
                    ResourceMetricSource::get_targetAverageValue_for_reflect,
                    ResourceMetricSource::mut_targetAverageValue_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResourceMetricSource>(
                    "ResourceMetricSource",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResourceMetricSource {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_targetAverageUtilization();
        self.clear_targetAverageValue();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResourceMetricSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResourceMetricSource {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResourceMetricStatus {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    currentAverageUtilization: ::std::option::Option<i32>,
    currentAverageValue: ::protobuf::SingularPtrField<super::generated::Quantity>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResourceMetricStatus {}

impl ResourceMetricStatus {
    pub fn new() -> ResourceMetricStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResourceMetricStatus {
        static mut instance: ::protobuf::lazy::Lazy<ResourceMetricStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResourceMetricStatus,
        };
        unsafe {
            instance.get(ResourceMetricStatus::new)
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        }
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_name_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.name
    }

    fn mut_name_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.name
    }

    // optional int32 currentAverageUtilization = 2;

    pub fn clear_currentAverageUtilization(&mut self) {
        self.currentAverageUtilization = ::std::option::Option::None;
    }

    pub fn has_currentAverageUtilization(&self) -> bool {
        self.currentAverageUtilization.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currentAverageUtilization(&mut self, v: i32) {
        self.currentAverageUtilization = ::std::option::Option::Some(v);
    }

    pub fn get_currentAverageUtilization(&self) -> i32 {
        self.currentAverageUtilization.unwrap_or(0)
    }

    fn get_currentAverageUtilization_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.currentAverageUtilization
    }

    fn mut_currentAverageUtilization_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.currentAverageUtilization
    }

    // optional .k8s.io.apimachinery.pkg.api.resource.Quantity currentAverageValue = 3;

    pub fn clear_currentAverageValue(&mut self) {
        self.currentAverageValue.clear();
    }

    pub fn has_currentAverageValue(&self) -> bool {
        self.currentAverageValue.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currentAverageValue(&mut self, v: super::generated::Quantity) {
        self.currentAverageValue = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_currentAverageValue(&mut self) -> &mut super::generated::Quantity {
        if self.currentAverageValue.is_none() {
            self.currentAverageValue.set_default();
        }
        self.currentAverageValue.as_mut().unwrap()
    }

    // Take field
    pub fn take_currentAverageValue(&mut self) -> super::generated::Quantity {
        self.currentAverageValue.take().unwrap_or_else(|| super::generated::Quantity::new())
    }

    pub fn get_currentAverageValue(&self) -> &super::generated::Quantity {
        self.currentAverageValue.as_ref().unwrap_or_else(|| super::generated::Quantity::default_instance())
    }

    fn get_currentAverageValue_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::Quantity> {
        &self.currentAverageValue
    }

    fn mut_currentAverageValue_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::Quantity> {
        &mut self.currentAverageValue
    }
}

impl ::protobuf::Message for ResourceMetricStatus {
    fn is_initialized(&self) -> bool {
        for v in &self.currentAverageValue {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.currentAverageUtilization = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.currentAverageValue)?;
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.currentAverageUtilization {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.currentAverageValue.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.currentAverageUtilization {
            os.write_int32(2, v)?;
        }
        if let Some(ref v) = self.currentAverageValue.as_ref() {
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

impl ::protobuf::MessageStatic for ResourceMetricStatus {
    fn new() -> ResourceMetricStatus {
        ResourceMetricStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResourceMetricStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ResourceMetricStatus::get_name_for_reflect,
                    ResourceMetricStatus::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "currentAverageUtilization",
                    ResourceMetricStatus::get_currentAverageUtilization_for_reflect,
                    ResourceMetricStatus::mut_currentAverageUtilization_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::Quantity>>(
                    "currentAverageValue",
                    ResourceMetricStatus::get_currentAverageValue_for_reflect,
                    ResourceMetricStatus::mut_currentAverageValue_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResourceMetricStatus>(
                    "ResourceMetricStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResourceMetricStatus {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_currentAverageUtilization();
        self.clear_currentAverageValue();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResourceMetricStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResourceMetricStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Scale {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    spec: ::protobuf::SingularPtrField<ScaleSpec>,
    status: ::protobuf::SingularPtrField<ScaleStatus>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Scale {}

impl Scale {
    pub fn new() -> Scale {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Scale {
        static mut instance: ::protobuf::lazy::Lazy<Scale> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Scale,
        };
        unsafe {
            instance.get(Scale::new)
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

    // optional .k8s.io.api.autoscaling.v1.ScaleSpec spec = 2;

    pub fn clear_spec(&mut self) {
        self.spec.clear();
    }

    pub fn has_spec(&self) -> bool {
        self.spec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spec(&mut self, v: ScaleSpec) {
        self.spec = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_spec(&mut self) -> &mut ScaleSpec {
        if self.spec.is_none() {
            self.spec.set_default();
        }
        self.spec.as_mut().unwrap()
    }

    // Take field
    pub fn take_spec(&mut self) -> ScaleSpec {
        self.spec.take().unwrap_or_else(|| ScaleSpec::new())
    }

    pub fn get_spec(&self) -> &ScaleSpec {
        self.spec.as_ref().unwrap_or_else(|| ScaleSpec::default_instance())
    }

    fn get_spec_for_reflect(&self) -> &::protobuf::SingularPtrField<ScaleSpec> {
        &self.spec
    }

    fn mut_spec_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ScaleSpec> {
        &mut self.spec
    }

    // optional .k8s.io.api.autoscaling.v1.ScaleStatus status = 3;

    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: ScaleStatus) {
        self.status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut ScaleStatus {
        if self.status.is_none() {
            self.status.set_default();
        }
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> ScaleStatus {
        self.status.take().unwrap_or_else(|| ScaleStatus::new())
    }

    pub fn get_status(&self) -> &ScaleStatus {
        self.status.as_ref().unwrap_or_else(|| ScaleStatus::default_instance())
    }

    fn get_status_for_reflect(&self) -> &::protobuf::SingularPtrField<ScaleStatus> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ScaleStatus> {
        &mut self.status
    }
}

impl ::protobuf::Message for Scale {
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

impl ::protobuf::MessageStatic for Scale {
    fn new() -> Scale {
        Scale::new()
    }

    fn descriptor_static(_: ::std::option::Option<Scale>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    Scale::get_metadata_for_reflect,
                    Scale::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ScaleSpec>>(
                    "spec",
                    Scale::get_spec_for_reflect,
                    Scale::mut_spec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ScaleStatus>>(
                    "status",
                    Scale::get_status_for_reflect,
                    Scale::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Scale>(
                    "Scale",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Scale {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_spec();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Scale {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Scale {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ScaleSpec {
    // message fields
    replicas: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ScaleSpec {}

impl ScaleSpec {
    pub fn new() -> ScaleSpec {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScaleSpec {
        static mut instance: ::protobuf::lazy::Lazy<ScaleSpec> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScaleSpec,
        };
        unsafe {
            instance.get(ScaleSpec::new)
        }
    }

    // optional int32 replicas = 1;

    pub fn clear_replicas(&mut self) {
        self.replicas = ::std::option::Option::None;
    }

    pub fn has_replicas(&self) -> bool {
        self.replicas.is_some()
    }

    // Param is passed by value, moved
    pub fn set_replicas(&mut self, v: i32) {
        self.replicas = ::std::option::Option::Some(v);
    }

    pub fn get_replicas(&self) -> i32 {
        self.replicas.unwrap_or(0)
    }

    fn get_replicas_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.replicas
    }

    fn mut_replicas_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.replicas
    }
}

impl ::protobuf::Message for ScaleSpec {
    fn is_initialized(&self) -> bool {
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
                    let tmp = is.read_int32()?;
                    self.replicas = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.replicas {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.replicas {
            os.write_int32(1, v)?;
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

impl ::protobuf::MessageStatic for ScaleSpec {
    fn new() -> ScaleSpec {
        ScaleSpec::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScaleSpec>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "replicas",
                    ScaleSpec::get_replicas_for_reflect,
                    ScaleSpec::mut_replicas_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ScaleSpec>(
                    "ScaleSpec",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScaleSpec {
    fn clear(&mut self) {
        self.clear_replicas();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ScaleSpec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ScaleSpec {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ScaleStatus {
    // message fields
    replicas: ::std::option::Option<i32>,
    selector: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ScaleStatus {}

impl ScaleStatus {
    pub fn new() -> ScaleStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScaleStatus {
        static mut instance: ::protobuf::lazy::Lazy<ScaleStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScaleStatus,
        };
        unsafe {
            instance.get(ScaleStatus::new)
        }
    }

    // optional int32 replicas = 1;

    pub fn clear_replicas(&mut self) {
        self.replicas = ::std::option::Option::None;
    }

    pub fn has_replicas(&self) -> bool {
        self.replicas.is_some()
    }

    // Param is passed by value, moved
    pub fn set_replicas(&mut self, v: i32) {
        self.replicas = ::std::option::Option::Some(v);
    }

    pub fn get_replicas(&self) -> i32 {
        self.replicas.unwrap_or(0)
    }

    fn get_replicas_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.replicas
    }

    fn mut_replicas_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.replicas
    }

    // optional string selector = 2;

    pub fn clear_selector(&mut self) {
        self.selector.clear();
    }

    pub fn has_selector(&self) -> bool {
        self.selector.is_some()
    }

    // Param is passed by value, moved
    pub fn set_selector(&mut self, v: ::std::string::String) {
        self.selector = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_selector(&mut self) -> &mut ::std::string::String {
        if self.selector.is_none() {
            self.selector.set_default();
        }
        self.selector.as_mut().unwrap()
    }

    // Take field
    pub fn take_selector(&mut self) -> ::std::string::String {
        self.selector.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_selector(&self) -> &str {
        match self.selector.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_selector_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.selector
    }

    fn mut_selector_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.selector
    }
}

impl ::protobuf::Message for ScaleStatus {
    fn is_initialized(&self) -> bool {
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
                    let tmp = is.read_int32()?;
                    self.replicas = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.selector)?;
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
        if let Some(v) = self.replicas {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.selector.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.replicas {
            os.write_int32(1, v)?;
        }
        if let Some(ref v) = self.selector.as_ref() {
            os.write_string(2, &v)?;
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

impl ::protobuf::MessageStatic for ScaleStatus {
    fn new() -> ScaleStatus {
        ScaleStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScaleStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "replicas",
                    ScaleStatus::get_replicas_for_reflect,
                    ScaleStatus::mut_replicas_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "selector",
                    ScaleStatus::get_selector_for_reflect,
                    ScaleStatus::mut_selector_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ScaleStatus>(
                    "ScaleStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScaleStatus {
    fn clear(&mut self) {
        self.clear_replicas();
        self.clear_selector();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ScaleStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ScaleStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n)k8s.io/api/autoscaling/v1/generated.proto\x12\x19k8s.io.api.autoscali\
    ng.v1\x1a\"k8s.io/api/core/v1/generated.proto\x1a4k8s.io/apimachinery/pk\
    g/api/resource/generated.proto\x1a4k8s.io/apimachinery/pkg/apis/meta/v1/\
    generated.proto\x1a/k8s.io/apimachinery/pkg/runtime/generated.proto\x1a6\
    k8s.io/apimachinery/pkg/runtime/schema/generated.proto\x1a3k8s.io/apimac\
    hinery/pkg/util/intstr/generated.proto\"e\n\x1bCrossVersionObjectReferen\
    ce\x12\x12\n\x04kind\x18\x01\x20\x01(\tR\x04kind\x12\x12\n\x04name\x18\
    \x02\x20\x01(\tR\x04name\x12\x1e\n\napiVersion\x18\x03\x20\x01(\tR\napiV\
    ersion\"\x85\x02\n\x17HorizontalPodAutoscaler\x12L\n\x08metadata\x18\x01\
    \x20\x01(\x0b20.k8s.io.apimachinery.pkg.apis.meta.v1.ObjectMetaR\x08meta\
    data\x12J\n\x04spec\x18\x02\x20\x01(\x0b26.k8s.io.api.autoscaling.v1.Hor\
    izontalPodAutoscalerSpecR\x04spec\x12P\n\x06status\x18\x03\x20\x01(\x0b2\
    8.k8s.io.api.autoscaling.v1.HorizontalPodAutoscalerStatusR\x06status\"\
    \xdc\x01\n\x20HorizontalPodAutoscalerCondition\x12\x12\n\x04type\x18\x01\
    \x20\x01(\tR\x04type\x12\x16\n\x06status\x18\x02\x20\x01(\tR\x06status\
    \x12Z\n\x12lastTransitionTime\x18\x03\x20\x01(\x0b2*.k8s.io.apimachinery\
    .pkg.apis.meta.v1.TimeR\x12lastTransitionTime\x12\x16\n\x06reason\x18\
    \x04\x20\x01(\tR\x06reason\x12\x18\n\x07message\x18\x05\x20\x01(\tR\x07m\
    essage\"\xb3\x01\n\x1bHorizontalPodAutoscalerList\x12J\n\x08metadata\x18\
    \x01\x20\x01(\x0b2..k8s.io.apimachinery.pkg.apis.meta.v1.ListMetaR\x08me\
    tadata\x12H\n\x05items\x18\x02\x20\x03(\x0b22.k8s.io.api.autoscaling.v1.\
    HorizontalPodAutoscalerR\x05items\"\x89\x02\n\x1bHorizontalPodAutoscaler\
    Spec\x12^\n\x0escaleTargetRef\x18\x01\x20\x01(\x0b26.k8s.io.api.autoscal\
    ing.v1.CrossVersionObjectReferenceR\x0escaleTargetRef\x12\x20\n\x0bminRe\
    plicas\x18\x02\x20\x01(\x05R\x0bminReplicas\x12\x20\n\x0bmaxReplicas\x18\
    \x03\x20\x01(\x05R\x0bmaxReplicas\x12F\n\x1etargetCPUUtilizationPercenta\
    ge\x18\x04\x20\x01(\x05R\x1etargetCPUUtilizationPercentage\"\xbf\x02\n\
    \x1dHorizontalPodAutoscalerStatus\x12.\n\x12observedGeneration\x18\x01\
    \x20\x01(\x03R\x12observedGeneration\x12P\n\rlastScaleTime\x18\x02\x20\
    \x01(\x0b2*.k8s.io.apimachinery.pkg.apis.meta.v1.TimeR\rlastScaleTime\
    \x12(\n\x0fcurrentReplicas\x18\x03\x20\x01(\x05R\x0fcurrentReplicas\x12(\
    \n\x0fdesiredReplicas\x18\x04\x20\x01(\x05R\x0fdesiredReplicas\x12H\n\
    \x1fcurrentCPUUtilizationPercentage\x18\x05\x20\x01(\x05R\x1fcurrentCPUU\
    tilizationPercentage\"\xf5\x01\n\nMetricSpec\x12\x12\n\x04type\x18\x01\
    \x20\x01(\tR\x04type\x12E\n\x06object\x18\x02\x20\x01(\x0b2-.k8s.io.api.\
    autoscaling.v1.ObjectMetricSourceR\x06object\x12?\n\x04pods\x18\x03\x20\
    \x01(\x0b2+.k8s.io.api.autoscaling.v1.PodsMetricSourceR\x04pods\x12K\n\
    \x08resource\x18\x04\x20\x01(\x0b2/.k8s.io.api.autoscaling.v1.ResourceMe\
    tricSourceR\x08resource\"\xf7\x01\n\x0cMetricStatus\x12\x12\n\x04type\
    \x18\x01\x20\x01(\tR\x04type\x12E\n\x06object\x18\x02\x20\x01(\x0b2-.k8s\
    .io.api.autoscaling.v1.ObjectMetricStatusR\x06object\x12?\n\x04pods\x18\
    \x03\x20\x01(\x0b2+.k8s.io.api.autoscaling.v1.PodsMetricStatusR\x04pods\
    \x12K\n\x08resource\x18\x04\x20\x01(\x0b2/.k8s.io.api.autoscaling.v1.Res\
    ourceMetricStatusR\x08resource\"\xd6\x01\n\x12ObjectMetricSource\x12N\n\
    \x06target\x18\x01\x20\x01(\x0b26.k8s.io.api.autoscaling.v1.CrossVersion\
    ObjectReferenceR\x06target\x12\x1e\n\nmetricName\x18\x02\x20\x01(\tR\nme\
    tricName\x12P\n\x0btargetValue\x18\x03\x20\x01(\x0b2..k8s.io.apimachiner\
    y.pkg.api.resource.QuantityR\x0btargetValue\"\xd8\x01\n\x12ObjectMetricS\
    tatus\x12N\n\x06target\x18\x01\x20\x01(\x0b26.k8s.io.api.autoscaling.v1.\
    CrossVersionObjectReferenceR\x06target\x12\x1e\n\nmetricName\x18\x02\x20\
    \x01(\tR\nmetricName\x12R\n\x0ccurrentValue\x18\x03\x20\x01(\x0b2..k8s.i\
    o.apimachinery.pkg.api.resource.QuantityR\x0ccurrentValue\"\x92\x01\n\
    \x10PodsMetricSource\x12\x1e\n\nmetricName\x18\x01\x20\x01(\tR\nmetricNa\
    me\x12^\n\x12targetAverageValue\x18\x02\x20\x01(\x0b2..k8s.io.apimachine\
    ry.pkg.api.resource.QuantityR\x12targetAverageValue\"\x94\x01\n\x10PodsM\
    etricStatus\x12\x1e\n\nmetricName\x18\x01\x20\x01(\tR\nmetricName\x12`\n\
    \x13currentAverageValue\x18\x02\x20\x01(\x0b2..k8s.io.apimachinery.pkg.a\
    pi.resource.QuantityR\x13currentAverageValue\"\xc6\x01\n\x14ResourceMetr\
    icSource\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12:\n\x18targetA\
    verageUtilization\x18\x02\x20\x01(\x05R\x18targetAverageUtilization\x12^\
    \n\x12targetAverageValue\x18\x03\x20\x01(\x0b2..k8s.io.apimachinery.pkg.\
    api.resource.QuantityR\x12targetAverageValue\"\xca\x01\n\x14ResourceMetr\
    icStatus\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12<\n\x19current\
    AverageUtilization\x18\x02\x20\x01(\x05R\x19currentAverageUtilization\
    \x12`\n\x13currentAverageValue\x18\x03\x20\x01(\x0b2..k8s.io.apimachiner\
    y.pkg.api.resource.QuantityR\x13currentAverageValue\"\xcf\x01\n\x05Scale\
    \x12L\n\x08metadata\x18\x01\x20\x01(\x0b20.k8s.io.apimachinery.pkg.apis.\
    meta.v1.ObjectMetaR\x08metadata\x128\n\x04spec\x18\x02\x20\x01(\x0b2$.k8\
    s.io.api.autoscaling.v1.ScaleSpecR\x04spec\x12>\n\x06status\x18\x03\x20\
    \x01(\x0b2&.k8s.io.api.autoscaling.v1.ScaleStatusR\x06status\"'\n\tScale\
    Spec\x12\x1a\n\x08replicas\x18\x01\x20\x01(\x05R\x08replicas\"E\n\x0bSca\
    leStatus\x12\x1a\n\x08replicas\x18\x01\x20\x01(\x05R\x08replicas\x12\x1a\
    \n\x08selector\x18\x02\x20\x01(\tR\x08selectorB\x04Z\x02v1\
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
