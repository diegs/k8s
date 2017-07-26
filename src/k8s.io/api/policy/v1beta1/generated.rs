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
pub struct Eviction {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    deleteOptions: ::protobuf::SingularPtrField<super::generated::DeleteOptions>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Eviction {}

impl Eviction {
    pub fn new() -> Eviction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Eviction {
        static mut instance: ::protobuf::lazy::Lazy<Eviction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Eviction,
        };
        unsafe {
            instance.get(Eviction::new)
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

    // optional .k8s.io.apimachinery.pkg.apis.meta.v1.DeleteOptions deleteOptions = 2;

    pub fn clear_deleteOptions(&mut self) {
        self.deleteOptions.clear();
    }

    pub fn has_deleteOptions(&self) -> bool {
        self.deleteOptions.is_some()
    }

    // Param is passed by value, moved
    pub fn set_deleteOptions(&mut self, v: super::generated::DeleteOptions) {
        self.deleteOptions = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_deleteOptions(&mut self) -> &mut super::generated::DeleteOptions {
        if self.deleteOptions.is_none() {
            self.deleteOptions.set_default();
        }
        self.deleteOptions.as_mut().unwrap()
    }

    // Take field
    pub fn take_deleteOptions(&mut self) -> super::generated::DeleteOptions {
        self.deleteOptions.take().unwrap_or_else(|| super::generated::DeleteOptions::new())
    }

    pub fn get_deleteOptions(&self) -> &super::generated::DeleteOptions {
        self.deleteOptions.as_ref().unwrap_or_else(|| super::generated::DeleteOptions::default_instance())
    }

    fn get_deleteOptions_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::DeleteOptions> {
        &self.deleteOptions
    }

    fn mut_deleteOptions_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::DeleteOptions> {
        &mut self.deleteOptions
    }
}

impl ::protobuf::Message for Eviction {
    fn is_initialized(&self) -> bool {
        for v in &self.metadata {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.deleteOptions {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.deleteOptions)?;
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
        if let Some(ref v) = self.deleteOptions.as_ref() {
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
        if let Some(ref v) = self.deleteOptions.as_ref() {
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

impl ::protobuf::MessageStatic for Eviction {
    fn new() -> Eviction {
        Eviction::new()
    }

    fn descriptor_static(_: ::std::option::Option<Eviction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    Eviction::get_metadata_for_reflect,
                    Eviction::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::DeleteOptions>>(
                    "deleteOptions",
                    Eviction::get_deleteOptions_for_reflect,
                    Eviction::mut_deleteOptions_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Eviction>(
                    "Eviction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Eviction {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_deleteOptions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Eviction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Eviction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PodDisruptionBudget {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    spec: ::protobuf::SingularPtrField<PodDisruptionBudgetSpec>,
    status: ::protobuf::SingularPtrField<PodDisruptionBudgetStatus>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PodDisruptionBudget {}

impl PodDisruptionBudget {
    pub fn new() -> PodDisruptionBudget {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PodDisruptionBudget {
        static mut instance: ::protobuf::lazy::Lazy<PodDisruptionBudget> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PodDisruptionBudget,
        };
        unsafe {
            instance.get(PodDisruptionBudget::new)
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

    // optional .k8s.io.api.policy.v1beta1.PodDisruptionBudgetSpec spec = 2;

    pub fn clear_spec(&mut self) {
        self.spec.clear();
    }

    pub fn has_spec(&self) -> bool {
        self.spec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spec(&mut self, v: PodDisruptionBudgetSpec) {
        self.spec = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_spec(&mut self) -> &mut PodDisruptionBudgetSpec {
        if self.spec.is_none() {
            self.spec.set_default();
        }
        self.spec.as_mut().unwrap()
    }

    // Take field
    pub fn take_spec(&mut self) -> PodDisruptionBudgetSpec {
        self.spec.take().unwrap_or_else(|| PodDisruptionBudgetSpec::new())
    }

    pub fn get_spec(&self) -> &PodDisruptionBudgetSpec {
        self.spec.as_ref().unwrap_or_else(|| PodDisruptionBudgetSpec::default_instance())
    }

    fn get_spec_for_reflect(&self) -> &::protobuf::SingularPtrField<PodDisruptionBudgetSpec> {
        &self.spec
    }

    fn mut_spec_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PodDisruptionBudgetSpec> {
        &mut self.spec
    }

    // optional .k8s.io.api.policy.v1beta1.PodDisruptionBudgetStatus status = 3;

    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: PodDisruptionBudgetStatus) {
        self.status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut PodDisruptionBudgetStatus {
        if self.status.is_none() {
            self.status.set_default();
        }
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> PodDisruptionBudgetStatus {
        self.status.take().unwrap_or_else(|| PodDisruptionBudgetStatus::new())
    }

    pub fn get_status(&self) -> &PodDisruptionBudgetStatus {
        self.status.as_ref().unwrap_or_else(|| PodDisruptionBudgetStatus::default_instance())
    }

    fn get_status_for_reflect(&self) -> &::protobuf::SingularPtrField<PodDisruptionBudgetStatus> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PodDisruptionBudgetStatus> {
        &mut self.status
    }
}

impl ::protobuf::Message for PodDisruptionBudget {
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

impl ::protobuf::MessageStatic for PodDisruptionBudget {
    fn new() -> PodDisruptionBudget {
        PodDisruptionBudget::new()
    }

    fn descriptor_static(_: ::std::option::Option<PodDisruptionBudget>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    PodDisruptionBudget::get_metadata_for_reflect,
                    PodDisruptionBudget::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PodDisruptionBudgetSpec>>(
                    "spec",
                    PodDisruptionBudget::get_spec_for_reflect,
                    PodDisruptionBudget::mut_spec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PodDisruptionBudgetStatus>>(
                    "status",
                    PodDisruptionBudget::get_status_for_reflect,
                    PodDisruptionBudget::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PodDisruptionBudget>(
                    "PodDisruptionBudget",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PodDisruptionBudget {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_spec();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PodDisruptionBudget {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PodDisruptionBudget {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PodDisruptionBudgetList {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ListMeta>,
    items: ::protobuf::RepeatedField<PodDisruptionBudget>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PodDisruptionBudgetList {}

impl PodDisruptionBudgetList {
    pub fn new() -> PodDisruptionBudgetList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PodDisruptionBudgetList {
        static mut instance: ::protobuf::lazy::Lazy<PodDisruptionBudgetList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PodDisruptionBudgetList,
        };
        unsafe {
            instance.get(PodDisruptionBudgetList::new)
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

    // repeated .k8s.io.api.policy.v1beta1.PodDisruptionBudget items = 2;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<PodDisruptionBudget>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<PodDisruptionBudget> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<PodDisruptionBudget> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[PodDisruptionBudget] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<PodDisruptionBudget> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PodDisruptionBudget> {
        &mut self.items
    }
}

impl ::protobuf::Message for PodDisruptionBudgetList {
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

impl ::protobuf::MessageStatic for PodDisruptionBudgetList {
    fn new() -> PodDisruptionBudgetList {
        PodDisruptionBudgetList::new()
    }

    fn descriptor_static(_: ::std::option::Option<PodDisruptionBudgetList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ListMeta>>(
                    "metadata",
                    PodDisruptionBudgetList::get_metadata_for_reflect,
                    PodDisruptionBudgetList::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PodDisruptionBudget>>(
                    "items",
                    PodDisruptionBudgetList::get_items_for_reflect,
                    PodDisruptionBudgetList::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PodDisruptionBudgetList>(
                    "PodDisruptionBudgetList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PodDisruptionBudgetList {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PodDisruptionBudgetList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PodDisruptionBudgetList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PodDisruptionBudgetSpec {
    // message fields
    minAvailable: ::protobuf::SingularPtrField<super::generated::IntOrString>,
    selector: ::protobuf::SingularPtrField<super::generated::LabelSelector>,
    maxUnavailable: ::protobuf::SingularPtrField<super::generated::IntOrString>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PodDisruptionBudgetSpec {}

impl PodDisruptionBudgetSpec {
    pub fn new() -> PodDisruptionBudgetSpec {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PodDisruptionBudgetSpec {
        static mut instance: ::protobuf::lazy::Lazy<PodDisruptionBudgetSpec> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PodDisruptionBudgetSpec,
        };
        unsafe {
            instance.get(PodDisruptionBudgetSpec::new)
        }
    }

    // optional .k8s.io.apimachinery.pkg.util.intstr.IntOrString minAvailable = 1;

    pub fn clear_minAvailable(&mut self) {
        self.minAvailable.clear();
    }

    pub fn has_minAvailable(&self) -> bool {
        self.minAvailable.is_some()
    }

    // Param is passed by value, moved
    pub fn set_minAvailable(&mut self, v: super::generated::IntOrString) {
        self.minAvailable = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_minAvailable(&mut self) -> &mut super::generated::IntOrString {
        if self.minAvailable.is_none() {
            self.minAvailable.set_default();
        }
        self.minAvailable.as_mut().unwrap()
    }

    // Take field
    pub fn take_minAvailable(&mut self) -> super::generated::IntOrString {
        self.minAvailable.take().unwrap_or_else(|| super::generated::IntOrString::new())
    }

    pub fn get_minAvailable(&self) -> &super::generated::IntOrString {
        self.minAvailable.as_ref().unwrap_or_else(|| super::generated::IntOrString::default_instance())
    }

    fn get_minAvailable_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::IntOrString> {
        &self.minAvailable
    }

    fn mut_minAvailable_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::IntOrString> {
        &mut self.minAvailable
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

    // optional .k8s.io.apimachinery.pkg.util.intstr.IntOrString maxUnavailable = 3;

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
}

impl ::protobuf::Message for PodDisruptionBudgetSpec {
    fn is_initialized(&self) -> bool {
        for v in &self.minAvailable {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.selector {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.maxUnavailable {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.minAvailable)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.selector)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.maxUnavailable)?;
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
        if let Some(ref v) = self.minAvailable.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.selector.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.maxUnavailable.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.minAvailable.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.selector.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.maxUnavailable.as_ref() {
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

impl ::protobuf::MessageStatic for PodDisruptionBudgetSpec {
    fn new() -> PodDisruptionBudgetSpec {
        PodDisruptionBudgetSpec::new()
    }

    fn descriptor_static(_: ::std::option::Option<PodDisruptionBudgetSpec>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::IntOrString>>(
                    "minAvailable",
                    PodDisruptionBudgetSpec::get_minAvailable_for_reflect,
                    PodDisruptionBudgetSpec::mut_minAvailable_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::LabelSelector>>(
                    "selector",
                    PodDisruptionBudgetSpec::get_selector_for_reflect,
                    PodDisruptionBudgetSpec::mut_selector_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::IntOrString>>(
                    "maxUnavailable",
                    PodDisruptionBudgetSpec::get_maxUnavailable_for_reflect,
                    PodDisruptionBudgetSpec::mut_maxUnavailable_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PodDisruptionBudgetSpec>(
                    "PodDisruptionBudgetSpec",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PodDisruptionBudgetSpec {
    fn clear(&mut self) {
        self.clear_minAvailable();
        self.clear_selector();
        self.clear_maxUnavailable();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PodDisruptionBudgetSpec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PodDisruptionBudgetSpec {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PodDisruptionBudgetStatus {
    // message fields
    observedGeneration: ::std::option::Option<i64>,
    pub disruptedPods: ::std::collections::HashMap<::std::string::String, super::generated::Time>,
    disruptionsAllowed: ::std::option::Option<i32>,
    currentHealthy: ::std::option::Option<i32>,
    desiredHealthy: ::std::option::Option<i32>,
    expectedPods: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PodDisruptionBudgetStatus {}

impl PodDisruptionBudgetStatus {
    pub fn new() -> PodDisruptionBudgetStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PodDisruptionBudgetStatus {
        static mut instance: ::protobuf::lazy::Lazy<PodDisruptionBudgetStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PodDisruptionBudgetStatus,
        };
        unsafe {
            instance.get(PodDisruptionBudgetStatus::new)
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

    // repeated .k8s.io.api.policy.v1beta1.PodDisruptionBudgetStatus.DisruptedPodsEntry disruptedPods = 2;

    pub fn clear_disruptedPods(&mut self) {
        self.disruptedPods.clear();
    }

    // Param is passed by value, moved
    pub fn set_disruptedPods(&mut self, v: ::std::collections::HashMap<::std::string::String, super::generated::Time>) {
        self.disruptedPods = v;
    }

    // Mutable pointer to the field.
    pub fn mut_disruptedPods(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, super::generated::Time> {
        &mut self.disruptedPods
    }

    // Take field
    pub fn take_disruptedPods(&mut self) -> ::std::collections::HashMap<::std::string::String, super::generated::Time> {
        ::std::mem::replace(&mut self.disruptedPods, ::std::collections::HashMap::new())
    }

    pub fn get_disruptedPods(&self) -> &::std::collections::HashMap<::std::string::String, super::generated::Time> {
        &self.disruptedPods
    }

    fn get_disruptedPods_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, super::generated::Time> {
        &self.disruptedPods
    }

    fn mut_disruptedPods_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, super::generated::Time> {
        &mut self.disruptedPods
    }

    // optional int32 disruptionsAllowed = 3;

    pub fn clear_disruptionsAllowed(&mut self) {
        self.disruptionsAllowed = ::std::option::Option::None;
    }

    pub fn has_disruptionsAllowed(&self) -> bool {
        self.disruptionsAllowed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_disruptionsAllowed(&mut self, v: i32) {
        self.disruptionsAllowed = ::std::option::Option::Some(v);
    }

    pub fn get_disruptionsAllowed(&self) -> i32 {
        self.disruptionsAllowed.unwrap_or(0)
    }

    fn get_disruptionsAllowed_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.disruptionsAllowed
    }

    fn mut_disruptionsAllowed_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.disruptionsAllowed
    }

    // optional int32 currentHealthy = 4;

    pub fn clear_currentHealthy(&mut self) {
        self.currentHealthy = ::std::option::Option::None;
    }

    pub fn has_currentHealthy(&self) -> bool {
        self.currentHealthy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currentHealthy(&mut self, v: i32) {
        self.currentHealthy = ::std::option::Option::Some(v);
    }

    pub fn get_currentHealthy(&self) -> i32 {
        self.currentHealthy.unwrap_or(0)
    }

    fn get_currentHealthy_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.currentHealthy
    }

    fn mut_currentHealthy_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.currentHealthy
    }

    // optional int32 desiredHealthy = 5;

    pub fn clear_desiredHealthy(&mut self) {
        self.desiredHealthy = ::std::option::Option::None;
    }

    pub fn has_desiredHealthy(&self) -> bool {
        self.desiredHealthy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_desiredHealthy(&mut self, v: i32) {
        self.desiredHealthy = ::std::option::Option::Some(v);
    }

    pub fn get_desiredHealthy(&self) -> i32 {
        self.desiredHealthy.unwrap_or(0)
    }

    fn get_desiredHealthy_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.desiredHealthy
    }

    fn mut_desiredHealthy_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.desiredHealthy
    }

    // optional int32 expectedPods = 6;

    pub fn clear_expectedPods(&mut self) {
        self.expectedPods = ::std::option::Option::None;
    }

    pub fn has_expectedPods(&self) -> bool {
        self.expectedPods.is_some()
    }

    // Param is passed by value, moved
    pub fn set_expectedPods(&mut self, v: i32) {
        self.expectedPods = ::std::option::Option::Some(v);
    }

    pub fn get_expectedPods(&self) -> i32 {
        self.expectedPods.unwrap_or(0)
    }

    fn get_expectedPods_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.expectedPods
    }

    fn mut_expectedPods_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.expectedPods
    }
}

impl ::protobuf::Message for PodDisruptionBudgetStatus {
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
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<super::generated::Time>>(wire_type, is, &mut self.disruptedPods)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.disruptionsAllowed = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.currentHealthy = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.desiredHealthy = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.expectedPods = ::std::option::Option::Some(tmp);
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
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<super::generated::Time>>(2, &self.disruptedPods);
        if let Some(v) = self.disruptionsAllowed {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.currentHealthy {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.desiredHealthy {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.expectedPods {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.observedGeneration {
            os.write_int64(1, v)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<super::generated::Time>>(2, &self.disruptedPods, os)?;
        if let Some(v) = self.disruptionsAllowed {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.currentHealthy {
            os.write_int32(4, v)?;
        }
        if let Some(v) = self.desiredHealthy {
            os.write_int32(5, v)?;
        }
        if let Some(v) = self.expectedPods {
            os.write_int32(6, v)?;
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

impl ::protobuf::MessageStatic for PodDisruptionBudgetStatus {
    fn new() -> PodDisruptionBudgetStatus {
        PodDisruptionBudgetStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<PodDisruptionBudgetStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "observedGeneration",
                    PodDisruptionBudgetStatus::get_observedGeneration_for_reflect,
                    PodDisruptionBudgetStatus::mut_observedGeneration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<super::generated::Time>>(
                    "disruptedPods",
                    PodDisruptionBudgetStatus::get_disruptedPods_for_reflect,
                    PodDisruptionBudgetStatus::mut_disruptedPods_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "disruptionsAllowed",
                    PodDisruptionBudgetStatus::get_disruptionsAllowed_for_reflect,
                    PodDisruptionBudgetStatus::mut_disruptionsAllowed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "currentHealthy",
                    PodDisruptionBudgetStatus::get_currentHealthy_for_reflect,
                    PodDisruptionBudgetStatus::mut_currentHealthy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "desiredHealthy",
                    PodDisruptionBudgetStatus::get_desiredHealthy_for_reflect,
                    PodDisruptionBudgetStatus::mut_desiredHealthy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "expectedPods",
                    PodDisruptionBudgetStatus::get_expectedPods_for_reflect,
                    PodDisruptionBudgetStatus::mut_expectedPods_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PodDisruptionBudgetStatus>(
                    "PodDisruptionBudgetStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PodDisruptionBudgetStatus {
    fn clear(&mut self) {
        self.clear_observedGeneration();
        self.clear_disruptedPods();
        self.clear_disruptionsAllowed();
        self.clear_currentHealthy();
        self.clear_desiredHealthy();
        self.clear_expectedPods();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PodDisruptionBudgetStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PodDisruptionBudgetStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n)k8s.io/api/policy/v1beta1/generated.proto\x12\x19k8s.io.api.policy.v1\
    beta1\x1a\"k8s.io/api/core/v1/generated.proto\x1a4k8s.io/apimachinery/pk\
    g/apis/meta/v1/generated.proto\x1a/k8s.io/apimachinery/pkg/runtime/gener\
    ated.proto\x1a6k8s.io/apimachinery/pkg/runtime/schema/generated.proto\
    \x1a3k8s.io/apimachinery/pkg/util/intstr/generated.proto\"\xb3\x01\n\x08\
    Eviction\x12L\n\x08metadata\x18\x01\x20\x01(\x0b20.k8s.io.apimachinery.p\
    kg.apis.meta.v1.ObjectMetaR\x08metadata\x12Y\n\rdeleteOptions\x18\x02\
    \x20\x01(\x0b23.k8s.io.apimachinery.pkg.apis.meta.v1.DeleteOptionsR\rdel\
    eteOptions\"\xf9\x01\n\x13PodDisruptionBudget\x12L\n\x08metadata\x18\x01\
    \x20\x01(\x0b20.k8s.io.apimachinery.pkg.apis.meta.v1.ObjectMetaR\x08meta\
    data\x12F\n\x04spec\x18\x02\x20\x01(\x0b22.k8s.io.api.policy.v1beta1.Pod\
    DisruptionBudgetSpecR\x04spec\x12L\n\x06status\x18\x03\x20\x01(\x0b24.k8\
    s.io.api.policy.v1beta1.PodDisruptionBudgetStatusR\x06status\"\xab\x01\n\
    \x17PodDisruptionBudgetList\x12J\n\x08metadata\x18\x01\x20\x01(\x0b2..k8\
    s.io.apimachinery.pkg.apis.meta.v1.ListMetaR\x08metadata\x12D\n\x05items\
    \x18\x02\x20\x03(\x0b2..k8s.io.api.policy.v1beta1.PodDisruptionBudgetR\
    \x05items\"\x9a\x02\n\x17PodDisruptionBudgetSpec\x12T\n\x0cminAvailable\
    \x18\x01\x20\x01(\x0b20.k8s.io.apimachinery.pkg.util.intstr.IntOrStringR\
    \x0cminAvailable\x12O\n\x08selector\x18\x02\x20\x01(\x0b23.k8s.io.apimac\
    hinery.pkg.apis.meta.v1.LabelSelectorR\x08selector\x12X\n\x0emaxUnavaila\
    ble\x18\x03\x20\x01(\x0b20.k8s.io.apimachinery.pkg.util.intstr.IntOrStri\
    ngR\x0emaxUnavailable\"\xcc\x03\n\x19PodDisruptionBudgetStatus\x12.\n\
    \x12observedGeneration\x18\x01\x20\x01(\x03R\x12observedGeneration\x12m\
    \n\rdisruptedPods\x18\x02\x20\x03(\x0b2G.k8s.io.api.policy.v1beta1.PodDi\
    sruptionBudgetStatus.DisruptedPodsEntryR\rdisruptedPods\x12.\n\x12disrup\
    tionsAllowed\x18\x03\x20\x01(\x05R\x12disruptionsAllowed\x12&\n\x0ecurre\
    ntHealthy\x18\x04\x20\x01(\x05R\x0ecurrentHealthy\x12&\n\x0edesiredHealt\
    hy\x18\x05\x20\x01(\x05R\x0edesiredHealthy\x12\"\n\x0cexpectedPods\x18\
    \x06\x20\x01(\x05R\x0cexpectedPods\x1al\n\x12DisruptedPodsEntry\x12\x10\
    \n\x03key\x18\x01\x20\x01(\tR\x03key\x12@\n\x05value\x18\x02\x20\x01(\
    \x0b2*.k8s.io.apimachinery.pkg.apis.meta.v1.TimeR\x05value:\x028\x01B\tZ\
    \x07v1beta1\
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
