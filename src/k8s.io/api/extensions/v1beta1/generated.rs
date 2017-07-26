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
pub struct APIVersion {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for APIVersion {}

impl APIVersion {
    pub fn new() -> APIVersion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static APIVersion {
        static mut instance: ::protobuf::lazy::Lazy<APIVersion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const APIVersion,
        };
        unsafe {
            instance.get(APIVersion::new)
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
}

impl ::protobuf::Message for APIVersion {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
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

impl ::protobuf::MessageStatic for APIVersion {
    fn new() -> APIVersion {
        APIVersion::new()
    }

    fn descriptor_static(_: ::std::option::Option<APIVersion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    APIVersion::get_name_for_reflect,
                    APIVersion::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<APIVersion>(
                    "APIVersion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for APIVersion {
    fn clear(&mut self) {
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for APIVersion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for APIVersion {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CustomMetricCurrentStatus {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularPtrField<super::generated::Quantity>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CustomMetricCurrentStatus {}

impl CustomMetricCurrentStatus {
    pub fn new() -> CustomMetricCurrentStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CustomMetricCurrentStatus {
        static mut instance: ::protobuf::lazy::Lazy<CustomMetricCurrentStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CustomMetricCurrentStatus,
        };
        unsafe {
            instance.get(CustomMetricCurrentStatus::new)
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

    // optional .k8s.io.apimachinery.pkg.api.resource.Quantity value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: super::generated::Quantity) {
        self.value = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut super::generated::Quantity {
        if self.value.is_none() {
            self.value.set_default();
        }
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> super::generated::Quantity {
        self.value.take().unwrap_or_else(|| super::generated::Quantity::new())
    }

    pub fn get_value(&self) -> &super::generated::Quantity {
        self.value.as_ref().unwrap_or_else(|| super::generated::Quantity::default_instance())
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::Quantity> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::Quantity> {
        &mut self.value
    }
}

impl ::protobuf::Message for CustomMetricCurrentStatus {
    fn is_initialized(&self) -> bool {
        for v in &self.value {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.value)?;
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
        if let Some(ref v) = self.value.as_ref() {
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
        if let Some(ref v) = self.value.as_ref() {
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

impl ::protobuf::MessageStatic for CustomMetricCurrentStatus {
    fn new() -> CustomMetricCurrentStatus {
        CustomMetricCurrentStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<CustomMetricCurrentStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CustomMetricCurrentStatus::get_name_for_reflect,
                    CustomMetricCurrentStatus::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::Quantity>>(
                    "value",
                    CustomMetricCurrentStatus::get_value_for_reflect,
                    CustomMetricCurrentStatus::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CustomMetricCurrentStatus>(
                    "CustomMetricCurrentStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CustomMetricCurrentStatus {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CustomMetricCurrentStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CustomMetricCurrentStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CustomMetricCurrentStatusList {
    // message fields
    items: ::protobuf::RepeatedField<CustomMetricCurrentStatus>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CustomMetricCurrentStatusList {}

impl CustomMetricCurrentStatusList {
    pub fn new() -> CustomMetricCurrentStatusList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CustomMetricCurrentStatusList {
        static mut instance: ::protobuf::lazy::Lazy<CustomMetricCurrentStatusList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CustomMetricCurrentStatusList,
        };
        unsafe {
            instance.get(CustomMetricCurrentStatusList::new)
        }
    }

    // repeated .k8s.io.api.extensions.v1beta1.CustomMetricCurrentStatus items = 1;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<CustomMetricCurrentStatus>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<CustomMetricCurrentStatus> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<CustomMetricCurrentStatus> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[CustomMetricCurrentStatus] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<CustomMetricCurrentStatus> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CustomMetricCurrentStatus> {
        &mut self.items
    }
}

impl ::protobuf::Message for CustomMetricCurrentStatusList {
    fn is_initialized(&self) -> bool {
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
        for value in &self.items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.items {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CustomMetricCurrentStatusList {
    fn new() -> CustomMetricCurrentStatusList {
        CustomMetricCurrentStatusList::new()
    }

    fn descriptor_static(_: ::std::option::Option<CustomMetricCurrentStatusList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CustomMetricCurrentStatus>>(
                    "items",
                    CustomMetricCurrentStatusList::get_items_for_reflect,
                    CustomMetricCurrentStatusList::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CustomMetricCurrentStatusList>(
                    "CustomMetricCurrentStatusList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CustomMetricCurrentStatusList {
    fn clear(&mut self) {
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CustomMetricCurrentStatusList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CustomMetricCurrentStatusList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CustomMetricTarget {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    value: ::protobuf::SingularPtrField<super::generated::Quantity>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CustomMetricTarget {}

impl CustomMetricTarget {
    pub fn new() -> CustomMetricTarget {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CustomMetricTarget {
        static mut instance: ::protobuf::lazy::Lazy<CustomMetricTarget> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CustomMetricTarget,
        };
        unsafe {
            instance.get(CustomMetricTarget::new)
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

    // optional .k8s.io.apimachinery.pkg.api.resource.Quantity value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: super::generated::Quantity) {
        self.value = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut super::generated::Quantity {
        if self.value.is_none() {
            self.value.set_default();
        }
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> super::generated::Quantity {
        self.value.take().unwrap_or_else(|| super::generated::Quantity::new())
    }

    pub fn get_value(&self) -> &super::generated::Quantity {
        self.value.as_ref().unwrap_or_else(|| super::generated::Quantity::default_instance())
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::Quantity> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::Quantity> {
        &mut self.value
    }
}

impl ::protobuf::Message for CustomMetricTarget {
    fn is_initialized(&self) -> bool {
        for v in &self.value {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.value)?;
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
        if let Some(ref v) = self.value.as_ref() {
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
        if let Some(ref v) = self.value.as_ref() {
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

impl ::protobuf::MessageStatic for CustomMetricTarget {
    fn new() -> CustomMetricTarget {
        CustomMetricTarget::new()
    }

    fn descriptor_static(_: ::std::option::Option<CustomMetricTarget>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    CustomMetricTarget::get_name_for_reflect,
                    CustomMetricTarget::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::Quantity>>(
                    "value",
                    CustomMetricTarget::get_value_for_reflect,
                    CustomMetricTarget::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CustomMetricTarget>(
                    "CustomMetricTarget",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CustomMetricTarget {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CustomMetricTarget {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CustomMetricTarget {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CustomMetricTargetList {
    // message fields
    items: ::protobuf::RepeatedField<CustomMetricTarget>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CustomMetricTargetList {}

impl CustomMetricTargetList {
    pub fn new() -> CustomMetricTargetList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CustomMetricTargetList {
        static mut instance: ::protobuf::lazy::Lazy<CustomMetricTargetList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CustomMetricTargetList,
        };
        unsafe {
            instance.get(CustomMetricTargetList::new)
        }
    }

    // repeated .k8s.io.api.extensions.v1beta1.CustomMetricTarget items = 1;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<CustomMetricTarget>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<CustomMetricTarget> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<CustomMetricTarget> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[CustomMetricTarget] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<CustomMetricTarget> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CustomMetricTarget> {
        &mut self.items
    }
}

impl ::protobuf::Message for CustomMetricTargetList {
    fn is_initialized(&self) -> bool {
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
        for value in &self.items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.items {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for CustomMetricTargetList {
    fn new() -> CustomMetricTargetList {
        CustomMetricTargetList::new()
    }

    fn descriptor_static(_: ::std::option::Option<CustomMetricTargetList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CustomMetricTarget>>(
                    "items",
                    CustomMetricTargetList::get_items_for_reflect,
                    CustomMetricTargetList::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CustomMetricTargetList>(
                    "CustomMetricTargetList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CustomMetricTargetList {
    fn clear(&mut self) {
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CustomMetricTargetList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CustomMetricTargetList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DaemonSet {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    spec: ::protobuf::SingularPtrField<DaemonSetSpec>,
    status: ::protobuf::SingularPtrField<DaemonSetStatus>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DaemonSet {}

impl DaemonSet {
    pub fn new() -> DaemonSet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DaemonSet {
        static mut instance: ::protobuf::lazy::Lazy<DaemonSet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DaemonSet,
        };
        unsafe {
            instance.get(DaemonSet::new)
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

    // optional .k8s.io.api.extensions.v1beta1.DaemonSetSpec spec = 2;

    pub fn clear_spec(&mut self) {
        self.spec.clear();
    }

    pub fn has_spec(&self) -> bool {
        self.spec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spec(&mut self, v: DaemonSetSpec) {
        self.spec = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_spec(&mut self) -> &mut DaemonSetSpec {
        if self.spec.is_none() {
            self.spec.set_default();
        }
        self.spec.as_mut().unwrap()
    }

    // Take field
    pub fn take_spec(&mut self) -> DaemonSetSpec {
        self.spec.take().unwrap_or_else(|| DaemonSetSpec::new())
    }

    pub fn get_spec(&self) -> &DaemonSetSpec {
        self.spec.as_ref().unwrap_or_else(|| DaemonSetSpec::default_instance())
    }

    fn get_spec_for_reflect(&self) -> &::protobuf::SingularPtrField<DaemonSetSpec> {
        &self.spec
    }

    fn mut_spec_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DaemonSetSpec> {
        &mut self.spec
    }

    // optional .k8s.io.api.extensions.v1beta1.DaemonSetStatus status = 3;

    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: DaemonSetStatus) {
        self.status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut DaemonSetStatus {
        if self.status.is_none() {
            self.status.set_default();
        }
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> DaemonSetStatus {
        self.status.take().unwrap_or_else(|| DaemonSetStatus::new())
    }

    pub fn get_status(&self) -> &DaemonSetStatus {
        self.status.as_ref().unwrap_or_else(|| DaemonSetStatus::default_instance())
    }

    fn get_status_for_reflect(&self) -> &::protobuf::SingularPtrField<DaemonSetStatus> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DaemonSetStatus> {
        &mut self.status
    }
}

impl ::protobuf::Message for DaemonSet {
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

impl ::protobuf::MessageStatic for DaemonSet {
    fn new() -> DaemonSet {
        DaemonSet::new()
    }

    fn descriptor_static(_: ::std::option::Option<DaemonSet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    DaemonSet::get_metadata_for_reflect,
                    DaemonSet::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DaemonSetSpec>>(
                    "spec",
                    DaemonSet::get_spec_for_reflect,
                    DaemonSet::mut_spec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DaemonSetStatus>>(
                    "status",
                    DaemonSet::get_status_for_reflect,
                    DaemonSet::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DaemonSet>(
                    "DaemonSet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DaemonSet {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_spec();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DaemonSet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DaemonSet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DaemonSetList {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ListMeta>,
    items: ::protobuf::RepeatedField<DaemonSet>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DaemonSetList {}

impl DaemonSetList {
    pub fn new() -> DaemonSetList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DaemonSetList {
        static mut instance: ::protobuf::lazy::Lazy<DaemonSetList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DaemonSetList,
        };
        unsafe {
            instance.get(DaemonSetList::new)
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

    // repeated .k8s.io.api.extensions.v1beta1.DaemonSet items = 2;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<DaemonSet>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<DaemonSet> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<DaemonSet> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[DaemonSet] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<DaemonSet> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<DaemonSet> {
        &mut self.items
    }
}

impl ::protobuf::Message for DaemonSetList {
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

impl ::protobuf::MessageStatic for DaemonSetList {
    fn new() -> DaemonSetList {
        DaemonSetList::new()
    }

    fn descriptor_static(_: ::std::option::Option<DaemonSetList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ListMeta>>(
                    "metadata",
                    DaemonSetList::get_metadata_for_reflect,
                    DaemonSetList::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DaemonSet>>(
                    "items",
                    DaemonSetList::get_items_for_reflect,
                    DaemonSetList::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DaemonSetList>(
                    "DaemonSetList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DaemonSetList {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DaemonSetList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DaemonSetList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DaemonSetSpec {
    // message fields
    selector: ::protobuf::SingularPtrField<super::generated::LabelSelector>,
    template: ::protobuf::SingularPtrField<super::generated::PodTemplateSpec>,
    updateStrategy: ::protobuf::SingularPtrField<DaemonSetUpdateStrategy>,
    minReadySeconds: ::std::option::Option<i32>,
    templateGeneration: ::std::option::Option<i64>,
    revisionHistoryLimit: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DaemonSetSpec {}

impl DaemonSetSpec {
    pub fn new() -> DaemonSetSpec {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DaemonSetSpec {
        static mut instance: ::protobuf::lazy::Lazy<DaemonSetSpec> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DaemonSetSpec,
        };
        unsafe {
            instance.get(DaemonSetSpec::new)
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

    // optional .k8s.io.api.core.v1.PodTemplateSpec template = 2;

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

    // optional .k8s.io.api.extensions.v1beta1.DaemonSetUpdateStrategy updateStrategy = 3;

    pub fn clear_updateStrategy(&mut self) {
        self.updateStrategy.clear();
    }

    pub fn has_updateStrategy(&self) -> bool {
        self.updateStrategy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_updateStrategy(&mut self, v: DaemonSetUpdateStrategy) {
        self.updateStrategy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_updateStrategy(&mut self) -> &mut DaemonSetUpdateStrategy {
        if self.updateStrategy.is_none() {
            self.updateStrategy.set_default();
        }
        self.updateStrategy.as_mut().unwrap()
    }

    // Take field
    pub fn take_updateStrategy(&mut self) -> DaemonSetUpdateStrategy {
        self.updateStrategy.take().unwrap_or_else(|| DaemonSetUpdateStrategy::new())
    }

    pub fn get_updateStrategy(&self) -> &DaemonSetUpdateStrategy {
        self.updateStrategy.as_ref().unwrap_or_else(|| DaemonSetUpdateStrategy::default_instance())
    }

    fn get_updateStrategy_for_reflect(&self) -> &::protobuf::SingularPtrField<DaemonSetUpdateStrategy> {
        &self.updateStrategy
    }

    fn mut_updateStrategy_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DaemonSetUpdateStrategy> {
        &mut self.updateStrategy
    }

    // optional int32 minReadySeconds = 4;

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

    // optional int64 templateGeneration = 5;

    pub fn clear_templateGeneration(&mut self) {
        self.templateGeneration = ::std::option::Option::None;
    }

    pub fn has_templateGeneration(&self) -> bool {
        self.templateGeneration.is_some()
    }

    // Param is passed by value, moved
    pub fn set_templateGeneration(&mut self, v: i64) {
        self.templateGeneration = ::std::option::Option::Some(v);
    }

    pub fn get_templateGeneration(&self) -> i64 {
        self.templateGeneration.unwrap_or(0)
    }

    fn get_templateGeneration_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.templateGeneration
    }

    fn mut_templateGeneration_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.templateGeneration
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
}

impl ::protobuf::Message for DaemonSetSpec {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.selector)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.template)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.updateStrategy)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.minReadySeconds = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.templateGeneration = ::std::option::Option::Some(tmp);
                },
                6 => {
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
        if let Some(ref v) = self.selector.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.template.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.updateStrategy.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.minReadySeconds {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.templateGeneration {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.revisionHistoryLimit {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
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
        if let Some(ref v) = self.template.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.updateStrategy.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.minReadySeconds {
            os.write_int32(4, v)?;
        }
        if let Some(v) = self.templateGeneration {
            os.write_int64(5, v)?;
        }
        if let Some(v) = self.revisionHistoryLimit {
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

impl ::protobuf::MessageStatic for DaemonSetSpec {
    fn new() -> DaemonSetSpec {
        DaemonSetSpec::new()
    }

    fn descriptor_static(_: ::std::option::Option<DaemonSetSpec>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::LabelSelector>>(
                    "selector",
                    DaemonSetSpec::get_selector_for_reflect,
                    DaemonSetSpec::mut_selector_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::PodTemplateSpec>>(
                    "template",
                    DaemonSetSpec::get_template_for_reflect,
                    DaemonSetSpec::mut_template_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DaemonSetUpdateStrategy>>(
                    "updateStrategy",
                    DaemonSetSpec::get_updateStrategy_for_reflect,
                    DaemonSetSpec::mut_updateStrategy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "minReadySeconds",
                    DaemonSetSpec::get_minReadySeconds_for_reflect,
                    DaemonSetSpec::mut_minReadySeconds_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "templateGeneration",
                    DaemonSetSpec::get_templateGeneration_for_reflect,
                    DaemonSetSpec::mut_templateGeneration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "revisionHistoryLimit",
                    DaemonSetSpec::get_revisionHistoryLimit_for_reflect,
                    DaemonSetSpec::mut_revisionHistoryLimit_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DaemonSetSpec>(
                    "DaemonSetSpec",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DaemonSetSpec {
    fn clear(&mut self) {
        self.clear_selector();
        self.clear_template();
        self.clear_updateStrategy();
        self.clear_minReadySeconds();
        self.clear_templateGeneration();
        self.clear_revisionHistoryLimit();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DaemonSetSpec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DaemonSetSpec {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DaemonSetStatus {
    // message fields
    currentNumberScheduled: ::std::option::Option<i32>,
    numberMisscheduled: ::std::option::Option<i32>,
    desiredNumberScheduled: ::std::option::Option<i32>,
    numberReady: ::std::option::Option<i32>,
    observedGeneration: ::std::option::Option<i64>,
    updatedNumberScheduled: ::std::option::Option<i32>,
    numberAvailable: ::std::option::Option<i32>,
    numberUnavailable: ::std::option::Option<i32>,
    collisionCount: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DaemonSetStatus {}

impl DaemonSetStatus {
    pub fn new() -> DaemonSetStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DaemonSetStatus {
        static mut instance: ::protobuf::lazy::Lazy<DaemonSetStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DaemonSetStatus,
        };
        unsafe {
            instance.get(DaemonSetStatus::new)
        }
    }

    // optional int32 currentNumberScheduled = 1;

    pub fn clear_currentNumberScheduled(&mut self) {
        self.currentNumberScheduled = ::std::option::Option::None;
    }

    pub fn has_currentNumberScheduled(&self) -> bool {
        self.currentNumberScheduled.is_some()
    }

    // Param is passed by value, moved
    pub fn set_currentNumberScheduled(&mut self, v: i32) {
        self.currentNumberScheduled = ::std::option::Option::Some(v);
    }

    pub fn get_currentNumberScheduled(&self) -> i32 {
        self.currentNumberScheduled.unwrap_or(0)
    }

    fn get_currentNumberScheduled_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.currentNumberScheduled
    }

    fn mut_currentNumberScheduled_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.currentNumberScheduled
    }

    // optional int32 numberMisscheduled = 2;

    pub fn clear_numberMisscheduled(&mut self) {
        self.numberMisscheduled = ::std::option::Option::None;
    }

    pub fn has_numberMisscheduled(&self) -> bool {
        self.numberMisscheduled.is_some()
    }

    // Param is passed by value, moved
    pub fn set_numberMisscheduled(&mut self, v: i32) {
        self.numberMisscheduled = ::std::option::Option::Some(v);
    }

    pub fn get_numberMisscheduled(&self) -> i32 {
        self.numberMisscheduled.unwrap_or(0)
    }

    fn get_numberMisscheduled_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.numberMisscheduled
    }

    fn mut_numberMisscheduled_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.numberMisscheduled
    }

    // optional int32 desiredNumberScheduled = 3;

    pub fn clear_desiredNumberScheduled(&mut self) {
        self.desiredNumberScheduled = ::std::option::Option::None;
    }

    pub fn has_desiredNumberScheduled(&self) -> bool {
        self.desiredNumberScheduled.is_some()
    }

    // Param is passed by value, moved
    pub fn set_desiredNumberScheduled(&mut self, v: i32) {
        self.desiredNumberScheduled = ::std::option::Option::Some(v);
    }

    pub fn get_desiredNumberScheduled(&self) -> i32 {
        self.desiredNumberScheduled.unwrap_or(0)
    }

    fn get_desiredNumberScheduled_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.desiredNumberScheduled
    }

    fn mut_desiredNumberScheduled_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.desiredNumberScheduled
    }

    // optional int32 numberReady = 4;

    pub fn clear_numberReady(&mut self) {
        self.numberReady = ::std::option::Option::None;
    }

    pub fn has_numberReady(&self) -> bool {
        self.numberReady.is_some()
    }

    // Param is passed by value, moved
    pub fn set_numberReady(&mut self, v: i32) {
        self.numberReady = ::std::option::Option::Some(v);
    }

    pub fn get_numberReady(&self) -> i32 {
        self.numberReady.unwrap_or(0)
    }

    fn get_numberReady_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.numberReady
    }

    fn mut_numberReady_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.numberReady
    }

    // optional int64 observedGeneration = 5;

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

    // optional int32 updatedNumberScheduled = 6;

    pub fn clear_updatedNumberScheduled(&mut self) {
        self.updatedNumberScheduled = ::std::option::Option::None;
    }

    pub fn has_updatedNumberScheduled(&self) -> bool {
        self.updatedNumberScheduled.is_some()
    }

    // Param is passed by value, moved
    pub fn set_updatedNumberScheduled(&mut self, v: i32) {
        self.updatedNumberScheduled = ::std::option::Option::Some(v);
    }

    pub fn get_updatedNumberScheduled(&self) -> i32 {
        self.updatedNumberScheduled.unwrap_or(0)
    }

    fn get_updatedNumberScheduled_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.updatedNumberScheduled
    }

    fn mut_updatedNumberScheduled_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.updatedNumberScheduled
    }

    // optional int32 numberAvailable = 7;

    pub fn clear_numberAvailable(&mut self) {
        self.numberAvailable = ::std::option::Option::None;
    }

    pub fn has_numberAvailable(&self) -> bool {
        self.numberAvailable.is_some()
    }

    // Param is passed by value, moved
    pub fn set_numberAvailable(&mut self, v: i32) {
        self.numberAvailable = ::std::option::Option::Some(v);
    }

    pub fn get_numberAvailable(&self) -> i32 {
        self.numberAvailable.unwrap_or(0)
    }

    fn get_numberAvailable_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.numberAvailable
    }

    fn mut_numberAvailable_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.numberAvailable
    }

    // optional int32 numberUnavailable = 8;

    pub fn clear_numberUnavailable(&mut self) {
        self.numberUnavailable = ::std::option::Option::None;
    }

    pub fn has_numberUnavailable(&self) -> bool {
        self.numberUnavailable.is_some()
    }

    // Param is passed by value, moved
    pub fn set_numberUnavailable(&mut self, v: i32) {
        self.numberUnavailable = ::std::option::Option::Some(v);
    }

    pub fn get_numberUnavailable(&self) -> i32 {
        self.numberUnavailable.unwrap_or(0)
    }

    fn get_numberUnavailable_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.numberUnavailable
    }

    fn mut_numberUnavailable_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.numberUnavailable
    }

    // optional int64 collisionCount = 9;

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

impl ::protobuf::Message for DaemonSetStatus {
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
                    self.currentNumberScheduled = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.numberMisscheduled = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.desiredNumberScheduled = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.numberReady = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.observedGeneration = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.updatedNumberScheduled = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.numberAvailable = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.numberUnavailable = ::std::option::Option::Some(tmp);
                },
                9 => {
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
        if let Some(v) = self.currentNumberScheduled {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.numberMisscheduled {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.desiredNumberScheduled {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.numberReady {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.observedGeneration {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.updatedNumberScheduled {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.numberAvailable {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.numberUnavailable {
            my_size += ::protobuf::rt::value_size(8, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.collisionCount {
            my_size += ::protobuf::rt::value_size(9, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.currentNumberScheduled {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.numberMisscheduled {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.desiredNumberScheduled {
            os.write_int32(3, v)?;
        }
        if let Some(v) = self.numberReady {
            os.write_int32(4, v)?;
        }
        if let Some(v) = self.observedGeneration {
            os.write_int64(5, v)?;
        }
        if let Some(v) = self.updatedNumberScheduled {
            os.write_int32(6, v)?;
        }
        if let Some(v) = self.numberAvailable {
            os.write_int32(7, v)?;
        }
        if let Some(v) = self.numberUnavailable {
            os.write_int32(8, v)?;
        }
        if let Some(v) = self.collisionCount {
            os.write_int64(9, v)?;
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

impl ::protobuf::MessageStatic for DaemonSetStatus {
    fn new() -> DaemonSetStatus {
        DaemonSetStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<DaemonSetStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "currentNumberScheduled",
                    DaemonSetStatus::get_currentNumberScheduled_for_reflect,
                    DaemonSetStatus::mut_currentNumberScheduled_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "numberMisscheduled",
                    DaemonSetStatus::get_numberMisscheduled_for_reflect,
                    DaemonSetStatus::mut_numberMisscheduled_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "desiredNumberScheduled",
                    DaemonSetStatus::get_desiredNumberScheduled_for_reflect,
                    DaemonSetStatus::mut_desiredNumberScheduled_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "numberReady",
                    DaemonSetStatus::get_numberReady_for_reflect,
                    DaemonSetStatus::mut_numberReady_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "observedGeneration",
                    DaemonSetStatus::get_observedGeneration_for_reflect,
                    DaemonSetStatus::mut_observedGeneration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "updatedNumberScheduled",
                    DaemonSetStatus::get_updatedNumberScheduled_for_reflect,
                    DaemonSetStatus::mut_updatedNumberScheduled_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "numberAvailable",
                    DaemonSetStatus::get_numberAvailable_for_reflect,
                    DaemonSetStatus::mut_numberAvailable_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "numberUnavailable",
                    DaemonSetStatus::get_numberUnavailable_for_reflect,
                    DaemonSetStatus::mut_numberUnavailable_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "collisionCount",
                    DaemonSetStatus::get_collisionCount_for_reflect,
                    DaemonSetStatus::mut_collisionCount_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DaemonSetStatus>(
                    "DaemonSetStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DaemonSetStatus {
    fn clear(&mut self) {
        self.clear_currentNumberScheduled();
        self.clear_numberMisscheduled();
        self.clear_desiredNumberScheduled();
        self.clear_numberReady();
        self.clear_observedGeneration();
        self.clear_updatedNumberScheduled();
        self.clear_numberAvailable();
        self.clear_numberUnavailable();
        self.clear_collisionCount();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DaemonSetStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DaemonSetStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DaemonSetUpdateStrategy {
    // message fields
    field_type: ::protobuf::SingularField<::std::string::String>,
    rollingUpdate: ::protobuf::SingularPtrField<RollingUpdateDaemonSet>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DaemonSetUpdateStrategy {}

impl DaemonSetUpdateStrategy {
    pub fn new() -> DaemonSetUpdateStrategy {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DaemonSetUpdateStrategy {
        static mut instance: ::protobuf::lazy::Lazy<DaemonSetUpdateStrategy> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DaemonSetUpdateStrategy,
        };
        unsafe {
            instance.get(DaemonSetUpdateStrategy::new)
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

    // optional .k8s.io.api.extensions.v1beta1.RollingUpdateDaemonSet rollingUpdate = 2;

    pub fn clear_rollingUpdate(&mut self) {
        self.rollingUpdate.clear();
    }

    pub fn has_rollingUpdate(&self) -> bool {
        self.rollingUpdate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rollingUpdate(&mut self, v: RollingUpdateDaemonSet) {
        self.rollingUpdate = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rollingUpdate(&mut self) -> &mut RollingUpdateDaemonSet {
        if self.rollingUpdate.is_none() {
            self.rollingUpdate.set_default();
        }
        self.rollingUpdate.as_mut().unwrap()
    }

    // Take field
    pub fn take_rollingUpdate(&mut self) -> RollingUpdateDaemonSet {
        self.rollingUpdate.take().unwrap_or_else(|| RollingUpdateDaemonSet::new())
    }

    pub fn get_rollingUpdate(&self) -> &RollingUpdateDaemonSet {
        self.rollingUpdate.as_ref().unwrap_or_else(|| RollingUpdateDaemonSet::default_instance())
    }

    fn get_rollingUpdate_for_reflect(&self) -> &::protobuf::SingularPtrField<RollingUpdateDaemonSet> {
        &self.rollingUpdate
    }

    fn mut_rollingUpdate_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RollingUpdateDaemonSet> {
        &mut self.rollingUpdate
    }
}

impl ::protobuf::Message for DaemonSetUpdateStrategy {
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

impl ::protobuf::MessageStatic for DaemonSetUpdateStrategy {
    fn new() -> DaemonSetUpdateStrategy {
        DaemonSetUpdateStrategy::new()
    }

    fn descriptor_static(_: ::std::option::Option<DaemonSetUpdateStrategy>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    DaemonSetUpdateStrategy::get_field_type_for_reflect,
                    DaemonSetUpdateStrategy::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RollingUpdateDaemonSet>>(
                    "rollingUpdate",
                    DaemonSetUpdateStrategy::get_rollingUpdate_for_reflect,
                    DaemonSetUpdateStrategy::mut_rollingUpdate_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DaemonSetUpdateStrategy>(
                    "DaemonSetUpdateStrategy",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DaemonSetUpdateStrategy {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_rollingUpdate();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DaemonSetUpdateStrategy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DaemonSetUpdateStrategy {
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

    // optional .k8s.io.api.extensions.v1beta1.DeploymentSpec spec = 2;

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

    // optional .k8s.io.api.extensions.v1beta1.DeploymentStatus status = 3;

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

    // repeated .k8s.io.api.extensions.v1beta1.Deployment items = 2;

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

    // repeated .k8s.io.api.extensions.v1beta1.DeploymentRollback.UpdatedAnnotationsEntry updatedAnnotations = 2;

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

    // optional .k8s.io.api.extensions.v1beta1.RollbackConfig rollbackTo = 3;

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

    // optional .k8s.io.api.extensions.v1beta1.DeploymentStrategy strategy = 4;

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

    // optional .k8s.io.api.extensions.v1beta1.RollbackConfig rollbackTo = 8;

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

    // repeated .k8s.io.api.extensions.v1beta1.DeploymentCondition conditions = 6;

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

    // optional .k8s.io.api.extensions.v1beta1.RollingUpdateDeployment rollingUpdate = 2;

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
pub struct FSGroupStrategyOptions {
    // message fields
    rule: ::protobuf::SingularField<::std::string::String>,
    ranges: ::protobuf::RepeatedField<IDRange>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for FSGroupStrategyOptions {}

impl FSGroupStrategyOptions {
    pub fn new() -> FSGroupStrategyOptions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static FSGroupStrategyOptions {
        static mut instance: ::protobuf::lazy::Lazy<FSGroupStrategyOptions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const FSGroupStrategyOptions,
        };
        unsafe {
            instance.get(FSGroupStrategyOptions::new)
        }
    }

    // optional string rule = 1;

    pub fn clear_rule(&mut self) {
        self.rule.clear();
    }

    pub fn has_rule(&self) -> bool {
        self.rule.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rule(&mut self, v: ::std::string::String) {
        self.rule = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rule(&mut self) -> &mut ::std::string::String {
        if self.rule.is_none() {
            self.rule.set_default();
        }
        self.rule.as_mut().unwrap()
    }

    // Take field
    pub fn take_rule(&mut self) -> ::std::string::String {
        self.rule.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_rule(&self) -> &str {
        match self.rule.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_rule_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.rule
    }

    fn mut_rule_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.rule
    }

    // repeated .k8s.io.api.extensions.v1beta1.IDRange ranges = 2;

    pub fn clear_ranges(&mut self) {
        self.ranges.clear();
    }

    // Param is passed by value, moved
    pub fn set_ranges(&mut self, v: ::protobuf::RepeatedField<IDRange>) {
        self.ranges = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ranges(&mut self) -> &mut ::protobuf::RepeatedField<IDRange> {
        &mut self.ranges
    }

    // Take field
    pub fn take_ranges(&mut self) -> ::protobuf::RepeatedField<IDRange> {
        ::std::mem::replace(&mut self.ranges, ::protobuf::RepeatedField::new())
    }

    pub fn get_ranges(&self) -> &[IDRange] {
        &self.ranges
    }

    fn get_ranges_for_reflect(&self) -> &::protobuf::RepeatedField<IDRange> {
        &self.ranges
    }

    fn mut_ranges_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<IDRange> {
        &mut self.ranges
    }
}

impl ::protobuf::Message for FSGroupStrategyOptions {
    fn is_initialized(&self) -> bool {
        for v in &self.ranges {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.rule)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.ranges)?;
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
        if let Some(ref v) = self.rule.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.ranges {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.rule.as_ref() {
            os.write_string(1, &v)?;
        }
        for v in &self.ranges {
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

impl ::protobuf::MessageStatic for FSGroupStrategyOptions {
    fn new() -> FSGroupStrategyOptions {
        FSGroupStrategyOptions::new()
    }

    fn descriptor_static(_: ::std::option::Option<FSGroupStrategyOptions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "rule",
                    FSGroupStrategyOptions::get_rule_for_reflect,
                    FSGroupStrategyOptions::mut_rule_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<IDRange>>(
                    "ranges",
                    FSGroupStrategyOptions::get_ranges_for_reflect,
                    FSGroupStrategyOptions::mut_ranges_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<FSGroupStrategyOptions>(
                    "FSGroupStrategyOptions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for FSGroupStrategyOptions {
    fn clear(&mut self) {
        self.clear_rule();
        self.clear_ranges();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FSGroupStrategyOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FSGroupStrategyOptions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HTTPIngressPath {
    // message fields
    path: ::protobuf::SingularField<::std::string::String>,
    backend: ::protobuf::SingularPtrField<IngressBackend>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HTTPIngressPath {}

impl HTTPIngressPath {
    pub fn new() -> HTTPIngressPath {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HTTPIngressPath {
        static mut instance: ::protobuf::lazy::Lazy<HTTPIngressPath> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HTTPIngressPath,
        };
        unsafe {
            instance.get(HTTPIngressPath::new)
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

    // optional .k8s.io.api.extensions.v1beta1.IngressBackend backend = 2;

    pub fn clear_backend(&mut self) {
        self.backend.clear();
    }

    pub fn has_backend(&self) -> bool {
        self.backend.is_some()
    }

    // Param is passed by value, moved
    pub fn set_backend(&mut self, v: IngressBackend) {
        self.backend = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_backend(&mut self) -> &mut IngressBackend {
        if self.backend.is_none() {
            self.backend.set_default();
        }
        self.backend.as_mut().unwrap()
    }

    // Take field
    pub fn take_backend(&mut self) -> IngressBackend {
        self.backend.take().unwrap_or_else(|| IngressBackend::new())
    }

    pub fn get_backend(&self) -> &IngressBackend {
        self.backend.as_ref().unwrap_or_else(|| IngressBackend::default_instance())
    }

    fn get_backend_for_reflect(&self) -> &::protobuf::SingularPtrField<IngressBackend> {
        &self.backend
    }

    fn mut_backend_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<IngressBackend> {
        &mut self.backend
    }
}

impl ::protobuf::Message for HTTPIngressPath {
    fn is_initialized(&self) -> bool {
        for v in &self.backend {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.path)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.backend)?;
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
        if let Some(ref v) = self.backend.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.path.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.backend.as_ref() {
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

impl ::protobuf::MessageStatic for HTTPIngressPath {
    fn new() -> HTTPIngressPath {
        HTTPIngressPath::new()
    }

    fn descriptor_static(_: ::std::option::Option<HTTPIngressPath>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "path",
                    HTTPIngressPath::get_path_for_reflect,
                    HTTPIngressPath::mut_path_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<IngressBackend>>(
                    "backend",
                    HTTPIngressPath::get_backend_for_reflect,
                    HTTPIngressPath::mut_backend_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HTTPIngressPath>(
                    "HTTPIngressPath",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HTTPIngressPath {
    fn clear(&mut self) {
        self.clear_path();
        self.clear_backend();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HTTPIngressPath {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HTTPIngressPath {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HTTPIngressRuleValue {
    // message fields
    paths: ::protobuf::RepeatedField<HTTPIngressPath>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HTTPIngressRuleValue {}

impl HTTPIngressRuleValue {
    pub fn new() -> HTTPIngressRuleValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HTTPIngressRuleValue {
        static mut instance: ::protobuf::lazy::Lazy<HTTPIngressRuleValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HTTPIngressRuleValue,
        };
        unsafe {
            instance.get(HTTPIngressRuleValue::new)
        }
    }

    // repeated .k8s.io.api.extensions.v1beta1.HTTPIngressPath paths = 1;

    pub fn clear_paths(&mut self) {
        self.paths.clear();
    }

    // Param is passed by value, moved
    pub fn set_paths(&mut self, v: ::protobuf::RepeatedField<HTTPIngressPath>) {
        self.paths = v;
    }

    // Mutable pointer to the field.
    pub fn mut_paths(&mut self) -> &mut ::protobuf::RepeatedField<HTTPIngressPath> {
        &mut self.paths
    }

    // Take field
    pub fn take_paths(&mut self) -> ::protobuf::RepeatedField<HTTPIngressPath> {
        ::std::mem::replace(&mut self.paths, ::protobuf::RepeatedField::new())
    }

    pub fn get_paths(&self) -> &[HTTPIngressPath] {
        &self.paths
    }

    fn get_paths_for_reflect(&self) -> &::protobuf::RepeatedField<HTTPIngressPath> {
        &self.paths
    }

    fn mut_paths_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<HTTPIngressPath> {
        &mut self.paths
    }
}

impl ::protobuf::Message for HTTPIngressRuleValue {
    fn is_initialized(&self) -> bool {
        for v in &self.paths {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.paths)?;
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
        for value in &self.paths {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.paths {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for HTTPIngressRuleValue {
    fn new() -> HTTPIngressRuleValue {
        HTTPIngressRuleValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<HTTPIngressRuleValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<HTTPIngressPath>>(
                    "paths",
                    HTTPIngressRuleValue::get_paths_for_reflect,
                    HTTPIngressRuleValue::mut_paths_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HTTPIngressRuleValue>(
                    "HTTPIngressRuleValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HTTPIngressRuleValue {
    fn clear(&mut self) {
        self.clear_paths();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HTTPIngressRuleValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HTTPIngressRuleValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HostPortRange {
    // message fields
    min: ::std::option::Option<i32>,
    max: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for HostPortRange {}

impl HostPortRange {
    pub fn new() -> HostPortRange {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static HostPortRange {
        static mut instance: ::protobuf::lazy::Lazy<HostPortRange> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const HostPortRange,
        };
        unsafe {
            instance.get(HostPortRange::new)
        }
    }

    // optional int32 min = 1;

    pub fn clear_min(&mut self) {
        self.min = ::std::option::Option::None;
    }

    pub fn has_min(&self) -> bool {
        self.min.is_some()
    }

    // Param is passed by value, moved
    pub fn set_min(&mut self, v: i32) {
        self.min = ::std::option::Option::Some(v);
    }

    pub fn get_min(&self) -> i32 {
        self.min.unwrap_or(0)
    }

    fn get_min_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.min
    }

    fn mut_min_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.min
    }

    // optional int32 max = 2;

    pub fn clear_max(&mut self) {
        self.max = ::std::option::Option::None;
    }

    pub fn has_max(&self) -> bool {
        self.max.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max(&mut self, v: i32) {
        self.max = ::std::option::Option::Some(v);
    }

    pub fn get_max(&self) -> i32 {
        self.max.unwrap_or(0)
    }

    fn get_max_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.max
    }

    fn mut_max_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.max
    }
}

impl ::protobuf::Message for HostPortRange {
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
                    self.min = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.max = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.min {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.max {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.min {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.max {
            os.write_int32(2, v)?;
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

impl ::protobuf::MessageStatic for HostPortRange {
    fn new() -> HostPortRange {
        HostPortRange::new()
    }

    fn descriptor_static(_: ::std::option::Option<HostPortRange>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "min",
                    HostPortRange::get_min_for_reflect,
                    HostPortRange::mut_min_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "max",
                    HostPortRange::get_max_for_reflect,
                    HostPortRange::mut_max_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<HostPortRange>(
                    "HostPortRange",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for HostPortRange {
    fn clear(&mut self) {
        self.clear_min();
        self.clear_max();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HostPortRange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HostPortRange {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct IDRange {
    // message fields
    min: ::std::option::Option<i64>,
    max: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IDRange {}

impl IDRange {
    pub fn new() -> IDRange {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IDRange {
        static mut instance: ::protobuf::lazy::Lazy<IDRange> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IDRange,
        };
        unsafe {
            instance.get(IDRange::new)
        }
    }

    // optional int64 min = 1;

    pub fn clear_min(&mut self) {
        self.min = ::std::option::Option::None;
    }

    pub fn has_min(&self) -> bool {
        self.min.is_some()
    }

    // Param is passed by value, moved
    pub fn set_min(&mut self, v: i64) {
        self.min = ::std::option::Option::Some(v);
    }

    pub fn get_min(&self) -> i64 {
        self.min.unwrap_or(0)
    }

    fn get_min_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.min
    }

    fn mut_min_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.min
    }

    // optional int64 max = 2;

    pub fn clear_max(&mut self) {
        self.max = ::std::option::Option::None;
    }

    pub fn has_max(&self) -> bool {
        self.max.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max(&mut self, v: i64) {
        self.max = ::std::option::Option::Some(v);
    }

    pub fn get_max(&self) -> i64 {
        self.max.unwrap_or(0)
    }

    fn get_max_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.max
    }

    fn mut_max_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.max
    }
}

impl ::protobuf::Message for IDRange {
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
                    self.min = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.max = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.min {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.max {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.min {
            os.write_int64(1, v)?;
        }
        if let Some(v) = self.max {
            os.write_int64(2, v)?;
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

impl ::protobuf::MessageStatic for IDRange {
    fn new() -> IDRange {
        IDRange::new()
    }

    fn descriptor_static(_: ::std::option::Option<IDRange>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "min",
                    IDRange::get_min_for_reflect,
                    IDRange::mut_min_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "max",
                    IDRange::get_max_for_reflect,
                    IDRange::mut_max_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IDRange>(
                    "IDRange",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IDRange {
    fn clear(&mut self) {
        self.clear_min();
        self.clear_max();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IDRange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IDRange {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Ingress {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    spec: ::protobuf::SingularPtrField<IngressSpec>,
    status: ::protobuf::SingularPtrField<IngressStatus>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Ingress {}

impl Ingress {
    pub fn new() -> Ingress {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Ingress {
        static mut instance: ::protobuf::lazy::Lazy<Ingress> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Ingress,
        };
        unsafe {
            instance.get(Ingress::new)
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

    // optional .k8s.io.api.extensions.v1beta1.IngressSpec spec = 2;

    pub fn clear_spec(&mut self) {
        self.spec.clear();
    }

    pub fn has_spec(&self) -> bool {
        self.spec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spec(&mut self, v: IngressSpec) {
        self.spec = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_spec(&mut self) -> &mut IngressSpec {
        if self.spec.is_none() {
            self.spec.set_default();
        }
        self.spec.as_mut().unwrap()
    }

    // Take field
    pub fn take_spec(&mut self) -> IngressSpec {
        self.spec.take().unwrap_or_else(|| IngressSpec::new())
    }

    pub fn get_spec(&self) -> &IngressSpec {
        self.spec.as_ref().unwrap_or_else(|| IngressSpec::default_instance())
    }

    fn get_spec_for_reflect(&self) -> &::protobuf::SingularPtrField<IngressSpec> {
        &self.spec
    }

    fn mut_spec_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<IngressSpec> {
        &mut self.spec
    }

    // optional .k8s.io.api.extensions.v1beta1.IngressStatus status = 3;

    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: IngressStatus) {
        self.status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut IngressStatus {
        if self.status.is_none() {
            self.status.set_default();
        }
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> IngressStatus {
        self.status.take().unwrap_or_else(|| IngressStatus::new())
    }

    pub fn get_status(&self) -> &IngressStatus {
        self.status.as_ref().unwrap_or_else(|| IngressStatus::default_instance())
    }

    fn get_status_for_reflect(&self) -> &::protobuf::SingularPtrField<IngressStatus> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<IngressStatus> {
        &mut self.status
    }
}

impl ::protobuf::Message for Ingress {
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

impl ::protobuf::MessageStatic for Ingress {
    fn new() -> Ingress {
        Ingress::new()
    }

    fn descriptor_static(_: ::std::option::Option<Ingress>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    Ingress::get_metadata_for_reflect,
                    Ingress::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<IngressSpec>>(
                    "spec",
                    Ingress::get_spec_for_reflect,
                    Ingress::mut_spec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<IngressStatus>>(
                    "status",
                    Ingress::get_status_for_reflect,
                    Ingress::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Ingress>(
                    "Ingress",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Ingress {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_spec();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Ingress {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Ingress {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct IngressBackend {
    // message fields
    serviceName: ::protobuf::SingularField<::std::string::String>,
    servicePort: ::protobuf::SingularPtrField<super::generated::IntOrString>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IngressBackend {}

impl IngressBackend {
    pub fn new() -> IngressBackend {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IngressBackend {
        static mut instance: ::protobuf::lazy::Lazy<IngressBackend> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IngressBackend,
        };
        unsafe {
            instance.get(IngressBackend::new)
        }
    }

    // optional string serviceName = 1;

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

    // optional .k8s.io.apimachinery.pkg.util.intstr.IntOrString servicePort = 2;

    pub fn clear_servicePort(&mut self) {
        self.servicePort.clear();
    }

    pub fn has_servicePort(&self) -> bool {
        self.servicePort.is_some()
    }

    // Param is passed by value, moved
    pub fn set_servicePort(&mut self, v: super::generated::IntOrString) {
        self.servicePort = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_servicePort(&mut self) -> &mut super::generated::IntOrString {
        if self.servicePort.is_none() {
            self.servicePort.set_default();
        }
        self.servicePort.as_mut().unwrap()
    }

    // Take field
    pub fn take_servicePort(&mut self) -> super::generated::IntOrString {
        self.servicePort.take().unwrap_or_else(|| super::generated::IntOrString::new())
    }

    pub fn get_servicePort(&self) -> &super::generated::IntOrString {
        self.servicePort.as_ref().unwrap_or_else(|| super::generated::IntOrString::default_instance())
    }

    fn get_servicePort_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::IntOrString> {
        &self.servicePort
    }

    fn mut_servicePort_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::IntOrString> {
        &mut self.servicePort
    }
}

impl ::protobuf::Message for IngressBackend {
    fn is_initialized(&self) -> bool {
        for v in &self.servicePort {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.serviceName)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.servicePort)?;
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
        if let Some(ref v) = self.serviceName.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.servicePort.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.serviceName.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.servicePort.as_ref() {
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

impl ::protobuf::MessageStatic for IngressBackend {
    fn new() -> IngressBackend {
        IngressBackend::new()
    }

    fn descriptor_static(_: ::std::option::Option<IngressBackend>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "serviceName",
                    IngressBackend::get_serviceName_for_reflect,
                    IngressBackend::mut_serviceName_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::IntOrString>>(
                    "servicePort",
                    IngressBackend::get_servicePort_for_reflect,
                    IngressBackend::mut_servicePort_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IngressBackend>(
                    "IngressBackend",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IngressBackend {
    fn clear(&mut self) {
        self.clear_serviceName();
        self.clear_servicePort();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IngressBackend {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IngressBackend {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct IngressList {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ListMeta>,
    items: ::protobuf::RepeatedField<Ingress>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IngressList {}

impl IngressList {
    pub fn new() -> IngressList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IngressList {
        static mut instance: ::protobuf::lazy::Lazy<IngressList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IngressList,
        };
        unsafe {
            instance.get(IngressList::new)
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

    // repeated .k8s.io.api.extensions.v1beta1.Ingress items = 2;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<Ingress>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<Ingress> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<Ingress> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[Ingress] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<Ingress> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Ingress> {
        &mut self.items
    }
}

impl ::protobuf::Message for IngressList {
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

impl ::protobuf::MessageStatic for IngressList {
    fn new() -> IngressList {
        IngressList::new()
    }

    fn descriptor_static(_: ::std::option::Option<IngressList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ListMeta>>(
                    "metadata",
                    IngressList::get_metadata_for_reflect,
                    IngressList::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Ingress>>(
                    "items",
                    IngressList::get_items_for_reflect,
                    IngressList::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IngressList>(
                    "IngressList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IngressList {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IngressList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IngressList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct IngressRule {
    // message fields
    host: ::protobuf::SingularField<::std::string::String>,
    ingressRuleValue: ::protobuf::SingularPtrField<IngressRuleValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IngressRule {}

impl IngressRule {
    pub fn new() -> IngressRule {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IngressRule {
        static mut instance: ::protobuf::lazy::Lazy<IngressRule> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IngressRule,
        };
        unsafe {
            instance.get(IngressRule::new)
        }
    }

    // optional string host = 1;

    pub fn clear_host(&mut self) {
        self.host.clear();
    }

    pub fn has_host(&self) -> bool {
        self.host.is_some()
    }

    // Param is passed by value, moved
    pub fn set_host(&mut self, v: ::std::string::String) {
        self.host = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_host(&mut self) -> &mut ::std::string::String {
        if self.host.is_none() {
            self.host.set_default();
        }
        self.host.as_mut().unwrap()
    }

    // Take field
    pub fn take_host(&mut self) -> ::std::string::String {
        self.host.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_host(&self) -> &str {
        match self.host.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_host_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.host
    }

    fn mut_host_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.host
    }

    // optional .k8s.io.api.extensions.v1beta1.IngressRuleValue ingressRuleValue = 2;

    pub fn clear_ingressRuleValue(&mut self) {
        self.ingressRuleValue.clear();
    }

    pub fn has_ingressRuleValue(&self) -> bool {
        self.ingressRuleValue.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ingressRuleValue(&mut self, v: IngressRuleValue) {
        self.ingressRuleValue = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ingressRuleValue(&mut self) -> &mut IngressRuleValue {
        if self.ingressRuleValue.is_none() {
            self.ingressRuleValue.set_default();
        }
        self.ingressRuleValue.as_mut().unwrap()
    }

    // Take field
    pub fn take_ingressRuleValue(&mut self) -> IngressRuleValue {
        self.ingressRuleValue.take().unwrap_or_else(|| IngressRuleValue::new())
    }

    pub fn get_ingressRuleValue(&self) -> &IngressRuleValue {
        self.ingressRuleValue.as_ref().unwrap_or_else(|| IngressRuleValue::default_instance())
    }

    fn get_ingressRuleValue_for_reflect(&self) -> &::protobuf::SingularPtrField<IngressRuleValue> {
        &self.ingressRuleValue
    }

    fn mut_ingressRuleValue_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<IngressRuleValue> {
        &mut self.ingressRuleValue
    }
}

impl ::protobuf::Message for IngressRule {
    fn is_initialized(&self) -> bool {
        for v in &self.ingressRuleValue {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.host)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ingressRuleValue)?;
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
        if let Some(ref v) = self.host.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.ingressRuleValue.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.host.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.ingressRuleValue.as_ref() {
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

impl ::protobuf::MessageStatic for IngressRule {
    fn new() -> IngressRule {
        IngressRule::new()
    }

    fn descriptor_static(_: ::std::option::Option<IngressRule>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "host",
                    IngressRule::get_host_for_reflect,
                    IngressRule::mut_host_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<IngressRuleValue>>(
                    "ingressRuleValue",
                    IngressRule::get_ingressRuleValue_for_reflect,
                    IngressRule::mut_ingressRuleValue_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IngressRule>(
                    "IngressRule",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IngressRule {
    fn clear(&mut self) {
        self.clear_host();
        self.clear_ingressRuleValue();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IngressRule {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IngressRule {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct IngressRuleValue {
    // message fields
    http: ::protobuf::SingularPtrField<HTTPIngressRuleValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IngressRuleValue {}

impl IngressRuleValue {
    pub fn new() -> IngressRuleValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IngressRuleValue {
        static mut instance: ::protobuf::lazy::Lazy<IngressRuleValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IngressRuleValue,
        };
        unsafe {
            instance.get(IngressRuleValue::new)
        }
    }

    // optional .k8s.io.api.extensions.v1beta1.HTTPIngressRuleValue http = 1;

    pub fn clear_http(&mut self) {
        self.http.clear();
    }

    pub fn has_http(&self) -> bool {
        self.http.is_some()
    }

    // Param is passed by value, moved
    pub fn set_http(&mut self, v: HTTPIngressRuleValue) {
        self.http = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_http(&mut self) -> &mut HTTPIngressRuleValue {
        if self.http.is_none() {
            self.http.set_default();
        }
        self.http.as_mut().unwrap()
    }

    // Take field
    pub fn take_http(&mut self) -> HTTPIngressRuleValue {
        self.http.take().unwrap_or_else(|| HTTPIngressRuleValue::new())
    }

    pub fn get_http(&self) -> &HTTPIngressRuleValue {
        self.http.as_ref().unwrap_or_else(|| HTTPIngressRuleValue::default_instance())
    }

    fn get_http_for_reflect(&self) -> &::protobuf::SingularPtrField<HTTPIngressRuleValue> {
        &self.http
    }

    fn mut_http_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<HTTPIngressRuleValue> {
        &mut self.http
    }
}

impl ::protobuf::Message for IngressRuleValue {
    fn is_initialized(&self) -> bool {
        for v in &self.http {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.http)?;
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
        if let Some(ref v) = self.http.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.http.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for IngressRuleValue {
    fn new() -> IngressRuleValue {
        IngressRuleValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<IngressRuleValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<HTTPIngressRuleValue>>(
                    "http",
                    IngressRuleValue::get_http_for_reflect,
                    IngressRuleValue::mut_http_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IngressRuleValue>(
                    "IngressRuleValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IngressRuleValue {
    fn clear(&mut self) {
        self.clear_http();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IngressRuleValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IngressRuleValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct IngressSpec {
    // message fields
    backend: ::protobuf::SingularPtrField<IngressBackend>,
    tls: ::protobuf::RepeatedField<IngressTLS>,
    rules: ::protobuf::RepeatedField<IngressRule>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IngressSpec {}

impl IngressSpec {
    pub fn new() -> IngressSpec {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IngressSpec {
        static mut instance: ::protobuf::lazy::Lazy<IngressSpec> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IngressSpec,
        };
        unsafe {
            instance.get(IngressSpec::new)
        }
    }

    // optional .k8s.io.api.extensions.v1beta1.IngressBackend backend = 1;

    pub fn clear_backend(&mut self) {
        self.backend.clear();
    }

    pub fn has_backend(&self) -> bool {
        self.backend.is_some()
    }

    // Param is passed by value, moved
    pub fn set_backend(&mut self, v: IngressBackend) {
        self.backend = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_backend(&mut self) -> &mut IngressBackend {
        if self.backend.is_none() {
            self.backend.set_default();
        }
        self.backend.as_mut().unwrap()
    }

    // Take field
    pub fn take_backend(&mut self) -> IngressBackend {
        self.backend.take().unwrap_or_else(|| IngressBackend::new())
    }

    pub fn get_backend(&self) -> &IngressBackend {
        self.backend.as_ref().unwrap_or_else(|| IngressBackend::default_instance())
    }

    fn get_backend_for_reflect(&self) -> &::protobuf::SingularPtrField<IngressBackend> {
        &self.backend
    }

    fn mut_backend_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<IngressBackend> {
        &mut self.backend
    }

    // repeated .k8s.io.api.extensions.v1beta1.IngressTLS tls = 2;

    pub fn clear_tls(&mut self) {
        self.tls.clear();
    }

    // Param is passed by value, moved
    pub fn set_tls(&mut self, v: ::protobuf::RepeatedField<IngressTLS>) {
        self.tls = v;
    }

    // Mutable pointer to the field.
    pub fn mut_tls(&mut self) -> &mut ::protobuf::RepeatedField<IngressTLS> {
        &mut self.tls
    }

    // Take field
    pub fn take_tls(&mut self) -> ::protobuf::RepeatedField<IngressTLS> {
        ::std::mem::replace(&mut self.tls, ::protobuf::RepeatedField::new())
    }

    pub fn get_tls(&self) -> &[IngressTLS] {
        &self.tls
    }

    fn get_tls_for_reflect(&self) -> &::protobuf::RepeatedField<IngressTLS> {
        &self.tls
    }

    fn mut_tls_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<IngressTLS> {
        &mut self.tls
    }

    // repeated .k8s.io.api.extensions.v1beta1.IngressRule rules = 3;

    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }

    // Param is passed by value, moved
    pub fn set_rules(&mut self, v: ::protobuf::RepeatedField<IngressRule>) {
        self.rules = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rules(&mut self) -> &mut ::protobuf::RepeatedField<IngressRule> {
        &mut self.rules
    }

    // Take field
    pub fn take_rules(&mut self) -> ::protobuf::RepeatedField<IngressRule> {
        ::std::mem::replace(&mut self.rules, ::protobuf::RepeatedField::new())
    }

    pub fn get_rules(&self) -> &[IngressRule] {
        &self.rules
    }

    fn get_rules_for_reflect(&self) -> &::protobuf::RepeatedField<IngressRule> {
        &self.rules
    }

    fn mut_rules_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<IngressRule> {
        &mut self.rules
    }
}

impl ::protobuf::Message for IngressSpec {
    fn is_initialized(&self) -> bool {
        for v in &self.backend {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.tls {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.backend)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.tls)?;
                },
                3 => {
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
        if let Some(ref v) = self.backend.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.tls {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.rules {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.backend.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.tls {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.rules {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for IngressSpec {
    fn new() -> IngressSpec {
        IngressSpec::new()
    }

    fn descriptor_static(_: ::std::option::Option<IngressSpec>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<IngressBackend>>(
                    "backend",
                    IngressSpec::get_backend_for_reflect,
                    IngressSpec::mut_backend_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<IngressTLS>>(
                    "tls",
                    IngressSpec::get_tls_for_reflect,
                    IngressSpec::mut_tls_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<IngressRule>>(
                    "rules",
                    IngressSpec::get_rules_for_reflect,
                    IngressSpec::mut_rules_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IngressSpec>(
                    "IngressSpec",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IngressSpec {
    fn clear(&mut self) {
        self.clear_backend();
        self.clear_tls();
        self.clear_rules();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IngressSpec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IngressSpec {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct IngressStatus {
    // message fields
    loadBalancer: ::protobuf::SingularPtrField<super::generated::LoadBalancerStatus>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IngressStatus {}

impl IngressStatus {
    pub fn new() -> IngressStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IngressStatus {
        static mut instance: ::protobuf::lazy::Lazy<IngressStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IngressStatus,
        };
        unsafe {
            instance.get(IngressStatus::new)
        }
    }

    // optional .k8s.io.api.core.v1.LoadBalancerStatus loadBalancer = 1;

    pub fn clear_loadBalancer(&mut self) {
        self.loadBalancer.clear();
    }

    pub fn has_loadBalancer(&self) -> bool {
        self.loadBalancer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_loadBalancer(&mut self, v: super::generated::LoadBalancerStatus) {
        self.loadBalancer = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_loadBalancer(&mut self) -> &mut super::generated::LoadBalancerStatus {
        if self.loadBalancer.is_none() {
            self.loadBalancer.set_default();
        }
        self.loadBalancer.as_mut().unwrap()
    }

    // Take field
    pub fn take_loadBalancer(&mut self) -> super::generated::LoadBalancerStatus {
        self.loadBalancer.take().unwrap_or_else(|| super::generated::LoadBalancerStatus::new())
    }

    pub fn get_loadBalancer(&self) -> &super::generated::LoadBalancerStatus {
        self.loadBalancer.as_ref().unwrap_or_else(|| super::generated::LoadBalancerStatus::default_instance())
    }

    fn get_loadBalancer_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::LoadBalancerStatus> {
        &self.loadBalancer
    }

    fn mut_loadBalancer_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::LoadBalancerStatus> {
        &mut self.loadBalancer
    }
}

impl ::protobuf::Message for IngressStatus {
    fn is_initialized(&self) -> bool {
        for v in &self.loadBalancer {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.loadBalancer)?;
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
        if let Some(ref v) = self.loadBalancer.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.loadBalancer.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for IngressStatus {
    fn new() -> IngressStatus {
        IngressStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<IngressStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::LoadBalancerStatus>>(
                    "loadBalancer",
                    IngressStatus::get_loadBalancer_for_reflect,
                    IngressStatus::mut_loadBalancer_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IngressStatus>(
                    "IngressStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IngressStatus {
    fn clear(&mut self) {
        self.clear_loadBalancer();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IngressStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IngressStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct IngressTLS {
    // message fields
    hosts: ::protobuf::RepeatedField<::std::string::String>,
    secretName: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IngressTLS {}

impl IngressTLS {
    pub fn new() -> IngressTLS {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IngressTLS {
        static mut instance: ::protobuf::lazy::Lazy<IngressTLS> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IngressTLS,
        };
        unsafe {
            instance.get(IngressTLS::new)
        }
    }

    // repeated string hosts = 1;

    pub fn clear_hosts(&mut self) {
        self.hosts.clear();
    }

    // Param is passed by value, moved
    pub fn set_hosts(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.hosts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_hosts(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.hosts
    }

    // Take field
    pub fn take_hosts(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.hosts, ::protobuf::RepeatedField::new())
    }

    pub fn get_hosts(&self) -> &[::std::string::String] {
        &self.hosts
    }

    fn get_hosts_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.hosts
    }

    fn mut_hosts_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.hosts
    }

    // optional string secretName = 2;

    pub fn clear_secretName(&mut self) {
        self.secretName.clear();
    }

    pub fn has_secretName(&self) -> bool {
        self.secretName.is_some()
    }

    // Param is passed by value, moved
    pub fn set_secretName(&mut self, v: ::std::string::String) {
        self.secretName = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_secretName(&mut self) -> &mut ::std::string::String {
        if self.secretName.is_none() {
            self.secretName.set_default();
        }
        self.secretName.as_mut().unwrap()
    }

    // Take field
    pub fn take_secretName(&mut self) -> ::std::string::String {
        self.secretName.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_secretName(&self) -> &str {
        match self.secretName.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_secretName_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.secretName
    }

    fn mut_secretName_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.secretName
    }
}

impl ::protobuf::Message for IngressTLS {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.hosts)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.secretName)?;
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
        for value in &self.hosts {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if let Some(ref v) = self.secretName.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.hosts {
            os.write_string(1, &v)?;
        };
        if let Some(ref v) = self.secretName.as_ref() {
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

impl ::protobuf::MessageStatic for IngressTLS {
    fn new() -> IngressTLS {
        IngressTLS::new()
    }

    fn descriptor_static(_: ::std::option::Option<IngressTLS>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "hosts",
                    IngressTLS::get_hosts_for_reflect,
                    IngressTLS::mut_hosts_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "secretName",
                    IngressTLS::get_secretName_for_reflect,
                    IngressTLS::mut_secretName_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IngressTLS>(
                    "IngressTLS",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IngressTLS {
    fn clear(&mut self) {
        self.clear_hosts();
        self.clear_secretName();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IngressTLS {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IngressTLS {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

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

    // optional .k8s.io.api.extensions.v1beta1.NetworkPolicySpec spec = 2;

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

    // repeated .k8s.io.api.extensions.v1beta1.NetworkPolicyPort ports = 1;

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

    // repeated .k8s.io.api.extensions.v1beta1.NetworkPolicyPeer from = 2;

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

    // repeated .k8s.io.api.extensions.v1beta1.NetworkPolicy items = 2;

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

    // repeated .k8s.io.api.extensions.v1beta1.NetworkPolicyIngressRule ingress = 2;

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

#[derive(PartialEq,Clone,Default)]
pub struct PodSecurityPolicy {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    spec: ::protobuf::SingularPtrField<PodSecurityPolicySpec>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PodSecurityPolicy {}

impl PodSecurityPolicy {
    pub fn new() -> PodSecurityPolicy {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PodSecurityPolicy {
        static mut instance: ::protobuf::lazy::Lazy<PodSecurityPolicy> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PodSecurityPolicy,
        };
        unsafe {
            instance.get(PodSecurityPolicy::new)
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

    // optional .k8s.io.api.extensions.v1beta1.PodSecurityPolicySpec spec = 2;

    pub fn clear_spec(&mut self) {
        self.spec.clear();
    }

    pub fn has_spec(&self) -> bool {
        self.spec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spec(&mut self, v: PodSecurityPolicySpec) {
        self.spec = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_spec(&mut self) -> &mut PodSecurityPolicySpec {
        if self.spec.is_none() {
            self.spec.set_default();
        }
        self.spec.as_mut().unwrap()
    }

    // Take field
    pub fn take_spec(&mut self) -> PodSecurityPolicySpec {
        self.spec.take().unwrap_or_else(|| PodSecurityPolicySpec::new())
    }

    pub fn get_spec(&self) -> &PodSecurityPolicySpec {
        self.spec.as_ref().unwrap_or_else(|| PodSecurityPolicySpec::default_instance())
    }

    fn get_spec_for_reflect(&self) -> &::protobuf::SingularPtrField<PodSecurityPolicySpec> {
        &self.spec
    }

    fn mut_spec_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PodSecurityPolicySpec> {
        &mut self.spec
    }
}

impl ::protobuf::Message for PodSecurityPolicy {
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

impl ::protobuf::MessageStatic for PodSecurityPolicy {
    fn new() -> PodSecurityPolicy {
        PodSecurityPolicy::new()
    }

    fn descriptor_static(_: ::std::option::Option<PodSecurityPolicy>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    PodSecurityPolicy::get_metadata_for_reflect,
                    PodSecurityPolicy::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PodSecurityPolicySpec>>(
                    "spec",
                    PodSecurityPolicy::get_spec_for_reflect,
                    PodSecurityPolicy::mut_spec_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PodSecurityPolicy>(
                    "PodSecurityPolicy",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PodSecurityPolicy {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_spec();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PodSecurityPolicy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PodSecurityPolicy {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PodSecurityPolicyList {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ListMeta>,
    items: ::protobuf::RepeatedField<PodSecurityPolicy>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PodSecurityPolicyList {}

impl PodSecurityPolicyList {
    pub fn new() -> PodSecurityPolicyList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PodSecurityPolicyList {
        static mut instance: ::protobuf::lazy::Lazy<PodSecurityPolicyList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PodSecurityPolicyList,
        };
        unsafe {
            instance.get(PodSecurityPolicyList::new)
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

    // repeated .k8s.io.api.extensions.v1beta1.PodSecurityPolicy items = 2;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<PodSecurityPolicy>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<PodSecurityPolicy> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<PodSecurityPolicy> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[PodSecurityPolicy] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<PodSecurityPolicy> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PodSecurityPolicy> {
        &mut self.items
    }
}

impl ::protobuf::Message for PodSecurityPolicyList {
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

impl ::protobuf::MessageStatic for PodSecurityPolicyList {
    fn new() -> PodSecurityPolicyList {
        PodSecurityPolicyList::new()
    }

    fn descriptor_static(_: ::std::option::Option<PodSecurityPolicyList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ListMeta>>(
                    "metadata",
                    PodSecurityPolicyList::get_metadata_for_reflect,
                    PodSecurityPolicyList::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PodSecurityPolicy>>(
                    "items",
                    PodSecurityPolicyList::get_items_for_reflect,
                    PodSecurityPolicyList::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PodSecurityPolicyList>(
                    "PodSecurityPolicyList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PodSecurityPolicyList {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PodSecurityPolicyList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PodSecurityPolicyList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PodSecurityPolicySpec {
    // message fields
    privileged: ::std::option::Option<bool>,
    defaultAddCapabilities: ::protobuf::RepeatedField<::std::string::String>,
    requiredDropCapabilities: ::protobuf::RepeatedField<::std::string::String>,
    allowedCapabilities: ::protobuf::RepeatedField<::std::string::String>,
    volumes: ::protobuf::RepeatedField<::std::string::String>,
    hostNetwork: ::std::option::Option<bool>,
    hostPorts: ::protobuf::RepeatedField<HostPortRange>,
    hostPID: ::std::option::Option<bool>,
    hostIPC: ::std::option::Option<bool>,
    seLinux: ::protobuf::SingularPtrField<SELinuxStrategyOptions>,
    runAsUser: ::protobuf::SingularPtrField<RunAsUserStrategyOptions>,
    supplementalGroups: ::protobuf::SingularPtrField<SupplementalGroupsStrategyOptions>,
    fsGroup: ::protobuf::SingularPtrField<FSGroupStrategyOptions>,
    readOnlyRootFilesystem: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PodSecurityPolicySpec {}

impl PodSecurityPolicySpec {
    pub fn new() -> PodSecurityPolicySpec {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PodSecurityPolicySpec {
        static mut instance: ::protobuf::lazy::Lazy<PodSecurityPolicySpec> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PodSecurityPolicySpec,
        };
        unsafe {
            instance.get(PodSecurityPolicySpec::new)
        }
    }

    // optional bool privileged = 1;

    pub fn clear_privileged(&mut self) {
        self.privileged = ::std::option::Option::None;
    }

    pub fn has_privileged(&self) -> bool {
        self.privileged.is_some()
    }

    // Param is passed by value, moved
    pub fn set_privileged(&mut self, v: bool) {
        self.privileged = ::std::option::Option::Some(v);
    }

    pub fn get_privileged(&self) -> bool {
        self.privileged.unwrap_or(false)
    }

    fn get_privileged_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.privileged
    }

    fn mut_privileged_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.privileged
    }

    // repeated string defaultAddCapabilities = 2;

    pub fn clear_defaultAddCapabilities(&mut self) {
        self.defaultAddCapabilities.clear();
    }

    // Param is passed by value, moved
    pub fn set_defaultAddCapabilities(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.defaultAddCapabilities = v;
    }

    // Mutable pointer to the field.
    pub fn mut_defaultAddCapabilities(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.defaultAddCapabilities
    }

    // Take field
    pub fn take_defaultAddCapabilities(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.defaultAddCapabilities, ::protobuf::RepeatedField::new())
    }

    pub fn get_defaultAddCapabilities(&self) -> &[::std::string::String] {
        &self.defaultAddCapabilities
    }

    fn get_defaultAddCapabilities_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.defaultAddCapabilities
    }

    fn mut_defaultAddCapabilities_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.defaultAddCapabilities
    }

    // repeated string requiredDropCapabilities = 3;

    pub fn clear_requiredDropCapabilities(&mut self) {
        self.requiredDropCapabilities.clear();
    }

    // Param is passed by value, moved
    pub fn set_requiredDropCapabilities(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.requiredDropCapabilities = v;
    }

    // Mutable pointer to the field.
    pub fn mut_requiredDropCapabilities(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.requiredDropCapabilities
    }

    // Take field
    pub fn take_requiredDropCapabilities(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.requiredDropCapabilities, ::protobuf::RepeatedField::new())
    }

    pub fn get_requiredDropCapabilities(&self) -> &[::std::string::String] {
        &self.requiredDropCapabilities
    }

    fn get_requiredDropCapabilities_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.requiredDropCapabilities
    }

    fn mut_requiredDropCapabilities_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.requiredDropCapabilities
    }

    // repeated string allowedCapabilities = 4;

    pub fn clear_allowedCapabilities(&mut self) {
        self.allowedCapabilities.clear();
    }

    // Param is passed by value, moved
    pub fn set_allowedCapabilities(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.allowedCapabilities = v;
    }

    // Mutable pointer to the field.
    pub fn mut_allowedCapabilities(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.allowedCapabilities
    }

    // Take field
    pub fn take_allowedCapabilities(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.allowedCapabilities, ::protobuf::RepeatedField::new())
    }

    pub fn get_allowedCapabilities(&self) -> &[::std::string::String] {
        &self.allowedCapabilities
    }

    fn get_allowedCapabilities_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.allowedCapabilities
    }

    fn mut_allowedCapabilities_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.allowedCapabilities
    }

    // repeated string volumes = 5;

    pub fn clear_volumes(&mut self) {
        self.volumes.clear();
    }

    // Param is passed by value, moved
    pub fn set_volumes(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.volumes = v;
    }

    // Mutable pointer to the field.
    pub fn mut_volumes(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.volumes
    }

    // Take field
    pub fn take_volumes(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.volumes, ::protobuf::RepeatedField::new())
    }

    pub fn get_volumes(&self) -> &[::std::string::String] {
        &self.volumes
    }

    fn get_volumes_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.volumes
    }

    fn mut_volumes_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.volumes
    }

    // optional bool hostNetwork = 6;

    pub fn clear_hostNetwork(&mut self) {
        self.hostNetwork = ::std::option::Option::None;
    }

    pub fn has_hostNetwork(&self) -> bool {
        self.hostNetwork.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hostNetwork(&mut self, v: bool) {
        self.hostNetwork = ::std::option::Option::Some(v);
    }

    pub fn get_hostNetwork(&self) -> bool {
        self.hostNetwork.unwrap_or(false)
    }

    fn get_hostNetwork_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.hostNetwork
    }

    fn mut_hostNetwork_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.hostNetwork
    }

    // repeated .k8s.io.api.extensions.v1beta1.HostPortRange hostPorts = 7;

    pub fn clear_hostPorts(&mut self) {
        self.hostPorts.clear();
    }

    // Param is passed by value, moved
    pub fn set_hostPorts(&mut self, v: ::protobuf::RepeatedField<HostPortRange>) {
        self.hostPorts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_hostPorts(&mut self) -> &mut ::protobuf::RepeatedField<HostPortRange> {
        &mut self.hostPorts
    }

    // Take field
    pub fn take_hostPorts(&mut self) -> ::protobuf::RepeatedField<HostPortRange> {
        ::std::mem::replace(&mut self.hostPorts, ::protobuf::RepeatedField::new())
    }

    pub fn get_hostPorts(&self) -> &[HostPortRange] {
        &self.hostPorts
    }

    fn get_hostPorts_for_reflect(&self) -> &::protobuf::RepeatedField<HostPortRange> {
        &self.hostPorts
    }

    fn mut_hostPorts_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<HostPortRange> {
        &mut self.hostPorts
    }

    // optional bool hostPID = 8;

    pub fn clear_hostPID(&mut self) {
        self.hostPID = ::std::option::Option::None;
    }

    pub fn has_hostPID(&self) -> bool {
        self.hostPID.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hostPID(&mut self, v: bool) {
        self.hostPID = ::std::option::Option::Some(v);
    }

    pub fn get_hostPID(&self) -> bool {
        self.hostPID.unwrap_or(false)
    }

    fn get_hostPID_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.hostPID
    }

    fn mut_hostPID_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.hostPID
    }

    // optional bool hostIPC = 9;

    pub fn clear_hostIPC(&mut self) {
        self.hostIPC = ::std::option::Option::None;
    }

    pub fn has_hostIPC(&self) -> bool {
        self.hostIPC.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hostIPC(&mut self, v: bool) {
        self.hostIPC = ::std::option::Option::Some(v);
    }

    pub fn get_hostIPC(&self) -> bool {
        self.hostIPC.unwrap_or(false)
    }

    fn get_hostIPC_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.hostIPC
    }

    fn mut_hostIPC_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.hostIPC
    }

    // optional .k8s.io.api.extensions.v1beta1.SELinuxStrategyOptions seLinux = 10;

    pub fn clear_seLinux(&mut self) {
        self.seLinux.clear();
    }

    pub fn has_seLinux(&self) -> bool {
        self.seLinux.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seLinux(&mut self, v: SELinuxStrategyOptions) {
        self.seLinux = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_seLinux(&mut self) -> &mut SELinuxStrategyOptions {
        if self.seLinux.is_none() {
            self.seLinux.set_default();
        }
        self.seLinux.as_mut().unwrap()
    }

    // Take field
    pub fn take_seLinux(&mut self) -> SELinuxStrategyOptions {
        self.seLinux.take().unwrap_or_else(|| SELinuxStrategyOptions::new())
    }

    pub fn get_seLinux(&self) -> &SELinuxStrategyOptions {
        self.seLinux.as_ref().unwrap_or_else(|| SELinuxStrategyOptions::default_instance())
    }

    fn get_seLinux_for_reflect(&self) -> &::protobuf::SingularPtrField<SELinuxStrategyOptions> {
        &self.seLinux
    }

    fn mut_seLinux_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SELinuxStrategyOptions> {
        &mut self.seLinux
    }

    // optional .k8s.io.api.extensions.v1beta1.RunAsUserStrategyOptions runAsUser = 11;

    pub fn clear_runAsUser(&mut self) {
        self.runAsUser.clear();
    }

    pub fn has_runAsUser(&self) -> bool {
        self.runAsUser.is_some()
    }

    // Param is passed by value, moved
    pub fn set_runAsUser(&mut self, v: RunAsUserStrategyOptions) {
        self.runAsUser = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_runAsUser(&mut self) -> &mut RunAsUserStrategyOptions {
        if self.runAsUser.is_none() {
            self.runAsUser.set_default();
        }
        self.runAsUser.as_mut().unwrap()
    }

    // Take field
    pub fn take_runAsUser(&mut self) -> RunAsUserStrategyOptions {
        self.runAsUser.take().unwrap_or_else(|| RunAsUserStrategyOptions::new())
    }

    pub fn get_runAsUser(&self) -> &RunAsUserStrategyOptions {
        self.runAsUser.as_ref().unwrap_or_else(|| RunAsUserStrategyOptions::default_instance())
    }

    fn get_runAsUser_for_reflect(&self) -> &::protobuf::SingularPtrField<RunAsUserStrategyOptions> {
        &self.runAsUser
    }

    fn mut_runAsUser_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RunAsUserStrategyOptions> {
        &mut self.runAsUser
    }

    // optional .k8s.io.api.extensions.v1beta1.SupplementalGroupsStrategyOptions supplementalGroups = 12;

    pub fn clear_supplementalGroups(&mut self) {
        self.supplementalGroups.clear();
    }

    pub fn has_supplementalGroups(&self) -> bool {
        self.supplementalGroups.is_some()
    }

    // Param is passed by value, moved
    pub fn set_supplementalGroups(&mut self, v: SupplementalGroupsStrategyOptions) {
        self.supplementalGroups = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_supplementalGroups(&mut self) -> &mut SupplementalGroupsStrategyOptions {
        if self.supplementalGroups.is_none() {
            self.supplementalGroups.set_default();
        }
        self.supplementalGroups.as_mut().unwrap()
    }

    // Take field
    pub fn take_supplementalGroups(&mut self) -> SupplementalGroupsStrategyOptions {
        self.supplementalGroups.take().unwrap_or_else(|| SupplementalGroupsStrategyOptions::new())
    }

    pub fn get_supplementalGroups(&self) -> &SupplementalGroupsStrategyOptions {
        self.supplementalGroups.as_ref().unwrap_or_else(|| SupplementalGroupsStrategyOptions::default_instance())
    }

    fn get_supplementalGroups_for_reflect(&self) -> &::protobuf::SingularPtrField<SupplementalGroupsStrategyOptions> {
        &self.supplementalGroups
    }

    fn mut_supplementalGroups_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SupplementalGroupsStrategyOptions> {
        &mut self.supplementalGroups
    }

    // optional .k8s.io.api.extensions.v1beta1.FSGroupStrategyOptions fsGroup = 13;

    pub fn clear_fsGroup(&mut self) {
        self.fsGroup.clear();
    }

    pub fn has_fsGroup(&self) -> bool {
        self.fsGroup.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fsGroup(&mut self, v: FSGroupStrategyOptions) {
        self.fsGroup = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_fsGroup(&mut self) -> &mut FSGroupStrategyOptions {
        if self.fsGroup.is_none() {
            self.fsGroup.set_default();
        }
        self.fsGroup.as_mut().unwrap()
    }

    // Take field
    pub fn take_fsGroup(&mut self) -> FSGroupStrategyOptions {
        self.fsGroup.take().unwrap_or_else(|| FSGroupStrategyOptions::new())
    }

    pub fn get_fsGroup(&self) -> &FSGroupStrategyOptions {
        self.fsGroup.as_ref().unwrap_or_else(|| FSGroupStrategyOptions::default_instance())
    }

    fn get_fsGroup_for_reflect(&self) -> &::protobuf::SingularPtrField<FSGroupStrategyOptions> {
        &self.fsGroup
    }

    fn mut_fsGroup_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<FSGroupStrategyOptions> {
        &mut self.fsGroup
    }

    // optional bool readOnlyRootFilesystem = 14;

    pub fn clear_readOnlyRootFilesystem(&mut self) {
        self.readOnlyRootFilesystem = ::std::option::Option::None;
    }

    pub fn has_readOnlyRootFilesystem(&self) -> bool {
        self.readOnlyRootFilesystem.is_some()
    }

    // Param is passed by value, moved
    pub fn set_readOnlyRootFilesystem(&mut self, v: bool) {
        self.readOnlyRootFilesystem = ::std::option::Option::Some(v);
    }

    pub fn get_readOnlyRootFilesystem(&self) -> bool {
        self.readOnlyRootFilesystem.unwrap_or(false)
    }

    fn get_readOnlyRootFilesystem_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.readOnlyRootFilesystem
    }

    fn mut_readOnlyRootFilesystem_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.readOnlyRootFilesystem
    }
}

impl ::protobuf::Message for PodSecurityPolicySpec {
    fn is_initialized(&self) -> bool {
        for v in &self.hostPorts {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.seLinux {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.runAsUser {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.supplementalGroups {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.fsGroup {
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
                    self.privileged = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.defaultAddCapabilities)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.requiredDropCapabilities)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.allowedCapabilities)?;
                },
                5 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.volumes)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.hostNetwork = ::std::option::Option::Some(tmp);
                },
                7 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.hostPorts)?;
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.hostPID = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.hostIPC = ::std::option::Option::Some(tmp);
                },
                10 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.seLinux)?;
                },
                11 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.runAsUser)?;
                },
                12 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.supplementalGroups)?;
                },
                13 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.fsGroup)?;
                },
                14 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.readOnlyRootFilesystem = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.privileged {
            my_size += 2;
        }
        for value in &self.defaultAddCapabilities {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.requiredDropCapabilities {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        for value in &self.allowedCapabilities {
            my_size += ::protobuf::rt::string_size(4, &value);
        };
        for value in &self.volumes {
            my_size += ::protobuf::rt::string_size(5, &value);
        };
        if let Some(v) = self.hostNetwork {
            my_size += 2;
        }
        for value in &self.hostPorts {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.hostPID {
            my_size += 2;
        }
        if let Some(v) = self.hostIPC {
            my_size += 2;
        }
        if let Some(ref v) = self.seLinux.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.runAsUser.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.supplementalGroups.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.fsGroup.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.readOnlyRootFilesystem {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.privileged {
            os.write_bool(1, v)?;
        }
        for v in &self.defaultAddCapabilities {
            os.write_string(2, &v)?;
        };
        for v in &self.requiredDropCapabilities {
            os.write_string(3, &v)?;
        };
        for v in &self.allowedCapabilities {
            os.write_string(4, &v)?;
        };
        for v in &self.volumes {
            os.write_string(5, &v)?;
        };
        if let Some(v) = self.hostNetwork {
            os.write_bool(6, v)?;
        }
        for v in &self.hostPorts {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.hostPID {
            os.write_bool(8, v)?;
        }
        if let Some(v) = self.hostIPC {
            os.write_bool(9, v)?;
        }
        if let Some(ref v) = self.seLinux.as_ref() {
            os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.runAsUser.as_ref() {
            os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.supplementalGroups.as_ref() {
            os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.fsGroup.as_ref() {
            os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.readOnlyRootFilesystem {
            os.write_bool(14, v)?;
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

impl ::protobuf::MessageStatic for PodSecurityPolicySpec {
    fn new() -> PodSecurityPolicySpec {
        PodSecurityPolicySpec::new()
    }

    fn descriptor_static(_: ::std::option::Option<PodSecurityPolicySpec>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "privileged",
                    PodSecurityPolicySpec::get_privileged_for_reflect,
                    PodSecurityPolicySpec::mut_privileged_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "defaultAddCapabilities",
                    PodSecurityPolicySpec::get_defaultAddCapabilities_for_reflect,
                    PodSecurityPolicySpec::mut_defaultAddCapabilities_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "requiredDropCapabilities",
                    PodSecurityPolicySpec::get_requiredDropCapabilities_for_reflect,
                    PodSecurityPolicySpec::mut_requiredDropCapabilities_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "allowedCapabilities",
                    PodSecurityPolicySpec::get_allowedCapabilities_for_reflect,
                    PodSecurityPolicySpec::mut_allowedCapabilities_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "volumes",
                    PodSecurityPolicySpec::get_volumes_for_reflect,
                    PodSecurityPolicySpec::mut_volumes_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "hostNetwork",
                    PodSecurityPolicySpec::get_hostNetwork_for_reflect,
                    PodSecurityPolicySpec::mut_hostNetwork_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<HostPortRange>>(
                    "hostPorts",
                    PodSecurityPolicySpec::get_hostPorts_for_reflect,
                    PodSecurityPolicySpec::mut_hostPorts_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "hostPID",
                    PodSecurityPolicySpec::get_hostPID_for_reflect,
                    PodSecurityPolicySpec::mut_hostPID_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "hostIPC",
                    PodSecurityPolicySpec::get_hostIPC_for_reflect,
                    PodSecurityPolicySpec::mut_hostIPC_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SELinuxStrategyOptions>>(
                    "seLinux",
                    PodSecurityPolicySpec::get_seLinux_for_reflect,
                    PodSecurityPolicySpec::mut_seLinux_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RunAsUserStrategyOptions>>(
                    "runAsUser",
                    PodSecurityPolicySpec::get_runAsUser_for_reflect,
                    PodSecurityPolicySpec::mut_runAsUser_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SupplementalGroupsStrategyOptions>>(
                    "supplementalGroups",
                    PodSecurityPolicySpec::get_supplementalGroups_for_reflect,
                    PodSecurityPolicySpec::mut_supplementalGroups_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<FSGroupStrategyOptions>>(
                    "fsGroup",
                    PodSecurityPolicySpec::get_fsGroup_for_reflect,
                    PodSecurityPolicySpec::mut_fsGroup_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "readOnlyRootFilesystem",
                    PodSecurityPolicySpec::get_readOnlyRootFilesystem_for_reflect,
                    PodSecurityPolicySpec::mut_readOnlyRootFilesystem_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PodSecurityPolicySpec>(
                    "PodSecurityPolicySpec",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PodSecurityPolicySpec {
    fn clear(&mut self) {
        self.clear_privileged();
        self.clear_defaultAddCapabilities();
        self.clear_requiredDropCapabilities();
        self.clear_allowedCapabilities();
        self.clear_volumes();
        self.clear_hostNetwork();
        self.clear_hostPorts();
        self.clear_hostPID();
        self.clear_hostIPC();
        self.clear_seLinux();
        self.clear_runAsUser();
        self.clear_supplementalGroups();
        self.clear_fsGroup();
        self.clear_readOnlyRootFilesystem();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PodSecurityPolicySpec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PodSecurityPolicySpec {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReplicaSet {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    spec: ::protobuf::SingularPtrField<ReplicaSetSpec>,
    status: ::protobuf::SingularPtrField<ReplicaSetStatus>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReplicaSet {}

impl ReplicaSet {
    pub fn new() -> ReplicaSet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReplicaSet {
        static mut instance: ::protobuf::lazy::Lazy<ReplicaSet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReplicaSet,
        };
        unsafe {
            instance.get(ReplicaSet::new)
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

    // optional .k8s.io.api.extensions.v1beta1.ReplicaSetSpec spec = 2;

    pub fn clear_spec(&mut self) {
        self.spec.clear();
    }

    pub fn has_spec(&self) -> bool {
        self.spec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spec(&mut self, v: ReplicaSetSpec) {
        self.spec = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_spec(&mut self) -> &mut ReplicaSetSpec {
        if self.spec.is_none() {
            self.spec.set_default();
        }
        self.spec.as_mut().unwrap()
    }

    // Take field
    pub fn take_spec(&mut self) -> ReplicaSetSpec {
        self.spec.take().unwrap_or_else(|| ReplicaSetSpec::new())
    }

    pub fn get_spec(&self) -> &ReplicaSetSpec {
        self.spec.as_ref().unwrap_or_else(|| ReplicaSetSpec::default_instance())
    }

    fn get_spec_for_reflect(&self) -> &::protobuf::SingularPtrField<ReplicaSetSpec> {
        &self.spec
    }

    fn mut_spec_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ReplicaSetSpec> {
        &mut self.spec
    }

    // optional .k8s.io.api.extensions.v1beta1.ReplicaSetStatus status = 3;

    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: ReplicaSetStatus) {
        self.status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut ReplicaSetStatus {
        if self.status.is_none() {
            self.status.set_default();
        }
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> ReplicaSetStatus {
        self.status.take().unwrap_or_else(|| ReplicaSetStatus::new())
    }

    pub fn get_status(&self) -> &ReplicaSetStatus {
        self.status.as_ref().unwrap_or_else(|| ReplicaSetStatus::default_instance())
    }

    fn get_status_for_reflect(&self) -> &::protobuf::SingularPtrField<ReplicaSetStatus> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ReplicaSetStatus> {
        &mut self.status
    }
}

impl ::protobuf::Message for ReplicaSet {
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

impl ::protobuf::MessageStatic for ReplicaSet {
    fn new() -> ReplicaSet {
        ReplicaSet::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReplicaSet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    ReplicaSet::get_metadata_for_reflect,
                    ReplicaSet::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ReplicaSetSpec>>(
                    "spec",
                    ReplicaSet::get_spec_for_reflect,
                    ReplicaSet::mut_spec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ReplicaSetStatus>>(
                    "status",
                    ReplicaSet::get_status_for_reflect,
                    ReplicaSet::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReplicaSet>(
                    "ReplicaSet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReplicaSet {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_spec();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReplicaSet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReplicaSet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReplicaSetCondition {
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
unsafe impl ::std::marker::Sync for ReplicaSetCondition {}

impl ReplicaSetCondition {
    pub fn new() -> ReplicaSetCondition {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReplicaSetCondition {
        static mut instance: ::protobuf::lazy::Lazy<ReplicaSetCondition> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReplicaSetCondition,
        };
        unsafe {
            instance.get(ReplicaSetCondition::new)
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

impl ::protobuf::Message for ReplicaSetCondition {
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

impl ::protobuf::MessageStatic for ReplicaSetCondition {
    fn new() -> ReplicaSetCondition {
        ReplicaSetCondition::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReplicaSetCondition>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    ReplicaSetCondition::get_field_type_for_reflect,
                    ReplicaSetCondition::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "status",
                    ReplicaSetCondition::get_status_for_reflect,
                    ReplicaSetCondition::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::Time>>(
                    "lastTransitionTime",
                    ReplicaSetCondition::get_lastTransitionTime_for_reflect,
                    ReplicaSetCondition::mut_lastTransitionTime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "reason",
                    ReplicaSetCondition::get_reason_for_reflect,
                    ReplicaSetCondition::mut_reason_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    ReplicaSetCondition::get_message_for_reflect,
                    ReplicaSetCondition::mut_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReplicaSetCondition>(
                    "ReplicaSetCondition",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReplicaSetCondition {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_status();
        self.clear_lastTransitionTime();
        self.clear_reason();
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReplicaSetCondition {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReplicaSetCondition {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReplicaSetList {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ListMeta>,
    items: ::protobuf::RepeatedField<ReplicaSet>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReplicaSetList {}

impl ReplicaSetList {
    pub fn new() -> ReplicaSetList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReplicaSetList {
        static mut instance: ::protobuf::lazy::Lazy<ReplicaSetList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReplicaSetList,
        };
        unsafe {
            instance.get(ReplicaSetList::new)
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

    // repeated .k8s.io.api.extensions.v1beta1.ReplicaSet items = 2;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<ReplicaSet>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<ReplicaSet> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<ReplicaSet> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[ReplicaSet] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<ReplicaSet> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ReplicaSet> {
        &mut self.items
    }
}

impl ::protobuf::Message for ReplicaSetList {
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

impl ::protobuf::MessageStatic for ReplicaSetList {
    fn new() -> ReplicaSetList {
        ReplicaSetList::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReplicaSetList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ListMeta>>(
                    "metadata",
                    ReplicaSetList::get_metadata_for_reflect,
                    ReplicaSetList::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ReplicaSet>>(
                    "items",
                    ReplicaSetList::get_items_for_reflect,
                    ReplicaSetList::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReplicaSetList>(
                    "ReplicaSetList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReplicaSetList {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReplicaSetList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReplicaSetList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReplicaSetSpec {
    // message fields
    replicas: ::std::option::Option<i32>,
    minReadySeconds: ::std::option::Option<i32>,
    selector: ::protobuf::SingularPtrField<super::generated::LabelSelector>,
    template: ::protobuf::SingularPtrField<super::generated::PodTemplateSpec>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReplicaSetSpec {}

impl ReplicaSetSpec {
    pub fn new() -> ReplicaSetSpec {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReplicaSetSpec {
        static mut instance: ::protobuf::lazy::Lazy<ReplicaSetSpec> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReplicaSetSpec,
        };
        unsafe {
            instance.get(ReplicaSetSpec::new)
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

    // optional int32 minReadySeconds = 4;

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
}

impl ::protobuf::Message for ReplicaSetSpec {
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
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.minReadySeconds = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.selector)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.template)?;
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
        if let Some(v) = self.minReadySeconds {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.selector.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.template.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.replicas {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.minReadySeconds {
            os.write_int32(4, v)?;
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

impl ::protobuf::MessageStatic for ReplicaSetSpec {
    fn new() -> ReplicaSetSpec {
        ReplicaSetSpec::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReplicaSetSpec>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "replicas",
                    ReplicaSetSpec::get_replicas_for_reflect,
                    ReplicaSetSpec::mut_replicas_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "minReadySeconds",
                    ReplicaSetSpec::get_minReadySeconds_for_reflect,
                    ReplicaSetSpec::mut_minReadySeconds_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::LabelSelector>>(
                    "selector",
                    ReplicaSetSpec::get_selector_for_reflect,
                    ReplicaSetSpec::mut_selector_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::PodTemplateSpec>>(
                    "template",
                    ReplicaSetSpec::get_template_for_reflect,
                    ReplicaSetSpec::mut_template_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReplicaSetSpec>(
                    "ReplicaSetSpec",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReplicaSetSpec {
    fn clear(&mut self) {
        self.clear_replicas();
        self.clear_minReadySeconds();
        self.clear_selector();
        self.clear_template();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReplicaSetSpec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReplicaSetSpec {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReplicaSetStatus {
    // message fields
    replicas: ::std::option::Option<i32>,
    fullyLabeledReplicas: ::std::option::Option<i32>,
    readyReplicas: ::std::option::Option<i32>,
    availableReplicas: ::std::option::Option<i32>,
    observedGeneration: ::std::option::Option<i64>,
    conditions: ::protobuf::RepeatedField<ReplicaSetCondition>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReplicaSetStatus {}

impl ReplicaSetStatus {
    pub fn new() -> ReplicaSetStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReplicaSetStatus {
        static mut instance: ::protobuf::lazy::Lazy<ReplicaSetStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReplicaSetStatus,
        };
        unsafe {
            instance.get(ReplicaSetStatus::new)
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

    // optional int32 fullyLabeledReplicas = 2;

    pub fn clear_fullyLabeledReplicas(&mut self) {
        self.fullyLabeledReplicas = ::std::option::Option::None;
    }

    pub fn has_fullyLabeledReplicas(&self) -> bool {
        self.fullyLabeledReplicas.is_some()
    }

    // Param is passed by value, moved
    pub fn set_fullyLabeledReplicas(&mut self, v: i32) {
        self.fullyLabeledReplicas = ::std::option::Option::Some(v);
    }

    pub fn get_fullyLabeledReplicas(&self) -> i32 {
        self.fullyLabeledReplicas.unwrap_or(0)
    }

    fn get_fullyLabeledReplicas_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.fullyLabeledReplicas
    }

    fn mut_fullyLabeledReplicas_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.fullyLabeledReplicas
    }

    // optional int32 readyReplicas = 4;

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

    // optional int32 availableReplicas = 5;

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

    // optional int64 observedGeneration = 3;

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

    // repeated .k8s.io.api.extensions.v1beta1.ReplicaSetCondition conditions = 6;

    pub fn clear_conditions(&mut self) {
        self.conditions.clear();
    }

    // Param is passed by value, moved
    pub fn set_conditions(&mut self, v: ::protobuf::RepeatedField<ReplicaSetCondition>) {
        self.conditions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_conditions(&mut self) -> &mut ::protobuf::RepeatedField<ReplicaSetCondition> {
        &mut self.conditions
    }

    // Take field
    pub fn take_conditions(&mut self) -> ::protobuf::RepeatedField<ReplicaSetCondition> {
        ::std::mem::replace(&mut self.conditions, ::protobuf::RepeatedField::new())
    }

    pub fn get_conditions(&self) -> &[ReplicaSetCondition] {
        &self.conditions
    }

    fn get_conditions_for_reflect(&self) -> &::protobuf::RepeatedField<ReplicaSetCondition> {
        &self.conditions
    }

    fn mut_conditions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ReplicaSetCondition> {
        &mut self.conditions
    }
}

impl ::protobuf::Message for ReplicaSetStatus {
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
                    let tmp = is.read_int32()?;
                    self.replicas = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.fullyLabeledReplicas = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.readyReplicas = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.availableReplicas = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.observedGeneration = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.conditions)?;
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
        if let Some(v) = self.fullyLabeledReplicas {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.readyReplicas {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.availableReplicas {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.observedGeneration {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.conditions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.replicas {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.fullyLabeledReplicas {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.readyReplicas {
            os.write_int32(4, v)?;
        }
        if let Some(v) = self.availableReplicas {
            os.write_int32(5, v)?;
        }
        if let Some(v) = self.observedGeneration {
            os.write_int64(3, v)?;
        }
        for v in &self.conditions {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ReplicaSetStatus {
    fn new() -> ReplicaSetStatus {
        ReplicaSetStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReplicaSetStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "replicas",
                    ReplicaSetStatus::get_replicas_for_reflect,
                    ReplicaSetStatus::mut_replicas_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "fullyLabeledReplicas",
                    ReplicaSetStatus::get_fullyLabeledReplicas_for_reflect,
                    ReplicaSetStatus::mut_fullyLabeledReplicas_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "readyReplicas",
                    ReplicaSetStatus::get_readyReplicas_for_reflect,
                    ReplicaSetStatus::mut_readyReplicas_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "availableReplicas",
                    ReplicaSetStatus::get_availableReplicas_for_reflect,
                    ReplicaSetStatus::mut_availableReplicas_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "observedGeneration",
                    ReplicaSetStatus::get_observedGeneration_for_reflect,
                    ReplicaSetStatus::mut_observedGeneration_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ReplicaSetCondition>>(
                    "conditions",
                    ReplicaSetStatus::get_conditions_for_reflect,
                    ReplicaSetStatus::mut_conditions_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReplicaSetStatus>(
                    "ReplicaSetStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReplicaSetStatus {
    fn clear(&mut self) {
        self.clear_replicas();
        self.clear_fullyLabeledReplicas();
        self.clear_readyReplicas();
        self.clear_availableReplicas();
        self.clear_observedGeneration();
        self.clear_conditions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReplicaSetStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReplicaSetStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ReplicationControllerDummy {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReplicationControllerDummy {}

impl ReplicationControllerDummy {
    pub fn new() -> ReplicationControllerDummy {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReplicationControllerDummy {
        static mut instance: ::protobuf::lazy::Lazy<ReplicationControllerDummy> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReplicationControllerDummy,
        };
        unsafe {
            instance.get(ReplicationControllerDummy::new)
        }
    }
}

impl ::protobuf::Message for ReplicationControllerDummy {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for ReplicationControllerDummy {
    fn new() -> ReplicationControllerDummy {
        ReplicationControllerDummy::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReplicationControllerDummy>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ReplicationControllerDummy>(
                    "ReplicationControllerDummy",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReplicationControllerDummy {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ReplicationControllerDummy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ReplicationControllerDummy {
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
pub struct RollingUpdateDaemonSet {
    // message fields
    maxUnavailable: ::protobuf::SingularPtrField<super::generated::IntOrString>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RollingUpdateDaemonSet {}

impl RollingUpdateDaemonSet {
    pub fn new() -> RollingUpdateDaemonSet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RollingUpdateDaemonSet {
        static mut instance: ::protobuf::lazy::Lazy<RollingUpdateDaemonSet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RollingUpdateDaemonSet,
        };
        unsafe {
            instance.get(RollingUpdateDaemonSet::new)
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
}

impl ::protobuf::Message for RollingUpdateDaemonSet {
    fn is_initialized(&self) -> bool {
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
        if let Some(ref v) = self.maxUnavailable.as_ref() {
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

impl ::protobuf::MessageStatic for RollingUpdateDaemonSet {
    fn new() -> RollingUpdateDaemonSet {
        RollingUpdateDaemonSet::new()
    }

    fn descriptor_static(_: ::std::option::Option<RollingUpdateDaemonSet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::IntOrString>>(
                    "maxUnavailable",
                    RollingUpdateDaemonSet::get_maxUnavailable_for_reflect,
                    RollingUpdateDaemonSet::mut_maxUnavailable_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RollingUpdateDaemonSet>(
                    "RollingUpdateDaemonSet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RollingUpdateDaemonSet {
    fn clear(&mut self) {
        self.clear_maxUnavailable();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RollingUpdateDaemonSet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RollingUpdateDaemonSet {
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
pub struct RunAsUserStrategyOptions {
    // message fields
    rule: ::protobuf::SingularField<::std::string::String>,
    ranges: ::protobuf::RepeatedField<IDRange>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RunAsUserStrategyOptions {}

impl RunAsUserStrategyOptions {
    pub fn new() -> RunAsUserStrategyOptions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RunAsUserStrategyOptions {
        static mut instance: ::protobuf::lazy::Lazy<RunAsUserStrategyOptions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RunAsUserStrategyOptions,
        };
        unsafe {
            instance.get(RunAsUserStrategyOptions::new)
        }
    }

    // optional string rule = 1;

    pub fn clear_rule(&mut self) {
        self.rule.clear();
    }

    pub fn has_rule(&self) -> bool {
        self.rule.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rule(&mut self, v: ::std::string::String) {
        self.rule = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rule(&mut self) -> &mut ::std::string::String {
        if self.rule.is_none() {
            self.rule.set_default();
        }
        self.rule.as_mut().unwrap()
    }

    // Take field
    pub fn take_rule(&mut self) -> ::std::string::String {
        self.rule.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_rule(&self) -> &str {
        match self.rule.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_rule_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.rule
    }

    fn mut_rule_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.rule
    }

    // repeated .k8s.io.api.extensions.v1beta1.IDRange ranges = 2;

    pub fn clear_ranges(&mut self) {
        self.ranges.clear();
    }

    // Param is passed by value, moved
    pub fn set_ranges(&mut self, v: ::protobuf::RepeatedField<IDRange>) {
        self.ranges = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ranges(&mut self) -> &mut ::protobuf::RepeatedField<IDRange> {
        &mut self.ranges
    }

    // Take field
    pub fn take_ranges(&mut self) -> ::protobuf::RepeatedField<IDRange> {
        ::std::mem::replace(&mut self.ranges, ::protobuf::RepeatedField::new())
    }

    pub fn get_ranges(&self) -> &[IDRange] {
        &self.ranges
    }

    fn get_ranges_for_reflect(&self) -> &::protobuf::RepeatedField<IDRange> {
        &self.ranges
    }

    fn mut_ranges_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<IDRange> {
        &mut self.ranges
    }
}

impl ::protobuf::Message for RunAsUserStrategyOptions {
    fn is_initialized(&self) -> bool {
        for v in &self.ranges {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.rule)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.ranges)?;
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
        if let Some(ref v) = self.rule.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.ranges {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.rule.as_ref() {
            os.write_string(1, &v)?;
        }
        for v in &self.ranges {
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

impl ::protobuf::MessageStatic for RunAsUserStrategyOptions {
    fn new() -> RunAsUserStrategyOptions {
        RunAsUserStrategyOptions::new()
    }

    fn descriptor_static(_: ::std::option::Option<RunAsUserStrategyOptions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "rule",
                    RunAsUserStrategyOptions::get_rule_for_reflect,
                    RunAsUserStrategyOptions::mut_rule_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<IDRange>>(
                    "ranges",
                    RunAsUserStrategyOptions::get_ranges_for_reflect,
                    RunAsUserStrategyOptions::mut_ranges_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RunAsUserStrategyOptions>(
                    "RunAsUserStrategyOptions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RunAsUserStrategyOptions {
    fn clear(&mut self) {
        self.clear_rule();
        self.clear_ranges();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RunAsUserStrategyOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RunAsUserStrategyOptions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SELinuxStrategyOptions {
    // message fields
    rule: ::protobuf::SingularField<::std::string::String>,
    seLinuxOptions: ::protobuf::SingularPtrField<super::generated::SELinuxOptions>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SELinuxStrategyOptions {}

impl SELinuxStrategyOptions {
    pub fn new() -> SELinuxStrategyOptions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SELinuxStrategyOptions {
        static mut instance: ::protobuf::lazy::Lazy<SELinuxStrategyOptions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SELinuxStrategyOptions,
        };
        unsafe {
            instance.get(SELinuxStrategyOptions::new)
        }
    }

    // optional string rule = 1;

    pub fn clear_rule(&mut self) {
        self.rule.clear();
    }

    pub fn has_rule(&self) -> bool {
        self.rule.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rule(&mut self, v: ::std::string::String) {
        self.rule = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rule(&mut self) -> &mut ::std::string::String {
        if self.rule.is_none() {
            self.rule.set_default();
        }
        self.rule.as_mut().unwrap()
    }

    // Take field
    pub fn take_rule(&mut self) -> ::std::string::String {
        self.rule.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_rule(&self) -> &str {
        match self.rule.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_rule_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.rule
    }

    fn mut_rule_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.rule
    }

    // optional .k8s.io.api.core.v1.SELinuxOptions seLinuxOptions = 2;

    pub fn clear_seLinuxOptions(&mut self) {
        self.seLinuxOptions.clear();
    }

    pub fn has_seLinuxOptions(&self) -> bool {
        self.seLinuxOptions.is_some()
    }

    // Param is passed by value, moved
    pub fn set_seLinuxOptions(&mut self, v: super::generated::SELinuxOptions) {
        self.seLinuxOptions = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_seLinuxOptions(&mut self) -> &mut super::generated::SELinuxOptions {
        if self.seLinuxOptions.is_none() {
            self.seLinuxOptions.set_default();
        }
        self.seLinuxOptions.as_mut().unwrap()
    }

    // Take field
    pub fn take_seLinuxOptions(&mut self) -> super::generated::SELinuxOptions {
        self.seLinuxOptions.take().unwrap_or_else(|| super::generated::SELinuxOptions::new())
    }

    pub fn get_seLinuxOptions(&self) -> &super::generated::SELinuxOptions {
        self.seLinuxOptions.as_ref().unwrap_or_else(|| super::generated::SELinuxOptions::default_instance())
    }

    fn get_seLinuxOptions_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::SELinuxOptions> {
        &self.seLinuxOptions
    }

    fn mut_seLinuxOptions_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::SELinuxOptions> {
        &mut self.seLinuxOptions
    }
}

impl ::protobuf::Message for SELinuxStrategyOptions {
    fn is_initialized(&self) -> bool {
        for v in &self.seLinuxOptions {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.rule)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.seLinuxOptions)?;
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
        if let Some(ref v) = self.rule.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.seLinuxOptions.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.rule.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.seLinuxOptions.as_ref() {
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

impl ::protobuf::MessageStatic for SELinuxStrategyOptions {
    fn new() -> SELinuxStrategyOptions {
        SELinuxStrategyOptions::new()
    }

    fn descriptor_static(_: ::std::option::Option<SELinuxStrategyOptions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "rule",
                    SELinuxStrategyOptions::get_rule_for_reflect,
                    SELinuxStrategyOptions::mut_rule_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::SELinuxOptions>>(
                    "seLinuxOptions",
                    SELinuxStrategyOptions::get_seLinuxOptions_for_reflect,
                    SELinuxStrategyOptions::mut_seLinuxOptions_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SELinuxStrategyOptions>(
                    "SELinuxStrategyOptions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SELinuxStrategyOptions {
    fn clear(&mut self) {
        self.clear_rule();
        self.clear_seLinuxOptions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SELinuxStrategyOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SELinuxStrategyOptions {
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

    // optional .k8s.io.api.extensions.v1beta1.ScaleSpec spec = 2;

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

    // optional .k8s.io.api.extensions.v1beta1.ScaleStatus status = 3;

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

    // repeated .k8s.io.api.extensions.v1beta1.ScaleStatus.SelectorEntry selector = 2;

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
pub struct SupplementalGroupsStrategyOptions {
    // message fields
    rule: ::protobuf::SingularField<::std::string::String>,
    ranges: ::protobuf::RepeatedField<IDRange>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SupplementalGroupsStrategyOptions {}

impl SupplementalGroupsStrategyOptions {
    pub fn new() -> SupplementalGroupsStrategyOptions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SupplementalGroupsStrategyOptions {
        static mut instance: ::protobuf::lazy::Lazy<SupplementalGroupsStrategyOptions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SupplementalGroupsStrategyOptions,
        };
        unsafe {
            instance.get(SupplementalGroupsStrategyOptions::new)
        }
    }

    // optional string rule = 1;

    pub fn clear_rule(&mut self) {
        self.rule.clear();
    }

    pub fn has_rule(&self) -> bool {
        self.rule.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rule(&mut self, v: ::std::string::String) {
        self.rule = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rule(&mut self) -> &mut ::std::string::String {
        if self.rule.is_none() {
            self.rule.set_default();
        }
        self.rule.as_mut().unwrap()
    }

    // Take field
    pub fn take_rule(&mut self) -> ::std::string::String {
        self.rule.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_rule(&self) -> &str {
        match self.rule.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_rule_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.rule
    }

    fn mut_rule_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.rule
    }

    // repeated .k8s.io.api.extensions.v1beta1.IDRange ranges = 2;

    pub fn clear_ranges(&mut self) {
        self.ranges.clear();
    }

    // Param is passed by value, moved
    pub fn set_ranges(&mut self, v: ::protobuf::RepeatedField<IDRange>) {
        self.ranges = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ranges(&mut self) -> &mut ::protobuf::RepeatedField<IDRange> {
        &mut self.ranges
    }

    // Take field
    pub fn take_ranges(&mut self) -> ::protobuf::RepeatedField<IDRange> {
        ::std::mem::replace(&mut self.ranges, ::protobuf::RepeatedField::new())
    }

    pub fn get_ranges(&self) -> &[IDRange] {
        &self.ranges
    }

    fn get_ranges_for_reflect(&self) -> &::protobuf::RepeatedField<IDRange> {
        &self.ranges
    }

    fn mut_ranges_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<IDRange> {
        &mut self.ranges
    }
}

impl ::protobuf::Message for SupplementalGroupsStrategyOptions {
    fn is_initialized(&self) -> bool {
        for v in &self.ranges {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.rule)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.ranges)?;
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
        if let Some(ref v) = self.rule.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        for value in &self.ranges {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.rule.as_ref() {
            os.write_string(1, &v)?;
        }
        for v in &self.ranges {
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

impl ::protobuf::MessageStatic for SupplementalGroupsStrategyOptions {
    fn new() -> SupplementalGroupsStrategyOptions {
        SupplementalGroupsStrategyOptions::new()
    }

    fn descriptor_static(_: ::std::option::Option<SupplementalGroupsStrategyOptions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "rule",
                    SupplementalGroupsStrategyOptions::get_rule_for_reflect,
                    SupplementalGroupsStrategyOptions::mut_rule_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<IDRange>>(
                    "ranges",
                    SupplementalGroupsStrategyOptions::get_ranges_for_reflect,
                    SupplementalGroupsStrategyOptions::mut_ranges_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SupplementalGroupsStrategyOptions>(
                    "SupplementalGroupsStrategyOptions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SupplementalGroupsStrategyOptions {
    fn clear(&mut self) {
        self.clear_rule();
        self.clear_ranges();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SupplementalGroupsStrategyOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SupplementalGroupsStrategyOptions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ThirdPartyResource {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    description: ::protobuf::SingularField<::std::string::String>,
    versions: ::protobuf::RepeatedField<APIVersion>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ThirdPartyResource {}

impl ThirdPartyResource {
    pub fn new() -> ThirdPartyResource {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ThirdPartyResource {
        static mut instance: ::protobuf::lazy::Lazy<ThirdPartyResource> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ThirdPartyResource,
        };
        unsafe {
            instance.get(ThirdPartyResource::new)
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

    // optional string description = 2;

    pub fn clear_description(&mut self) {
        self.description.clear();
    }

    pub fn has_description(&self) -> bool {
        self.description.is_some()
    }

    // Param is passed by value, moved
    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_description(&mut self) -> &mut ::std::string::String {
        if self.description.is_none() {
            self.description.set_default();
        }
        self.description.as_mut().unwrap()
    }

    // Take field
    pub fn take_description(&mut self) -> ::std::string::String {
        self.description.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_description(&self) -> &str {
        match self.description.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_description_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.description
    }

    fn mut_description_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.description
    }

    // repeated .k8s.io.api.extensions.v1beta1.APIVersion versions = 3;

    pub fn clear_versions(&mut self) {
        self.versions.clear();
    }

    // Param is passed by value, moved
    pub fn set_versions(&mut self, v: ::protobuf::RepeatedField<APIVersion>) {
        self.versions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_versions(&mut self) -> &mut ::protobuf::RepeatedField<APIVersion> {
        &mut self.versions
    }

    // Take field
    pub fn take_versions(&mut self) -> ::protobuf::RepeatedField<APIVersion> {
        ::std::mem::replace(&mut self.versions, ::protobuf::RepeatedField::new())
    }

    pub fn get_versions(&self) -> &[APIVersion] {
        &self.versions
    }

    fn get_versions_for_reflect(&self) -> &::protobuf::RepeatedField<APIVersion> {
        &self.versions
    }

    fn mut_versions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<APIVersion> {
        &mut self.versions
    }
}

impl ::protobuf::Message for ThirdPartyResource {
    fn is_initialized(&self) -> bool {
        for v in &self.metadata {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.versions {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.description)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.versions)?;
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
        if let Some(ref v) = self.description.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        for value in &self.versions {
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
        if let Some(ref v) = self.description.as_ref() {
            os.write_string(2, &v)?;
        }
        for v in &self.versions {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for ThirdPartyResource {
    fn new() -> ThirdPartyResource {
        ThirdPartyResource::new()
    }

    fn descriptor_static(_: ::std::option::Option<ThirdPartyResource>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    ThirdPartyResource::get_metadata_for_reflect,
                    ThirdPartyResource::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "description",
                    ThirdPartyResource::get_description_for_reflect,
                    ThirdPartyResource::mut_description_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<APIVersion>>(
                    "versions",
                    ThirdPartyResource::get_versions_for_reflect,
                    ThirdPartyResource::mut_versions_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ThirdPartyResource>(
                    "ThirdPartyResource",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ThirdPartyResource {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_description();
        self.clear_versions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ThirdPartyResource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ThirdPartyResource {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ThirdPartyResourceData {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ThirdPartyResourceData {}

impl ThirdPartyResourceData {
    pub fn new() -> ThirdPartyResourceData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ThirdPartyResourceData {
        static mut instance: ::protobuf::lazy::Lazy<ThirdPartyResourceData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ThirdPartyResourceData,
        };
        unsafe {
            instance.get(ThirdPartyResourceData::new)
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

    // optional bytes data = 2;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        }
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_data_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.data
    }
}

impl ::protobuf::Message for ThirdPartyResourceData {
    fn is_initialized(&self) -> bool {
        for v in &self.metadata {
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
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data)?;
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
            my_size += ::protobuf::rt::bytes_size(2, &v);
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

impl ::protobuf::MessageStatic for ThirdPartyResourceData {
    fn new() -> ThirdPartyResourceData {
        ThirdPartyResourceData::new()
    }

    fn descriptor_static(_: ::std::option::Option<ThirdPartyResourceData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    ThirdPartyResourceData::get_metadata_for_reflect,
                    ThirdPartyResourceData::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    ThirdPartyResourceData::get_data_for_reflect,
                    ThirdPartyResourceData::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ThirdPartyResourceData>(
                    "ThirdPartyResourceData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ThirdPartyResourceData {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ThirdPartyResourceData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ThirdPartyResourceData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ThirdPartyResourceDataList {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ListMeta>,
    items: ::protobuf::RepeatedField<ThirdPartyResourceData>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ThirdPartyResourceDataList {}

impl ThirdPartyResourceDataList {
    pub fn new() -> ThirdPartyResourceDataList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ThirdPartyResourceDataList {
        static mut instance: ::protobuf::lazy::Lazy<ThirdPartyResourceDataList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ThirdPartyResourceDataList,
        };
        unsafe {
            instance.get(ThirdPartyResourceDataList::new)
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

    // repeated .k8s.io.api.extensions.v1beta1.ThirdPartyResourceData items = 2;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<ThirdPartyResourceData>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<ThirdPartyResourceData> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<ThirdPartyResourceData> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[ThirdPartyResourceData] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<ThirdPartyResourceData> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ThirdPartyResourceData> {
        &mut self.items
    }
}

impl ::protobuf::Message for ThirdPartyResourceDataList {
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

impl ::protobuf::MessageStatic for ThirdPartyResourceDataList {
    fn new() -> ThirdPartyResourceDataList {
        ThirdPartyResourceDataList::new()
    }

    fn descriptor_static(_: ::std::option::Option<ThirdPartyResourceDataList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ListMeta>>(
                    "metadata",
                    ThirdPartyResourceDataList::get_metadata_for_reflect,
                    ThirdPartyResourceDataList::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ThirdPartyResourceData>>(
                    "items",
                    ThirdPartyResourceDataList::get_items_for_reflect,
                    ThirdPartyResourceDataList::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ThirdPartyResourceDataList>(
                    "ThirdPartyResourceDataList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ThirdPartyResourceDataList {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ThirdPartyResourceDataList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ThirdPartyResourceDataList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ThirdPartyResourceList {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ListMeta>,
    items: ::protobuf::RepeatedField<ThirdPartyResource>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ThirdPartyResourceList {}

impl ThirdPartyResourceList {
    pub fn new() -> ThirdPartyResourceList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ThirdPartyResourceList {
        static mut instance: ::protobuf::lazy::Lazy<ThirdPartyResourceList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ThirdPartyResourceList,
        };
        unsafe {
            instance.get(ThirdPartyResourceList::new)
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

    // repeated .k8s.io.api.extensions.v1beta1.ThirdPartyResource items = 2;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<ThirdPartyResource>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<ThirdPartyResource> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<ThirdPartyResource> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[ThirdPartyResource] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<ThirdPartyResource> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ThirdPartyResource> {
        &mut self.items
    }
}

impl ::protobuf::Message for ThirdPartyResourceList {
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

impl ::protobuf::MessageStatic for ThirdPartyResourceList {
    fn new() -> ThirdPartyResourceList {
        ThirdPartyResourceList::new()
    }

    fn descriptor_static(_: ::std::option::Option<ThirdPartyResourceList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ListMeta>>(
                    "metadata",
                    ThirdPartyResourceList::get_metadata_for_reflect,
                    ThirdPartyResourceList::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ThirdPartyResource>>(
                    "items",
                    ThirdPartyResourceList::get_items_for_reflect,
                    ThirdPartyResourceList::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ThirdPartyResourceList>(
                    "ThirdPartyResourceList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ThirdPartyResourceList {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ThirdPartyResourceList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ThirdPartyResourceList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n-k8s.io/api/extensions/v1beta1/generated.proto\x12\x1dk8s.io.api.exten\
    sions.v1beta1\x1a\"k8s.io/api/core/v1/generated.proto\x1a)k8s.io/api/pol\
    icy/v1beta1/generated.proto\x1a4k8s.io/apimachinery/pkg/api/resource/gen\
    erated.proto\x1a4k8s.io/apimachinery/pkg/apis/meta/v1/generated.proto\
    \x1a/k8s.io/apimachinery/pkg/runtime/generated.proto\x1a6k8s.io/apimachi\
    nery/pkg/runtime/schema/generated.proto\x1a3k8s.io/apimachinery/pkg/util\
    /intstr/generated.proto\"\x20\n\nAPIVersion\x12\x12\n\x04name\x18\x01\
    \x20\x01(\tR\x04name\"u\n\x19CustomMetricCurrentStatus\x12\x12\n\x04name\
    \x18\x01\x20\x01(\tR\x04name\x12D\n\x05value\x18\x02\x20\x01(\x0b2..k8s.\
    io.apimachinery.pkg.api.resource.QuantityR\x05value\"o\n\x1dCustomMetric\
    CurrentStatusList\x12N\n\x05items\x18\x01\x20\x03(\x0b28.k8s.io.api.exte\
    nsions.v1beta1.CustomMetricCurrentStatusR\x05items\"n\n\x12CustomMetricT\
    arget\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12D\n\x05value\x18\
    \x02\x20\x01(\x0b2..k8s.io.apimachinery.pkg.api.resource.QuantityR\x05va\
    lue\"a\n\x16CustomMetricTargetList\x12G\n\x05items\x18\x01\x20\x03(\x0b2\
    1.k8s.io.api.extensions.v1beta1.CustomMetricTargetR\x05items\"\xe3\x01\n\
    \tDaemonSet\x12L\n\x08metadata\x18\x01\x20\x01(\x0b20.k8s.io.apimachiner\
    y.pkg.apis.meta.v1.ObjectMetaR\x08metadata\x12@\n\x04spec\x18\x02\x20\
    \x01(\x0b2,.k8s.io.api.extensions.v1beta1.DaemonSetSpecR\x04spec\x12F\n\
    \x06status\x18\x03\x20\x01(\x0b2..k8s.io.api.extensions.v1beta1.DaemonSe\
    tStatusR\x06status\"\x9b\x01\n\rDaemonSetList\x12J\n\x08metadata\x18\x01\
    \x20\x01(\x0b2..k8s.io.apimachinery.pkg.apis.meta.v1.ListMetaR\x08metada\
    ta\x12>\n\x05items\x18\x02\x20\x03(\x0b2(.k8s.io.api.extensions.v1beta1.\
    DaemonSetR\x05items\"\x8f\x03\n\rDaemonSetSpec\x12O\n\x08selector\x18\
    \x01\x20\x01(\x0b23.k8s.io.apimachinery.pkg.apis.meta.v1.LabelSelectorR\
    \x08selector\x12?\n\x08template\x18\x02\x20\x01(\x0b2#.k8s.io.api.core.v\
    1.PodTemplateSpecR\x08template\x12^\n\x0eupdateStrategy\x18\x03\x20\x01(\
    \x0b26.k8s.io.api.extensions.v1beta1.DaemonSetUpdateStrategyR\x0eupdateS\
    trategy\x12(\n\x0fminReadySeconds\x18\x04\x20\x01(\x05R\x0fminReadySecon\
    ds\x12.\n\x12templateGeneration\x18\x05\x20\x01(\x03R\x12templateGenerat\
    ion\x122\n\x14revisionHistoryLimit\x18\x06\x20\x01(\x05R\x14revisionHist\
    oryLimit\"\xbb\x03\n\x0fDaemonSetStatus\x126\n\x16currentNumberScheduled\
    \x18\x01\x20\x01(\x05R\x16currentNumberScheduled\x12.\n\x12numberMissche\
    duled\x18\x02\x20\x01(\x05R\x12numberMisscheduled\x126\n\x16desiredNumbe\
    rScheduled\x18\x03\x20\x01(\x05R\x16desiredNumberScheduled\x12\x20\n\x0b\
    numberReady\x18\x04\x20\x01(\x05R\x0bnumberReady\x12.\n\x12observedGener\
    ation\x18\x05\x20\x01(\x03R\x12observedGeneration\x126\n\x16updatedNumbe\
    rScheduled\x18\x06\x20\x01(\x05R\x16updatedNumberScheduled\x12(\n\x0fnum\
    berAvailable\x18\x07\x20\x01(\x05R\x0fnumberAvailable\x12,\n\x11numberUn\
    available\x18\x08\x20\x01(\x05R\x11numberUnavailable\x12&\n\x0ecollision\
    Count\x18\t\x20\x01(\x03R\x0ecollisionCount\"\x8a\x01\n\x17DaemonSetUpda\
    teStrategy\x12\x12\n\x04type\x18\x01\x20\x01(\tR\x04type\x12[\n\rrolling\
    Update\x18\x02\x20\x01(\x0b25.k8s.io.api.extensions.v1beta1.RollingUpdat\
    eDaemonSetR\rrollingUpdate\"\xe6\x01\n\nDeployment\x12L\n\x08metadata\
    \x18\x01\x20\x01(\x0b20.k8s.io.apimachinery.pkg.apis.meta.v1.ObjectMetaR\
    \x08metadata\x12A\n\x04spec\x18\x02\x20\x01(\x0b2-.k8s.io.api.extensions\
    .v1beta1.DeploymentSpecR\x04spec\x12G\n\x06status\x18\x03\x20\x01(\x0b2/\
    .k8s.io.api.extensions.v1beta1.DeploymentStatusR\x06status\"\xa3\x02\n\
    \x13DeploymentCondition\x12\x12\n\x04type\x18\x01\x20\x01(\tR\x04type\
    \x12\x16\n\x06status\x18\x02\x20\x01(\tR\x06status\x12R\n\x0elastUpdateT\
    ime\x18\x06\x20\x01(\x0b2*.k8s.io.apimachinery.pkg.apis.meta.v1.TimeR\
    \x0elastUpdateTime\x12Z\n\x12lastTransitionTime\x18\x07\x20\x01(\x0b2*.k\
    8s.io.apimachinery.pkg.apis.meta.v1.TimeR\x12lastTransitionTime\x12\x16\
    \n\x06reason\x18\x04\x20\x01(\tR\x06reason\x12\x18\n\x07message\x18\x05\
    \x20\x01(\tR\x07message\"\x9d\x01\n\x0eDeploymentList\x12J\n\x08metadata\
    \x18\x01\x20\x01(\x0b2..k8s.io.apimachinery.pkg.apis.meta.v1.ListMetaR\
    \x08metadata\x12?\n\x05items\x18\x02\x20\x03(\x0b2).k8s.io.api.extension\
    s.v1beta1.DeploymentR\x05items\"\xb9\x02\n\x12DeploymentRollback\x12\x12\
    \n\x04name\x18\x01\x20\x01(\tR\x04name\x12y\n\x12updatedAnnotations\x18\
    \x02\x20\x03(\x0b2I.k8s.io.api.extensions.v1beta1.DeploymentRollback.Upd\
    atedAnnotationsEntryR\x12updatedAnnotations\x12M\n\nrollbackTo\x18\x03\
    \x20\x01(\x0b2-.k8s.io.api.extensions.v1beta1.RollbackConfigR\nrollbackT\
    o\x1aE\n\x17UpdatedAnnotationsEntry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\
    \x03key\x12\x14\n\x05value\x18\x02\x20\x01(\tR\x05value:\x028\x01\"\x8c\
    \x04\n\x0eDeploymentSpec\x12\x1a\n\x08replicas\x18\x01\x20\x01(\x05R\x08\
    replicas\x12O\n\x08selector\x18\x02\x20\x01(\x0b23.k8s.io.apimachinery.p\
    kg.apis.meta.v1.LabelSelectorR\x08selector\x12?\n\x08template\x18\x03\
    \x20\x01(\x0b2#.k8s.io.api.core.v1.PodTemplateSpecR\x08template\x12M\n\
    \x08strategy\x18\x04\x20\x01(\x0b21.k8s.io.api.extensions.v1beta1.Deploy\
    mentStrategyR\x08strategy\x12(\n\x0fminReadySeconds\x18\x05\x20\x01(\x05\
    R\x0fminReadySeconds\x122\n\x14revisionHistoryLimit\x18\x06\x20\x01(\x05\
    R\x14revisionHistoryLimit\x12\x16\n\x06paused\x18\x07\x20\x01(\x08R\x06p\
    aused\x12M\n\nrollbackTo\x18\x08\x20\x01(\x0b2-.k8s.io.api.extensions.v1\
    beta1.RollbackConfigR\nrollbackTo\x128\n\x17progressDeadlineSeconds\x18\
    \t\x20\x01(\x05R\x17progressDeadlineSeconds\"\x8a\x03\n\x10DeploymentSta\
    tus\x12.\n\x12observedGeneration\x18\x01\x20\x01(\x03R\x12observedGenera\
    tion\x12\x1a\n\x08replicas\x18\x02\x20\x01(\x05R\x08replicas\x12(\n\x0fu\
    pdatedReplicas\x18\x03\x20\x01(\x05R\x0fupdatedReplicas\x12$\n\rreadyRep\
    licas\x18\x07\x20\x01(\x05R\rreadyReplicas\x12,\n\x11availableReplicas\
    \x18\x04\x20\x01(\x05R\x11availableReplicas\x120\n\x13unavailableReplica\
    s\x18\x05\x20\x01(\x05R\x13unavailableReplicas\x12R\n\nconditions\x18\
    \x06\x20\x03(\x0b22.k8s.io.api.extensions.v1beta1.DeploymentConditionR\n\
    conditions\x12&\n\x0ecollisionCount\x18\x08\x20\x01(\x03R\x0ecollisionCo\
    unt\"\x86\x01\n\x12DeploymentStrategy\x12\x12\n\x04type\x18\x01\x20\x01(\
    \tR\x04type\x12\\\n\rrollingUpdate\x18\x02\x20\x01(\x0b26.k8s.io.api.ext\
    ensions.v1beta1.RollingUpdateDeploymentR\rrollingUpdate\"l\n\x16FSGroupS\
    trategyOptions\x12\x12\n\x04rule\x18\x01\x20\x01(\tR\x04rule\x12>\n\x06r\
    anges\x18\x02\x20\x03(\x0b2&.k8s.io.api.extensions.v1beta1.IDRangeR\x06r\
    anges\"n\n\x0fHTTPIngressPath\x12\x12\n\x04path\x18\x01\x20\x01(\tR\x04p\
    ath\x12G\n\x07backend\x18\x02\x20\x01(\x0b2-.k8s.io.api.extensions.v1bet\
    a1.IngressBackendR\x07backend\"\\\n\x14HTTPIngressRuleValue\x12D\n\x05pa\
    ths\x18\x01\x20\x03(\x0b2..k8s.io.api.extensions.v1beta1.HTTPIngressPath\
    R\x05paths\"3\n\rHostPortRange\x12\x10\n\x03min\x18\x01\x20\x01(\x05R\
    \x03min\x12\x10\n\x03max\x18\x02\x20\x01(\x05R\x03max\"-\n\x07IDRange\
    \x12\x10\n\x03min\x18\x01\x20\x01(\x03R\x03min\x12\x10\n\x03max\x18\x02\
    \x20\x01(\x03R\x03max\"\xdd\x01\n\x07Ingress\x12L\n\x08metadata\x18\x01\
    \x20\x01(\x0b20.k8s.io.apimachinery.pkg.apis.meta.v1.ObjectMetaR\x08meta\
    data\x12>\n\x04spec\x18\x02\x20\x01(\x0b2*.k8s.io.api.extensions.v1beta1\
    .IngressSpecR\x04spec\x12D\n\x06status\x18\x03\x20\x01(\x0b2,.k8s.io.api\
    .extensions.v1beta1.IngressStatusR\x06status\"\x86\x01\n\x0eIngressBacke\
    nd\x12\x20\n\x0bserviceName\x18\x01\x20\x01(\tR\x0bserviceName\x12R\n\
    \x0bservicePort\x18\x02\x20\x01(\x0b20.k8s.io.apimachinery.pkg.util.ints\
    tr.IntOrStringR\x0bservicePort\"\x97\x01\n\x0bIngressList\x12J\n\x08meta\
    data\x18\x01\x20\x01(\x0b2..k8s.io.apimachinery.pkg.apis.meta.v1.ListMet\
    aR\x08metadata\x12<\n\x05items\x18\x02\x20\x03(\x0b2&.k8s.io.api.extensi\
    ons.v1beta1.IngressR\x05items\"~\n\x0bIngressRule\x12\x12\n\x04host\x18\
    \x01\x20\x01(\tR\x04host\x12[\n\x10ingressRuleValue\x18\x02\x20\x01(\x0b\
    2/.k8s.io.api.extensions.v1beta1.IngressRuleValueR\x10ingressRuleValue\"\
    [\n\x10IngressRuleValue\x12G\n\x04http\x18\x01\x20\x01(\x0b23.k8s.io.api\
    .extensions.v1beta1.HTTPIngressRuleValueR\x04http\"\xd5\x01\n\x0bIngress\
    Spec\x12G\n\x07backend\x18\x01\x20\x01(\x0b2-.k8s.io.api.extensions.v1be\
    ta1.IngressBackendR\x07backend\x12;\n\x03tls\x18\x02\x20\x03(\x0b2).k8s.\
    io.api.extensions.v1beta1.IngressTLSR\x03tls\x12@\n\x05rules\x18\x03\x20\
    \x03(\x0b2*.k8s.io.api.extensions.v1beta1.IngressRuleR\x05rules\"[\n\rIn\
    gressStatus\x12J\n\x0cloadBalancer\x18\x01\x20\x01(\x0b2&.k8s.io.api.cor\
    e.v1.LoadBalancerStatusR\x0cloadBalancer\"B\n\nIngressTLS\x12\x14\n\x05h\
    osts\x18\x01\x20\x03(\tR\x05hosts\x12\x1e\n\nsecretName\x18\x02\x20\x01(\
    \tR\nsecretName\"\xa3\x01\n\rNetworkPolicy\x12L\n\x08metadata\x18\x01\
    \x20\x01(\x0b20.k8s.io.apimachinery.pkg.apis.meta.v1.ObjectMetaR\x08meta\
    data\x12D\n\x04spec\x18\x02\x20\x01(\x0b20.k8s.io.api.extensions.v1beta1\
    .NetworkPolicySpecR\x04spec\"\xa8\x01\n\x18NetworkPolicyIngressRule\x12F\
    \n\x05ports\x18\x01\x20\x03(\x0b20.k8s.io.api.extensions.v1beta1.Network\
    PolicyPortR\x05ports\x12D\n\x04from\x18\x02\x20\x03(\x0b20.k8s.io.api.ex\
    tensions.v1beta1.NetworkPolicyPeerR\x04from\"\xa3\x01\n\x11NetworkPolicy\
    List\x12J\n\x08metadata\x18\x01\x20\x01(\x0b2..k8s.io.apimachinery.pkg.a\
    pis.meta.v1.ListMetaR\x08metadata\x12B\n\x05items\x18\x02\x20\x03(\x0b2,\
    .k8s.io.api.extensions.v1beta1.NetworkPolicyR\x05items\"\xcd\x01\n\x11Ne\
    tworkPolicyPeer\x12U\n\x0bpodSelector\x18\x01\x20\x01(\x0b23.k8s.io.apim\
    achinery.pkg.apis.meta.v1.LabelSelectorR\x0bpodSelector\x12a\n\x11namesp\
    aceSelector\x18\x02\x20\x01(\x0b23.k8s.io.apimachinery.pkg.apis.meta.v1.\
    LabelSelectorR\x11namespaceSelector\"u\n\x11NetworkPolicyPort\x12\x1a\n\
    \x08protocol\x18\x01\x20\x01(\tR\x08protocol\x12D\n\x04port\x18\x02\x20\
    \x01(\x0b20.k8s.io.apimachinery.pkg.util.intstr.IntOrStringR\x04port\"\
    \xbd\x01\n\x11NetworkPolicySpec\x12U\n\x0bpodSelector\x18\x01\x20\x01(\
    \x0b23.k8s.io.apimachinery.pkg.apis.meta.v1.LabelSelectorR\x0bpodSelecto\
    r\x12Q\n\x07ingress\x18\x02\x20\x03(\x0b27.k8s.io.api.extensions.v1beta1\
    .NetworkPolicyIngressRuleR\x07ingress\"\xab\x01\n\x11PodSecurityPolicy\
    \x12L\n\x08metadata\x18\x01\x20\x01(\x0b20.k8s.io.apimachinery.pkg.apis.\
    meta.v1.ObjectMetaR\x08metadata\x12H\n\x04spec\x18\x02\x20\x01(\x0b24.k8\
    s.io.api.extensions.v1beta1.PodSecurityPolicySpecR\x04spec\"\xab\x01\n\
    \x15PodSecurityPolicyList\x12J\n\x08metadata\x18\x01\x20\x01(\x0b2..k8s.\
    io.apimachinery.pkg.apis.meta.v1.ListMetaR\x08metadata\x12F\n\x05items\
    \x18\x02\x20\x03(\x0b20.k8s.io.api.extensions.v1beta1.PodSecurityPolicyR\
    \x05items\"\xbc\x06\n\x15PodSecurityPolicySpec\x12\x1e\n\nprivileged\x18\
    \x01\x20\x01(\x08R\nprivileged\x126\n\x16defaultAddCapabilities\x18\x02\
    \x20\x03(\tR\x16defaultAddCapabilities\x12:\n\x18requiredDropCapabilitie\
    s\x18\x03\x20\x03(\tR\x18requiredDropCapabilities\x120\n\x13allowedCapab\
    ilities\x18\x04\x20\x03(\tR\x13allowedCapabilities\x12\x18\n\x07volumes\
    \x18\x05\x20\x03(\tR\x07volumes\x12\x20\n\x0bhostNetwork\x18\x06\x20\x01\
    (\x08R\x0bhostNetwork\x12J\n\thostPorts\x18\x07\x20\x03(\x0b2,.k8s.io.ap\
    i.extensions.v1beta1.HostPortRangeR\thostPorts\x12\x18\n\x07hostPID\x18\
    \x08\x20\x01(\x08R\x07hostPID\x12\x18\n\x07hostIPC\x18\t\x20\x01(\x08R\
    \x07hostIPC\x12O\n\x07seLinux\x18\n\x20\x01(\x0b25.k8s.io.api.extensions\
    .v1beta1.SELinuxStrategyOptionsR\x07seLinux\x12U\n\trunAsUser\x18\x0b\
    \x20\x01(\x0b27.k8s.io.api.extensions.v1beta1.RunAsUserStrategyOptionsR\
    \trunAsUser\x12p\n\x12supplementalGroups\x18\x0c\x20\x01(\x0b2@.k8s.io.a\
    pi.extensions.v1beta1.SupplementalGroupsStrategyOptionsR\x12supplemental\
    Groups\x12O\n\x07fsGroup\x18\r\x20\x01(\x0b25.k8s.io.api.extensions.v1be\
    ta1.FSGroupStrategyOptionsR\x07fsGroup\x126\n\x16readOnlyRootFilesystem\
    \x18\x0e\x20\x01(\x08R\x16readOnlyRootFilesystem\"\xe6\x01\n\nReplicaSet\
    \x12L\n\x08metadata\x18\x01\x20\x01(\x0b20.k8s.io.apimachinery.pkg.apis.\
    meta.v1.ObjectMetaR\x08metadata\x12A\n\x04spec\x18\x02\x20\x01(\x0b2-.k8\
    s.io.api.extensions.v1beta1.ReplicaSetSpecR\x04spec\x12G\n\x06status\x18\
    \x03\x20\x01(\x0b2/.k8s.io.api.extensions.v1beta1.ReplicaSetStatusR\x06s\
    tatus\"\xcf\x01\n\x13ReplicaSetCondition\x12\x12\n\x04type\x18\x01\x20\
    \x01(\tR\x04type\x12\x16\n\x06status\x18\x02\x20\x01(\tR\x06status\x12Z\
    \n\x12lastTransitionTime\x18\x03\x20\x01(\x0b2*.k8s.io.apimachinery.pkg.\
    apis.meta.v1.TimeR\x12lastTransitionTime\x12\x16\n\x06reason\x18\x04\x20\
    \x01(\tR\x06reason\x12\x18\n\x07message\x18\x05\x20\x01(\tR\x07message\"\
    \x9d\x01\n\x0eReplicaSetList\x12J\n\x08metadata\x18\x01\x20\x01(\x0b2..k\
    8s.io.apimachinery.pkg.apis.meta.v1.ListMetaR\x08metadata\x12?\n\x05item\
    s\x18\x02\x20\x03(\x0b2).k8s.io.api.extensions.v1beta1.ReplicaSetR\x05it\
    ems\"\xe8\x01\n\x0eReplicaSetSpec\x12\x1a\n\x08replicas\x18\x01\x20\x01(\
    \x05R\x08replicas\x12(\n\x0fminReadySeconds\x18\x04\x20\x01(\x05R\x0fmin\
    ReadySeconds\x12O\n\x08selector\x18\x02\x20\x01(\x0b23.k8s.io.apimachine\
    ry.pkg.apis.meta.v1.LabelSelectorR\x08selector\x12?\n\x08template\x18\
    \x03\x20\x01(\x0b2#.k8s.io.api.core.v1.PodTemplateSpecR\x08template\"\
    \xba\x02\n\x10ReplicaSetStatus\x12\x1a\n\x08replicas\x18\x01\x20\x01(\
    \x05R\x08replicas\x122\n\x14fullyLabeledReplicas\x18\x02\x20\x01(\x05R\
    \x14fullyLabeledReplicas\x12$\n\rreadyReplicas\x18\x04\x20\x01(\x05R\rre\
    adyReplicas\x12,\n\x11availableReplicas\x18\x05\x20\x01(\x05R\x11availab\
    leReplicas\x12.\n\x12observedGeneration\x18\x03\x20\x01(\x03R\x12observe\
    dGeneration\x12R\n\nconditions\x18\x06\x20\x03(\x0b22.k8s.io.api.extensi\
    ons.v1beta1.ReplicaSetConditionR\nconditions\"\x1c\n\x1aReplicationContr\
    ollerDummy\",\n\x0eRollbackConfig\x12\x1a\n\x08revision\x18\x01\x20\x01(\
    \x03R\x08revision\"r\n\x16RollingUpdateDaemonSet\x12X\n\x0emaxUnavailabl\
    e\x18\x01\x20\x01(\x0b20.k8s.io.apimachinery.pkg.util.intstr.IntOrString\
    R\x0emaxUnavailable\"\xc1\x01\n\x17RollingUpdateDeployment\x12X\n\x0emax\
    Unavailable\x18\x01\x20\x01(\x0b20.k8s.io.apimachinery.pkg.util.intstr.I\
    ntOrStringR\x0emaxUnavailable\x12L\n\x08maxSurge\x18\x02\x20\x01(\x0b20.\
    k8s.io.apimachinery.pkg.util.intstr.IntOrStringR\x08maxSurge\"n\n\x18Run\
    AsUserStrategyOptions\x12\x12\n\x04rule\x18\x01\x20\x01(\tR\x04rule\x12>\
    \n\x06ranges\x18\x02\x20\x03(\x0b2&.k8s.io.api.extensions.v1beta1.IDRang\
    eR\x06ranges\"x\n\x16SELinuxStrategyOptions\x12\x12\n\x04rule\x18\x01\
    \x20\x01(\tR\x04rule\x12J\n\x0eseLinuxOptions\x18\x02\x20\x01(\x0b2\".k8\
    s.io.api.core.v1.SELinuxOptionsR\x0eseLinuxOptions\"\xd7\x01\n\x05Scale\
    \x12L\n\x08metadata\x18\x01\x20\x01(\x0b20.k8s.io.apimachinery.pkg.apis.\
    meta.v1.ObjectMetaR\x08metadata\x12<\n\x04spec\x18\x02\x20\x01(\x0b2(.k8\
    s.io.api.extensions.v1beta1.ScaleSpecR\x04spec\x12B\n\x06status\x18\x03\
    \x20\x01(\x0b2*.k8s.io.api.extensions.v1beta1.ScaleStatusR\x06status\"'\
    \n\tScaleSpec\x12\x1a\n\x08replicas\x18\x01\x20\x01(\x05R\x08replicas\"\
    \xe4\x01\n\x0bScaleStatus\x12\x1a\n\x08replicas\x18\x01\x20\x01(\x05R\
    \x08replicas\x12T\n\x08selector\x18\x02\x20\x03(\x0b28.k8s.io.api.extens\
    ions.v1beta1.ScaleStatus.SelectorEntryR\x08selector\x12&\n\x0etargetSele\
    ctor\x18\x03\x20\x01(\tR\x0etargetSelector\x1a;\n\rSelectorEntry\x12\x10\
    \n\x03key\x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05value\x18\x02\x20\x01(\
    \tR\x05value:\x028\x01\"w\n!SupplementalGroupsStrategyOptions\x12\x12\n\
    \x04rule\x18\x01\x20\x01(\tR\x04rule\x12>\n\x06ranges\x18\x02\x20\x03(\
    \x0b2&.k8s.io.api.extensions.v1beta1.IDRangeR\x06ranges\"\xcb\x01\n\x12T\
    hirdPartyResource\x12L\n\x08metadata\x18\x01\x20\x01(\x0b20.k8s.io.apima\
    chinery.pkg.apis.meta.v1.ObjectMetaR\x08metadata\x12\x20\n\x0bdescriptio\
    n\x18\x02\x20\x01(\tR\x0bdescription\x12E\n\x08versions\x18\x03\x20\x03(\
    \x0b2).k8s.io.api.extensions.v1beta1.APIVersionR\x08versions\"z\n\x16Thi\
    rdPartyResourceData\x12L\n\x08metadata\x18\x01\x20\x01(\x0b20.k8s.io.api\
    machinery.pkg.apis.meta.v1.ObjectMetaR\x08metadata\x12\x12\n\x04data\x18\
    \x02\x20\x01(\x0cR\x04data\"\xb5\x01\n\x1aThirdPartyResourceDataList\x12\
    J\n\x08metadata\x18\x01\x20\x01(\x0b2..k8s.io.apimachinery.pkg.apis.meta\
    .v1.ListMetaR\x08metadata\x12K\n\x05items\x18\x02\x20\x03(\x0b25.k8s.io.\
    api.extensions.v1beta1.ThirdPartyResourceDataR\x05items\"\xad\x01\n\x16T\
    hirdPartyResourceList\x12J\n\x08metadata\x18\x01\x20\x01(\x0b2..k8s.io.a\
    pimachinery.pkg.apis.meta.v1.ListMetaR\x08metadata\x12G\n\x05items\x18\
    \x02\x20\x03(\x0b21.k8s.io.api.extensions.v1beta1.ThirdPartyResourceR\
    \x05itemsB\tZ\x07v1beta1\
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
