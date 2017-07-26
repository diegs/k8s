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
pub struct PodPreset {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    spec: ::protobuf::SingularPtrField<PodPresetSpec>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PodPreset {}

impl PodPreset {
    pub fn new() -> PodPreset {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PodPreset {
        static mut instance: ::protobuf::lazy::Lazy<PodPreset> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PodPreset,
        };
        unsafe {
            instance.get(PodPreset::new)
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

    // optional .k8s.io.api.settings.v1alpha1.PodPresetSpec spec = 2;

    pub fn clear_spec(&mut self) {
        self.spec.clear();
    }

    pub fn has_spec(&self) -> bool {
        self.spec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spec(&mut self, v: PodPresetSpec) {
        self.spec = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_spec(&mut self) -> &mut PodPresetSpec {
        if self.spec.is_none() {
            self.spec.set_default();
        }
        self.spec.as_mut().unwrap()
    }

    // Take field
    pub fn take_spec(&mut self) -> PodPresetSpec {
        self.spec.take().unwrap_or_else(|| PodPresetSpec::new())
    }

    pub fn get_spec(&self) -> &PodPresetSpec {
        self.spec.as_ref().unwrap_or_else(|| PodPresetSpec::default_instance())
    }

    fn get_spec_for_reflect(&self) -> &::protobuf::SingularPtrField<PodPresetSpec> {
        &self.spec
    }

    fn mut_spec_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PodPresetSpec> {
        &mut self.spec
    }
}

impl ::protobuf::Message for PodPreset {
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

impl ::protobuf::MessageStatic for PodPreset {
    fn new() -> PodPreset {
        PodPreset::new()
    }

    fn descriptor_static(_: ::std::option::Option<PodPreset>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    PodPreset::get_metadata_for_reflect,
                    PodPreset::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PodPresetSpec>>(
                    "spec",
                    PodPreset::get_spec_for_reflect,
                    PodPreset::mut_spec_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PodPreset>(
                    "PodPreset",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PodPreset {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_spec();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PodPreset {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PodPreset {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PodPresetList {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ListMeta>,
    items: ::protobuf::RepeatedField<PodPreset>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PodPresetList {}

impl PodPresetList {
    pub fn new() -> PodPresetList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PodPresetList {
        static mut instance: ::protobuf::lazy::Lazy<PodPresetList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PodPresetList,
        };
        unsafe {
            instance.get(PodPresetList::new)
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

    // repeated .k8s.io.api.settings.v1alpha1.PodPreset items = 2;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<PodPreset>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<PodPreset> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<PodPreset> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[PodPreset] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<PodPreset> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PodPreset> {
        &mut self.items
    }
}

impl ::protobuf::Message for PodPresetList {
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

impl ::protobuf::MessageStatic for PodPresetList {
    fn new() -> PodPresetList {
        PodPresetList::new()
    }

    fn descriptor_static(_: ::std::option::Option<PodPresetList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ListMeta>>(
                    "metadata",
                    PodPresetList::get_metadata_for_reflect,
                    PodPresetList::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PodPreset>>(
                    "items",
                    PodPresetList::get_items_for_reflect,
                    PodPresetList::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PodPresetList>(
                    "PodPresetList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PodPresetList {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PodPresetList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PodPresetList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PodPresetSpec {
    // message fields
    selector: ::protobuf::SingularPtrField<super::generated::LabelSelector>,
    env: ::protobuf::RepeatedField<super::generated::EnvVar>,
    envFrom: ::protobuf::RepeatedField<super::generated::EnvFromSource>,
    volumes: ::protobuf::RepeatedField<super::generated::Volume>,
    volumeMounts: ::protobuf::RepeatedField<super::generated::VolumeMount>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PodPresetSpec {}

impl PodPresetSpec {
    pub fn new() -> PodPresetSpec {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PodPresetSpec {
        static mut instance: ::protobuf::lazy::Lazy<PodPresetSpec> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PodPresetSpec,
        };
        unsafe {
            instance.get(PodPresetSpec::new)
        }
    }

    // optional .k8s.io.apimachinery.pkg.apis.meta.v1.LabelSelector selector = 1;

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

    // repeated .k8s.io.api.core.v1.EnvVar env = 2;

    pub fn clear_env(&mut self) {
        self.env.clear();
    }

    // Param is passed by value, moved
    pub fn set_env(&mut self, v: ::protobuf::RepeatedField<super::generated::EnvVar>) {
        self.env = v;
    }

    // Mutable pointer to the field.
    pub fn mut_env(&mut self) -> &mut ::protobuf::RepeatedField<super::generated::EnvVar> {
        &mut self.env
    }

    // Take field
    pub fn take_env(&mut self) -> ::protobuf::RepeatedField<super::generated::EnvVar> {
        ::std::mem::replace(&mut self.env, ::protobuf::RepeatedField::new())
    }

    pub fn get_env(&self) -> &[super::generated::EnvVar] {
        &self.env
    }

    fn get_env_for_reflect(&self) -> &::protobuf::RepeatedField<super::generated::EnvVar> {
        &self.env
    }

    fn mut_env_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::generated::EnvVar> {
        &mut self.env
    }

    // repeated .k8s.io.api.core.v1.EnvFromSource envFrom = 3;

    pub fn clear_envFrom(&mut self) {
        self.envFrom.clear();
    }

    // Param is passed by value, moved
    pub fn set_envFrom(&mut self, v: ::protobuf::RepeatedField<super::generated::EnvFromSource>) {
        self.envFrom = v;
    }

    // Mutable pointer to the field.
    pub fn mut_envFrom(&mut self) -> &mut ::protobuf::RepeatedField<super::generated::EnvFromSource> {
        &mut self.envFrom
    }

    // Take field
    pub fn take_envFrom(&mut self) -> ::protobuf::RepeatedField<super::generated::EnvFromSource> {
        ::std::mem::replace(&mut self.envFrom, ::protobuf::RepeatedField::new())
    }

    pub fn get_envFrom(&self) -> &[super::generated::EnvFromSource] {
        &self.envFrom
    }

    fn get_envFrom_for_reflect(&self) -> &::protobuf::RepeatedField<super::generated::EnvFromSource> {
        &self.envFrom
    }

    fn mut_envFrom_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::generated::EnvFromSource> {
        &mut self.envFrom
    }

    // repeated .k8s.io.api.core.v1.Volume volumes = 4;

    pub fn clear_volumes(&mut self) {
        self.volumes.clear();
    }

    // Param is passed by value, moved
    pub fn set_volumes(&mut self, v: ::protobuf::RepeatedField<super::generated::Volume>) {
        self.volumes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_volumes(&mut self) -> &mut ::protobuf::RepeatedField<super::generated::Volume> {
        &mut self.volumes
    }

    // Take field
    pub fn take_volumes(&mut self) -> ::protobuf::RepeatedField<super::generated::Volume> {
        ::std::mem::replace(&mut self.volumes, ::protobuf::RepeatedField::new())
    }

    pub fn get_volumes(&self) -> &[super::generated::Volume] {
        &self.volumes
    }

    fn get_volumes_for_reflect(&self) -> &::protobuf::RepeatedField<super::generated::Volume> {
        &self.volumes
    }

    fn mut_volumes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::generated::Volume> {
        &mut self.volumes
    }

    // repeated .k8s.io.api.core.v1.VolumeMount volumeMounts = 5;

    pub fn clear_volumeMounts(&mut self) {
        self.volumeMounts.clear();
    }

    // Param is passed by value, moved
    pub fn set_volumeMounts(&mut self, v: ::protobuf::RepeatedField<super::generated::VolumeMount>) {
        self.volumeMounts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_volumeMounts(&mut self) -> &mut ::protobuf::RepeatedField<super::generated::VolumeMount> {
        &mut self.volumeMounts
    }

    // Take field
    pub fn take_volumeMounts(&mut self) -> ::protobuf::RepeatedField<super::generated::VolumeMount> {
        ::std::mem::replace(&mut self.volumeMounts, ::protobuf::RepeatedField::new())
    }

    pub fn get_volumeMounts(&self) -> &[super::generated::VolumeMount] {
        &self.volumeMounts
    }

    fn get_volumeMounts_for_reflect(&self) -> &::protobuf::RepeatedField<super::generated::VolumeMount> {
        &self.volumeMounts
    }

    fn mut_volumeMounts_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::generated::VolumeMount> {
        &mut self.volumeMounts
    }
}

impl ::protobuf::Message for PodPresetSpec {
    fn is_initialized(&self) -> bool {
        for v in &self.selector {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.env {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.envFrom {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.volumes {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.volumeMounts {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.selector)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.env)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.envFrom)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.volumes)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.volumeMounts)?;
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
        if let Some(ref v) = self.selector.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.env {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.envFrom {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.volumes {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.volumeMounts {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.selector.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.env {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.envFrom {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.volumes {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.volumeMounts {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for PodPresetSpec {
    fn new() -> PodPresetSpec {
        PodPresetSpec::new()
    }

    fn descriptor_static(_: ::std::option::Option<PodPresetSpec>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::LabelSelector>>(
                    "selector",
                    PodPresetSpec::get_selector_for_reflect,
                    PodPresetSpec::mut_selector_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::EnvVar>>(
                    "env",
                    PodPresetSpec::get_env_for_reflect,
                    PodPresetSpec::mut_env_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::EnvFromSource>>(
                    "envFrom",
                    PodPresetSpec::get_envFrom_for_reflect,
                    PodPresetSpec::mut_envFrom_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::Volume>>(
                    "volumes",
                    PodPresetSpec::get_volumes_for_reflect,
                    PodPresetSpec::mut_volumes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::VolumeMount>>(
                    "volumeMounts",
                    PodPresetSpec::get_volumeMounts_for_reflect,
                    PodPresetSpec::mut_volumeMounts_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PodPresetSpec>(
                    "PodPresetSpec",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PodPresetSpec {
    fn clear(&mut self) {
        self.clear_selector();
        self.clear_env();
        self.clear_envFrom();
        self.clear_volumes();
        self.clear_volumeMounts();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PodPresetSpec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PodPresetSpec {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n,k8s.io/api/settings/v1alpha1/generated.proto\x12\x1ck8s.io.api.settin\
    gs.v1alpha1\x1a\"k8s.io/api/core/v1/generated.proto\x1a4k8s.io/apimachin\
    ery/pkg/apis/meta/v1/generated.proto\x1a/k8s.io/apimachinery/pkg/runtime\
    /generated.proto\x1a6k8s.io/apimachinery/pkg/runtime/schema/generated.pr\
    oto\x1a3k8s.io/apimachinery/pkg/util/intstr/generated.proto\"\x9a\x01\n\
    \tPodPreset\x12L\n\x08metadata\x18\x01\x20\x01(\x0b20.k8s.io.apimachiner\
    y.pkg.apis.meta.v1.ObjectMetaR\x08metadata\x12?\n\x04spec\x18\x02\x20\
    \x01(\x0b2+.k8s.io.api.settings.v1alpha1.PodPresetSpecR\x04spec\"\x9a\
    \x01\n\rPodPresetList\x12J\n\x08metadata\x18\x01\x20\x01(\x0b2..k8s.io.a\
    pimachinery.pkg.apis.meta.v1.ListMetaR\x08metadata\x12=\n\x05items\x18\
    \x02\x20\x03(\x0b2'.k8s.io.api.settings.v1alpha1.PodPresetR\x05items\"\
    \xc6\x02\n\rPodPresetSpec\x12O\n\x08selector\x18\x01\x20\x01(\x0b23.k8s.\
    io.apimachinery.pkg.apis.meta.v1.LabelSelectorR\x08selector\x12,\n\x03en\
    v\x18\x02\x20\x03(\x0b2\x1a.k8s.io.api.core.v1.EnvVarR\x03env\x12;\n\x07\
    envFrom\x18\x03\x20\x03(\x0b2!.k8s.io.api.core.v1.EnvFromSourceR\x07envF\
    rom\x124\n\x07volumes\x18\x04\x20\x03(\x0b2\x1a.k8s.io.api.core.v1.Volum\
    eR\x07volumes\x12C\n\x0cvolumeMounts\x18\x05\x20\x03(\x0b2\x1f.k8s.io.ap\
    i.core.v1.VolumeMountR\x0cvolumeMountsB\nZ\x08v1alpha1\
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
