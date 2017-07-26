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
pub struct LocalSubjectAccessReview {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    spec: ::protobuf::SingularPtrField<SubjectAccessReviewSpec>,
    status: ::protobuf::SingularPtrField<SubjectAccessReviewStatus>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LocalSubjectAccessReview {}

impl LocalSubjectAccessReview {
    pub fn new() -> LocalSubjectAccessReview {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LocalSubjectAccessReview {
        static mut instance: ::protobuf::lazy::Lazy<LocalSubjectAccessReview> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LocalSubjectAccessReview,
        };
        unsafe {
            instance.get(LocalSubjectAccessReview::new)
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

    // optional .k8s.io.api.authorization.v1beta1.SubjectAccessReviewSpec spec = 2;

    pub fn clear_spec(&mut self) {
        self.spec.clear();
    }

    pub fn has_spec(&self) -> bool {
        self.spec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spec(&mut self, v: SubjectAccessReviewSpec) {
        self.spec = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_spec(&mut self) -> &mut SubjectAccessReviewSpec {
        if self.spec.is_none() {
            self.spec.set_default();
        }
        self.spec.as_mut().unwrap()
    }

    // Take field
    pub fn take_spec(&mut self) -> SubjectAccessReviewSpec {
        self.spec.take().unwrap_or_else(|| SubjectAccessReviewSpec::new())
    }

    pub fn get_spec(&self) -> &SubjectAccessReviewSpec {
        self.spec.as_ref().unwrap_or_else(|| SubjectAccessReviewSpec::default_instance())
    }

    fn get_spec_for_reflect(&self) -> &::protobuf::SingularPtrField<SubjectAccessReviewSpec> {
        &self.spec
    }

    fn mut_spec_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SubjectAccessReviewSpec> {
        &mut self.spec
    }

    // optional .k8s.io.api.authorization.v1beta1.SubjectAccessReviewStatus status = 3;

    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: SubjectAccessReviewStatus) {
        self.status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut SubjectAccessReviewStatus {
        if self.status.is_none() {
            self.status.set_default();
        }
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> SubjectAccessReviewStatus {
        self.status.take().unwrap_or_else(|| SubjectAccessReviewStatus::new())
    }

    pub fn get_status(&self) -> &SubjectAccessReviewStatus {
        self.status.as_ref().unwrap_or_else(|| SubjectAccessReviewStatus::default_instance())
    }

    fn get_status_for_reflect(&self) -> &::protobuf::SingularPtrField<SubjectAccessReviewStatus> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SubjectAccessReviewStatus> {
        &mut self.status
    }
}

impl ::protobuf::Message for LocalSubjectAccessReview {
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

impl ::protobuf::MessageStatic for LocalSubjectAccessReview {
    fn new() -> LocalSubjectAccessReview {
        LocalSubjectAccessReview::new()
    }

    fn descriptor_static(_: ::std::option::Option<LocalSubjectAccessReview>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    LocalSubjectAccessReview::get_metadata_for_reflect,
                    LocalSubjectAccessReview::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SubjectAccessReviewSpec>>(
                    "spec",
                    LocalSubjectAccessReview::get_spec_for_reflect,
                    LocalSubjectAccessReview::mut_spec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SubjectAccessReviewStatus>>(
                    "status",
                    LocalSubjectAccessReview::get_status_for_reflect,
                    LocalSubjectAccessReview::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LocalSubjectAccessReview>(
                    "LocalSubjectAccessReview",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LocalSubjectAccessReview {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_spec();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for LocalSubjectAccessReview {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for LocalSubjectAccessReview {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct NonResourceAttributes {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    verb: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NonResourceAttributes {}

impl NonResourceAttributes {
    pub fn new() -> NonResourceAttributes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NonResourceAttributes {
        static mut instance: ::protobuf::lazy::Lazy<NonResourceAttributes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NonResourceAttributes,
        };
        unsafe {
            instance.get(NonResourceAttributes::new)
        }
    }

    // optional string path = 1;

    pub fn clear_path(&mut self) {
        self.path.clear();
    }

    pub fn has_path(&self) -> bool {
        self.path.is_some()
    }

    // Param is passed by value, moved
    pub fn set_path(&mut self, v: ::std::string::String) {
        self.path = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_path(&mut self) -> &mut ::std::string::String {
        if self.path.is_none() {
            self.path.set_default();
        }
        self.path.as_mut().unwrap()
    }

    // Take field
    pub fn take_path(&mut self) -> ::std::string::String {
        self.path.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_path(&self) -> &str {
        match self.path.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_path_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.path
    }

    fn mut_path_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.path
    }

    // optional string verb = 2;

    pub fn clear_verb(&mut self) {
        self.verb.clear();
    }

    pub fn has_verb(&self) -> bool {
        self.verb.is_some()
    }

    // Param is passed by value, moved
    pub fn set_verb(&mut self, v: ::std::string::String) {
        self.verb = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_verb(&mut self) -> &mut ::std::string::String {
        if self.verb.is_none() {
            self.verb.set_default();
        }
        self.verb.as_mut().unwrap()
    }

    // Take field
    pub fn take_verb(&mut self) -> ::std::string::String {
        self.verb.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_verb(&self) -> &str {
        match self.verb.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_verb_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.verb
    }

    fn mut_verb_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.verb
    }
}

impl ::protobuf::Message for NonResourceAttributes {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.verb)?;
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
        if let Some(ref v) = self.path.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.verb.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.verb.as_ref() {
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

impl ::protobuf::MessageStatic for NonResourceAttributes {
    fn new() -> NonResourceAttributes {
        NonResourceAttributes::new()
    }

    fn descriptor_static(_: ::std::option::Option<NonResourceAttributes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    NonResourceAttributes::get_path_for_reflect,
                    NonResourceAttributes::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "verb",
                    NonResourceAttributes::get_verb_for_reflect,
                    NonResourceAttributes::mut_verb_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NonResourceAttributes>(
                    "NonResourceAttributes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NonResourceAttributes {
    fn clear(&mut self) {
        self.clear_path();
        self.clear_verb();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for NonResourceAttributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for NonResourceAttributes {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ResourceAttributes {
    // message fields
    namespace: ::protobuf::SingularField<::std::string::String>,
    verb: ::protobuf::SingularField<::std::string::String>,
    group: ::protobuf::SingularField<::std::string::String>,
    version: ::protobuf::SingularField<::std::string::String>,
    resource: ::protobuf::SingularField<::std::string::String>,
    subresource: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResourceAttributes {}

impl ResourceAttributes {
    pub fn new() -> ResourceAttributes {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResourceAttributes {
        static mut instance: ::protobuf::lazy::Lazy<ResourceAttributes> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResourceAttributes,
        };
        unsafe {
            instance.get(ResourceAttributes::new)
        }
    }

    // optional string namespace = 1;

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

    // optional string verb = 2;

    pub fn clear_verb(&mut self) {
        self.verb.clear();
    }

    pub fn has_verb(&self) -> bool {
        self.verb.is_some()
    }

    // Param is passed by value, moved
    pub fn set_verb(&mut self, v: ::std::string::String) {
        self.verb = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_verb(&mut self) -> &mut ::std::string::String {
        if self.verb.is_none() {
            self.verb.set_default();
        }
        self.verb.as_mut().unwrap()
    }

    // Take field
    pub fn take_verb(&mut self) -> ::std::string::String {
        self.verb.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_verb(&self) -> &str {
        match self.verb.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_verb_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.verb
    }

    fn mut_verb_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.verb
    }

    // optional string group = 3;

    pub fn clear_group(&mut self) {
        self.group.clear();
    }

    pub fn has_group(&self) -> bool {
        self.group.is_some()
    }

    // Param is passed by value, moved
    pub fn set_group(&mut self, v: ::std::string::String) {
        self.group = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_group(&mut self) -> &mut ::std::string::String {
        if self.group.is_none() {
            self.group.set_default();
        }
        self.group.as_mut().unwrap()
    }

    // Take field
    pub fn take_group(&mut self) -> ::std::string::String {
        self.group.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_group(&self) -> &str {
        match self.group.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_group_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.group
    }

    fn mut_group_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.group
    }

    // optional string version = 4;

    pub fn clear_version(&mut self) {
        self.version.clear();
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: ::std::string::String) {
        self.version = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_version(&mut self) -> &mut ::std::string::String {
        if self.version.is_none() {
            self.version.set_default();
        }
        self.version.as_mut().unwrap()
    }

    // Take field
    pub fn take_version(&mut self) -> ::std::string::String {
        self.version.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_version(&self) -> &str {
        match self.version.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_version_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.version
    }

    // optional string resource = 5;

    pub fn clear_resource(&mut self) {
        self.resource.clear();
    }

    pub fn has_resource(&self) -> bool {
        self.resource.is_some()
    }

    // Param is passed by value, moved
    pub fn set_resource(&mut self, v: ::std::string::String) {
        self.resource = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_resource(&mut self) -> &mut ::std::string::String {
        if self.resource.is_none() {
            self.resource.set_default();
        }
        self.resource.as_mut().unwrap()
    }

    // Take field
    pub fn take_resource(&mut self) -> ::std::string::String {
        self.resource.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_resource(&self) -> &str {
        match self.resource.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_resource_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.resource
    }

    fn mut_resource_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.resource
    }

    // optional string subresource = 6;

    pub fn clear_subresource(&mut self) {
        self.subresource.clear();
    }

    pub fn has_subresource(&self) -> bool {
        self.subresource.is_some()
    }

    // Param is passed by value, moved
    pub fn set_subresource(&mut self, v: ::std::string::String) {
        self.subresource = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_subresource(&mut self) -> &mut ::std::string::String {
        if self.subresource.is_none() {
            self.subresource.set_default();
        }
        self.subresource.as_mut().unwrap()
    }

    // Take field
    pub fn take_subresource(&mut self) -> ::std::string::String {
        self.subresource.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_subresource(&self) -> &str {
        match self.subresource.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_subresource_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.subresource
    }

    fn mut_subresource_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.subresource
    }

    // optional string name = 7;

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

impl ::protobuf::Message for ResourceAttributes {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.namespace)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.verb)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.group)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.version)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.resource)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.subresource)?;
                },
                7 => {
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
        if let Some(ref v) = self.namespace.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.verb.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.group.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(ref v) = self.version.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        if let Some(ref v) = self.resource.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(ref v) = self.subresource.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
        }
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(7, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.namespace.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.verb.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.group.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(ref v) = self.version.as_ref() {
            os.write_string(4, &v)?;
        }
        if let Some(ref v) = self.resource.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(ref v) = self.subresource.as_ref() {
            os.write_string(6, &v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
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

impl ::protobuf::MessageStatic for ResourceAttributes {
    fn new() -> ResourceAttributes {
        ResourceAttributes::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResourceAttributes>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "namespace",
                    ResourceAttributes::get_namespace_for_reflect,
                    ResourceAttributes::mut_namespace_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "verb",
                    ResourceAttributes::get_verb_for_reflect,
                    ResourceAttributes::mut_verb_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "group",
                    ResourceAttributes::get_group_for_reflect,
                    ResourceAttributes::mut_group_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "version",
                    ResourceAttributes::get_version_for_reflect,
                    ResourceAttributes::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "resource",
                    ResourceAttributes::get_resource_for_reflect,
                    ResourceAttributes::mut_resource_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "subresource",
                    ResourceAttributes::get_subresource_for_reflect,
                    ResourceAttributes::mut_subresource_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ResourceAttributes::get_name_for_reflect,
                    ResourceAttributes::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResourceAttributes>(
                    "ResourceAttributes",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResourceAttributes {
    fn clear(&mut self) {
        self.clear_namespace();
        self.clear_verb();
        self.clear_group();
        self.clear_version();
        self.clear_resource();
        self.clear_subresource();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ResourceAttributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ResourceAttributes {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SelfSubjectAccessReview {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    spec: ::protobuf::SingularPtrField<SelfSubjectAccessReviewSpec>,
    status: ::protobuf::SingularPtrField<SubjectAccessReviewStatus>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SelfSubjectAccessReview {}

impl SelfSubjectAccessReview {
    pub fn new() -> SelfSubjectAccessReview {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SelfSubjectAccessReview {
        static mut instance: ::protobuf::lazy::Lazy<SelfSubjectAccessReview> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SelfSubjectAccessReview,
        };
        unsafe {
            instance.get(SelfSubjectAccessReview::new)
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

    // optional .k8s.io.api.authorization.v1beta1.SelfSubjectAccessReviewSpec spec = 2;

    pub fn clear_spec(&mut self) {
        self.spec.clear();
    }

    pub fn has_spec(&self) -> bool {
        self.spec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spec(&mut self, v: SelfSubjectAccessReviewSpec) {
        self.spec = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_spec(&mut self) -> &mut SelfSubjectAccessReviewSpec {
        if self.spec.is_none() {
            self.spec.set_default();
        }
        self.spec.as_mut().unwrap()
    }

    // Take field
    pub fn take_spec(&mut self) -> SelfSubjectAccessReviewSpec {
        self.spec.take().unwrap_or_else(|| SelfSubjectAccessReviewSpec::new())
    }

    pub fn get_spec(&self) -> &SelfSubjectAccessReviewSpec {
        self.spec.as_ref().unwrap_or_else(|| SelfSubjectAccessReviewSpec::default_instance())
    }

    fn get_spec_for_reflect(&self) -> &::protobuf::SingularPtrField<SelfSubjectAccessReviewSpec> {
        &self.spec
    }

    fn mut_spec_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SelfSubjectAccessReviewSpec> {
        &mut self.spec
    }

    // optional .k8s.io.api.authorization.v1beta1.SubjectAccessReviewStatus status = 3;

    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: SubjectAccessReviewStatus) {
        self.status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut SubjectAccessReviewStatus {
        if self.status.is_none() {
            self.status.set_default();
        }
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> SubjectAccessReviewStatus {
        self.status.take().unwrap_or_else(|| SubjectAccessReviewStatus::new())
    }

    pub fn get_status(&self) -> &SubjectAccessReviewStatus {
        self.status.as_ref().unwrap_or_else(|| SubjectAccessReviewStatus::default_instance())
    }

    fn get_status_for_reflect(&self) -> &::protobuf::SingularPtrField<SubjectAccessReviewStatus> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SubjectAccessReviewStatus> {
        &mut self.status
    }
}

impl ::protobuf::Message for SelfSubjectAccessReview {
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

impl ::protobuf::MessageStatic for SelfSubjectAccessReview {
    fn new() -> SelfSubjectAccessReview {
        SelfSubjectAccessReview::new()
    }

    fn descriptor_static(_: ::std::option::Option<SelfSubjectAccessReview>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    SelfSubjectAccessReview::get_metadata_for_reflect,
                    SelfSubjectAccessReview::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SelfSubjectAccessReviewSpec>>(
                    "spec",
                    SelfSubjectAccessReview::get_spec_for_reflect,
                    SelfSubjectAccessReview::mut_spec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SubjectAccessReviewStatus>>(
                    "status",
                    SelfSubjectAccessReview::get_status_for_reflect,
                    SelfSubjectAccessReview::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SelfSubjectAccessReview>(
                    "SelfSubjectAccessReview",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SelfSubjectAccessReview {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_spec();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SelfSubjectAccessReview {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SelfSubjectAccessReview {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SelfSubjectAccessReviewSpec {
    // message fields
    resourceAttributes: ::protobuf::SingularPtrField<ResourceAttributes>,
    nonResourceAttributes: ::protobuf::SingularPtrField<NonResourceAttributes>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SelfSubjectAccessReviewSpec {}

impl SelfSubjectAccessReviewSpec {
    pub fn new() -> SelfSubjectAccessReviewSpec {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SelfSubjectAccessReviewSpec {
        static mut instance: ::protobuf::lazy::Lazy<SelfSubjectAccessReviewSpec> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SelfSubjectAccessReviewSpec,
        };
        unsafe {
            instance.get(SelfSubjectAccessReviewSpec::new)
        }
    }

    // optional .k8s.io.api.authorization.v1beta1.ResourceAttributes resourceAttributes = 1;

    pub fn clear_resourceAttributes(&mut self) {
        self.resourceAttributes.clear();
    }

    pub fn has_resourceAttributes(&self) -> bool {
        self.resourceAttributes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_resourceAttributes(&mut self, v: ResourceAttributes) {
        self.resourceAttributes = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_resourceAttributes(&mut self) -> &mut ResourceAttributes {
        if self.resourceAttributes.is_none() {
            self.resourceAttributes.set_default();
        }
        self.resourceAttributes.as_mut().unwrap()
    }

    // Take field
    pub fn take_resourceAttributes(&mut self) -> ResourceAttributes {
        self.resourceAttributes.take().unwrap_or_else(|| ResourceAttributes::new())
    }

    pub fn get_resourceAttributes(&self) -> &ResourceAttributes {
        self.resourceAttributes.as_ref().unwrap_or_else(|| ResourceAttributes::default_instance())
    }

    fn get_resourceAttributes_for_reflect(&self) -> &::protobuf::SingularPtrField<ResourceAttributes> {
        &self.resourceAttributes
    }

    fn mut_resourceAttributes_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResourceAttributes> {
        &mut self.resourceAttributes
    }

    // optional .k8s.io.api.authorization.v1beta1.NonResourceAttributes nonResourceAttributes = 2;

    pub fn clear_nonResourceAttributes(&mut self) {
        self.nonResourceAttributes.clear();
    }

    pub fn has_nonResourceAttributes(&self) -> bool {
        self.nonResourceAttributes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nonResourceAttributes(&mut self, v: NonResourceAttributes) {
        self.nonResourceAttributes = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nonResourceAttributes(&mut self) -> &mut NonResourceAttributes {
        if self.nonResourceAttributes.is_none() {
            self.nonResourceAttributes.set_default();
        }
        self.nonResourceAttributes.as_mut().unwrap()
    }

    // Take field
    pub fn take_nonResourceAttributes(&mut self) -> NonResourceAttributes {
        self.nonResourceAttributes.take().unwrap_or_else(|| NonResourceAttributes::new())
    }

    pub fn get_nonResourceAttributes(&self) -> &NonResourceAttributes {
        self.nonResourceAttributes.as_ref().unwrap_or_else(|| NonResourceAttributes::default_instance())
    }

    fn get_nonResourceAttributes_for_reflect(&self) -> &::protobuf::SingularPtrField<NonResourceAttributes> {
        &self.nonResourceAttributes
    }

    fn mut_nonResourceAttributes_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<NonResourceAttributes> {
        &mut self.nonResourceAttributes
    }
}

impl ::protobuf::Message for SelfSubjectAccessReviewSpec {
    fn is_initialized(&self) -> bool {
        for v in &self.resourceAttributes {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.nonResourceAttributes {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.resourceAttributes)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.nonResourceAttributes)?;
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
        if let Some(ref v) = self.resourceAttributes.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.nonResourceAttributes.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.resourceAttributes.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.nonResourceAttributes.as_ref() {
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

impl ::protobuf::MessageStatic for SelfSubjectAccessReviewSpec {
    fn new() -> SelfSubjectAccessReviewSpec {
        SelfSubjectAccessReviewSpec::new()
    }

    fn descriptor_static(_: ::std::option::Option<SelfSubjectAccessReviewSpec>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResourceAttributes>>(
                    "resourceAttributes",
                    SelfSubjectAccessReviewSpec::get_resourceAttributes_for_reflect,
                    SelfSubjectAccessReviewSpec::mut_resourceAttributes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<NonResourceAttributes>>(
                    "nonResourceAttributes",
                    SelfSubjectAccessReviewSpec::get_nonResourceAttributes_for_reflect,
                    SelfSubjectAccessReviewSpec::mut_nonResourceAttributes_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SelfSubjectAccessReviewSpec>(
                    "SelfSubjectAccessReviewSpec",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SelfSubjectAccessReviewSpec {
    fn clear(&mut self) {
        self.clear_resourceAttributes();
        self.clear_nonResourceAttributes();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SelfSubjectAccessReviewSpec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SelfSubjectAccessReviewSpec {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SubjectAccessReview {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    spec: ::protobuf::SingularPtrField<SubjectAccessReviewSpec>,
    status: ::protobuf::SingularPtrField<SubjectAccessReviewStatus>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SubjectAccessReview {}

impl SubjectAccessReview {
    pub fn new() -> SubjectAccessReview {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SubjectAccessReview {
        static mut instance: ::protobuf::lazy::Lazy<SubjectAccessReview> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SubjectAccessReview,
        };
        unsafe {
            instance.get(SubjectAccessReview::new)
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

    // optional .k8s.io.api.authorization.v1beta1.SubjectAccessReviewSpec spec = 2;

    pub fn clear_spec(&mut self) {
        self.spec.clear();
    }

    pub fn has_spec(&self) -> bool {
        self.spec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spec(&mut self, v: SubjectAccessReviewSpec) {
        self.spec = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_spec(&mut self) -> &mut SubjectAccessReviewSpec {
        if self.spec.is_none() {
            self.spec.set_default();
        }
        self.spec.as_mut().unwrap()
    }

    // Take field
    pub fn take_spec(&mut self) -> SubjectAccessReviewSpec {
        self.spec.take().unwrap_or_else(|| SubjectAccessReviewSpec::new())
    }

    pub fn get_spec(&self) -> &SubjectAccessReviewSpec {
        self.spec.as_ref().unwrap_or_else(|| SubjectAccessReviewSpec::default_instance())
    }

    fn get_spec_for_reflect(&self) -> &::protobuf::SingularPtrField<SubjectAccessReviewSpec> {
        &self.spec
    }

    fn mut_spec_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SubjectAccessReviewSpec> {
        &mut self.spec
    }

    // optional .k8s.io.api.authorization.v1beta1.SubjectAccessReviewStatus status = 3;

    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: SubjectAccessReviewStatus) {
        self.status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut SubjectAccessReviewStatus {
        if self.status.is_none() {
            self.status.set_default();
        }
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> SubjectAccessReviewStatus {
        self.status.take().unwrap_or_else(|| SubjectAccessReviewStatus::new())
    }

    pub fn get_status(&self) -> &SubjectAccessReviewStatus {
        self.status.as_ref().unwrap_or_else(|| SubjectAccessReviewStatus::default_instance())
    }

    fn get_status_for_reflect(&self) -> &::protobuf::SingularPtrField<SubjectAccessReviewStatus> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SubjectAccessReviewStatus> {
        &mut self.status
    }
}

impl ::protobuf::Message for SubjectAccessReview {
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

impl ::protobuf::MessageStatic for SubjectAccessReview {
    fn new() -> SubjectAccessReview {
        SubjectAccessReview::new()
    }

    fn descriptor_static(_: ::std::option::Option<SubjectAccessReview>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    SubjectAccessReview::get_metadata_for_reflect,
                    SubjectAccessReview::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SubjectAccessReviewSpec>>(
                    "spec",
                    SubjectAccessReview::get_spec_for_reflect,
                    SubjectAccessReview::mut_spec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SubjectAccessReviewStatus>>(
                    "status",
                    SubjectAccessReview::get_status_for_reflect,
                    SubjectAccessReview::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SubjectAccessReview>(
                    "SubjectAccessReview",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SubjectAccessReview {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_spec();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SubjectAccessReview {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SubjectAccessReview {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SubjectAccessReviewSpec {
    // message fields
    resourceAttributes: ::protobuf::SingularPtrField<ResourceAttributes>,
    nonResourceAttributes: ::protobuf::SingularPtrField<NonResourceAttributes>,
    verb: ::protobuf::SingularField<::std::string::String>,
    group: ::protobuf::RepeatedField<::std::string::String>,
    pub extra: ::std::collections::HashMap<::std::string::String, ExtraValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SubjectAccessReviewSpec {}

impl SubjectAccessReviewSpec {
    pub fn new() -> SubjectAccessReviewSpec {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SubjectAccessReviewSpec {
        static mut instance: ::protobuf::lazy::Lazy<SubjectAccessReviewSpec> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SubjectAccessReviewSpec,
        };
        unsafe {
            instance.get(SubjectAccessReviewSpec::new)
        }
    }

    // optional .k8s.io.api.authorization.v1beta1.ResourceAttributes resourceAttributes = 1;

    pub fn clear_resourceAttributes(&mut self) {
        self.resourceAttributes.clear();
    }

    pub fn has_resourceAttributes(&self) -> bool {
        self.resourceAttributes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_resourceAttributes(&mut self, v: ResourceAttributes) {
        self.resourceAttributes = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_resourceAttributes(&mut self) -> &mut ResourceAttributes {
        if self.resourceAttributes.is_none() {
            self.resourceAttributes.set_default();
        }
        self.resourceAttributes.as_mut().unwrap()
    }

    // Take field
    pub fn take_resourceAttributes(&mut self) -> ResourceAttributes {
        self.resourceAttributes.take().unwrap_or_else(|| ResourceAttributes::new())
    }

    pub fn get_resourceAttributes(&self) -> &ResourceAttributes {
        self.resourceAttributes.as_ref().unwrap_or_else(|| ResourceAttributes::default_instance())
    }

    fn get_resourceAttributes_for_reflect(&self) -> &::protobuf::SingularPtrField<ResourceAttributes> {
        &self.resourceAttributes
    }

    fn mut_resourceAttributes_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ResourceAttributes> {
        &mut self.resourceAttributes
    }

    // optional .k8s.io.api.authorization.v1beta1.NonResourceAttributes nonResourceAttributes = 2;

    pub fn clear_nonResourceAttributes(&mut self) {
        self.nonResourceAttributes.clear();
    }

    pub fn has_nonResourceAttributes(&self) -> bool {
        self.nonResourceAttributes.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nonResourceAttributes(&mut self, v: NonResourceAttributes) {
        self.nonResourceAttributes = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_nonResourceAttributes(&mut self) -> &mut NonResourceAttributes {
        if self.nonResourceAttributes.is_none() {
            self.nonResourceAttributes.set_default();
        }
        self.nonResourceAttributes.as_mut().unwrap()
    }

    // Take field
    pub fn take_nonResourceAttributes(&mut self) -> NonResourceAttributes {
        self.nonResourceAttributes.take().unwrap_or_else(|| NonResourceAttributes::new())
    }

    pub fn get_nonResourceAttributes(&self) -> &NonResourceAttributes {
        self.nonResourceAttributes.as_ref().unwrap_or_else(|| NonResourceAttributes::default_instance())
    }

    fn get_nonResourceAttributes_for_reflect(&self) -> &::protobuf::SingularPtrField<NonResourceAttributes> {
        &self.nonResourceAttributes
    }

    fn mut_nonResourceAttributes_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<NonResourceAttributes> {
        &mut self.nonResourceAttributes
    }

    // optional string verb = 3;

    pub fn clear_verb(&mut self) {
        self.verb.clear();
    }

    pub fn has_verb(&self) -> bool {
        self.verb.is_some()
    }

    // Param is passed by value, moved
    pub fn set_verb(&mut self, v: ::std::string::String) {
        self.verb = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_verb(&mut self) -> &mut ::std::string::String {
        if self.verb.is_none() {
            self.verb.set_default();
        }
        self.verb.as_mut().unwrap()
    }

    // Take field
    pub fn take_verb(&mut self) -> ::std::string::String {
        self.verb.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_verb(&self) -> &str {
        match self.verb.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_verb_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.verb
    }

    fn mut_verb_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.verb
    }

    // repeated string group = 4;

    pub fn clear_group(&mut self) {
        self.group.clear();
    }

    // Param is passed by value, moved
    pub fn set_group(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.group = v;
    }

    // Mutable pointer to the field.
    pub fn mut_group(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.group
    }

    // Take field
    pub fn take_group(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.group, ::protobuf::RepeatedField::new())
    }

    pub fn get_group(&self) -> &[::std::string::String] {
        &self.group
    }

    fn get_group_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.group
    }

    fn mut_group_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.group
    }

    // repeated .k8s.io.api.authorization.v1beta1.SubjectAccessReviewSpec.ExtraEntry extra = 5;

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

impl ::protobuf::Message for SubjectAccessReviewSpec {
    fn is_initialized(&self) -> bool {
        for v in &self.resourceAttributes {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.nonResourceAttributes {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.resourceAttributes)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.nonResourceAttributes)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.verb)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.group)?;
                },
                5 => {
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
        if let Some(ref v) = self.resourceAttributes.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.nonResourceAttributes.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.verb.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        for value in &self.group {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<ExtraValue>>(5, &self.extra);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.resourceAttributes.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.nonResourceAttributes.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.verb.as_ref() {
            os.write_string(3, &v)?;
        }
        for v in &self.group {
            os.write_string(4, &v)?;
        };
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<ExtraValue>>(5, &self.extra, os)?;
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

impl ::protobuf::MessageStatic for SubjectAccessReviewSpec {
    fn new() -> SubjectAccessReviewSpec {
        SubjectAccessReviewSpec::new()
    }

    fn descriptor_static(_: ::std::option::Option<SubjectAccessReviewSpec>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ResourceAttributes>>(
                    "resourceAttributes",
                    SubjectAccessReviewSpec::get_resourceAttributes_for_reflect,
                    SubjectAccessReviewSpec::mut_resourceAttributes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<NonResourceAttributes>>(
                    "nonResourceAttributes",
                    SubjectAccessReviewSpec::get_nonResourceAttributes_for_reflect,
                    SubjectAccessReviewSpec::mut_nonResourceAttributes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "verb",
                    SubjectAccessReviewSpec::get_verb_for_reflect,
                    SubjectAccessReviewSpec::mut_verb_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "group",
                    SubjectAccessReviewSpec::get_group_for_reflect,
                    SubjectAccessReviewSpec::mut_group_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<ExtraValue>>(
                    "extra",
                    SubjectAccessReviewSpec::get_extra_for_reflect,
                    SubjectAccessReviewSpec::mut_extra_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SubjectAccessReviewSpec>(
                    "SubjectAccessReviewSpec",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SubjectAccessReviewSpec {
    fn clear(&mut self) {
        self.clear_resourceAttributes();
        self.clear_nonResourceAttributes();
        self.clear_verb();
        self.clear_group();
        self.clear_extra();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SubjectAccessReviewSpec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SubjectAccessReviewSpec {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SubjectAccessReviewStatus {
    // message fields
    allowed: ::std::option::Option<bool>,
    reason: ::protobuf::SingularField<::std::string::String>,
    evaluationError: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SubjectAccessReviewStatus {}

impl SubjectAccessReviewStatus {
    pub fn new() -> SubjectAccessReviewStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SubjectAccessReviewStatus {
        static mut instance: ::protobuf::lazy::Lazy<SubjectAccessReviewStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SubjectAccessReviewStatus,
        };
        unsafe {
            instance.get(SubjectAccessReviewStatus::new)
        }
    }

    // optional bool allowed = 1;

    pub fn clear_allowed(&mut self) {
        self.allowed = ::std::option::Option::None;
    }

    pub fn has_allowed(&self) -> bool {
        self.allowed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_allowed(&mut self, v: bool) {
        self.allowed = ::std::option::Option::Some(v);
    }

    pub fn get_allowed(&self) -> bool {
        self.allowed.unwrap_or(false)
    }

    fn get_allowed_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.allowed
    }

    fn mut_allowed_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.allowed
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

    // optional string evaluationError = 3;

    pub fn clear_evaluationError(&mut self) {
        self.evaluationError.clear();
    }

    pub fn has_evaluationError(&self) -> bool {
        self.evaluationError.is_some()
    }

    // Param is passed by value, moved
    pub fn set_evaluationError(&mut self, v: ::std::string::String) {
        self.evaluationError = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_evaluationError(&mut self) -> &mut ::std::string::String {
        if self.evaluationError.is_none() {
            self.evaluationError.set_default();
        }
        self.evaluationError.as_mut().unwrap()
    }

    // Take field
    pub fn take_evaluationError(&mut self) -> ::std::string::String {
        self.evaluationError.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_evaluationError(&self) -> &str {
        match self.evaluationError.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_evaluationError_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.evaluationError
    }

    fn mut_evaluationError_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.evaluationError
    }
}

impl ::protobuf::Message for SubjectAccessReviewStatus {
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
                    let tmp = is.read_bool()?;
                    self.allowed = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.reason)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.evaluationError)?;
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
        if let Some(v) = self.allowed {
            my_size += 2;
        }
        if let Some(ref v) = self.reason.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        if let Some(ref v) = self.evaluationError.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.allowed {
            os.write_bool(1, v)?;
        }
        if let Some(ref v) = self.reason.as_ref() {
            os.write_string(2, &v)?;
        }
        if let Some(ref v) = self.evaluationError.as_ref() {
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

impl ::protobuf::MessageStatic for SubjectAccessReviewStatus {
    fn new() -> SubjectAccessReviewStatus {
        SubjectAccessReviewStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<SubjectAccessReviewStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "allowed",
                    SubjectAccessReviewStatus::get_allowed_for_reflect,
                    SubjectAccessReviewStatus::mut_allowed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "reason",
                    SubjectAccessReviewStatus::get_reason_for_reflect,
                    SubjectAccessReviewStatus::mut_reason_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "evaluationError",
                    SubjectAccessReviewStatus::get_evaluationError_for_reflect,
                    SubjectAccessReviewStatus::mut_evaluationError_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SubjectAccessReviewStatus>(
                    "SubjectAccessReviewStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SubjectAccessReviewStatus {
    fn clear(&mut self) {
        self.clear_allowed();
        self.clear_reason();
        self.clear_evaluationError();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SubjectAccessReviewStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SubjectAccessReviewStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n0k8s.io/api/authorization/v1beta1/generated.proto\x12\x20k8s.io.api.au\
    thorization.v1beta1\x1a4k8s.io/apimachinery/pkg/apis/meta/v1/generated.p\
    roto\x1a/k8s.io/apimachinery/pkg/runtime/generated.proto\x1a6k8s.io/apim\
    achinery/pkg/runtime/schema/generated.proto\x1a3k8s.io/apimachinery/pkg/\
    util/intstr/generated.proto\"\"\n\nExtraValue\x12\x14\n\x05items\x18\x01\
    \x20\x03(\tR\x05items\"\x8c\x02\n\x18LocalSubjectAccessReview\x12L\n\x08\
    metadata\x18\x01\x20\x01(\x0b20.k8s.io.apimachinery.pkg.apis.meta.v1.Obj\
    ectMetaR\x08metadata\x12M\n\x04spec\x18\x02\x20\x01(\x0b29.k8s.io.api.au\
    thorization.v1beta1.SubjectAccessReviewSpecR\x04spec\x12S\n\x06status\
    \x18\x03\x20\x01(\x0b2;.k8s.io.api.authorization.v1beta1.SubjectAccessRe\
    viewStatusR\x06status\"?\n\x15NonResourceAttributes\x12\x12\n\x04path\
    \x18\x01\x20\x01(\tR\x04path\x12\x12\n\x04verb\x18\x02\x20\x01(\tR\x04ve\
    rb\"\xc8\x01\n\x12ResourceAttributes\x12\x1c\n\tnamespace\x18\x01\x20\
    \x01(\tR\tnamespace\x12\x12\n\x04verb\x18\x02\x20\x01(\tR\x04verb\x12\
    \x14\n\x05group\x18\x03\x20\x01(\tR\x05group\x12\x18\n\x07version\x18\
    \x04\x20\x01(\tR\x07version\x12\x1a\n\x08resource\x18\x05\x20\x01(\tR\
    \x08resource\x12\x20\n\x0bsubresource\x18\x06\x20\x01(\tR\x0bsubresource\
    \x12\x12\n\x04name\x18\x07\x20\x01(\tR\x04name\"\x8f\x02\n\x17SelfSubjec\
    tAccessReview\x12L\n\x08metadata\x18\x01\x20\x01(\x0b20.k8s.io.apimachin\
    ery.pkg.apis.meta.v1.ObjectMetaR\x08metadata\x12Q\n\x04spec\x18\x02\x20\
    \x01(\x0b2=.k8s.io.api.authorization.v1beta1.SelfSubjectAccessReviewSpec\
    R\x04spec\x12S\n\x06status\x18\x03\x20\x01(\x0b2;.k8s.io.api.authorizati\
    on.v1beta1.SubjectAccessReviewStatusR\x06status\"\xf2\x01\n\x1bSelfSubje\
    ctAccessReviewSpec\x12d\n\x12resourceAttributes\x18\x01\x20\x01(\x0b24.k\
    8s.io.api.authorization.v1beta1.ResourceAttributesR\x12resourceAttribute\
    s\x12m\n\x15nonResourceAttributes\x18\x02\x20\x01(\x0b27.k8s.io.api.auth\
    orization.v1beta1.NonResourceAttributesR\x15nonResourceAttributes\"\x87\
    \x02\n\x13SubjectAccessReview\x12L\n\x08metadata\x18\x01\x20\x01(\x0b20.\
    k8s.io.apimachinery.pkg.apis.meta.v1.ObjectMetaR\x08metadata\x12M\n\x04s\
    pec\x18\x02\x20\x01(\x0b29.k8s.io.api.authorization.v1beta1.SubjectAcces\
    sReviewSpecR\x04spec\x12S\n\x06status\x18\x03\x20\x01(\x0b2;.k8s.io.api.\
    authorization.v1beta1.SubjectAccessReviewStatusR\x06status\"\xdc\x03\n\
    \x17SubjectAccessReviewSpec\x12d\n\x12resourceAttributes\x18\x01\x20\x01\
    (\x0b24.k8s.io.api.authorization.v1beta1.ResourceAttributesR\x12resource\
    Attributes\x12m\n\x15nonResourceAttributes\x18\x02\x20\x01(\x0b27.k8s.io\
    .api.authorization.v1beta1.NonResourceAttributesR\x15nonResourceAttribut\
    es\x12\x12\n\x04verb\x18\x03\x20\x01(\tR\x04verb\x12\x14\n\x05group\x18\
    \x04\x20\x03(\tR\x05group\x12Z\n\x05extra\x18\x05\x20\x03(\x0b2D.k8s.io.\
    api.authorization.v1beta1.SubjectAccessReviewSpec.ExtraEntryR\x05extra\
    \x1af\n\nExtraEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12B\n\
    \x05value\x18\x02\x20\x01(\x0b2,.k8s.io.api.authorization.v1beta1.ExtraV\
    alueR\x05value:\x028\x01\"w\n\x19SubjectAccessReviewStatus\x12\x18\n\x07\
    allowed\x18\x01\x20\x01(\x08R\x07allowed\x12\x16\n\x06reason\x18\x02\x20\
    \x01(\tR\x06reason\x12(\n\x0fevaluationError\x18\x03\x20\x01(\tR\x0feval\
    uationErrorB\tZ\x07v1beta1\
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
