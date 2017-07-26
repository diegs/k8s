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
pub struct ClusterRole {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    rules: ::protobuf::RepeatedField<PolicyRule>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClusterRole {}

impl ClusterRole {
    pub fn new() -> ClusterRole {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClusterRole {
        static mut instance: ::protobuf::lazy::Lazy<ClusterRole> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClusterRole,
        };
        unsafe {
            instance.get(ClusterRole::new)
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

    // repeated .k8s.io.api.rbac.v1beta1.PolicyRule rules = 2;

    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }

    // Param is passed by value, moved
    pub fn set_rules(&mut self, v: ::protobuf::RepeatedField<PolicyRule>) {
        self.rules = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rules(&mut self) -> &mut ::protobuf::RepeatedField<PolicyRule> {
        &mut self.rules
    }

    // Take field
    pub fn take_rules(&mut self) -> ::protobuf::RepeatedField<PolicyRule> {
        ::std::mem::replace(&mut self.rules, ::protobuf::RepeatedField::new())
    }

    pub fn get_rules(&self) -> &[PolicyRule] {
        &self.rules
    }

    fn get_rules_for_reflect(&self) -> &::protobuf::RepeatedField<PolicyRule> {
        &self.rules
    }

    fn mut_rules_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PolicyRule> {
        &mut self.rules
    }
}

impl ::protobuf::Message for ClusterRole {
    fn is_initialized(&self) -> bool {
        for v in &self.metadata {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.rules {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rules)?;
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
        for value in &self.rules {
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
        for v in &self.rules {
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

impl ::protobuf::MessageStatic for ClusterRole {
    fn new() -> ClusterRole {
        ClusterRole::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClusterRole>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    ClusterRole::get_metadata_for_reflect,
                    ClusterRole::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PolicyRule>>(
                    "rules",
                    ClusterRole::get_rules_for_reflect,
                    ClusterRole::mut_rules_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClusterRole>(
                    "ClusterRole",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClusterRole {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_rules();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClusterRole {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClusterRole {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClusterRoleBinding {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    subjects: ::protobuf::RepeatedField<Subject>,
    roleRef: ::protobuf::SingularPtrField<RoleRef>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClusterRoleBinding {}

impl ClusterRoleBinding {
    pub fn new() -> ClusterRoleBinding {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClusterRoleBinding {
        static mut instance: ::protobuf::lazy::Lazy<ClusterRoleBinding> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClusterRoleBinding,
        };
        unsafe {
            instance.get(ClusterRoleBinding::new)
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

    // repeated .k8s.io.api.rbac.v1beta1.Subject subjects = 2;

    pub fn clear_subjects(&mut self) {
        self.subjects.clear();
    }

    // Param is passed by value, moved
    pub fn set_subjects(&mut self, v: ::protobuf::RepeatedField<Subject>) {
        self.subjects = v;
    }

    // Mutable pointer to the field.
    pub fn mut_subjects(&mut self) -> &mut ::protobuf::RepeatedField<Subject> {
        &mut self.subjects
    }

    // Take field
    pub fn take_subjects(&mut self) -> ::protobuf::RepeatedField<Subject> {
        ::std::mem::replace(&mut self.subjects, ::protobuf::RepeatedField::new())
    }

    pub fn get_subjects(&self) -> &[Subject] {
        &self.subjects
    }

    fn get_subjects_for_reflect(&self) -> &::protobuf::RepeatedField<Subject> {
        &self.subjects
    }

    fn mut_subjects_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Subject> {
        &mut self.subjects
    }

    // optional .k8s.io.api.rbac.v1beta1.RoleRef roleRef = 3;

    pub fn clear_roleRef(&mut self) {
        self.roleRef.clear();
    }

    pub fn has_roleRef(&self) -> bool {
        self.roleRef.is_some()
    }

    // Param is passed by value, moved
    pub fn set_roleRef(&mut self, v: RoleRef) {
        self.roleRef = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_roleRef(&mut self) -> &mut RoleRef {
        if self.roleRef.is_none() {
            self.roleRef.set_default();
        }
        self.roleRef.as_mut().unwrap()
    }

    // Take field
    pub fn take_roleRef(&mut self) -> RoleRef {
        self.roleRef.take().unwrap_or_else(|| RoleRef::new())
    }

    pub fn get_roleRef(&self) -> &RoleRef {
        self.roleRef.as_ref().unwrap_or_else(|| RoleRef::default_instance())
    }

    fn get_roleRef_for_reflect(&self) -> &::protobuf::SingularPtrField<RoleRef> {
        &self.roleRef
    }

    fn mut_roleRef_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RoleRef> {
        &mut self.roleRef
    }
}

impl ::protobuf::Message for ClusterRoleBinding {
    fn is_initialized(&self) -> bool {
        for v in &self.metadata {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.subjects {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.roleRef {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.subjects)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.roleRef)?;
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
        for value in &self.subjects {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.roleRef.as_ref() {
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
        for v in &self.subjects {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.roleRef.as_ref() {
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

impl ::protobuf::MessageStatic for ClusterRoleBinding {
    fn new() -> ClusterRoleBinding {
        ClusterRoleBinding::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClusterRoleBinding>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    ClusterRoleBinding::get_metadata_for_reflect,
                    ClusterRoleBinding::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Subject>>(
                    "subjects",
                    ClusterRoleBinding::get_subjects_for_reflect,
                    ClusterRoleBinding::mut_subjects_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RoleRef>>(
                    "roleRef",
                    ClusterRoleBinding::get_roleRef_for_reflect,
                    ClusterRoleBinding::mut_roleRef_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClusterRoleBinding>(
                    "ClusterRoleBinding",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClusterRoleBinding {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_subjects();
        self.clear_roleRef();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClusterRoleBinding {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClusterRoleBinding {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClusterRoleBindingList {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ListMeta>,
    items: ::protobuf::RepeatedField<ClusterRoleBinding>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClusterRoleBindingList {}

impl ClusterRoleBindingList {
    pub fn new() -> ClusterRoleBindingList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClusterRoleBindingList {
        static mut instance: ::protobuf::lazy::Lazy<ClusterRoleBindingList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClusterRoleBindingList,
        };
        unsafe {
            instance.get(ClusterRoleBindingList::new)
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

    // repeated .k8s.io.api.rbac.v1beta1.ClusterRoleBinding items = 2;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<ClusterRoleBinding>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<ClusterRoleBinding> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<ClusterRoleBinding> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[ClusterRoleBinding] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<ClusterRoleBinding> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ClusterRoleBinding> {
        &mut self.items
    }
}

impl ::protobuf::Message for ClusterRoleBindingList {
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

impl ::protobuf::MessageStatic for ClusterRoleBindingList {
    fn new() -> ClusterRoleBindingList {
        ClusterRoleBindingList::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClusterRoleBindingList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ListMeta>>(
                    "metadata",
                    ClusterRoleBindingList::get_metadata_for_reflect,
                    ClusterRoleBindingList::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ClusterRoleBinding>>(
                    "items",
                    ClusterRoleBindingList::get_items_for_reflect,
                    ClusterRoleBindingList::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClusterRoleBindingList>(
                    "ClusterRoleBindingList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClusterRoleBindingList {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClusterRoleBindingList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClusterRoleBindingList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ClusterRoleList {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ListMeta>,
    items: ::protobuf::RepeatedField<ClusterRole>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ClusterRoleList {}

impl ClusterRoleList {
    pub fn new() -> ClusterRoleList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ClusterRoleList {
        static mut instance: ::protobuf::lazy::Lazy<ClusterRoleList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ClusterRoleList,
        };
        unsafe {
            instance.get(ClusterRoleList::new)
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

    // repeated .k8s.io.api.rbac.v1beta1.ClusterRole items = 2;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<ClusterRole>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<ClusterRole> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<ClusterRole> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[ClusterRole] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<ClusterRole> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ClusterRole> {
        &mut self.items
    }
}

impl ::protobuf::Message for ClusterRoleList {
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

impl ::protobuf::MessageStatic for ClusterRoleList {
    fn new() -> ClusterRoleList {
        ClusterRoleList::new()
    }

    fn descriptor_static(_: ::std::option::Option<ClusterRoleList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ListMeta>>(
                    "metadata",
                    ClusterRoleList::get_metadata_for_reflect,
                    ClusterRoleList::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ClusterRole>>(
                    "items",
                    ClusterRoleList::get_items_for_reflect,
                    ClusterRoleList::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ClusterRoleList>(
                    "ClusterRoleList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ClusterRoleList {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ClusterRoleList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ClusterRoleList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PolicyRule {
    // message fields
    verbs: ::protobuf::RepeatedField<::std::string::String>,
    apiGroups: ::protobuf::RepeatedField<::std::string::String>,
    resources: ::protobuf::RepeatedField<::std::string::String>,
    resourceNames: ::protobuf::RepeatedField<::std::string::String>,
    nonResourceURLs: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PolicyRule {}

impl PolicyRule {
    pub fn new() -> PolicyRule {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PolicyRule {
        static mut instance: ::protobuf::lazy::Lazy<PolicyRule> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PolicyRule,
        };
        unsafe {
            instance.get(PolicyRule::new)
        }
    }

    // repeated string verbs = 1;

    pub fn clear_verbs(&mut self) {
        self.verbs.clear();
    }

    // Param is passed by value, moved
    pub fn set_verbs(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.verbs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_verbs(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.verbs
    }

    // Take field
    pub fn take_verbs(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.verbs, ::protobuf::RepeatedField::new())
    }

    pub fn get_verbs(&self) -> &[::std::string::String] {
        &self.verbs
    }

    fn get_verbs_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.verbs
    }

    fn mut_verbs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.verbs
    }

    // repeated string apiGroups = 2;

    pub fn clear_apiGroups(&mut self) {
        self.apiGroups.clear();
    }

    // Param is passed by value, moved
    pub fn set_apiGroups(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.apiGroups = v;
    }

    // Mutable pointer to the field.
    pub fn mut_apiGroups(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.apiGroups
    }

    // Take field
    pub fn take_apiGroups(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.apiGroups, ::protobuf::RepeatedField::new())
    }

    pub fn get_apiGroups(&self) -> &[::std::string::String] {
        &self.apiGroups
    }

    fn get_apiGroups_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.apiGroups
    }

    fn mut_apiGroups_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.apiGroups
    }

    // repeated string resources = 3;

    pub fn clear_resources(&mut self) {
        self.resources.clear();
    }

    // Param is passed by value, moved
    pub fn set_resources(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.resources = v;
    }

    // Mutable pointer to the field.
    pub fn mut_resources(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.resources
    }

    // Take field
    pub fn take_resources(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.resources, ::protobuf::RepeatedField::new())
    }

    pub fn get_resources(&self) -> &[::std::string::String] {
        &self.resources
    }

    fn get_resources_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.resources
    }

    fn mut_resources_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.resources
    }

    // repeated string resourceNames = 4;

    pub fn clear_resourceNames(&mut self) {
        self.resourceNames.clear();
    }

    // Param is passed by value, moved
    pub fn set_resourceNames(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.resourceNames = v;
    }

    // Mutable pointer to the field.
    pub fn mut_resourceNames(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.resourceNames
    }

    // Take field
    pub fn take_resourceNames(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.resourceNames, ::protobuf::RepeatedField::new())
    }

    pub fn get_resourceNames(&self) -> &[::std::string::String] {
        &self.resourceNames
    }

    fn get_resourceNames_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.resourceNames
    }

    fn mut_resourceNames_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.resourceNames
    }

    // repeated string nonResourceURLs = 5;

    pub fn clear_nonResourceURLs(&mut self) {
        self.nonResourceURLs.clear();
    }

    // Param is passed by value, moved
    pub fn set_nonResourceURLs(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.nonResourceURLs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_nonResourceURLs(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.nonResourceURLs
    }

    // Take field
    pub fn take_nonResourceURLs(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.nonResourceURLs, ::protobuf::RepeatedField::new())
    }

    pub fn get_nonResourceURLs(&self) -> &[::std::string::String] {
        &self.nonResourceURLs
    }

    fn get_nonResourceURLs_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.nonResourceURLs
    }

    fn mut_nonResourceURLs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.nonResourceURLs
    }
}

impl ::protobuf::Message for PolicyRule {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.verbs)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.apiGroups)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.resources)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.resourceNames)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.nonResourceURLs)?;
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
        for value in &self.verbs {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.apiGroups {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.resources {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.resourceNames {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in &self.nonResourceURLs {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.verbs {
            os.write_string(1, &v)?;
        };
        for v in &self.apiGroups {
            os.write_string(2, &v)?;
        };
        for v in &self.resources {
            os.write_string(3, &v)?;
        };
        for v in &self.resourceNames {
            os.write_string(4, &v)?;
        };
        for v in &self.nonResourceURLs {
            os.write_string(5, &v)?;
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

impl ::protobuf::MessageStatic for PolicyRule {
    fn new() -> PolicyRule {
        PolicyRule::new()
    }

    fn descriptor_static(_: ::std::option::Option<PolicyRule>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "verbs",
                    PolicyRule::get_verbs_for_reflect,
                    PolicyRule::mut_verbs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "apiGroups",
                    PolicyRule::get_apiGroups_for_reflect,
                    PolicyRule::mut_apiGroups_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "resources",
                    PolicyRule::get_resources_for_reflect,
                    PolicyRule::mut_resources_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "resourceNames",
                    PolicyRule::get_resourceNames_for_reflect,
                    PolicyRule::mut_resourceNames_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "nonResourceURLs",
                    PolicyRule::get_nonResourceURLs_for_reflect,
                    PolicyRule::mut_nonResourceURLs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PolicyRule>(
                    "PolicyRule",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PolicyRule {
    fn clear(&mut self) {
        self.clear_verbs();
        self.clear_apiGroups();
        self.clear_resources();
        self.clear_resourceNames();
        self.clear_nonResourceURLs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PolicyRule {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PolicyRule {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Role {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    rules: ::protobuf::RepeatedField<PolicyRule>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Role {}

impl Role {
    pub fn new() -> Role {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Role {
        static mut instance: ::protobuf::lazy::Lazy<Role> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Role,
        };
        unsafe {
            instance.get(Role::new)
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

    // repeated .k8s.io.api.rbac.v1beta1.PolicyRule rules = 2;

    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }

    // Param is passed by value, moved
    pub fn set_rules(&mut self, v: ::protobuf::RepeatedField<PolicyRule>) {
        self.rules = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rules(&mut self) -> &mut ::protobuf::RepeatedField<PolicyRule> {
        &mut self.rules
    }

    // Take field
    pub fn take_rules(&mut self) -> ::protobuf::RepeatedField<PolicyRule> {
        ::std::mem::replace(&mut self.rules, ::protobuf::RepeatedField::new())
    }

    pub fn get_rules(&self) -> &[PolicyRule] {
        &self.rules
    }

    fn get_rules_for_reflect(&self) -> &::protobuf::RepeatedField<PolicyRule> {
        &self.rules
    }

    fn mut_rules_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PolicyRule> {
        &mut self.rules
    }
}

impl ::protobuf::Message for Role {
    fn is_initialized(&self) -> bool {
        for v in &self.metadata {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.rules {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rules)?;
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
        for value in &self.rules {
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
        for v in &self.rules {
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

impl ::protobuf::MessageStatic for Role {
    fn new() -> Role {
        Role::new()
    }

    fn descriptor_static(_: ::std::option::Option<Role>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    Role::get_metadata_for_reflect,
                    Role::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PolicyRule>>(
                    "rules",
                    Role::get_rules_for_reflect,
                    Role::mut_rules_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Role>(
                    "Role",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Role {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_rules();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Role {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Role {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RoleBinding {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    subjects: ::protobuf::RepeatedField<Subject>,
    roleRef: ::protobuf::SingularPtrField<RoleRef>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RoleBinding {}

impl RoleBinding {
    pub fn new() -> RoleBinding {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RoleBinding {
        static mut instance: ::protobuf::lazy::Lazy<RoleBinding> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RoleBinding,
        };
        unsafe {
            instance.get(RoleBinding::new)
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

    // repeated .k8s.io.api.rbac.v1beta1.Subject subjects = 2;

    pub fn clear_subjects(&mut self) {
        self.subjects.clear();
    }

    // Param is passed by value, moved
    pub fn set_subjects(&mut self, v: ::protobuf::RepeatedField<Subject>) {
        self.subjects = v;
    }

    // Mutable pointer to the field.
    pub fn mut_subjects(&mut self) -> &mut ::protobuf::RepeatedField<Subject> {
        &mut self.subjects
    }

    // Take field
    pub fn take_subjects(&mut self) -> ::protobuf::RepeatedField<Subject> {
        ::std::mem::replace(&mut self.subjects, ::protobuf::RepeatedField::new())
    }

    pub fn get_subjects(&self) -> &[Subject] {
        &self.subjects
    }

    fn get_subjects_for_reflect(&self) -> &::protobuf::RepeatedField<Subject> {
        &self.subjects
    }

    fn mut_subjects_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Subject> {
        &mut self.subjects
    }

    // optional .k8s.io.api.rbac.v1beta1.RoleRef roleRef = 3;

    pub fn clear_roleRef(&mut self) {
        self.roleRef.clear();
    }

    pub fn has_roleRef(&self) -> bool {
        self.roleRef.is_some()
    }

    // Param is passed by value, moved
    pub fn set_roleRef(&mut self, v: RoleRef) {
        self.roleRef = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_roleRef(&mut self) -> &mut RoleRef {
        if self.roleRef.is_none() {
            self.roleRef.set_default();
        }
        self.roleRef.as_mut().unwrap()
    }

    // Take field
    pub fn take_roleRef(&mut self) -> RoleRef {
        self.roleRef.take().unwrap_or_else(|| RoleRef::new())
    }

    pub fn get_roleRef(&self) -> &RoleRef {
        self.roleRef.as_ref().unwrap_or_else(|| RoleRef::default_instance())
    }

    fn get_roleRef_for_reflect(&self) -> &::protobuf::SingularPtrField<RoleRef> {
        &self.roleRef
    }

    fn mut_roleRef_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RoleRef> {
        &mut self.roleRef
    }
}

impl ::protobuf::Message for RoleBinding {
    fn is_initialized(&self) -> bool {
        for v in &self.metadata {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.subjects {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.roleRef {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.subjects)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.roleRef)?;
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
        for value in &self.subjects {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.roleRef.as_ref() {
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
        for v in &self.subjects {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.roleRef.as_ref() {
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

impl ::protobuf::MessageStatic for RoleBinding {
    fn new() -> RoleBinding {
        RoleBinding::new()
    }

    fn descriptor_static(_: ::std::option::Option<RoleBinding>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    RoleBinding::get_metadata_for_reflect,
                    RoleBinding::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Subject>>(
                    "subjects",
                    RoleBinding::get_subjects_for_reflect,
                    RoleBinding::mut_subjects_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RoleRef>>(
                    "roleRef",
                    RoleBinding::get_roleRef_for_reflect,
                    RoleBinding::mut_roleRef_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RoleBinding>(
                    "RoleBinding",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RoleBinding {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_subjects();
        self.clear_roleRef();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RoleBinding {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RoleBinding {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RoleBindingList {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ListMeta>,
    items: ::protobuf::RepeatedField<RoleBinding>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RoleBindingList {}

impl RoleBindingList {
    pub fn new() -> RoleBindingList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RoleBindingList {
        static mut instance: ::protobuf::lazy::Lazy<RoleBindingList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RoleBindingList,
        };
        unsafe {
            instance.get(RoleBindingList::new)
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

    // repeated .k8s.io.api.rbac.v1beta1.RoleBinding items = 2;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<RoleBinding>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<RoleBinding> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<RoleBinding> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[RoleBinding] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<RoleBinding> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RoleBinding> {
        &mut self.items
    }
}

impl ::protobuf::Message for RoleBindingList {
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

impl ::protobuf::MessageStatic for RoleBindingList {
    fn new() -> RoleBindingList {
        RoleBindingList::new()
    }

    fn descriptor_static(_: ::std::option::Option<RoleBindingList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ListMeta>>(
                    "metadata",
                    RoleBindingList::get_metadata_for_reflect,
                    RoleBindingList::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RoleBinding>>(
                    "items",
                    RoleBindingList::get_items_for_reflect,
                    RoleBindingList::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RoleBindingList>(
                    "RoleBindingList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RoleBindingList {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RoleBindingList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RoleBindingList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RoleList {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ListMeta>,
    items: ::protobuf::RepeatedField<Role>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RoleList {}

impl RoleList {
    pub fn new() -> RoleList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RoleList {
        static mut instance: ::protobuf::lazy::Lazy<RoleList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RoleList,
        };
        unsafe {
            instance.get(RoleList::new)
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

    // repeated .k8s.io.api.rbac.v1beta1.Role items = 2;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<Role>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<Role> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<Role> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[Role] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<Role> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Role> {
        &mut self.items
    }
}

impl ::protobuf::Message for RoleList {
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

impl ::protobuf::MessageStatic for RoleList {
    fn new() -> RoleList {
        RoleList::new()
    }

    fn descriptor_static(_: ::std::option::Option<RoleList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ListMeta>>(
                    "metadata",
                    RoleList::get_metadata_for_reflect,
                    RoleList::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Role>>(
                    "items",
                    RoleList::get_items_for_reflect,
                    RoleList::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RoleList>(
                    "RoleList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RoleList {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RoleList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RoleList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RoleRef {
    // message fields
    apiGroup: ::protobuf::SingularField<::std::string::String>,
    kind: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RoleRef {}

impl RoleRef {
    pub fn new() -> RoleRef {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RoleRef {
        static mut instance: ::protobuf::lazy::Lazy<RoleRef> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RoleRef,
        };
        unsafe {
            instance.get(RoleRef::new)
        }
    }

    // optional string apiGroup = 1;

    pub fn clear_apiGroup(&mut self) {
        self.apiGroup.clear();
    }

    pub fn has_apiGroup(&self) -> bool {
        self.apiGroup.is_some()
    }

    // Param is passed by value, moved
    pub fn set_apiGroup(&mut self, v: ::std::string::String) {
        self.apiGroup = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_apiGroup(&mut self) -> &mut ::std::string::String {
        if self.apiGroup.is_none() {
            self.apiGroup.set_default();
        }
        self.apiGroup.as_mut().unwrap()
    }

    // Take field
    pub fn take_apiGroup(&mut self) -> ::std::string::String {
        self.apiGroup.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_apiGroup(&self) -> &str {
        match self.apiGroup.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_apiGroup_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.apiGroup
    }

    fn mut_apiGroup_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.apiGroup
    }

    // optional string kind = 2;

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

    // optional string name = 3;

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
}

impl ::protobuf::Message for RoleRef {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.apiGroup)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.kind)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
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
        if let Some(ref v) = self.apiGroup.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.kind.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.apiGroup.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.kind.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
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

impl ::protobuf::MessageStatic for RoleRef {
    fn new() -> RoleRef {
        RoleRef::new()
    }

    fn descriptor_static(_: ::std::option::Option<RoleRef>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "apiGroup",
                    RoleRef::get_apiGroup_for_reflect,
                    RoleRef::mut_apiGroup_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "kind",
                    RoleRef::get_kind_for_reflect,
                    RoleRef::mut_kind_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    RoleRef::get_name_for_reflect,
                    RoleRef::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RoleRef>(
                    "RoleRef",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RoleRef {
    fn clear(&mut self) {
        self.clear_apiGroup();
        self.clear_kind();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RoleRef {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RoleRef {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Subject {
    // message fields
    kind: ::protobuf::SingularField<::std::string::String>,
    apiGroup: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    namespace: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Subject {}

impl Subject {
    pub fn new() -> Subject {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Subject {
        static mut instance: ::protobuf::lazy::Lazy<Subject> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Subject,
        };
        unsafe {
            instance.get(Subject::new)
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

    // optional string apiGroup = 2;

    pub fn clear_apiGroup(&mut self) {
        self.apiGroup.clear();
    }

    pub fn has_apiGroup(&self) -> bool {
        self.apiGroup.is_some()
    }

    // Param is passed by value, moved
    pub fn set_apiGroup(&mut self, v: ::std::string::String) {
        self.apiGroup = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_apiGroup(&mut self) -> &mut ::std::string::String {
        if self.apiGroup.is_none() {
            self.apiGroup.set_default();
        }
        self.apiGroup.as_mut().unwrap()
    }

    // Take field
    pub fn take_apiGroup(&mut self) -> ::std::string::String {
        self.apiGroup.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_apiGroup(&self) -> &str {
        match self.apiGroup.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_apiGroup_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.apiGroup
    }

    fn mut_apiGroup_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.apiGroup
    }

    // optional string name = 3;

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

    // optional string namespace = 4;

    pub fn clear_namespace(&mut self) {
        self.namespace.clear();
    }

    pub fn has_namespace(&self) -> bool {
        self.namespace.is_some()
    }

    // Param is passed by value, moved
    pub fn set_namespace(&mut self, v: ::std::string::String) {
        self.namespace = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_namespace(&mut self) -> &mut ::std::string::String {
        if self.namespace.is_none() {
            self.namespace.set_default();
        }
        self.namespace.as_mut().unwrap()
    }

    // Take field
    pub fn take_namespace(&mut self) -> ::std::string::String {
        self.namespace.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_namespace(&self) -> &str {
        match self.namespace.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_namespace_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.namespace
    }

    fn mut_namespace_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.namespace
    }
}

impl ::protobuf::Message for Subject {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.apiGroup)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.namespace)?;
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
        if let Some(ref v) = self.apiGroup.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.namespace.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.kind.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.apiGroup.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.namespace.as_ref() {
            os.write_string(4, &v)?;
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

impl ::protobuf::MessageStatic for Subject {
    fn new() -> Subject {
        Subject::new()
    }

    fn descriptor_static(_: ::std::option::Option<Subject>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "kind",
                    Subject::get_kind_for_reflect,
                    Subject::mut_kind_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "apiGroup",
                    Subject::get_apiGroup_for_reflect,
                    Subject::mut_apiGroup_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Subject::get_name_for_reflect,
                    Subject::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "namespace",
                    Subject::get_namespace_for_reflect,
                    Subject::mut_namespace_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Subject>(
                    "Subject",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Subject {
    fn clear(&mut self) {
        self.clear_kind();
        self.clear_apiGroup();
        self.clear_name();
        self.clear_namespace();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Subject {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Subject {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'k8s.io/api/rbac/v1beta1/generated.proto\x12\x17k8s.io.api.rbac.v1beta\
    1\x1a4k8s.io/apimachinery/pkg/apis/meta/v1/generated.proto\x1a/k8s.io/ap\
    imachinery/pkg/runtime/generated.proto\x1a6k8s.io/apimachinery/pkg/runti\
    me/schema/generated.proto\x1a3k8s.io/apimachinery/pkg/util/intstr/genera\
    ted.proto\"\x96\x01\n\x0bClusterRole\x12L\n\x08metadata\x18\x01\x20\x01(\
    \x0b20.k8s.io.apimachinery.pkg.apis.meta.v1.ObjectMetaR\x08metadata\x129\
    \n\x05rules\x18\x02\x20\x03(\x0b2#.k8s.io.api.rbac.v1beta1.PolicyRuleR\
    \x05rules\"\xdc\x01\n\x12ClusterRoleBinding\x12L\n\x08metadata\x18\x01\
    \x20\x01(\x0b20.k8s.io.apimachinery.pkg.apis.meta.v1.ObjectMetaR\x08meta\
    data\x12<\n\x08subjects\x18\x02\x20\x03(\x0b2\x20.k8s.io.api.rbac.v1beta\
    1.SubjectR\x08subjects\x12:\n\x07roleRef\x18\x03\x20\x01(\x0b2\x20.k8s.i\
    o.api.rbac.v1beta1.RoleRefR\x07roleRef\"\xa7\x01\n\x16ClusterRoleBinding\
    List\x12J\n\x08metadata\x18\x01\x20\x01(\x0b2..k8s.io.apimachinery.pkg.a\
    pis.meta.v1.ListMetaR\x08metadata\x12A\n\x05items\x18\x02\x20\x03(\x0b2+\
    .k8s.io.api.rbac.v1beta1.ClusterRoleBindingR\x05items\"\x99\x01\n\x0fClu\
    sterRoleList\x12J\n\x08metadata\x18\x01\x20\x01(\x0b2..k8s.io.apimachine\
    ry.pkg.apis.meta.v1.ListMetaR\x08metadata\x12:\n\x05items\x18\x02\x20\
    \x03(\x0b2$.k8s.io.api.rbac.v1beta1.ClusterRoleR\x05items\"\xae\x01\n\nP\
    olicyRule\x12\x14\n\x05verbs\x18\x01\x20\x03(\tR\x05verbs\x12\x1c\n\tapi\
    Groups\x18\x02\x20\x03(\tR\tapiGroups\x12\x1c\n\tresources\x18\x03\x20\
    \x03(\tR\tresources\x12$\n\rresourceNames\x18\x04\x20\x03(\tR\rresourceN\
    ames\x12(\n\x0fnonResourceURLs\x18\x05\x20\x03(\tR\x0fnonResourceURLs\"\
    \x8f\x01\n\x04Role\x12L\n\x08metadata\x18\x01\x20\x01(\x0b20.k8s.io.apim\
    achinery.pkg.apis.meta.v1.ObjectMetaR\x08metadata\x129\n\x05rules\x18\
    \x02\x20\x03(\x0b2#.k8s.io.api.rbac.v1beta1.PolicyRuleR\x05rules\"\xd5\
    \x01\n\x0bRoleBinding\x12L\n\x08metadata\x18\x01\x20\x01(\x0b20.k8s.io.a\
    pimachinery.pkg.apis.meta.v1.ObjectMetaR\x08metadata\x12<\n\x08subjects\
    \x18\x02\x20\x03(\x0b2\x20.k8s.io.api.rbac.v1beta1.SubjectR\x08subjects\
    \x12:\n\x07roleRef\x18\x03\x20\x01(\x0b2\x20.k8s.io.api.rbac.v1beta1.Rol\
    eRefR\x07roleRef\"\x99\x01\n\x0fRoleBindingList\x12J\n\x08metadata\x18\
    \x01\x20\x01(\x0b2..k8s.io.apimachinery.pkg.apis.meta.v1.ListMetaR\x08me\
    tadata\x12:\n\x05items\x18\x02\x20\x03(\x0b2$.k8s.io.api.rbac.v1beta1.Ro\
    leBindingR\x05items\"\x8b\x01\n\x08RoleList\x12J\n\x08metadata\x18\x01\
    \x20\x01(\x0b2..k8s.io.apimachinery.pkg.apis.meta.v1.ListMetaR\x08metada\
    ta\x123\n\x05items\x18\x02\x20\x03(\x0b2\x1d.k8s.io.api.rbac.v1beta1.Rol\
    eR\x05items\"M\n\x07RoleRef\x12\x1a\n\x08apiGroup\x18\x01\x20\x01(\tR\
    \x08apiGroup\x12\x12\n\x04kind\x18\x02\x20\x01(\tR\x04kind\x12\x12\n\x04\
    name\x18\x03\x20\x01(\tR\x04name\"k\n\x07Subject\x12\x12\n\x04kind\x18\
    \x01\x20\x01(\tR\x04kind\x12\x1a\n\x08apiGroup\x18\x02\x20\x01(\tR\x08ap\
    iGroup\x12\x12\n\x04name\x18\x03\x20\x01(\tR\x04name\x12\x1c\n\tnamespac\
    e\x18\x04\x20\x01(\tR\tnamespaceB\tZ\x07v1beta1\
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
