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
pub struct NetworkPolicy {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    spec: ::protobuf::SingularPtrField<NetworkPolicySpec>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NetworkPolicy {}

impl NetworkPolicy {
    pub fn new() -> NetworkPolicy {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NetworkPolicy {
        static mut instance: ::protobuf::lazy::Lazy<NetworkPolicy> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NetworkPolicy,
        };
        unsafe {
            instance.get(NetworkPolicy::new)
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

    // optional .k8s.io.api.networking.v1.NetworkPolicySpec spec = 2;

    pub fn clear_spec(&mut self) {
        self.spec.clear();
    }

    pub fn has_spec(&self) -> bool {
        self.spec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spec(&mut self, v: NetworkPolicySpec) {
        self.spec = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_spec(&mut self) -> &mut NetworkPolicySpec {
        if self.spec.is_none() {
            self.spec.set_default();
        }
        self.spec.as_mut().unwrap()
    }

    // Take field
    pub fn take_spec(&mut self) -> NetworkPolicySpec {
        self.spec.take().unwrap_or_else(|| NetworkPolicySpec::new())
    }

    pub fn get_spec(&self) -> &NetworkPolicySpec {
        self.spec.as_ref().unwrap_or_else(|| NetworkPolicySpec::default_instance())
    }

    fn get_spec_for_reflect(&self) -> &::protobuf::SingularPtrField<NetworkPolicySpec> {
        &self.spec
    }

    fn mut_spec_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<NetworkPolicySpec> {
        &mut self.spec
    }
}

impl ::protobuf::Message for NetworkPolicy {
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

impl ::protobuf::MessageStatic for NetworkPolicy {
    fn new() -> NetworkPolicy {
        NetworkPolicy::new()
    }

    fn descriptor_static(_: ::std::option::Option<NetworkPolicy>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    NetworkPolicy::get_metadata_for_reflect,
                    NetworkPolicy::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<NetworkPolicySpec>>(
                    "spec",
                    NetworkPolicy::get_spec_for_reflect,
                    NetworkPolicy::mut_spec_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NetworkPolicy>(
                    "NetworkPolicy",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NetworkPolicy {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_spec();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NetworkPolicy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NetworkPolicy {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NetworkPolicyIngressRule {
    // message fields
    ports: ::protobuf::RepeatedField<NetworkPolicyPort>,
    from: ::protobuf::RepeatedField<NetworkPolicyPeer>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NetworkPolicyIngressRule {}

impl NetworkPolicyIngressRule {
    pub fn new() -> NetworkPolicyIngressRule {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NetworkPolicyIngressRule {
        static mut instance: ::protobuf::lazy::Lazy<NetworkPolicyIngressRule> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NetworkPolicyIngressRule,
        };
        unsafe {
            instance.get(NetworkPolicyIngressRule::new)
        }
    }

    // repeated .k8s.io.api.networking.v1.NetworkPolicyPort ports = 1;

    pub fn clear_ports(&mut self) {
        self.ports.clear();
    }

    // Param is passed by value, moved
    pub fn set_ports(&mut self, v: ::protobuf::RepeatedField<NetworkPolicyPort>) {
        self.ports = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ports(&mut self) -> &mut ::protobuf::RepeatedField<NetworkPolicyPort> {
        &mut self.ports
    }

    // Take field
    pub fn take_ports(&mut self) -> ::protobuf::RepeatedField<NetworkPolicyPort> {
        ::std::mem::replace(&mut self.ports, ::protobuf::RepeatedField::new())
    }

    pub fn get_ports(&self) -> &[NetworkPolicyPort] {
        &self.ports
    }

    fn get_ports_for_reflect(&self) -> &::protobuf::RepeatedField<NetworkPolicyPort> {
        &self.ports
    }

    fn mut_ports_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<NetworkPolicyPort> {
        &mut self.ports
    }

    // repeated .k8s.io.api.networking.v1.NetworkPolicyPeer from = 2;

    pub fn clear_from(&mut self) {
        self.from.clear();
    }

    // Param is passed by value, moved
    pub fn set_from(&mut self, v: ::protobuf::RepeatedField<NetworkPolicyPeer>) {
        self.from = v;
    }

    // Mutable pointer to the field.
    pub fn mut_from(&mut self) -> &mut ::protobuf::RepeatedField<NetworkPolicyPeer> {
        &mut self.from
    }

    // Take field
    pub fn take_from(&mut self) -> ::protobuf::RepeatedField<NetworkPolicyPeer> {
        ::std::mem::replace(&mut self.from, ::protobuf::RepeatedField::new())
    }

    pub fn get_from(&self) -> &[NetworkPolicyPeer] {
        &self.from
    }

    fn get_from_for_reflect(&self) -> &::protobuf::RepeatedField<NetworkPolicyPeer> {
        &self.from
    }

    fn mut_from_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<NetworkPolicyPeer> {
        &mut self.from
    }
}

impl ::protobuf::Message for NetworkPolicyIngressRule {
    fn is_initialized(&self) -> bool {
        for v in &self.ports {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.from {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.ports)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.from)?;
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
        for value in &self.ports {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.from {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.ports {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.from {
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

impl ::protobuf::MessageStatic for NetworkPolicyIngressRule {
    fn new() -> NetworkPolicyIngressRule {
        NetworkPolicyIngressRule::new()
    }

    fn descriptor_static(_: ::std::option::Option<NetworkPolicyIngressRule>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<NetworkPolicyPort>>(
                    "ports",
                    NetworkPolicyIngressRule::get_ports_for_reflect,
                    NetworkPolicyIngressRule::mut_ports_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<NetworkPolicyPeer>>(
                    "from",
                    NetworkPolicyIngressRule::get_from_for_reflect,
                    NetworkPolicyIngressRule::mut_from_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NetworkPolicyIngressRule>(
                    "NetworkPolicyIngressRule",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NetworkPolicyIngressRule {
    fn clear(&mut self) {
        self.clear_ports();
        self.clear_from();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NetworkPolicyIngressRule {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NetworkPolicyIngressRule {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NetworkPolicyList {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ListMeta>,
    items: ::protobuf::RepeatedField<NetworkPolicy>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NetworkPolicyList {}

impl NetworkPolicyList {
    pub fn new() -> NetworkPolicyList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NetworkPolicyList {
        static mut instance: ::protobuf::lazy::Lazy<NetworkPolicyList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NetworkPolicyList,
        };
        unsafe {
            instance.get(NetworkPolicyList::new)
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

    // repeated .k8s.io.api.networking.v1.NetworkPolicy items = 2;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<NetworkPolicy>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<NetworkPolicy> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<NetworkPolicy> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[NetworkPolicy] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<NetworkPolicy> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<NetworkPolicy> {
        &mut self.items
    }
}

impl ::protobuf::Message for NetworkPolicyList {
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

impl ::protobuf::MessageStatic for NetworkPolicyList {
    fn new() -> NetworkPolicyList {
        NetworkPolicyList::new()
    }

    fn descriptor_static(_: ::std::option::Option<NetworkPolicyList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ListMeta>>(
                    "metadata",
                    NetworkPolicyList::get_metadata_for_reflect,
                    NetworkPolicyList::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<NetworkPolicy>>(
                    "items",
                    NetworkPolicyList::get_items_for_reflect,
                    NetworkPolicyList::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NetworkPolicyList>(
                    "NetworkPolicyList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NetworkPolicyList {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NetworkPolicyList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NetworkPolicyList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NetworkPolicyPeer {
    // message fields
    podSelector: ::protobuf::SingularPtrField<super::generated::LabelSelector>,
    namespaceSelector: ::protobuf::SingularPtrField<super::generated::LabelSelector>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NetworkPolicyPeer {}

impl NetworkPolicyPeer {
    pub fn new() -> NetworkPolicyPeer {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NetworkPolicyPeer {
        static mut instance: ::protobuf::lazy::Lazy<NetworkPolicyPeer> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NetworkPolicyPeer,
        };
        unsafe {
            instance.get(NetworkPolicyPeer::new)
        }
    }

    // optional .k8s.io.apimachinery.pkg.apis.meta.v1.LabelSelector podSelector = 1;

    pub fn clear_podSelector(&mut self) {
        self.podSelector.clear();
    }

    pub fn has_podSelector(&self) -> bool {
        self.podSelector.is_some()
    }

    // Param is passed by value, moved
    pub fn set_podSelector(&mut self, v: super::generated::LabelSelector) {
        self.podSelector = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_podSelector(&mut self) -> &mut super::generated::LabelSelector {
        if self.podSelector.is_none() {
            self.podSelector.set_default();
        }
        self.podSelector.as_mut().unwrap()
    }

    // Take field
    pub fn take_podSelector(&mut self) -> super::generated::LabelSelector {
        self.podSelector.take().unwrap_or_else(|| super::generated::LabelSelector::new())
    }

    pub fn get_podSelector(&self) -> &super::generated::LabelSelector {
        self.podSelector.as_ref().unwrap_or_else(|| super::generated::LabelSelector::default_instance())
    }

    fn get_podSelector_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::LabelSelector> {
        &self.podSelector
    }

    fn mut_podSelector_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::LabelSelector> {
        &mut self.podSelector
    }

    // optional .k8s.io.apimachinery.pkg.apis.meta.v1.LabelSelector namespaceSelector = 2;

    pub fn clear_namespaceSelector(&mut self) {
        self.namespaceSelector.clear();
    }

    pub fn has_namespaceSelector(&self) -> bool {
        self.namespaceSelector.is_some()
    }

    // Param is passed by value, moved
    pub fn set_namespaceSelector(&mut self, v: super::generated::LabelSelector) {
        self.namespaceSelector = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_namespaceSelector(&mut self) -> &mut super::generated::LabelSelector {
        if self.namespaceSelector.is_none() {
            self.namespaceSelector.set_default();
        }
        self.namespaceSelector.as_mut().unwrap()
    }

    // Take field
    pub fn take_namespaceSelector(&mut self) -> super::generated::LabelSelector {
        self.namespaceSelector.take().unwrap_or_else(|| super::generated::LabelSelector::new())
    }

    pub fn get_namespaceSelector(&self) -> &super::generated::LabelSelector {
        self.namespaceSelector.as_ref().unwrap_or_else(|| super::generated::LabelSelector::default_instance())
    }

    fn get_namespaceSelector_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::LabelSelector> {
        &self.namespaceSelector
    }

    fn mut_namespaceSelector_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::LabelSelector> {
        &mut self.namespaceSelector
    }
}

impl ::protobuf::Message for NetworkPolicyPeer {
    fn is_initialized(&self) -> bool {
        for v in &self.podSelector {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.namespaceSelector {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.podSelector)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.namespaceSelector)?;
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
        if let Some(ref v) = self.podSelector.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.namespaceSelector.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.podSelector.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.namespaceSelector.as_ref() {
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

impl ::protobuf::MessageStatic for NetworkPolicyPeer {
    fn new() -> NetworkPolicyPeer {
        NetworkPolicyPeer::new()
    }

    fn descriptor_static(_: ::std::option::Option<NetworkPolicyPeer>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::LabelSelector>>(
                    "podSelector",
                    NetworkPolicyPeer::get_podSelector_for_reflect,
                    NetworkPolicyPeer::mut_podSelector_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::LabelSelector>>(
                    "namespaceSelector",
                    NetworkPolicyPeer::get_namespaceSelector_for_reflect,
                    NetworkPolicyPeer::mut_namespaceSelector_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NetworkPolicyPeer>(
                    "NetworkPolicyPeer",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NetworkPolicyPeer {
    fn clear(&mut self) {
        self.clear_podSelector();
        self.clear_namespaceSelector();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NetworkPolicyPeer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NetworkPolicyPeer {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NetworkPolicyPort {
    // message fields
    protocol: ::protobuf::SingularField<::std::string::String>,
    port: ::protobuf::SingularPtrField<super::generated::IntOrString>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NetworkPolicyPort {}

impl NetworkPolicyPort {
    pub fn new() -> NetworkPolicyPort {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NetworkPolicyPort {
        static mut instance: ::protobuf::lazy::Lazy<NetworkPolicyPort> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NetworkPolicyPort,
        };
        unsafe {
            instance.get(NetworkPolicyPort::new)
        }
    }

    // optional string protocol = 1;

    pub fn clear_protocol(&mut self) {
        self.protocol.clear();
    }

    pub fn has_protocol(&self) -> bool {
        self.protocol.is_some()
    }

    // Param is passed by value, moved
    pub fn set_protocol(&mut self, v: ::std::string::String) {
        self.protocol = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_protocol(&mut self) -> &mut ::std::string::String {
        if self.protocol.is_none() {
            self.protocol.set_default();
        }
        self.protocol.as_mut().unwrap()
    }

    // Take field
    pub fn take_protocol(&mut self) -> ::std::string::String {
        self.protocol.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_protocol(&self) -> &str {
        match self.protocol.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_protocol_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.protocol
    }

    fn mut_protocol_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.protocol
    }

    // optional .k8s.io.apimachinery.pkg.util.intstr.IntOrString port = 2;

    pub fn clear_port(&mut self) {
        self.port.clear();
    }

    pub fn has_port(&self) -> bool {
        self.port.is_some()
    }

    // Param is passed by value, moved
    pub fn set_port(&mut self, v: super::generated::IntOrString) {
        self.port = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_port(&mut self) -> &mut super::generated::IntOrString {
        if self.port.is_none() {
            self.port.set_default();
        }
        self.port.as_mut().unwrap()
    }

    // Take field
    pub fn take_port(&mut self) -> super::generated::IntOrString {
        self.port.take().unwrap_or_else(|| super::generated::IntOrString::new())
    }

    pub fn get_port(&self) -> &super::generated::IntOrString {
        self.port.as_ref().unwrap_or_else(|| super::generated::IntOrString::default_instance())
    }

    fn get_port_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::IntOrString> {
        &self.port
    }

    fn mut_port_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::IntOrString> {
        &mut self.port
    }
}

impl ::protobuf::Message for NetworkPolicyPort {
    fn is_initialized(&self) -> bool {
        for v in &self.port {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.protocol)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.port)?;
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
        if let Some(ref v) = self.protocol.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.port.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.protocol.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.port.as_ref() {
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

impl ::protobuf::MessageStatic for NetworkPolicyPort {
    fn new() -> NetworkPolicyPort {
        NetworkPolicyPort::new()
    }

    fn descriptor_static(_: ::std::option::Option<NetworkPolicyPort>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "protocol",
                    NetworkPolicyPort::get_protocol_for_reflect,
                    NetworkPolicyPort::mut_protocol_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::IntOrString>>(
                    "port",
                    NetworkPolicyPort::get_port_for_reflect,
                    NetworkPolicyPort::mut_port_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NetworkPolicyPort>(
                    "NetworkPolicyPort",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NetworkPolicyPort {
    fn clear(&mut self) {
        self.clear_protocol();
        self.clear_port();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NetworkPolicyPort {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NetworkPolicyPort {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NetworkPolicySpec {
    // message fields
    podSelector: ::protobuf::SingularPtrField<super::generated::LabelSelector>,
    ingress: ::protobuf::RepeatedField<NetworkPolicyIngressRule>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NetworkPolicySpec {}

impl NetworkPolicySpec {
    pub fn new() -> NetworkPolicySpec {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NetworkPolicySpec {
        static mut instance: ::protobuf::lazy::Lazy<NetworkPolicySpec> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NetworkPolicySpec,
        };
        unsafe {
            instance.get(NetworkPolicySpec::new)
        }
    }

    // optional .k8s.io.apimachinery.pkg.apis.meta.v1.LabelSelector podSelector = 1;

    pub fn clear_podSelector(&mut self) {
        self.podSelector.clear();
    }

    pub fn has_podSelector(&self) -> bool {
        self.podSelector.is_some()
    }

    // Param is passed by value, moved
    pub fn set_podSelector(&mut self, v: super::generated::LabelSelector) {
        self.podSelector = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_podSelector(&mut self) -> &mut super::generated::LabelSelector {
        if self.podSelector.is_none() {
            self.podSelector.set_default();
        }
        self.podSelector.as_mut().unwrap()
    }

    // Take field
    pub fn take_podSelector(&mut self) -> super::generated::LabelSelector {
        self.podSelector.take().unwrap_or_else(|| super::generated::LabelSelector::new())
    }

    pub fn get_podSelector(&self) -> &super::generated::LabelSelector {
        self.podSelector.as_ref().unwrap_or_else(|| super::generated::LabelSelector::default_instance())
    }

    fn get_podSelector_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::LabelSelector> {
        &self.podSelector
    }

    fn mut_podSelector_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::LabelSelector> {
        &mut self.podSelector
    }

    // repeated .k8s.io.api.networking.v1.NetworkPolicyIngressRule ingress = 2;

    pub fn clear_ingress(&mut self) {
        self.ingress.clear();
    }

    // Param is passed by value, moved
    pub fn set_ingress(&mut self, v: ::protobuf::RepeatedField<NetworkPolicyIngressRule>) {
        self.ingress = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ingress(&mut self) -> &mut ::protobuf::RepeatedField<NetworkPolicyIngressRule> {
        &mut self.ingress
    }

    // Take field
    pub fn take_ingress(&mut self) -> ::protobuf::RepeatedField<NetworkPolicyIngressRule> {
        ::std::mem::replace(&mut self.ingress, ::protobuf::RepeatedField::new())
    }

    pub fn get_ingress(&self) -> &[NetworkPolicyIngressRule] {
        &self.ingress
    }

    fn get_ingress_for_reflect(&self) -> &::protobuf::RepeatedField<NetworkPolicyIngressRule> {
        &self.ingress
    }

    fn mut_ingress_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<NetworkPolicyIngressRule> {
        &mut self.ingress
    }
}

impl ::protobuf::Message for NetworkPolicySpec {
    fn is_initialized(&self) -> bool {
        for v in &self.podSelector {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.ingress {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.podSelector)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.ingress)?;
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
        if let Some(ref v) = self.podSelector.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.ingress {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.podSelector.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.ingress {
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

impl ::protobuf::MessageStatic for NetworkPolicySpec {
    fn new() -> NetworkPolicySpec {
        NetworkPolicySpec::new()
    }

    fn descriptor_static(_: ::std::option::Option<NetworkPolicySpec>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::LabelSelector>>(
                    "podSelector",
                    NetworkPolicySpec::get_podSelector_for_reflect,
                    NetworkPolicySpec::mut_podSelector_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<NetworkPolicyIngressRule>>(
                    "ingress",
                    NetworkPolicySpec::get_ingress_for_reflect,
                    NetworkPolicySpec::mut_ingress_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NetworkPolicySpec>(
                    "NetworkPolicySpec",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NetworkPolicySpec {
    fn clear(&mut self) {
        self.clear_podSelector();
        self.clear_ingress();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NetworkPolicySpec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NetworkPolicySpec {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n(k8s.io/api/networking/v1/generated.proto\x12\x18k8s.io.api.networking\
    .v1\x1a\"k8s.io/api/core/v1/generated.proto\x1a-k8s.io/api/extensions/v1\
    beta1/generated.proto\x1a)k8s.io/api/policy/v1beta1/generated.proto\x1a4\
    k8s.io/apimachinery/pkg/apis/meta/v1/generated.proto\x1a/k8s.io/apimachi\
    nery/pkg/runtime/generated.proto\x1a6k8s.io/apimachinery/pkg/runtime/sch\
    ema/generated.proto\x1a3k8s.io/apimachinery/pkg/util/intstr/generated.pr\
    oto\"\x9e\x01\n\rNetworkPolicy\x12L\n\x08metadata\x18\x01\x20\x01(\x0b20\
    .k8s.io.apimachinery.pkg.apis.meta.v1.ObjectMetaR\x08metadata\x12?\n\x04\
    spec\x18\x02\x20\x01(\x0b2+.k8s.io.api.networking.v1.NetworkPolicySpecR\
    \x04spec\"\x9e\x01\n\x18NetworkPolicyIngressRule\x12A\n\x05ports\x18\x01\
    \x20\x03(\x0b2+.k8s.io.api.networking.v1.NetworkPolicyPortR\x05ports\x12\
    ?\n\x04from\x18\x02\x20\x03(\x0b2+.k8s.io.api.networking.v1.NetworkPolic\
    yPeerR\x04from\"\x9e\x01\n\x11NetworkPolicyList\x12J\n\x08metadata\x18\
    \x01\x20\x01(\x0b2..k8s.io.apimachinery.pkg.apis.meta.v1.ListMetaR\x08me\
    tadata\x12=\n\x05items\x18\x02\x20\x03(\x0b2'.k8s.io.api.networking.v1.N\
    etworkPolicyR\x05items\"\xcd\x01\n\x11NetworkPolicyPeer\x12U\n\x0bpodSel\
    ector\x18\x01\x20\x01(\x0b23.k8s.io.apimachinery.pkg.apis.meta.v1.LabelS\
    electorR\x0bpodSelector\x12a\n\x11namespaceSelector\x18\x02\x20\x01(\x0b\
    23.k8s.io.apimachinery.pkg.apis.meta.v1.LabelSelectorR\x11namespaceSelec\
    tor\"u\n\x11NetworkPolicyPort\x12\x1a\n\x08protocol\x18\x01\x20\x01(\tR\
    \x08protocol\x12D\n\x04port\x18\x02\x20\x01(\x0b20.k8s.io.apimachinery.p\
    kg.util.intstr.IntOrStringR\x04port\"\xb8\x01\n\x11NetworkPolicySpec\x12\
    U\n\x0bpodSelector\x18\x01\x20\x01(\x0b23.k8s.io.apimachinery.pkg.apis.m\
    eta.v1.LabelSelectorR\x0bpodSelector\x12L\n\x07ingress\x18\x02\x20\x03(\
    \x0b22.k8s.io.api.networking.v1.NetworkPolicyIngressRuleR\x07ingressB\
    \x04Z\x02v1\
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
