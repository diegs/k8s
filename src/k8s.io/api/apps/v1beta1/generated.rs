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
pub struct ControllerRevision {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    data: ::protobuf::SingularPtrField<super::generated::RawExtension>,
    revision: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ControllerRevision {}

impl ControllerRevision {
    pub fn new() -> ControllerRevision {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ControllerRevision {
        static mut instance: ::protobuf::lazy::Lazy<ControllerRevision> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ControllerRevision,
        };
        unsafe {
            instance.get(ControllerRevision::new)
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

    // optional .k8s.io.apimachinery.pkg.runtime.RawExtension data = 2;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: super::generated::RawExtension) {
        self.data = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut super::generated::RawExtension {
        if self.data.is_none() {
            self.data.set_default();
        }
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> super::generated::RawExtension {
        self.data.take().unwrap_or_else(|| super::generated::RawExtension::new())
    }

    pub fn get_data(&self) -> &super::generated::RawExtension {
        self.data.as_ref().unwrap_or_else(|| super::generated::RawExtension::default_instance())
    }

    fn get_data_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::RawExtension> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::RawExtension> {
        &mut self.data
    }

    // optional int64 revision = 3;

    pub fn clear_revision(&mut self) {
        self.revision = ::std::option::Option::None;
    }

    pub fn has_revision(&self) -> bool {
        self.revision.is_some()
    }

    // Param is passed by value, moved
    pub fn set_revision(&mut self, v: i64) {
        self.revision = ::std::option::Option::Some(v);
    }

    pub fn get_revision(&self) -> i64 {
        self.revision.unwrap_or(0)
    }

    fn get_revision_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.revision
    }

    fn mut_revision_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.revision
    }
}

impl ::protobuf::Message for ControllerRevision {
    fn is_initialized(&self) -> bool {
        for v in &self.metadata {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.data {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.data)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.revision = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.data.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.revision {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
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
        if let Some(ref v) = self.data.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.revision {
            os.write_int64(3, v)?;
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

impl ::protobuf::MessageStatic for ControllerRevision {
    fn new() -> ControllerRevision {
        ControllerRevision::new()
    }

    fn descriptor_static(_: ::std::option::Option<ControllerRevision>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    ControllerRevision::get_metadata_for_reflect,
                    ControllerRevision::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::RawExtension>>(
                    "data",
                    ControllerRevision::get_data_for_reflect,
                    ControllerRevision::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "revision",
                    ControllerRevision::get_revision_for_reflect,
                    ControllerRevision::mut_revision_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ControllerRevision>(
                    "ControllerRevision",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ControllerRevision {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_data();
        self.clear_revision();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ControllerRevision {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ControllerRevision {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ControllerRevisionList {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ListMeta>,
    items: ::protobuf::RepeatedField<ControllerRevision>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ControllerRevisionList {}

impl ControllerRevisionList {
    pub fn new() -> ControllerRevisionList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ControllerRevisionList {
        static mut instance: ::protobuf::lazy::Lazy<ControllerRevisionList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ControllerRevisionList,
        };
        unsafe {
            instance.get(ControllerRevisionList::new)
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

    // repeated .k8s.io.api.apps.v1beta1.ControllerRevision items = 2;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<ControllerRevision>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<ControllerRevision> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<ControllerRevision> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[ControllerRevision] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<ControllerRevision> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ControllerRevision> {
        &mut self.items
    }
}

impl ::protobuf::Message for ControllerRevisionList {
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

impl ::protobuf::MessageStatic for ControllerRevisionList {
    fn new() -> ControllerRevisionList {
        ControllerRevisionList::new()
    }

    fn descriptor_static(_: ::std::option::Option<ControllerRevisionList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ListMeta>>(
                    "metadata",
                    ControllerRevisionList::get_metadata_for_reflect,
                    ControllerRevisionList::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ControllerRevision>>(
                    "items",
                    ControllerRevisionList::get_items_for_reflect,
                    ControllerRevisionList::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ControllerRevisionList>(
                    "ControllerRevisionList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ControllerRevisionList {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ControllerRevisionList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ControllerRevisionList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Deployment {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    spec: ::protobuf::SingularPtrField<DeploymentSpec>,
    status: ::protobuf::SingularPtrField<DeploymentStatus>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Deployment {}

impl Deployment {
    pub fn new() -> Deployment {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Deployment {
        static mut instance: ::protobuf::lazy::Lazy<Deployment> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Deployment,
        };
        unsafe {
            instance.get(Deployment::new)
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

    // optional .k8s.io.api.apps.v1beta1.DeploymentSpec spec = 2;

    pub fn clear_spec(&mut self) {
        self.spec.clear();
    }

    pub fn has_spec(&self) -> bool {
        self.spec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spec(&mut self, v: DeploymentSpec) {
        self.spec = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_spec(&mut self) -> &mut DeploymentSpec {
        if self.spec.is_none() {
            self.spec.set_default();
        }
        self.spec.as_mut().unwrap()
    }

    // Take field
    pub fn take_spec(&mut self) -> DeploymentSpec {
        self.spec.take().unwrap_or_else(|| DeploymentSpec::new())
    }

    pub fn get_spec(&self) -> &DeploymentSpec {
        self.spec.as_ref().unwrap_or_else(|| DeploymentSpec::default_instance())
    }

    fn get_spec_for_reflect(&self) -> &::protobuf::SingularPtrField<DeploymentSpec> {
        &self.spec
    }

    fn mut_spec_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DeploymentSpec> {
        &mut self.spec
    }

    // optional .k8s.io.api.apps.v1beta1.DeploymentStatus status = 3;

    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: DeploymentStatus) {
        self.status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut DeploymentStatus {
        if self.status.is_none() {
            self.status.set_default();
        }
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> DeploymentStatus {
        self.status.take().unwrap_or_else(|| DeploymentStatus::new())
    }

    pub fn get_status(&self) -> &DeploymentStatus {
        self.status.as_ref().unwrap_or_else(|| DeploymentStatus::default_instance())
    }

    fn get_status_for_reflect(&self) -> &::protobuf::SingularPtrField<DeploymentStatus> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DeploymentStatus> {
        &mut self.status
    }
}

impl ::protobuf::Message for Deployment {
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

impl ::protobuf::MessageStatic for Deployment {
    fn new() -> Deployment {
        Deployment::new()
    }

    fn descriptor_static(_: ::std::option::Option<Deployment>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    Deployment::get_metadata_for_reflect,
                    Deployment::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DeploymentSpec>>(
                    "spec",
                    Deployment::get_spec_for_reflect,
                    Deployment::mut_spec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DeploymentStatus>>(
                    "status",
                    Deployment::get_status_for_reflect,
                    Deployment::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Deployment>(
                    "Deployment",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Deployment {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_spec();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Deployment {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Deployment {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeploymentCondition {
    // message fields
    field_type: ::protobuf::SingularField<::std::string::String>,
    status: ::protobuf::SingularField<::std::string::String>,
    lastUpdateTime: ::protobuf::SingularPtrField<super::generated::Time>,
    lastTransitionTime: ::protobuf::SingularPtrField<super::generated::Time>,
    reason: ::protobuf::SingularField<::std::string::String>,
    message: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeploymentCondition {}

impl DeploymentCondition {
    pub fn new() -> DeploymentCondition {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeploymentCondition {
        static mut instance: ::protobuf::lazy::Lazy<DeploymentCondition> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeploymentCondition,
        };
        unsafe {
            instance.get(DeploymentCondition::new)
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

    // optional .k8s.io.apimachinery.pkg.apis.meta.v1.Time lastUpdateTime = 6;

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

    // optional .k8s.io.apimachinery.pkg.apis.meta.v1.Time lastTransitionTime = 7;

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

impl ::protobuf::Message for DeploymentCondition {
    fn is_initialized(&self) -> bool {
        for v in &self.lastUpdateTime {
            if !v.is_initialized() {
                return false;
            }
        };
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
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.lastUpdateTime)?;
                },
                7 => {
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
        if let Some(ref v) = self.lastUpdateTime.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
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
        if let Some(ref v) = self.lastUpdateTime.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.lastTransitionTime.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for DeploymentCondition {
    fn new() -> DeploymentCondition {
        DeploymentCondition::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeploymentCondition>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    DeploymentCondition::get_field_type_for_reflect,
                    DeploymentCondition::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "status",
                    DeploymentCondition::get_status_for_reflect,
                    DeploymentCondition::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::Time>>(
                    "lastUpdateTime",
                    DeploymentCondition::get_lastUpdateTime_for_reflect,
                    DeploymentCondition::mut_lastUpdateTime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::Time>>(
                    "lastTransitionTime",
                    DeploymentCondition::get_lastTransitionTime_for_reflect,
                    DeploymentCondition::mut_lastTransitionTime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "reason",
                    DeploymentCondition::get_reason_for_reflect,
                    DeploymentCondition::mut_reason_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    DeploymentCondition::get_message_for_reflect,
                    DeploymentCondition::mut_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeploymentCondition>(
                    "DeploymentCondition",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeploymentCondition {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_status();
        self.clear_lastUpdateTime();
        self.clear_lastTransitionTime();
        self.clear_reason();
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeploymentCondition {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeploymentCondition {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeploymentList {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ListMeta>,
    items: ::protobuf::RepeatedField<Deployment>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeploymentList {}

impl DeploymentList {
    pub fn new() -> DeploymentList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeploymentList {
        static mut instance: ::protobuf::lazy::Lazy<DeploymentList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeploymentList,
        };
        unsafe {
            instance.get(DeploymentList::new)
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

    // repeated .k8s.io.api.apps.v1beta1.Deployment items = 2;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<Deployment>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<Deployment> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<Deployment> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[Deployment] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<Deployment> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Deployment> {
        &mut self.items
    }
}

impl ::protobuf::Message for DeploymentList {
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

impl ::protobuf::MessageStatic for DeploymentList {
    fn new() -> DeploymentList {
        DeploymentList::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeploymentList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ListMeta>>(
                    "metadata",
                    DeploymentList::get_metadata_for_reflect,
                    DeploymentList::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Deployment>>(
                    "items",
                    DeploymentList::get_items_for_reflect,
                    DeploymentList::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeploymentList>(
                    "DeploymentList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeploymentList {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeploymentList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeploymentList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeploymentRollback {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    pub updatedAnnotations: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    rollbackTo: ::protobuf::SingularPtrField<RollbackConfig>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeploymentRollback {}

impl DeploymentRollback {
    pub fn new() -> DeploymentRollback {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeploymentRollback {
        static mut instance: ::protobuf::lazy::Lazy<DeploymentRollback> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeploymentRollback,
        };
        unsafe {
            instance.get(DeploymentRollback::new)
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

    // repeated .k8s.io.api.apps.v1beta1.DeploymentRollback.UpdatedAnnotationsEntry updatedAnnotations = 2;

    pub fn clear_updatedAnnotations(&mut self) {
        self.updatedAnnotations.clear();
    }

    // Param is passed by value, moved
    pub fn set_updatedAnnotations(&mut self, v: ::std::collections::HashMap<::std::string::String, ::std::string::String>) {
        self.updatedAnnotations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_updatedAnnotations(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.updatedAnnotations
    }

    // Take field
    pub fn take_updatedAnnotations(&mut self) -> ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        ::std::mem::replace(&mut self.updatedAnnotations, ::std::collections::HashMap::new())
    }

    pub fn get_updatedAnnotations(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.updatedAnnotations
    }

    fn get_updatedAnnotations_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.updatedAnnotations
    }

    fn mut_updatedAnnotations_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.updatedAnnotations
    }

    // optional .k8s.io.api.apps.v1beta1.RollbackConfig rollbackTo = 3;

    pub fn clear_rollbackTo(&mut self) {
        self.rollbackTo.clear();
    }

    pub fn has_rollbackTo(&self) -> bool {
        self.rollbackTo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rollbackTo(&mut self, v: RollbackConfig) {
        self.rollbackTo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rollbackTo(&mut self) -> &mut RollbackConfig {
        if self.rollbackTo.is_none() {
            self.rollbackTo.set_default();
        }
        self.rollbackTo.as_mut().unwrap()
    }

    // Take field
    pub fn take_rollbackTo(&mut self) -> RollbackConfig {
        self.rollbackTo.take().unwrap_or_else(|| RollbackConfig::new())
    }

    pub fn get_rollbackTo(&self) -> &RollbackConfig {
        self.rollbackTo.as_ref().unwrap_or_else(|| RollbackConfig::default_instance())
    }

    fn get_rollbackTo_for_reflect(&self) -> &::protobuf::SingularPtrField<RollbackConfig> {
        &self.rollbackTo
    }

    fn mut_rollbackTo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RollbackConfig> {
        &mut self.rollbackTo
    }
}

impl ::protobuf::Message for DeploymentRollback {
    fn is_initialized(&self) -> bool {
        for v in &self.rollbackTo {
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
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(wire_type, is, &mut self.updatedAnnotations)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.rollbackTo)?;
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
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(2, &self.updatedAnnotations);
        if let Some(ref v) = self.rollbackTo.as_ref() {
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
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(2, &self.updatedAnnotations, os)?;
        if let Some(ref v) = self.rollbackTo.as_ref() {
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

impl ::protobuf::MessageStatic for DeploymentRollback {
    fn new() -> DeploymentRollback {
        DeploymentRollback::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeploymentRollback>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    DeploymentRollback::get_name_for_reflect,
                    DeploymentRollback::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(
                    "updatedAnnotations",
                    DeploymentRollback::get_updatedAnnotations_for_reflect,
                    DeploymentRollback::mut_updatedAnnotations_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RollbackConfig>>(
                    "rollbackTo",
                    DeploymentRollback::get_rollbackTo_for_reflect,
                    DeploymentRollback::mut_rollbackTo_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeploymentRollback>(
                    "DeploymentRollback",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeploymentRollback {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_updatedAnnotations();
        self.clear_rollbackTo();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeploymentRollback {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeploymentRollback {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeploymentSpec {
    // message fields
    replicas: ::std::option::Option<i32>,
    selector: ::protobuf::SingularPtrField<super::generated::LabelSelector>,
    template: ::protobuf::SingularPtrField<super::generated::PodTemplateSpec>,
    strategy: ::protobuf::SingularPtrField<DeploymentStrategy>,
    minReadySeconds: ::std::option::Option<i32>,
    revisionHistoryLimit: ::std::option::Option<i32>,
    paused: ::std::option::Option<bool>,
    rollbackTo: ::protobuf::SingularPtrField<RollbackConfig>,
    progressDeadlineSeconds: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeploymentSpec {}

impl DeploymentSpec {
    pub fn new() -> DeploymentSpec {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeploymentSpec {
        static mut instance: ::protobuf::lazy::Lazy<DeploymentSpec> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeploymentSpec,
        };
        unsafe {
            instance.get(DeploymentSpec::new)
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

    // optional .k8s.io.apimachinery.pkg.apis.meta.v1.LabelSelector selector = 2;

    pub fn clear_selector(&mut self) {
        self.selector.clear();
    }

    pub fn has_selector(&self) -> bool {
        self.selector.is_some()
    }

    // Param is passed by value, moved
    pub fn set_selector(&mut self, v: super::generated::LabelSelector) {
        self.selector = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_selector(&mut self) -> &mut super::generated::LabelSelector {
        if self.selector.is_none() {
            self.selector.set_default();
        }
        self.selector.as_mut().unwrap()
    }

    // Take field
    pub fn take_selector(&mut self) -> super::generated::LabelSelector {
        self.selector.take().unwrap_or_else(|| super::generated::LabelSelector::new())
    }

    pub fn get_selector(&self) -> &super::generated::LabelSelector {
        self.selector.as_ref().unwrap_or_else(|| super::generated::LabelSelector::default_instance())
    }

    fn get_selector_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::LabelSelector> {
        &self.selector
    }

    fn mut_selector_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::LabelSelector> {
        &mut self.selector
    }

    // optional .k8s.io.api.core.v1.PodTemplateSpec template = 3;

    pub fn clear_template(&mut self) {
        self.template.clear();
    }

    pub fn has_template(&self) -> bool {
        self.template.is_some()
    }

    // Param is passed by value, moved
    pub fn set_template(&mut self, v: super::generated::PodTemplateSpec) {
        self.template = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_template(&mut self) -> &mut super::generated::PodTemplateSpec {
        if self.template.is_none() {
            self.template.set_default();
        }
        self.template.as_mut().unwrap()
    }

    // Take field
    pub fn take_template(&mut self) -> super::generated::PodTemplateSpec {
        self.template.take().unwrap_or_else(|| super::generated::PodTemplateSpec::new())
    }

    pub fn get_template(&self) -> &super::generated::PodTemplateSpec {
        self.template.as_ref().unwrap_or_else(|| super::generated::PodTemplateSpec::default_instance())
    }

    fn get_template_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::PodTemplateSpec> {
        &self.template
    }

    fn mut_template_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::PodTemplateSpec> {
        &mut self.template
    }

    // optional .k8s.io.api.apps.v1beta1.DeploymentStrategy strategy = 4;

    pub fn clear_strategy(&mut self) {
        self.strategy.clear();
    }

    pub fn has_strategy(&self) -> bool {
        self.strategy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_strategy(&mut self, v: DeploymentStrategy) {
        self.strategy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_strategy(&mut self) -> &mut DeploymentStrategy {
        if self.strategy.is_none() {
            self.strategy.set_default();
        }
        self.strategy.as_mut().unwrap()
    }

    // Take field
    pub fn take_strategy(&mut self) -> DeploymentStrategy {
        self.strategy.take().unwrap_or_else(|| DeploymentStrategy::new())
    }

    pub fn get_strategy(&self) -> &DeploymentStrategy {
        self.strategy.as_ref().unwrap_or_else(|| DeploymentStrategy::default_instance())
    }

    fn get_strategy_for_reflect(&self) -> &::protobuf::SingularPtrField<DeploymentStrategy> {
        &self.strategy
    }

    fn mut_strategy_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DeploymentStrategy> {
        &mut self.strategy
    }

    // optional int32 minReadySeconds = 5;

    pub fn clear_minReadySeconds(&mut self) {
        self.minReadySeconds = ::std::option::Option::None;
    }

    pub fn has_minReadySeconds(&self) -> bool {
        self.minReadySeconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_minReadySeconds(&mut self, v: i32) {
        self.minReadySeconds = ::std::option::Option::Some(v);
    }

    pub fn get_minReadySeconds(&self) -> i32 {
        self.minReadySeconds.unwrap_or(0)
    }

    fn get_minReadySeconds_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.minReadySeconds
    }

    fn mut_minReadySeconds_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.minReadySeconds
    }

    // optional int32 revisionHistoryLimit = 6;

    pub fn clear_revisionHistoryLimit(&mut self) {
        self.revisionHistoryLimit = ::std::option::Option::None;
    }

    pub fn has_revisionHistoryLimit(&self) -> bool {
        self.revisionHistoryLimit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_revisionHistoryLimit(&mut self, v: i32) {
        self.revisionHistoryLimit = ::std::option::Option::Some(v);
    }

    pub fn get_revisionHistoryLimit(&self) -> i32 {
        self.revisionHistoryLimit.unwrap_or(0)
    }

    fn get_revisionHistoryLimit_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.revisionHistoryLimit
    }

    fn mut_revisionHistoryLimit_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.revisionHistoryLimit
    }

    // optional bool paused = 7;

    pub fn clear_paused(&mut self) {
        self.paused = ::std::option::Option::None;
    }

    pub fn has_paused(&self) -> bool {
        self.paused.is_some()
    }

    // Param is passed by value, moved
    pub fn set_paused(&mut self, v: bool) {
        self.paused = ::std::option::Option::Some(v);
    }

    pub fn get_paused(&self) -> bool {
        self.paused.unwrap_or(false)
    }

    fn get_paused_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.paused
    }

    fn mut_paused_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.paused
    }

    // optional .k8s.io.api.apps.v1beta1.RollbackConfig rollbackTo = 8;

    pub fn clear_rollbackTo(&mut self) {
        self.rollbackTo.clear();
    }

    pub fn has_rollbackTo(&self) -> bool {
        self.rollbackTo.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rollbackTo(&mut self, v: RollbackConfig) {
        self.rollbackTo = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rollbackTo(&mut self) -> &mut RollbackConfig {
        if self.rollbackTo.is_none() {
            self.rollbackTo.set_default();
        }
        self.rollbackTo.as_mut().unwrap()
    }

    // Take field
    pub fn take_rollbackTo(&mut self) -> RollbackConfig {
        self.rollbackTo.take().unwrap_or_else(|| RollbackConfig::new())
    }

    pub fn get_rollbackTo(&self) -> &RollbackConfig {
        self.rollbackTo.as_ref().unwrap_or_else(|| RollbackConfig::default_instance())
    }

    fn get_rollbackTo_for_reflect(&self) -> &::protobuf::SingularPtrField<RollbackConfig> {
        &self.rollbackTo
    }

    fn mut_rollbackTo_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RollbackConfig> {
        &mut self.rollbackTo
    }

    // optional int32 progressDeadlineSeconds = 9;

    pub fn clear_progressDeadlineSeconds(&mut self) {
        self.progressDeadlineSeconds = ::std::option::Option::None;
    }

    pub fn has_progressDeadlineSeconds(&self) -> bool {
        self.progressDeadlineSeconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_progressDeadlineSeconds(&mut self, v: i32) {
        self.progressDeadlineSeconds = ::std::option::Option::Some(v);
    }

    pub fn get_progressDeadlineSeconds(&self) -> i32 {
        self.progressDeadlineSeconds.unwrap_or(0)
    }

    fn get_progressDeadlineSeconds_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.progressDeadlineSeconds
    }

    fn mut_progressDeadlineSeconds_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.progressDeadlineSeconds
    }
}

impl ::protobuf::Message for DeploymentSpec {
    fn is_initialized(&self) -> bool {
        for v in &self.selector {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.template {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.strategy {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.rollbackTo {
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
                    let tmp = is.read_int32()?;
                    self.replicas = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.selector)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.template)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.strategy)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.minReadySeconds = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.revisionHistoryLimit = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.paused = ::std::option::Option::Some(tmp);
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.rollbackTo)?;
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.progressDeadlineSeconds = ::std::option::Option::Some(tmp);
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
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.template.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.strategy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.minReadySeconds {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.revisionHistoryLimit {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.paused {
            my_size += 2;
        }
        if let Some(ref v) = self.rollbackTo.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.progressDeadlineSeconds {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
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
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.template.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.strategy.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.minReadySeconds {
            os.write_int32(5, v)?;
        }
        if let Some(v) = self.revisionHistoryLimit {
            os.write_int32(6, v)?;
        }
        if let Some(v) = self.paused {
            os.write_bool(7, v)?;
        }
        if let Some(ref v) = self.rollbackTo.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.progressDeadlineSeconds {
            os.write_int32(9, v)?;
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

impl ::protobuf::MessageStatic for DeploymentSpec {
    fn new() -> DeploymentSpec {
        DeploymentSpec::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeploymentSpec>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "replicas",
                    DeploymentSpec::get_replicas_for_reflect,
                    DeploymentSpec::mut_replicas_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::LabelSelector>>(
                    "selector",
                    DeploymentSpec::get_selector_for_reflect,
                    DeploymentSpec::mut_selector_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::PodTemplateSpec>>(
                    "template",
                    DeploymentSpec::get_template_for_reflect,
                    DeploymentSpec::mut_template_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DeploymentStrategy>>(
                    "strategy",
                    DeploymentSpec::get_strategy_for_reflect,
                    DeploymentSpec::mut_strategy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "minReadySeconds",
                    DeploymentSpec::get_minReadySeconds_for_reflect,
                    DeploymentSpec::mut_minReadySeconds_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "revisionHistoryLimit",
                    DeploymentSpec::get_revisionHistoryLimit_for_reflect,
                    DeploymentSpec::mut_revisionHistoryLimit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "paused",
                    DeploymentSpec::get_paused_for_reflect,
                    DeploymentSpec::mut_paused_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RollbackConfig>>(
                    "rollbackTo",
                    DeploymentSpec::get_rollbackTo_for_reflect,
                    DeploymentSpec::mut_rollbackTo_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "progressDeadlineSeconds",
                    DeploymentSpec::get_progressDeadlineSeconds_for_reflect,
                    DeploymentSpec::mut_progressDeadlineSeconds_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeploymentSpec>(
                    "DeploymentSpec",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeploymentSpec {
    fn clear(&mut self) {
        self.clear_replicas();
        self.clear_selector();
        self.clear_template();
        self.clear_strategy();
        self.clear_minReadySeconds();
        self.clear_revisionHistoryLimit();
        self.clear_paused();
        self.clear_rollbackTo();
        self.clear_progressDeadlineSeconds();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeploymentSpec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeploymentSpec {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeploymentStatus {
    // message fields
    observedGeneration: ::std::option::Option<i64>,
    replicas: ::std::option::Option<i32>,
    updatedReplicas: ::std::option::Option<i32>,
    readyReplicas: ::std::option::Option<i32>,
    availableReplicas: ::std::option::Option<i32>,
    unavailableReplicas: ::std::option::Option<i32>,
    conditions: ::protobuf::RepeatedField<DeploymentCondition>,
    collisionCount: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeploymentStatus {}

impl DeploymentStatus {
    pub fn new() -> DeploymentStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeploymentStatus {
        static mut instance: ::protobuf::lazy::Lazy<DeploymentStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeploymentStatus,
        };
        unsafe {
            instance.get(DeploymentStatus::new)
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

    // optional int32 replicas = 2;

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

    // optional int32 updatedReplicas = 3;

    pub fn clear_updatedReplicas(&mut self) {
        self.updatedReplicas = ::std::option::Option::None;
    }

    pub fn has_updatedReplicas(&self) -> bool {
        self.updatedReplicas.is_some()
    }

    // Param is passed by value, moved
    pub fn set_updatedReplicas(&mut self, v: i32) {
        self.updatedReplicas = ::std::option::Option::Some(v);
    }

    pub fn get_updatedReplicas(&self) -> i32 {
        self.updatedReplicas.unwrap_or(0)
    }

    fn get_updatedReplicas_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.updatedReplicas
    }

    fn mut_updatedReplicas_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.updatedReplicas
    }

    // optional int32 readyReplicas = 7;

    pub fn clear_readyReplicas(&mut self) {
        self.readyReplicas = ::std::option::Option::None;
    }

    pub fn has_readyReplicas(&self) -> bool {
        self.readyReplicas.is_some()
    }

    // Param is passed by value, moved
    pub fn set_readyReplicas(&mut self, v: i32) {
        self.readyReplicas = ::std::option::Option::Some(v);
    }

    pub fn get_readyReplicas(&self) -> i32 {
        self.readyReplicas.unwrap_or(0)
    }

    fn get_readyReplicas_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.readyReplicas
    }

    fn mut_readyReplicas_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.readyReplicas
    }

    // optional int32 availableReplicas = 4;

    pub fn clear_availableReplicas(&mut self) {
        self.availableReplicas = ::std::option::Option::None;
    }

    pub fn has_availableReplicas(&self) -> bool {
        self.availableReplicas.is_some()
    }

    // Param is passed by value, moved
    pub fn set_availableReplicas(&mut self, v: i32) {
        self.availableReplicas = ::std::option::Option::Some(v);
    }

    pub fn get_availableReplicas(&self) -> i32 {
        self.availableReplicas.unwrap_or(0)
    }

    fn get_availableReplicas_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.availableReplicas
    }

    fn mut_availableReplicas_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.availableReplicas
    }

    // optional int32 unavailableReplicas = 5;

    pub fn clear_unavailableReplicas(&mut self) {
        self.unavailableReplicas = ::std::option::Option::None;
    }

    pub fn has_unavailableReplicas(&self) -> bool {
        self.unavailableReplicas.is_some()
    }

    // Param is passed by value, moved
    pub fn set_unavailableReplicas(&mut self, v: i32) {
        self.unavailableReplicas = ::std::option::Option::Some(v);
    }

    pub fn get_unavailableReplicas(&self) -> i32 {
        self.unavailableReplicas.unwrap_or(0)
    }

    fn get_unavailableReplicas_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.unavailableReplicas
    }

    fn mut_unavailableReplicas_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.unavailableReplicas
    }

    // repeated .k8s.io.api.apps.v1beta1.DeploymentCondition conditions = 6;

    pub fn clear_conditions(&mut self) {
        self.conditions.clear();
    }

    // Param is passed by value, moved
    pub fn set_conditions(&mut self, v: ::protobuf::RepeatedField<DeploymentCondition>) {
        self.conditions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_conditions(&mut self) -> &mut ::protobuf::RepeatedField<DeploymentCondition> {
        &mut self.conditions
    }

    // Take field
    pub fn take_conditions(&mut self) -> ::protobuf::RepeatedField<DeploymentCondition> {
        ::std::mem::replace(&mut self.conditions, ::protobuf::RepeatedField::new())
    }

    pub fn get_conditions(&self) -> &[DeploymentCondition] {
        &self.conditions
    }

    fn get_conditions_for_reflect(&self) -> &::protobuf::RepeatedField<DeploymentCondition> {
        &self.conditions
    }

    fn mut_conditions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<DeploymentCondition> {
        &mut self.conditions
    }

    // optional int64 collisionCount = 8;

    pub fn clear_collisionCount(&mut self) {
        self.collisionCount = ::std::option::Option::None;
    }

    pub fn has_collisionCount(&self) -> bool {
        self.collisionCount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_collisionCount(&mut self, v: i64) {
        self.collisionCount = ::std::option::Option::Some(v);
    }

    pub fn get_collisionCount(&self) -> i64 {
        self.collisionCount.unwrap_or(0)
    }

    fn get_collisionCount_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.collisionCount
    }

    fn mut_collisionCount_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.collisionCount
    }
}

impl ::protobuf::Message for DeploymentStatus {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.observedGeneration = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.replicas = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.updatedReplicas = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.readyReplicas = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.availableReplicas = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.unavailableReplicas = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.conditions)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.collisionCount = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.replicas {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.updatedReplicas {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.readyReplicas {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.availableReplicas {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.unavailableReplicas {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.conditions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.collisionCount {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.observedGeneration {
            os.write_int64(1, v)?;
        }
        if let Some(v) = self.replicas {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.updatedReplicas {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.readyReplicas {
            os.write_int32(7, v)?;
        }
        if let Some(v) = self.availableReplicas {
            os.write_int32(4, v)?;
        }
        if let Some(v) = self.unavailableReplicas {
            os.write_int32(5, v)?;
        }
        for v in &self.conditions {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.collisionCount {
            os.write_int64(8, v)?;
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

impl ::protobuf::MessageStatic for DeploymentStatus {
    fn new() -> DeploymentStatus {
        DeploymentStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeploymentStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "observedGeneration",
                    DeploymentStatus::get_observedGeneration_for_reflect,
                    DeploymentStatus::mut_observedGeneration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "replicas",
                    DeploymentStatus::get_replicas_for_reflect,
                    DeploymentStatus::mut_replicas_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "updatedReplicas",
                    DeploymentStatus::get_updatedReplicas_for_reflect,
                    DeploymentStatus::mut_updatedReplicas_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "readyReplicas",
                    DeploymentStatus::get_readyReplicas_for_reflect,
                    DeploymentStatus::mut_readyReplicas_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "availableReplicas",
                    DeploymentStatus::get_availableReplicas_for_reflect,
                    DeploymentStatus::mut_availableReplicas_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "unavailableReplicas",
                    DeploymentStatus::get_unavailableReplicas_for_reflect,
                    DeploymentStatus::mut_unavailableReplicas_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DeploymentCondition>>(
                    "conditions",
                    DeploymentStatus::get_conditions_for_reflect,
                    DeploymentStatus::mut_conditions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "collisionCount",
                    DeploymentStatus::get_collisionCount_for_reflect,
                    DeploymentStatus::mut_collisionCount_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeploymentStatus>(
                    "DeploymentStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeploymentStatus {
    fn clear(&mut self) {
        self.clear_observedGeneration();
        self.clear_replicas();
        self.clear_updatedReplicas();
        self.clear_readyReplicas();
        self.clear_availableReplicas();
        self.clear_unavailableReplicas();
        self.clear_conditions();
        self.clear_collisionCount();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeploymentStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeploymentStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeploymentStrategy {
    // message fields
    field_type: ::protobuf::SingularField<::std::string::String>,
    rollingUpdate: ::protobuf::SingularPtrField<RollingUpdateDeployment>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeploymentStrategy {}

impl DeploymentStrategy {
    pub fn new() -> DeploymentStrategy {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeploymentStrategy {
        static mut instance: ::protobuf::lazy::Lazy<DeploymentStrategy> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeploymentStrategy,
        };
        unsafe {
            instance.get(DeploymentStrategy::new)
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

    // optional .k8s.io.api.apps.v1beta1.RollingUpdateDeployment rollingUpdate = 2;

    pub fn clear_rollingUpdate(&mut self) {
        self.rollingUpdate.clear();
    }

    pub fn has_rollingUpdate(&self) -> bool {
        self.rollingUpdate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rollingUpdate(&mut self, v: RollingUpdateDeployment) {
        self.rollingUpdate = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rollingUpdate(&mut self) -> &mut RollingUpdateDeployment {
        if self.rollingUpdate.is_none() {
            self.rollingUpdate.set_default();
        }
        self.rollingUpdate.as_mut().unwrap()
    }

    // Take field
    pub fn take_rollingUpdate(&mut self) -> RollingUpdateDeployment {
        self.rollingUpdate.take().unwrap_or_else(|| RollingUpdateDeployment::new())
    }

    pub fn get_rollingUpdate(&self) -> &RollingUpdateDeployment {
        self.rollingUpdate.as_ref().unwrap_or_else(|| RollingUpdateDeployment::default_instance())
    }

    fn get_rollingUpdate_for_reflect(&self) -> &::protobuf::SingularPtrField<RollingUpdateDeployment> {
        &self.rollingUpdate
    }

    fn mut_rollingUpdate_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RollingUpdateDeployment> {
        &mut self.rollingUpdate
    }
}

impl ::protobuf::Message for DeploymentStrategy {
    fn is_initialized(&self) -> bool {
        for v in &self.rollingUpdate {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.rollingUpdate)?;
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
        if let Some(ref v) = self.rollingUpdate.as_ref() {
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
        if let Some(ref v) = self.rollingUpdate.as_ref() {
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

impl ::protobuf::MessageStatic for DeploymentStrategy {
    fn new() -> DeploymentStrategy {
        DeploymentStrategy::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeploymentStrategy>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    DeploymentStrategy::get_field_type_for_reflect,
                    DeploymentStrategy::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RollingUpdateDeployment>>(
                    "rollingUpdate",
                    DeploymentStrategy::get_rollingUpdate_for_reflect,
                    DeploymentStrategy::mut_rollingUpdate_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeploymentStrategy>(
                    "DeploymentStrategy",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeploymentStrategy {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_rollingUpdate();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeploymentStrategy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeploymentStrategy {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RollbackConfig {
    // message fields
    revision: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RollbackConfig {}

impl RollbackConfig {
    pub fn new() -> RollbackConfig {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RollbackConfig {
        static mut instance: ::protobuf::lazy::Lazy<RollbackConfig> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RollbackConfig,
        };
        unsafe {
            instance.get(RollbackConfig::new)
        }
    }

    // optional int64 revision = 1;

    pub fn clear_revision(&mut self) {
        self.revision = ::std::option::Option::None;
    }

    pub fn has_revision(&self) -> bool {
        self.revision.is_some()
    }

    // Param is passed by value, moved
    pub fn set_revision(&mut self, v: i64) {
        self.revision = ::std::option::Option::Some(v);
    }

    pub fn get_revision(&self) -> i64 {
        self.revision.unwrap_or(0)
    }

    fn get_revision_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.revision
    }

    fn mut_revision_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.revision
    }
}

impl ::protobuf::Message for RollbackConfig {
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
                    let tmp = is.read_int64()?;
                    self.revision = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.revision {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.revision {
            os.write_int64(1, v)?;
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

impl ::protobuf::MessageStatic for RollbackConfig {
    fn new() -> RollbackConfig {
        RollbackConfig::new()
    }

    fn descriptor_static(_: ::std::option::Option<RollbackConfig>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "revision",
                    RollbackConfig::get_revision_for_reflect,
                    RollbackConfig::mut_revision_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RollbackConfig>(
                    "RollbackConfig",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RollbackConfig {
    fn clear(&mut self) {
        self.clear_revision();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RollbackConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RollbackConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RollingUpdateDeployment {
    // message fields
    maxUnavailable: ::protobuf::SingularPtrField<super::generated::IntOrString>,
    maxSurge: ::protobuf::SingularPtrField<super::generated::IntOrString>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RollingUpdateDeployment {}

impl RollingUpdateDeployment {
    pub fn new() -> RollingUpdateDeployment {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RollingUpdateDeployment {
        static mut instance: ::protobuf::lazy::Lazy<RollingUpdateDeployment> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RollingUpdateDeployment,
        };
        unsafe {
            instance.get(RollingUpdateDeployment::new)
        }
    }

    // optional .k8s.io.apimachinery.pkg.util.intstr.IntOrString maxUnavailable = 1;

    pub fn clear_maxUnavailable(&mut self) {
        self.maxUnavailable.clear();
    }

    pub fn has_maxUnavailable(&self) -> bool {
        self.maxUnavailable.is_some()
    }

    // Param is passed by value, moved
    pub fn set_maxUnavailable(&mut self, v: super::generated::IntOrString) {
        self.maxUnavailable = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_maxUnavailable(&mut self) -> &mut super::generated::IntOrString {
        if self.maxUnavailable.is_none() {
            self.maxUnavailable.set_default();
        }
        self.maxUnavailable.as_mut().unwrap()
    }

    // Take field
    pub fn take_maxUnavailable(&mut self) -> super::generated::IntOrString {
        self.maxUnavailable.take().unwrap_or_else(|| super::generated::IntOrString::new())
    }

    pub fn get_maxUnavailable(&self) -> &super::generated::IntOrString {
        self.maxUnavailable.as_ref().unwrap_or_else(|| super::generated::IntOrString::default_instance())
    }

    fn get_maxUnavailable_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::IntOrString> {
        &self.maxUnavailable
    }

    fn mut_maxUnavailable_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::IntOrString> {
        &mut self.maxUnavailable
    }

    // optional .k8s.io.apimachinery.pkg.util.intstr.IntOrString maxSurge = 2;

    pub fn clear_maxSurge(&mut self) {
        self.maxSurge.clear();
    }

    pub fn has_maxSurge(&self) -> bool {
        self.maxSurge.is_some()
    }

    // Param is passed by value, moved
    pub fn set_maxSurge(&mut self, v: super::generated::IntOrString) {
        self.maxSurge = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_maxSurge(&mut self) -> &mut super::generated::IntOrString {
        if self.maxSurge.is_none() {
            self.maxSurge.set_default();
        }
        self.maxSurge.as_mut().unwrap()
    }

    // Take field
    pub fn take_maxSurge(&mut self) -> super::generated::IntOrString {
        self.maxSurge.take().unwrap_or_else(|| super::generated::IntOrString::new())
    }

    pub fn get_maxSurge(&self) -> &super::generated::IntOrString {
        self.maxSurge.as_ref().unwrap_or_else(|| super::generated::IntOrString::default_instance())
    }

    fn get_maxSurge_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::IntOrString> {
        &self.maxSurge
    }

    fn mut_maxSurge_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::IntOrString> {
        &mut self.maxSurge
    }
}

impl ::protobuf::Message for RollingUpdateDeployment {
    fn is_initialized(&self) -> bool {
        for v in &self.maxUnavailable {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.maxSurge {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.maxUnavailable)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.maxSurge)?;
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
        if let Some(ref v) = self.maxUnavailable.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.maxSurge.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.maxUnavailable.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.maxSurge.as_ref() {
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

impl ::protobuf::MessageStatic for RollingUpdateDeployment {
    fn new() -> RollingUpdateDeployment {
        RollingUpdateDeployment::new()
    }

    fn descriptor_static(_: ::std::option::Option<RollingUpdateDeployment>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::IntOrString>>(
                    "maxUnavailable",
                    RollingUpdateDeployment::get_maxUnavailable_for_reflect,
                    RollingUpdateDeployment::mut_maxUnavailable_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::IntOrString>>(
                    "maxSurge",
                    RollingUpdateDeployment::get_maxSurge_for_reflect,
                    RollingUpdateDeployment::mut_maxSurge_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RollingUpdateDeployment>(
                    "RollingUpdateDeployment",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RollingUpdateDeployment {
    fn clear(&mut self) {
        self.clear_maxUnavailable();
        self.clear_maxSurge();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RollingUpdateDeployment {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RollingUpdateDeployment {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RollingUpdateStatefulSetStrategy {
    // message fields
    partition: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RollingUpdateStatefulSetStrategy {}

impl RollingUpdateStatefulSetStrategy {
    pub fn new() -> RollingUpdateStatefulSetStrategy {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RollingUpdateStatefulSetStrategy {
        static mut instance: ::protobuf::lazy::Lazy<RollingUpdateStatefulSetStrategy> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RollingUpdateStatefulSetStrategy,
        };
        unsafe {
            instance.get(RollingUpdateStatefulSetStrategy::new)
        }
    }

    // optional int32 partition = 1;

    pub fn clear_partition(&mut self) {
        self.partition = ::std::option::Option::None;
    }

    pub fn has_partition(&self) -> bool {
        self.partition.is_some()
    }

    // Param is passed by value, moved
    pub fn set_partition(&mut self, v: i32) {
        self.partition = ::std::option::Option::Some(v);
    }

    pub fn get_partition(&self) -> i32 {
        self.partition.unwrap_or(0)
    }

    fn get_partition_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.partition
    }

    fn mut_partition_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.partition
    }
}

impl ::protobuf::Message for RollingUpdateStatefulSetStrategy {
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
                    self.partition = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.partition {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.partition {
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

impl ::protobuf::MessageStatic for RollingUpdateStatefulSetStrategy {
    fn new() -> RollingUpdateStatefulSetStrategy {
        RollingUpdateStatefulSetStrategy::new()
    }

    fn descriptor_static(_: ::std::option::Option<RollingUpdateStatefulSetStrategy>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "partition",
                    RollingUpdateStatefulSetStrategy::get_partition_for_reflect,
                    RollingUpdateStatefulSetStrategy::mut_partition_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RollingUpdateStatefulSetStrategy>(
                    "RollingUpdateStatefulSetStrategy",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RollingUpdateStatefulSetStrategy {
    fn clear(&mut self) {
        self.clear_partition();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RollingUpdateStatefulSetStrategy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RollingUpdateStatefulSetStrategy {
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

    // optional .k8s.io.api.apps.v1beta1.ScaleSpec spec = 2;

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

    // optional .k8s.io.api.apps.v1beta1.ScaleStatus status = 3;

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
    pub selector: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    targetSelector: ::protobuf::SingularField<::std::string::String>,
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

    // repeated .k8s.io.api.apps.v1beta1.ScaleStatus.SelectorEntry selector = 2;

    pub fn clear_selector(&mut self) {
        self.selector.clear();
    }

    // Param is passed by value, moved
    pub fn set_selector(&mut self, v: ::std::collections::HashMap<::std::string::String, ::std::string::String>) {
        self.selector = v;
    }

    // Mutable pointer to the field.
    pub fn mut_selector(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.selector
    }

    // Take field
    pub fn take_selector(&mut self) -> ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        ::std::mem::replace(&mut self.selector, ::std::collections::HashMap::new())
    }

    pub fn get_selector(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.selector
    }

    fn get_selector_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.selector
    }

    fn mut_selector_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.selector
    }

    // optional string targetSelector = 3;

    pub fn clear_targetSelector(&mut self) {
        self.targetSelector.clear();
    }

    pub fn has_targetSelector(&self) -> bool {
        self.targetSelector.is_some()
    }

    // Param is passed by value, moved
    pub fn set_targetSelector(&mut self, v: ::std::string::String) {
        self.targetSelector = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_targetSelector(&mut self) -> &mut ::std::string::String {
        if self.targetSelector.is_none() {
            self.targetSelector.set_default();
        }
        self.targetSelector.as_mut().unwrap()
    }

    // Take field
    pub fn take_targetSelector(&mut self) -> ::std::string::String {
        self.targetSelector.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_targetSelector(&self) -> &str {
        match self.targetSelector.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_targetSelector_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.targetSelector
    }

    fn mut_targetSelector_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.targetSelector
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
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(wire_type, is, &mut self.selector)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.targetSelector)?;
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
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(2, &self.selector);
        if let Some(ref v) = self.targetSelector.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.replicas {
            os.write_int32(1, v)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(2, &self.selector, os)?;
        if let Some(ref v) = self.targetSelector.as_ref() {
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
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(
                    "selector",
                    ScaleStatus::get_selector_for_reflect,
                    ScaleStatus::mut_selector_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "targetSelector",
                    ScaleStatus::get_targetSelector_for_reflect,
                    ScaleStatus::mut_targetSelector_for_reflect,
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
        self.clear_targetSelector();
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

#[derive(PartialEq,Clone,Default)]
pub struct StatefulSet {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    spec: ::protobuf::SingularPtrField<StatefulSetSpec>,
    status: ::protobuf::SingularPtrField<StatefulSetStatus>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StatefulSet {}

impl StatefulSet {
    pub fn new() -> StatefulSet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StatefulSet {
        static mut instance: ::protobuf::lazy::Lazy<StatefulSet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StatefulSet,
        };
        unsafe {
            instance.get(StatefulSet::new)
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

    // optional .k8s.io.api.apps.v1beta1.StatefulSetSpec spec = 2;

    pub fn clear_spec(&mut self) {
        self.spec.clear();
    }

    pub fn has_spec(&self) -> bool {
        self.spec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spec(&mut self, v: StatefulSetSpec) {
        self.spec = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_spec(&mut self) -> &mut StatefulSetSpec {
        if self.spec.is_none() {
            self.spec.set_default();
        }
        self.spec.as_mut().unwrap()
    }

    // Take field
    pub fn take_spec(&mut self) -> StatefulSetSpec {
        self.spec.take().unwrap_or_else(|| StatefulSetSpec::new())
    }

    pub fn get_spec(&self) -> &StatefulSetSpec {
        self.spec.as_ref().unwrap_or_else(|| StatefulSetSpec::default_instance())
    }

    fn get_spec_for_reflect(&self) -> &::protobuf::SingularPtrField<StatefulSetSpec> {
        &self.spec
    }

    fn mut_spec_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<StatefulSetSpec> {
        &mut self.spec
    }

    // optional .k8s.io.api.apps.v1beta1.StatefulSetStatus status = 3;

    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: StatefulSetStatus) {
        self.status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut StatefulSetStatus {
        if self.status.is_none() {
            self.status.set_default();
        }
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> StatefulSetStatus {
        self.status.take().unwrap_or_else(|| StatefulSetStatus::new())
    }

    pub fn get_status(&self) -> &StatefulSetStatus {
        self.status.as_ref().unwrap_or_else(|| StatefulSetStatus::default_instance())
    }

    fn get_status_for_reflect(&self) -> &::protobuf::SingularPtrField<StatefulSetStatus> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<StatefulSetStatus> {
        &mut self.status
    }
}

impl ::protobuf::Message for StatefulSet {
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

impl ::protobuf::MessageStatic for StatefulSet {
    fn new() -> StatefulSet {
        StatefulSet::new()
    }

    fn descriptor_static(_: ::std::option::Option<StatefulSet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    StatefulSet::get_metadata_for_reflect,
                    StatefulSet::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StatefulSetSpec>>(
                    "spec",
                    StatefulSet::get_spec_for_reflect,
                    StatefulSet::mut_spec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StatefulSetStatus>>(
                    "status",
                    StatefulSet::get_status_for_reflect,
                    StatefulSet::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StatefulSet>(
                    "StatefulSet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StatefulSet {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_spec();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StatefulSet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StatefulSet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StatefulSetList {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ListMeta>,
    items: ::protobuf::RepeatedField<StatefulSet>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StatefulSetList {}

impl StatefulSetList {
    pub fn new() -> StatefulSetList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StatefulSetList {
        static mut instance: ::protobuf::lazy::Lazy<StatefulSetList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StatefulSetList,
        };
        unsafe {
            instance.get(StatefulSetList::new)
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

    // repeated .k8s.io.api.apps.v1beta1.StatefulSet items = 2;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<StatefulSet>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<StatefulSet> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<StatefulSet> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[StatefulSet] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<StatefulSet> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<StatefulSet> {
        &mut self.items
    }
}

impl ::protobuf::Message for StatefulSetList {
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

impl ::protobuf::MessageStatic for StatefulSetList {
    fn new() -> StatefulSetList {
        StatefulSetList::new()
    }

    fn descriptor_static(_: ::std::option::Option<StatefulSetList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ListMeta>>(
                    "metadata",
                    StatefulSetList::get_metadata_for_reflect,
                    StatefulSetList::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StatefulSet>>(
                    "items",
                    StatefulSetList::get_items_for_reflect,
                    StatefulSetList::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StatefulSetList>(
                    "StatefulSetList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StatefulSetList {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StatefulSetList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StatefulSetList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StatefulSetSpec {
    // message fields
    replicas: ::std::option::Option<i32>,
    selector: ::protobuf::SingularPtrField<super::generated::LabelSelector>,
    template: ::protobuf::SingularPtrField<super::generated::PodTemplateSpec>,
    volumeClaimTemplates: ::protobuf::RepeatedField<super::generated::PersistentVolumeClaim>,
    serviceName: ::protobuf::SingularField<::std::string::String>,
    podManagementPolicy: ::protobuf::SingularField<::std::string::String>,
    updateStrategy: ::protobuf::SingularPtrField<StatefulSetUpdateStrategy>,
    revisionHistoryLimit: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StatefulSetSpec {}

impl StatefulSetSpec {
    pub fn new() -> StatefulSetSpec {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StatefulSetSpec {
        static mut instance: ::protobuf::lazy::Lazy<StatefulSetSpec> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StatefulSetSpec,
        };
        unsafe {
            instance.get(StatefulSetSpec::new)
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

    // optional .k8s.io.apimachinery.pkg.apis.meta.v1.LabelSelector selector = 2;

    pub fn clear_selector(&mut self) {
        self.selector.clear();
    }

    pub fn has_selector(&self) -> bool {
        self.selector.is_some()
    }

    // Param is passed by value, moved
    pub fn set_selector(&mut self, v: super::generated::LabelSelector) {
        self.selector = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_selector(&mut self) -> &mut super::generated::LabelSelector {
        if self.selector.is_none() {
            self.selector.set_default();
        }
        self.selector.as_mut().unwrap()
    }

    // Take field
    pub fn take_selector(&mut self) -> super::generated::LabelSelector {
        self.selector.take().unwrap_or_else(|| super::generated::LabelSelector::new())
    }

    pub fn get_selector(&self) -> &super::generated::LabelSelector {
        self.selector.as_ref().unwrap_or_else(|| super::generated::LabelSelector::default_instance())
    }

    fn get_selector_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::LabelSelector> {
        &self.selector
    }

    fn mut_selector_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::LabelSelector> {
        &mut self.selector
    }

    // optional .k8s.io.api.core.v1.PodTemplateSpec template = 3;

    pub fn clear_template(&mut self) {
        self.template.clear();
    }

    pub fn has_template(&self) -> bool {
        self.template.is_some()
    }

    // Param is passed by value, moved
    pub fn set_template(&mut self, v: super::generated::PodTemplateSpec) {
        self.template = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_template(&mut self) -> &mut super::generated::PodTemplateSpec {
        if self.template.is_none() {
            self.template.set_default();
        }
        self.template.as_mut().unwrap()
    }

    // Take field
    pub fn take_template(&mut self) -> super::generated::PodTemplateSpec {
        self.template.take().unwrap_or_else(|| super::generated::PodTemplateSpec::new())
    }

    pub fn get_template(&self) -> &super::generated::PodTemplateSpec {
        self.template.as_ref().unwrap_or_else(|| super::generated::PodTemplateSpec::default_instance())
    }

    fn get_template_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::PodTemplateSpec> {
        &self.template
    }

    fn mut_template_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::PodTemplateSpec> {
        &mut self.template
    }

    // repeated .k8s.io.api.core.v1.PersistentVolumeClaim volumeClaimTemplates = 4;

    pub fn clear_volumeClaimTemplates(&mut self) {
        self.volumeClaimTemplates.clear();
    }

    // Param is passed by value, moved
    pub fn set_volumeClaimTemplates(&mut self, v: ::protobuf::RepeatedField<super::generated::PersistentVolumeClaim>) {
        self.volumeClaimTemplates = v;
    }

    // Mutable pointer to the field.
    pub fn mut_volumeClaimTemplates(&mut self) -> &mut ::protobuf::RepeatedField<super::generated::PersistentVolumeClaim> {
        &mut self.volumeClaimTemplates
    }

    // Take field
    pub fn take_volumeClaimTemplates(&mut self) -> ::protobuf::RepeatedField<super::generated::PersistentVolumeClaim> {
        ::std::mem::replace(&mut self.volumeClaimTemplates, ::protobuf::RepeatedField::new())
    }

    pub fn get_volumeClaimTemplates(&self) -> &[super::generated::PersistentVolumeClaim] {
        &self.volumeClaimTemplates
    }

    fn get_volumeClaimTemplates_for_reflect(&self) -> &::protobuf::RepeatedField<super::generated::PersistentVolumeClaim> {
        &self.volumeClaimTemplates
    }

    fn mut_volumeClaimTemplates_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::generated::PersistentVolumeClaim> {
        &mut self.volumeClaimTemplates
    }

    // optional string serviceName = 5;

    pub fn clear_serviceName(&mut self) {
        self.serviceName.clear();
    }

    pub fn has_serviceName(&self) -> bool {
        self.serviceName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_serviceName(&mut self, v: ::std::string::String) {
        self.serviceName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_serviceName(&mut self) -> &mut ::std::string::String {
        if self.serviceName.is_none() {
            self.serviceName.set_default();
        }
        self.serviceName.as_mut().unwrap()
    }

    // Take field
    pub fn take_serviceName(&mut self) -> ::std::string::String {
        self.serviceName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_serviceName(&self) -> &str {
        match self.serviceName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_serviceName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.serviceName
    }

    fn mut_serviceName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.serviceName
    }

    // optional string podManagementPolicy = 6;

    pub fn clear_podManagementPolicy(&mut self) {
        self.podManagementPolicy.clear();
    }

    pub fn has_podManagementPolicy(&self) -> bool {
        self.podManagementPolicy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_podManagementPolicy(&mut self, v: ::std::string::String) {
        self.podManagementPolicy = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_podManagementPolicy(&mut self) -> &mut ::std::string::String {
        if self.podManagementPolicy.is_none() {
            self.podManagementPolicy.set_default();
        }
        self.podManagementPolicy.as_mut().unwrap()
    }

    // Take field
    pub fn take_podManagementPolicy(&mut self) -> ::std::string::String {
        self.podManagementPolicy.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_podManagementPolicy(&self) -> &str {
        match self.podManagementPolicy.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_podManagementPolicy_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.podManagementPolicy
    }

    fn mut_podManagementPolicy_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.podManagementPolicy
    }

    // optional .k8s.io.api.apps.v1beta1.StatefulSetUpdateStrategy updateStrategy = 7;

    pub fn clear_updateStrategy(&mut self) {
        self.updateStrategy.clear();
    }

    pub fn has_updateStrategy(&self) -> bool {
        self.updateStrategy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_updateStrategy(&mut self, v: StatefulSetUpdateStrategy) {
        self.updateStrategy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_updateStrategy(&mut self) -> &mut StatefulSetUpdateStrategy {
        if self.updateStrategy.is_none() {
            self.updateStrategy.set_default();
        }
        self.updateStrategy.as_mut().unwrap()
    }

    // Take field
    pub fn take_updateStrategy(&mut self) -> StatefulSetUpdateStrategy {
        self.updateStrategy.take().unwrap_or_else(|| StatefulSetUpdateStrategy::new())
    }

    pub fn get_updateStrategy(&self) -> &StatefulSetUpdateStrategy {
        self.updateStrategy.as_ref().unwrap_or_else(|| StatefulSetUpdateStrategy::default_instance())
    }

    fn get_updateStrategy_for_reflect(&self) -> &::protobuf::SingularPtrField<StatefulSetUpdateStrategy> {
        &self.updateStrategy
    }

    fn mut_updateStrategy_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<StatefulSetUpdateStrategy> {
        &mut self.updateStrategy
    }

    // optional int32 revisionHistoryLimit = 8;

    pub fn clear_revisionHistoryLimit(&mut self) {
        self.revisionHistoryLimit = ::std::option::Option::None;
    }

    pub fn has_revisionHistoryLimit(&self) -> bool {
        self.revisionHistoryLimit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_revisionHistoryLimit(&mut self, v: i32) {
        self.revisionHistoryLimit = ::std::option::Option::Some(v);
    }

    pub fn get_revisionHistoryLimit(&self) -> i32 {
        self.revisionHistoryLimit.unwrap_or(0)
    }

    fn get_revisionHistoryLimit_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.revisionHistoryLimit
    }

    fn mut_revisionHistoryLimit_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.revisionHistoryLimit
    }
}

impl ::protobuf::Message for StatefulSetSpec {
    fn is_initialized(&self) -> bool {
        for v in &self.selector {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.template {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.volumeClaimTemplates {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.updateStrategy {
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
                    let tmp = is.read_int32()?;
                    self.replicas = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.selector)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.template)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.volumeClaimTemplates)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.serviceName)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.podManagementPolicy)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.updateStrategy)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.revisionHistoryLimit = ::std::option::Option::Some(tmp);
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
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.template.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.volumeClaimTemplates {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.serviceName.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(ref v) = self.podManagementPolicy.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        if let Some(ref v) = self.updateStrategy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.revisionHistoryLimit {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
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
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.template.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.volumeClaimTemplates {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.serviceName.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(ref v) = self.podManagementPolicy.as_ref() {
            os.write_string(6, &v)?;
        }
        if let Some(ref v) = self.updateStrategy.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.revisionHistoryLimit {
            os.write_int32(8, v)?;
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

impl ::protobuf::MessageStatic for StatefulSetSpec {
    fn new() -> StatefulSetSpec {
        StatefulSetSpec::new()
    }

    fn descriptor_static(_: ::std::option::Option<StatefulSetSpec>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "replicas",
                    StatefulSetSpec::get_replicas_for_reflect,
                    StatefulSetSpec::mut_replicas_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::LabelSelector>>(
                    "selector",
                    StatefulSetSpec::get_selector_for_reflect,
                    StatefulSetSpec::mut_selector_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::PodTemplateSpec>>(
                    "template",
                    StatefulSetSpec::get_template_for_reflect,
                    StatefulSetSpec::mut_template_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::PersistentVolumeClaim>>(
                    "volumeClaimTemplates",
                    StatefulSetSpec::get_volumeClaimTemplates_for_reflect,
                    StatefulSetSpec::mut_volumeClaimTemplates_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "serviceName",
                    StatefulSetSpec::get_serviceName_for_reflect,
                    StatefulSetSpec::mut_serviceName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "podManagementPolicy",
                    StatefulSetSpec::get_podManagementPolicy_for_reflect,
                    StatefulSetSpec::mut_podManagementPolicy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StatefulSetUpdateStrategy>>(
                    "updateStrategy",
                    StatefulSetSpec::get_updateStrategy_for_reflect,
                    StatefulSetSpec::mut_updateStrategy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "revisionHistoryLimit",
                    StatefulSetSpec::get_revisionHistoryLimit_for_reflect,
                    StatefulSetSpec::mut_revisionHistoryLimit_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StatefulSetSpec>(
                    "StatefulSetSpec",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StatefulSetSpec {
    fn clear(&mut self) {
        self.clear_replicas();
        self.clear_selector();
        self.clear_template();
        self.clear_volumeClaimTemplates();
        self.clear_serviceName();
        self.clear_podManagementPolicy();
        self.clear_updateStrategy();
        self.clear_revisionHistoryLimit();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StatefulSetSpec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StatefulSetSpec {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StatefulSetStatus {
    // message fields
    observedGeneration: ::std::option::Option<i64>,
    replicas: ::std::option::Option<i32>,
    readyReplicas: ::std::option::Option<i32>,
    currentReplicas: ::std::option::Option<i32>,
    updatedReplicas: ::std::option::Option<i32>,
    currentRevision: ::protobuf::SingularField<::std::string::String>,
    updateRevision: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StatefulSetStatus {}

impl StatefulSetStatus {
    pub fn new() -> StatefulSetStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StatefulSetStatus {
        static mut instance: ::protobuf::lazy::Lazy<StatefulSetStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StatefulSetStatus,
        };
        unsafe {
            instance.get(StatefulSetStatus::new)
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

    // optional int32 replicas = 2;

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

    // optional int32 readyReplicas = 3;

    pub fn clear_readyReplicas(&mut self) {
        self.readyReplicas = ::std::option::Option::None;
    }

    pub fn has_readyReplicas(&self) -> bool {
        self.readyReplicas.is_some()
    }

    // Param is passed by value, moved
    pub fn set_readyReplicas(&mut self, v: i32) {
        self.readyReplicas = ::std::option::Option::Some(v);
    }

    pub fn get_readyReplicas(&self) -> i32 {
        self.readyReplicas.unwrap_or(0)
    }

    fn get_readyReplicas_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.readyReplicas
    }

    fn mut_readyReplicas_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.readyReplicas
    }

    // optional int32 currentReplicas = 4;

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

    // optional int32 updatedReplicas = 5;

    pub fn clear_updatedReplicas(&mut self) {
        self.updatedReplicas = ::std::option::Option::None;
    }

    pub fn has_updatedReplicas(&self) -> bool {
        self.updatedReplicas.is_some()
    }

    // Param is passed by value, moved
    pub fn set_updatedReplicas(&mut self, v: i32) {
        self.updatedReplicas = ::std::option::Option::Some(v);
    }

    pub fn get_updatedReplicas(&self) -> i32 {
        self.updatedReplicas.unwrap_or(0)
    }

    fn get_updatedReplicas_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.updatedReplicas
    }

    fn mut_updatedReplicas_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.updatedReplicas
    }

    // optional string currentRevision = 6;

    pub fn clear_currentRevision(&mut self) {
        self.currentRevision.clear();
    }

    pub fn has_currentRevision(&self) -> bool {
        self.currentRevision.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currentRevision(&mut self, v: ::std::string::String) {
        self.currentRevision = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_currentRevision(&mut self) -> &mut ::std::string::String {
        if self.currentRevision.is_none() {
            self.currentRevision.set_default();
        }
        self.currentRevision.as_mut().unwrap()
    }

    // Take field
    pub fn take_currentRevision(&mut self) -> ::std::string::String {
        self.currentRevision.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_currentRevision(&self) -> &str {
        match self.currentRevision.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_currentRevision_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.currentRevision
    }

    fn mut_currentRevision_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.currentRevision
    }

    // optional string updateRevision = 7;

    pub fn clear_updateRevision(&mut self) {
        self.updateRevision.clear();
    }

    pub fn has_updateRevision(&self) -> bool {
        self.updateRevision.is_some()
    }

    // Param is passed by value, moved
    pub fn set_updateRevision(&mut self, v: ::std::string::String) {
        self.updateRevision = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_updateRevision(&mut self) -> &mut ::std::string::String {
        if self.updateRevision.is_none() {
            self.updateRevision.set_default();
        }
        self.updateRevision.as_mut().unwrap()
    }

    // Take field
    pub fn take_updateRevision(&mut self) -> ::std::string::String {
        self.updateRevision.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_updateRevision(&self) -> &str {
        match self.updateRevision.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_updateRevision_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.updateRevision
    }

    fn mut_updateRevision_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.updateRevision
    }
}

impl ::protobuf::Message for StatefulSetStatus {
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
                    let tmp = is.read_int64()?;
                    self.observedGeneration = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.replicas = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.readyReplicas = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.currentReplicas = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.updatedReplicas = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.currentRevision)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.updateRevision)?;
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
        if let Some(v) = self.replicas {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.readyReplicas {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.currentReplicas {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.updatedReplicas {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.currentRevision.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        if let Some(ref v) = self.updateRevision.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.observedGeneration {
            os.write_int64(1, v)?;
        }
        if let Some(v) = self.replicas {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.readyReplicas {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.currentReplicas {
            os.write_int32(4, v)?;
        }
        if let Some(v) = self.updatedReplicas {
            os.write_int32(5, v)?;
        }
        if let Some(ref v) = self.currentRevision.as_ref() {
            os.write_string(6, &v)?;
        }
        if let Some(ref v) = self.updateRevision.as_ref() {
            os.write_string(7, &v)?;
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

impl ::protobuf::MessageStatic for StatefulSetStatus {
    fn new() -> StatefulSetStatus {
        StatefulSetStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<StatefulSetStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "observedGeneration",
                    StatefulSetStatus::get_observedGeneration_for_reflect,
                    StatefulSetStatus::mut_observedGeneration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "replicas",
                    StatefulSetStatus::get_replicas_for_reflect,
                    StatefulSetStatus::mut_replicas_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "readyReplicas",
                    StatefulSetStatus::get_readyReplicas_for_reflect,
                    StatefulSetStatus::mut_readyReplicas_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "currentReplicas",
                    StatefulSetStatus::get_currentReplicas_for_reflect,
                    StatefulSetStatus::mut_currentReplicas_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "updatedReplicas",
                    StatefulSetStatus::get_updatedReplicas_for_reflect,
                    StatefulSetStatus::mut_updatedReplicas_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "currentRevision",
                    StatefulSetStatus::get_currentRevision_for_reflect,
                    StatefulSetStatus::mut_currentRevision_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "updateRevision",
                    StatefulSetStatus::get_updateRevision_for_reflect,
                    StatefulSetStatus::mut_updateRevision_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StatefulSetStatus>(
                    "StatefulSetStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StatefulSetStatus {
    fn clear(&mut self) {
        self.clear_observedGeneration();
        self.clear_replicas();
        self.clear_readyReplicas();
        self.clear_currentReplicas();
        self.clear_updatedReplicas();
        self.clear_currentRevision();
        self.clear_updateRevision();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StatefulSetStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StatefulSetStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StatefulSetUpdateStrategy {
    // message fields
    field_type: ::protobuf::SingularField<::std::string::String>,
    rollingUpdate: ::protobuf::SingularPtrField<RollingUpdateStatefulSetStrategy>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StatefulSetUpdateStrategy {}

impl StatefulSetUpdateStrategy {
    pub fn new() -> StatefulSetUpdateStrategy {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StatefulSetUpdateStrategy {
        static mut instance: ::protobuf::lazy::Lazy<StatefulSetUpdateStrategy> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StatefulSetUpdateStrategy,
        };
        unsafe {
            instance.get(StatefulSetUpdateStrategy::new)
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

    // optional .k8s.io.api.apps.v1beta1.RollingUpdateStatefulSetStrategy rollingUpdate = 2;

    pub fn clear_rollingUpdate(&mut self) {
        self.rollingUpdate.clear();
    }

    pub fn has_rollingUpdate(&self) -> bool {
        self.rollingUpdate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rollingUpdate(&mut self, v: RollingUpdateStatefulSetStrategy) {
        self.rollingUpdate = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rollingUpdate(&mut self) -> &mut RollingUpdateStatefulSetStrategy {
        if self.rollingUpdate.is_none() {
            self.rollingUpdate.set_default();
        }
        self.rollingUpdate.as_mut().unwrap()
    }

    // Take field
    pub fn take_rollingUpdate(&mut self) -> RollingUpdateStatefulSetStrategy {
        self.rollingUpdate.take().unwrap_or_else(|| RollingUpdateStatefulSetStrategy::new())
    }

    pub fn get_rollingUpdate(&self) -> &RollingUpdateStatefulSetStrategy {
        self.rollingUpdate.as_ref().unwrap_or_else(|| RollingUpdateStatefulSetStrategy::default_instance())
    }

    fn get_rollingUpdate_for_reflect(&self) -> &::protobuf::SingularPtrField<RollingUpdateStatefulSetStrategy> {
        &self.rollingUpdate
    }

    fn mut_rollingUpdate_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RollingUpdateStatefulSetStrategy> {
        &mut self.rollingUpdate
    }
}

impl ::protobuf::Message for StatefulSetUpdateStrategy {
    fn is_initialized(&self) -> bool {
        for v in &self.rollingUpdate {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.rollingUpdate)?;
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
        if let Some(ref v) = self.rollingUpdate.as_ref() {
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
        if let Some(ref v) = self.rollingUpdate.as_ref() {
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

impl ::protobuf::MessageStatic for StatefulSetUpdateStrategy {
    fn new() -> StatefulSetUpdateStrategy {
        StatefulSetUpdateStrategy::new()
    }

    fn descriptor_static(_: ::std::option::Option<StatefulSetUpdateStrategy>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    StatefulSetUpdateStrategy::get_field_type_for_reflect,
                    StatefulSetUpdateStrategy::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RollingUpdateStatefulSetStrategy>>(
                    "rollingUpdate",
                    StatefulSetUpdateStrategy::get_rollingUpdate_for_reflect,
                    StatefulSetUpdateStrategy::mut_rollingUpdate_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StatefulSetUpdateStrategy>(
                    "StatefulSetUpdateStrategy",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StatefulSetUpdateStrategy {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_rollingUpdate();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StatefulSetUpdateStrategy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StatefulSetUpdateStrategy {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'k8s.io/api/apps/v1beta1/generated.proto\x12\x17k8s.io.api.apps.v1beta\
    1\x1a\"k8s.io/api/core/v1/generated.proto\x1a)k8s.io/api/policy/v1beta1/\
    generated.proto\x1a4k8s.io/apimachinery/pkg/apis/meta/v1/generated.proto\
    \x1a/k8s.io/apimachinery/pkg/runtime/generated.proto\x1a6k8s.io/apimachi\
    nery/pkg/runtime/schema/generated.proto\x1a3k8s.io/apimachinery/pkg/util\
    /intstr/generated.proto\"\xc1\x01\n\x12ControllerRevision\x12L\n\x08meta\
    data\x18\x01\x20\x01(\x0b20.k8s.io.apimachinery.pkg.apis.meta.v1.ObjectM\
    etaR\x08metadata\x12A\n\x04data\x18\x02\x20\x01(\x0b2-.k8s.io.apimachine\
    ry.pkg.runtime.RawExtensionR\x04data\x12\x1a\n\x08revision\x18\x03\x20\
    \x01(\x03R\x08revision\"\xa7\x01\n\x16ControllerRevisionList\x12J\n\x08m\
    etadata\x18\x01\x20\x01(\x0b2..k8s.io.apimachinery.pkg.apis.meta.v1.List\
    MetaR\x08metadata\x12A\n\x05items\x18\x02\x20\x03(\x0b2+.k8s.io.api.apps\
    .v1beta1.ControllerRevisionR\x05items\"\xda\x01\n\nDeployment\x12L\n\x08\
    metadata\x18\x01\x20\x01(\x0b20.k8s.io.apimachinery.pkg.apis.meta.v1.Obj\
    ectMetaR\x08metadata\x12;\n\x04spec\x18\x02\x20\x01(\x0b2'.k8s.io.api.ap\
    ps.v1beta1.DeploymentSpecR\x04spec\x12A\n\x06status\x18\x03\x20\x01(\x0b\
    2).k8s.io.api.apps.v1beta1.DeploymentStatusR\x06status\"\xa3\x02\n\x13De\
    ploymentCondition\x12\x12\n\x04type\x18\x01\x20\x01(\tR\x04type\x12\x16\
    \n\x06status\x18\x02\x20\x01(\tR\x06status\x12R\n\x0elastUpdateTime\x18\
    \x06\x20\x01(\x0b2*.k8s.io.apimachinery.pkg.apis.meta.v1.TimeR\x0elastUp\
    dateTime\x12Z\n\x12lastTransitionTime\x18\x07\x20\x01(\x0b2*.k8s.io.apim\
    achinery.pkg.apis.meta.v1.TimeR\x12lastTransitionTime\x12\x16\n\x06reaso\
    n\x18\x04\x20\x01(\tR\x06reason\x12\x18\n\x07message\x18\x05\x20\x01(\tR\
    \x07message\"\x97\x01\n\x0eDeploymentList\x12J\n\x08metadata\x18\x01\x20\
    \x01(\x0b2..k8s.io.apimachinery.pkg.apis.meta.v1.ListMetaR\x08metadata\
    \x129\n\x05items\x18\x02\x20\x03(\x0b2#.k8s.io.api.apps.v1beta1.Deployme\
    ntR\x05items\"\xad\x02\n\x12DeploymentRollback\x12\x12\n\x04name\x18\x01\
    \x20\x01(\tR\x04name\x12s\n\x12updatedAnnotations\x18\x02\x20\x03(\x0b2C\
    .k8s.io.api.apps.v1beta1.DeploymentRollback.UpdatedAnnotationsEntryR\x12\
    updatedAnnotations\x12G\n\nrollbackTo\x18\x03\x20\x01(\x0b2'.k8s.io.api.\
    apps.v1beta1.RollbackConfigR\nrollbackTo\x1aE\n\x17UpdatedAnnotationsEnt\
    ry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05value\x18\
    \x02\x20\x01(\tR\x05value:\x028\x01\"\x80\x04\n\x0eDeploymentSpec\x12\
    \x1a\n\x08replicas\x18\x01\x20\x01(\x05R\x08replicas\x12O\n\x08selector\
    \x18\x02\x20\x01(\x0b23.k8s.io.apimachinery.pkg.apis.meta.v1.LabelSelect\
    orR\x08selector\x12?\n\x08template\x18\x03\x20\x01(\x0b2#.k8s.io.api.cor\
    e.v1.PodTemplateSpecR\x08template\x12G\n\x08strategy\x18\x04\x20\x01(\
    \x0b2+.k8s.io.api.apps.v1beta1.DeploymentStrategyR\x08strategy\x12(\n\
    \x0fminReadySeconds\x18\x05\x20\x01(\x05R\x0fminReadySeconds\x122\n\x14r\
    evisionHistoryLimit\x18\x06\x20\x01(\x05R\x14revisionHistoryLimit\x12\
    \x16\n\x06paused\x18\x07\x20\x01(\x08R\x06paused\x12G\n\nrollbackTo\x18\
    \x08\x20\x01(\x0b2'.k8s.io.api.apps.v1beta1.RollbackConfigR\nrollbackTo\
    \x128\n\x17progressDeadlineSeconds\x18\t\x20\x01(\x05R\x17progressDeadli\
    neSeconds\"\x84\x03\n\x10DeploymentStatus\x12.\n\x12observedGeneration\
    \x18\x01\x20\x01(\x03R\x12observedGeneration\x12\x1a\n\x08replicas\x18\
    \x02\x20\x01(\x05R\x08replicas\x12(\n\x0fupdatedReplicas\x18\x03\x20\x01\
    (\x05R\x0fupdatedReplicas\x12$\n\rreadyReplicas\x18\x07\x20\x01(\x05R\rr\
    eadyReplicas\x12,\n\x11availableReplicas\x18\x04\x20\x01(\x05R\x11availa\
    bleReplicas\x120\n\x13unavailableReplicas\x18\x05\x20\x01(\x05R\x13unava\
    ilableReplicas\x12L\n\nconditions\x18\x06\x20\x03(\x0b2,.k8s.io.api.apps\
    .v1beta1.DeploymentConditionR\nconditions\x12&\n\x0ecollisionCount\x18\
    \x08\x20\x01(\x03R\x0ecollisionCount\"\x80\x01\n\x12DeploymentStrategy\
    \x12\x12\n\x04type\x18\x01\x20\x01(\tR\x04type\x12V\n\rrollingUpdate\x18\
    \x02\x20\x01(\x0b20.k8s.io.api.apps.v1beta1.RollingUpdateDeploymentR\rro\
    llingUpdate\",\n\x0eRollbackConfig\x12\x1a\n\x08revision\x18\x01\x20\x01\
    (\x03R\x08revision\"\xc1\x01\n\x17RollingUpdateDeployment\x12X\n\x0emaxU\
    navailable\x18\x01\x20\x01(\x0b20.k8s.io.apimachinery.pkg.util.intstr.In\
    tOrStringR\x0emaxUnavailable\x12L\n\x08maxSurge\x18\x02\x20\x01(\x0b20.k\
    8s.io.apimachinery.pkg.util.intstr.IntOrStringR\x08maxSurge\"@\n\x20Roll\
    ingUpdateStatefulSetStrategy\x12\x1c\n\tpartition\x18\x01\x20\x01(\x05R\
    \tpartition\"\xcb\x01\n\x05Scale\x12L\n\x08metadata\x18\x01\x20\x01(\x0b\
    20.k8s.io.apimachinery.pkg.apis.meta.v1.ObjectMetaR\x08metadata\x126\n\
    \x04spec\x18\x02\x20\x01(\x0b2\".k8s.io.api.apps.v1beta1.ScaleSpecR\x04s\
    pec\x12<\n\x06status\x18\x03\x20\x01(\x0b2$.k8s.io.api.apps.v1beta1.Scal\
    eStatusR\x06status\"'\n\tScaleSpec\x12\x1a\n\x08replicas\x18\x01\x20\x01\
    (\x05R\x08replicas\"\xde\x01\n\x0bScaleStatus\x12\x1a\n\x08replicas\x18\
    \x01\x20\x01(\x05R\x08replicas\x12N\n\x08selector\x18\x02\x20\x03(\x0b22\
    .k8s.io.api.apps.v1beta1.ScaleStatus.SelectorEntryR\x08selector\x12&\n\
    \x0etargetSelector\x18\x03\x20\x01(\tR\x0etargetSelector\x1a;\n\rSelecto\
    rEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05value\
    \x18\x02\x20\x01(\tR\x05value:\x028\x01\"\xdd\x01\n\x0bStatefulSet\x12L\
    \n\x08metadata\x18\x01\x20\x01(\x0b20.k8s.io.apimachinery.pkg.apis.meta.\
    v1.ObjectMetaR\x08metadata\x12<\n\x04spec\x18\x02\x20\x01(\x0b2(.k8s.io.\
    api.apps.v1beta1.StatefulSetSpecR\x04spec\x12B\n\x06status\x18\x03\x20\
    \x01(\x0b2*.k8s.io.api.apps.v1beta1.StatefulSetStatusR\x06status\"\x99\
    \x01\n\x0fStatefulSetList\x12J\n\x08metadata\x18\x01\x20\x01(\x0b2..k8s.\
    io.apimachinery.pkg.apis.meta.v1.ListMetaR\x08metadata\x12:\n\x05items\
    \x18\x02\x20\x03(\x0b2$.k8s.io.api.apps.v1beta1.StatefulSetR\x05items\"\
    \x82\x04\n\x0fStatefulSetSpec\x12\x1a\n\x08replicas\x18\x01\x20\x01(\x05\
    R\x08replicas\x12O\n\x08selector\x18\x02\x20\x01(\x0b23.k8s.io.apimachin\
    ery.pkg.apis.meta.v1.LabelSelectorR\x08selector\x12?\n\x08template\x18\
    \x03\x20\x01(\x0b2#.k8s.io.api.core.v1.PodTemplateSpecR\x08template\x12]\
    \n\x14volumeClaimTemplates\x18\x04\x20\x03(\x0b2).k8s.io.api.core.v1.Per\
    sistentVolumeClaimR\x14volumeClaimTemplates\x12\x20\n\x0bserviceName\x18\
    \x05\x20\x01(\tR\x0bserviceName\x120\n\x13podManagementPolicy\x18\x06\
    \x20\x01(\tR\x13podManagementPolicy\x12Z\n\x0eupdateStrategy\x18\x07\x20\
    \x01(\x0b22.k8s.io.api.apps.v1beta1.StatefulSetUpdateStrategyR\x0eupdate\
    Strategy\x122\n\x14revisionHistoryLimit\x18\x08\x20\x01(\x05R\x14revisio\
    nHistoryLimit\"\xab\x02\n\x11StatefulSetStatus\x12.\n\x12observedGenerat\
    ion\x18\x01\x20\x01(\x03R\x12observedGeneration\x12\x1a\n\x08replicas\
    \x18\x02\x20\x01(\x05R\x08replicas\x12$\n\rreadyReplicas\x18\x03\x20\x01\
    (\x05R\rreadyReplicas\x12(\n\x0fcurrentReplicas\x18\x04\x20\x01(\x05R\
    \x0fcurrentReplicas\x12(\n\x0fupdatedReplicas\x18\x05\x20\x01(\x05R\x0fu\
    pdatedReplicas\x12(\n\x0fcurrentRevision\x18\x06\x20\x01(\tR\x0fcurrentR\
    evision\x12&\n\x0eupdateRevision\x18\x07\x20\x01(\tR\x0eupdateRevision\"\
    \x90\x01\n\x19StatefulSetUpdateStrategy\x12\x12\n\x04type\x18\x01\x20\
    \x01(\tR\x04type\x12_\n\rrollingUpdate\x18\x02\x20\x01(\x0b29.k8s.io.api\
    .apps.v1beta1.RollingUpdateStatefulSetStrategyR\rrollingUpdateB\tZ\x07v1\
    beta1\
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
