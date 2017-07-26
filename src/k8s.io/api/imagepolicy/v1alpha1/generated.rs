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
pub struct ImageReview {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    spec: ::protobuf::SingularPtrField<ImageReviewSpec>,
    status: ::protobuf::SingularPtrField<ImageReviewStatus>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ImageReview {}

impl ImageReview {
    pub fn new() -> ImageReview {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ImageReview {
        static mut instance: ::protobuf::lazy::Lazy<ImageReview> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ImageReview,
        };
        unsafe {
            instance.get(ImageReview::new)
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

    // optional .k8s.io.api.imagepolicy.v1alpha1.ImageReviewSpec spec = 2;

    pub fn clear_spec(&mut self) {
        self.spec.clear();
    }

    pub fn has_spec(&self) -> bool {
        self.spec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spec(&mut self, v: ImageReviewSpec) {
        self.spec = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_spec(&mut self) -> &mut ImageReviewSpec {
        if self.spec.is_none() {
            self.spec.set_default();
        }
        self.spec.as_mut().unwrap()
    }

    // Take field
    pub fn take_spec(&mut self) -> ImageReviewSpec {
        self.spec.take().unwrap_or_else(|| ImageReviewSpec::new())
    }

    pub fn get_spec(&self) -> &ImageReviewSpec {
        self.spec.as_ref().unwrap_or_else(|| ImageReviewSpec::default_instance())
    }

    fn get_spec_for_reflect(&self) -> &::protobuf::SingularPtrField<ImageReviewSpec> {
        &self.spec
    }

    fn mut_spec_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ImageReviewSpec> {
        &mut self.spec
    }

    // optional .k8s.io.api.imagepolicy.v1alpha1.ImageReviewStatus status = 3;

    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: ImageReviewStatus) {
        self.status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut ImageReviewStatus {
        if self.status.is_none() {
            self.status.set_default();
        }
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> ImageReviewStatus {
        self.status.take().unwrap_or_else(|| ImageReviewStatus::new())
    }

    pub fn get_status(&self) -> &ImageReviewStatus {
        self.status.as_ref().unwrap_or_else(|| ImageReviewStatus::default_instance())
    }

    fn get_status_for_reflect(&self) -> &::protobuf::SingularPtrField<ImageReviewStatus> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ImageReviewStatus> {
        &mut self.status
    }
}

impl ::protobuf::Message for ImageReview {
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

impl ::protobuf::MessageStatic for ImageReview {
    fn new() -> ImageReview {
        ImageReview::new()
    }

    fn descriptor_static(_: ::std::option::Option<ImageReview>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    ImageReview::get_metadata_for_reflect,
                    ImageReview::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ImageReviewSpec>>(
                    "spec",
                    ImageReview::get_spec_for_reflect,
                    ImageReview::mut_spec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ImageReviewStatus>>(
                    "status",
                    ImageReview::get_status_for_reflect,
                    ImageReview::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ImageReview>(
                    "ImageReview",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ImageReview {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_spec();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ImageReview {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ImageReview {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ImageReviewContainerSpec {
    // message fields
    image: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ImageReviewContainerSpec {}

impl ImageReviewContainerSpec {
    pub fn new() -> ImageReviewContainerSpec {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ImageReviewContainerSpec {
        static mut instance: ::protobuf::lazy::Lazy<ImageReviewContainerSpec> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ImageReviewContainerSpec,
        };
        unsafe {
            instance.get(ImageReviewContainerSpec::new)
        }
    }

    // optional string image = 1;

    pub fn clear_image(&mut self) {
        self.image.clear();
    }

    pub fn has_image(&self) -> bool {
        self.image.is_some()
    }

    // Param is passed by value, moved
    pub fn set_image(&mut self, v: ::std::string::String) {
        self.image = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_image(&mut self) -> &mut ::std::string::String {
        if self.image.is_none() {
            self.image.set_default();
        }
        self.image.as_mut().unwrap()
    }

    // Take field
    pub fn take_image(&mut self) -> ::std::string::String {
        self.image.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_image(&self) -> &str {
        match self.image.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_image_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.image
    }

    fn mut_image_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.image
    }
}

impl ::protobuf::Message for ImageReviewContainerSpec {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.image)?;
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
        if let Some(ref v) = self.image.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.image.as_ref() {
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

impl ::protobuf::MessageStatic for ImageReviewContainerSpec {
    fn new() -> ImageReviewContainerSpec {
        ImageReviewContainerSpec::new()
    }

    fn descriptor_static(_: ::std::option::Option<ImageReviewContainerSpec>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "image",
                    ImageReviewContainerSpec::get_image_for_reflect,
                    ImageReviewContainerSpec::mut_image_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ImageReviewContainerSpec>(
                    "ImageReviewContainerSpec",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ImageReviewContainerSpec {
    fn clear(&mut self) {
        self.clear_image();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ImageReviewContainerSpec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ImageReviewContainerSpec {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ImageReviewSpec {
    // message fields
    containers: ::protobuf::RepeatedField<ImageReviewContainerSpec>,
    pub annotations: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    namespace: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ImageReviewSpec {}

impl ImageReviewSpec {
    pub fn new() -> ImageReviewSpec {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ImageReviewSpec {
        static mut instance: ::protobuf::lazy::Lazy<ImageReviewSpec> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ImageReviewSpec,
        };
        unsafe {
            instance.get(ImageReviewSpec::new)
        }
    }

    // repeated .k8s.io.api.imagepolicy.v1alpha1.ImageReviewContainerSpec containers = 1;

    pub fn clear_containers(&mut self) {
        self.containers.clear();
    }

    // Param is passed by value, moved
    pub fn set_containers(&mut self, v: ::protobuf::RepeatedField<ImageReviewContainerSpec>) {
        self.containers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_containers(&mut self) -> &mut ::protobuf::RepeatedField<ImageReviewContainerSpec> {
        &mut self.containers
    }

    // Take field
    pub fn take_containers(&mut self) -> ::protobuf::RepeatedField<ImageReviewContainerSpec> {
        ::std::mem::replace(&mut self.containers, ::protobuf::RepeatedField::new())
    }

    pub fn get_containers(&self) -> &[ImageReviewContainerSpec] {
        &self.containers
    }

    fn get_containers_for_reflect(&self) -> &::protobuf::RepeatedField<ImageReviewContainerSpec> {
        &self.containers
    }

    fn mut_containers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ImageReviewContainerSpec> {
        &mut self.containers
    }

    // repeated .k8s.io.api.imagepolicy.v1alpha1.ImageReviewSpec.AnnotationsEntry annotations = 2;

    pub fn clear_annotations(&mut self) {
        self.annotations.clear();
    }

    // Param is passed by value, moved
    pub fn set_annotations(&mut self, v: ::std::collections::HashMap<::std::string::String, ::std::string::String>) {
        self.annotations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_annotations(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.annotations
    }

    // Take field
    pub fn take_annotations(&mut self) -> ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        ::std::mem::replace(&mut self.annotations, ::std::collections::HashMap::new())
    }

    pub fn get_annotations(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.annotations
    }

    fn get_annotations_for_reflect(&self) -> &::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &self.annotations
    }

    fn mut_annotations_for_reflect(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, ::std::string::String> {
        &mut self.annotations
    }

    // optional string namespace = 3;

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

impl ::protobuf::Message for ImageReviewSpec {
    fn is_initialized(&self) -> bool {
        for v in &self.containers {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.containers)?;
                },
                2 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(wire_type, is, &mut self.annotations)?;
                },
                3 => {
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
        for value in &self.containers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(2, &self.annotations);
        if let Some(ref v) = self.namespace.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.containers {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(2, &self.annotations, os)?;
        if let Some(ref v) = self.namespace.as_ref() {
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

impl ::protobuf::MessageStatic for ImageReviewSpec {
    fn new() -> ImageReviewSpec {
        ImageReviewSpec::new()
    }

    fn descriptor_static(_: ::std::option::Option<ImageReviewSpec>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ImageReviewContainerSpec>>(
                    "containers",
                    ImageReviewSpec::get_containers_for_reflect,
                    ImageReviewSpec::mut_containers_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeString>(
                    "annotations",
                    ImageReviewSpec::get_annotations_for_reflect,
                    ImageReviewSpec::mut_annotations_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "namespace",
                    ImageReviewSpec::get_namespace_for_reflect,
                    ImageReviewSpec::mut_namespace_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ImageReviewSpec>(
                    "ImageReviewSpec",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ImageReviewSpec {
    fn clear(&mut self) {
        self.clear_containers();
        self.clear_annotations();
        self.clear_namespace();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ImageReviewSpec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ImageReviewSpec {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ImageReviewStatus {
    // message fields
    allowed: ::std::option::Option<bool>,
    reason: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ImageReviewStatus {}

impl ImageReviewStatus {
    pub fn new() -> ImageReviewStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ImageReviewStatus {
        static mut instance: ::protobuf::lazy::Lazy<ImageReviewStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ImageReviewStatus,
        };
        unsafe {
            instance.get(ImageReviewStatus::new)
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
}

impl ::protobuf::Message for ImageReviewStatus {
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

impl ::protobuf::MessageStatic for ImageReviewStatus {
    fn new() -> ImageReviewStatus {
        ImageReviewStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<ImageReviewStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "allowed",
                    ImageReviewStatus::get_allowed_for_reflect,
                    ImageReviewStatus::mut_allowed_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "reason",
                    ImageReviewStatus::get_reason_for_reflect,
                    ImageReviewStatus::mut_reason_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ImageReviewStatus>(
                    "ImageReviewStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ImageReviewStatus {
    fn clear(&mut self) {
        self.clear_allowed();
        self.clear_reason();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ImageReviewStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ImageReviewStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n/k8s.io/api/imagepolicy/v1alpha1/generated.proto\x12\x1fk8s.io.api.ima\
    gepolicy.v1alpha1\x1a4k8s.io/apimachinery/pkg/apis/meta/v1/generated.pro\
    to\x1a/k8s.io/apimachinery/pkg/runtime/generated.proto\x1a6k8s.io/apimac\
    hinery/pkg/runtime/schema/generated.proto\x1a3k8s.io/apimachinery/pkg/ut\
    il/intstr/generated.proto\"\xed\x01\n\x0bImageReview\x12L\n\x08metadata\
    \x18\x01\x20\x01(\x0b20.k8s.io.apimachinery.pkg.apis.meta.v1.ObjectMetaR\
    \x08metadata\x12D\n\x04spec\x18\x02\x20\x01(\x0b20.k8s.io.api.imagepolic\
    y.v1alpha1.ImageReviewSpecR\x04spec\x12J\n\x06status\x18\x03\x20\x01(\
    \x0b22.k8s.io.api.imagepolicy.v1alpha1.ImageReviewStatusR\x06status\"0\n\
    \x18ImageReviewContainerSpec\x12\x14\n\x05image\x18\x01\x20\x01(\tR\x05i\
    mage\"\xaf\x02\n\x0fImageReviewSpec\x12Y\n\ncontainers\x18\x01\x20\x03(\
    \x0b29.k8s.io.api.imagepolicy.v1alpha1.ImageReviewContainerSpecR\ncontai\
    ners\x12c\n\x0bannotations\x18\x02\x20\x03(\x0b2A.k8s.io.api.imagepolicy\
    .v1alpha1.ImageReviewSpec.AnnotationsEntryR\x0bannotations\x12\x1c\n\tna\
    mespace\x18\x03\x20\x01(\tR\tnamespace\x1a>\n\x10AnnotationsEntry\x12\
    \x10\n\x03key\x18\x01\x20\x01(\tR\x03key\x12\x14\n\x05value\x18\x02\x20\
    \x01(\tR\x05value:\x028\x01\"E\n\x11ImageReviewStatus\x12\x18\n\x07allow\
    ed\x18\x01\x20\x01(\x08R\x07allowed\x12\x16\n\x06reason\x18\x02\x20\x01(\
    \tR\x06reasonB\nZ\x08v1alpha1\
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
