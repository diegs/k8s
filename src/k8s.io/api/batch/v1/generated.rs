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
pub struct Job {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    spec: ::protobuf::SingularPtrField<JobSpec>,
    status: ::protobuf::SingularPtrField<JobStatus>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Job {}

impl Job {
    pub fn new() -> Job {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Job {
        static mut instance: ::protobuf::lazy::Lazy<Job> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Job,
        };
        unsafe {
            instance.get(Job::new)
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

    // optional .k8s.io.api.batch.v1.JobSpec spec = 2;

    pub fn clear_spec(&mut self) {
        self.spec.clear();
    }

    pub fn has_spec(&self) -> bool {
        self.spec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spec(&mut self, v: JobSpec) {
        self.spec = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_spec(&mut self) -> &mut JobSpec {
        if self.spec.is_none() {
            self.spec.set_default();
        }
        self.spec.as_mut().unwrap()
    }

    // Take field
    pub fn take_spec(&mut self) -> JobSpec {
        self.spec.take().unwrap_or_else(|| JobSpec::new())
    }

    pub fn get_spec(&self) -> &JobSpec {
        self.spec.as_ref().unwrap_or_else(|| JobSpec::default_instance())
    }

    fn get_spec_for_reflect(&self) -> &::protobuf::SingularPtrField<JobSpec> {
        &self.spec
    }

    fn mut_spec_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<JobSpec> {
        &mut self.spec
    }

    // optional .k8s.io.api.batch.v1.JobStatus status = 3;

    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: JobStatus) {
        self.status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut JobStatus {
        if self.status.is_none() {
            self.status.set_default();
        }
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> JobStatus {
        self.status.take().unwrap_or_else(|| JobStatus::new())
    }

    pub fn get_status(&self) -> &JobStatus {
        self.status.as_ref().unwrap_or_else(|| JobStatus::default_instance())
    }

    fn get_status_for_reflect(&self) -> &::protobuf::SingularPtrField<JobStatus> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<JobStatus> {
        &mut self.status
    }
}

impl ::protobuf::Message for Job {
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

impl ::protobuf::MessageStatic for Job {
    fn new() -> Job {
        Job::new()
    }

    fn descriptor_static(_: ::std::option::Option<Job>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    Job::get_metadata_for_reflect,
                    Job::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<JobSpec>>(
                    "spec",
                    Job::get_spec_for_reflect,
                    Job::mut_spec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<JobStatus>>(
                    "status",
                    Job::get_status_for_reflect,
                    Job::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Job>(
                    "Job",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Job {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_spec();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Job {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Job {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JobCondition {
    // message fields
    field_type: ::protobuf::SingularField<::std::string::String>,
    status: ::protobuf::SingularField<::std::string::String>,
    lastProbeTime: ::protobuf::SingularPtrField<super::generated::Time>,
    lastTransitionTime: ::protobuf::SingularPtrField<super::generated::Time>,
    reason: ::protobuf::SingularField<::std::string::String>,
    message: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JobCondition {}

impl JobCondition {
    pub fn new() -> JobCondition {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JobCondition {
        static mut instance: ::protobuf::lazy::Lazy<JobCondition> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JobCondition,
        };
        unsafe {
            instance.get(JobCondition::new)
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

    // optional .k8s.io.apimachinery.pkg.apis.meta.v1.Time lastProbeTime = 3;

    pub fn clear_lastProbeTime(&mut self) {
        self.lastProbeTime.clear();
    }

    pub fn has_lastProbeTime(&self) -> bool {
        self.lastProbeTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastProbeTime(&mut self, v: super::generated::Time) {
        self.lastProbeTime = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lastProbeTime(&mut self) -> &mut super::generated::Time {
        if self.lastProbeTime.is_none() {
            self.lastProbeTime.set_default();
        }
        self.lastProbeTime.as_mut().unwrap()
    }

    // Take field
    pub fn take_lastProbeTime(&mut self) -> super::generated::Time {
        self.lastProbeTime.take().unwrap_or_else(|| super::generated::Time::new())
    }

    pub fn get_lastProbeTime(&self) -> &super::generated::Time {
        self.lastProbeTime.as_ref().unwrap_or_else(|| super::generated::Time::default_instance())
    }

    fn get_lastProbeTime_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::Time> {
        &self.lastProbeTime
    }

    fn mut_lastProbeTime_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::Time> {
        &mut self.lastProbeTime
    }

    // optional .k8s.io.apimachinery.pkg.apis.meta.v1.Time lastTransitionTime = 4;

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

    // optional string reason = 5;

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

    // optional string message = 6;

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

impl ::protobuf::Message for JobCondition {
    fn is_initialized(&self) -> bool {
        for v in &self.lastProbeTime {
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
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.lastProbeTime)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.lastTransitionTime)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.reason)?;
                },
                6 => {
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
        if let Some(ref v) = self.lastProbeTime.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.lastTransitionTime.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.reason.as_ref() {
            my_size += ::protobuf::rt::string_size(5, &v);
        }
        if let Some(ref v) = self.message.as_ref() {
            my_size += ::protobuf::rt::string_size(6, &v);
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
        if let Some(ref v) = self.lastProbeTime.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.lastTransitionTime.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.reason.as_ref() {
            os.write_string(5, &v)?;
        }
        if let Some(ref v) = self.message.as_ref() {
            os.write_string(6, &v)?;
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

impl ::protobuf::MessageStatic for JobCondition {
    fn new() -> JobCondition {
        JobCondition::new()
    }

    fn descriptor_static(_: ::std::option::Option<JobCondition>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "type",
                    JobCondition::get_field_type_for_reflect,
                    JobCondition::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "status",
                    JobCondition::get_status_for_reflect,
                    JobCondition::mut_status_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::Time>>(
                    "lastProbeTime",
                    JobCondition::get_lastProbeTime_for_reflect,
                    JobCondition::mut_lastProbeTime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::Time>>(
                    "lastTransitionTime",
                    JobCondition::get_lastTransitionTime_for_reflect,
                    JobCondition::mut_lastTransitionTime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "reason",
                    JobCondition::get_reason_for_reflect,
                    JobCondition::mut_reason_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    JobCondition::get_message_for_reflect,
                    JobCondition::mut_message_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JobCondition>(
                    "JobCondition",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JobCondition {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_status();
        self.clear_lastProbeTime();
        self.clear_lastTransitionTime();
        self.clear_reason();
        self.clear_message();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JobCondition {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JobCondition {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JobList {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ListMeta>,
    items: ::protobuf::RepeatedField<Job>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JobList {}

impl JobList {
    pub fn new() -> JobList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JobList {
        static mut instance: ::protobuf::lazy::Lazy<JobList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JobList,
        };
        unsafe {
            instance.get(JobList::new)
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

    // repeated .k8s.io.api.batch.v1.Job items = 2;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<Job>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<Job> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<Job> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[Job] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<Job> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Job> {
        &mut self.items
    }
}

impl ::protobuf::Message for JobList {
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

impl ::protobuf::MessageStatic for JobList {
    fn new() -> JobList {
        JobList::new()
    }

    fn descriptor_static(_: ::std::option::Option<JobList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ListMeta>>(
                    "metadata",
                    JobList::get_metadata_for_reflect,
                    JobList::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Job>>(
                    "items",
                    JobList::get_items_for_reflect,
                    JobList::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JobList>(
                    "JobList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JobList {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JobList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JobList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JobSpec {
    // message fields
    parallelism: ::std::option::Option<i32>,
    completions: ::std::option::Option<i32>,
    activeDeadlineSeconds: ::std::option::Option<i64>,
    selector: ::protobuf::SingularPtrField<super::generated::LabelSelector>,
    manualSelector: ::std::option::Option<bool>,
    template: ::protobuf::SingularPtrField<super::generated::PodTemplateSpec>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JobSpec {}

impl JobSpec {
    pub fn new() -> JobSpec {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JobSpec {
        static mut instance: ::protobuf::lazy::Lazy<JobSpec> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JobSpec,
        };
        unsafe {
            instance.get(JobSpec::new)
        }
    }

    // optional int32 parallelism = 1;

    pub fn clear_parallelism(&mut self) {
        self.parallelism = ::std::option::Option::None;
    }

    pub fn has_parallelism(&self) -> bool {
        self.parallelism.is_some()
    }

    // Param is passed by value, moved
    pub fn set_parallelism(&mut self, v: i32) {
        self.parallelism = ::std::option::Option::Some(v);
    }

    pub fn get_parallelism(&self) -> i32 {
        self.parallelism.unwrap_or(0)
    }

    fn get_parallelism_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.parallelism
    }

    fn mut_parallelism_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.parallelism
    }

    // optional int32 completions = 2;

    pub fn clear_completions(&mut self) {
        self.completions = ::std::option::Option::None;
    }

    pub fn has_completions(&self) -> bool {
        self.completions.is_some()
    }

    // Param is passed by value, moved
    pub fn set_completions(&mut self, v: i32) {
        self.completions = ::std::option::Option::Some(v);
    }

    pub fn get_completions(&self) -> i32 {
        self.completions.unwrap_or(0)
    }

    fn get_completions_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.completions
    }

    fn mut_completions_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.completions
    }

    // optional int64 activeDeadlineSeconds = 3;

    pub fn clear_activeDeadlineSeconds(&mut self) {
        self.activeDeadlineSeconds = ::std::option::Option::None;
    }

    pub fn has_activeDeadlineSeconds(&self) -> bool {
        self.activeDeadlineSeconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_activeDeadlineSeconds(&mut self, v: i64) {
        self.activeDeadlineSeconds = ::std::option::Option::Some(v);
    }

    pub fn get_activeDeadlineSeconds(&self) -> i64 {
        self.activeDeadlineSeconds.unwrap_or(0)
    }

    fn get_activeDeadlineSeconds_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.activeDeadlineSeconds
    }

    fn mut_activeDeadlineSeconds_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.activeDeadlineSeconds
    }

    // optional .k8s.io.apimachinery.pkg.apis.meta.v1.LabelSelector selector = 4;

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

    // optional bool manualSelector = 5;

    pub fn clear_manualSelector(&mut self) {
        self.manualSelector = ::std::option::Option::None;
    }

    pub fn has_manualSelector(&self) -> bool {
        self.manualSelector.is_some()
    }

    // Param is passed by value, moved
    pub fn set_manualSelector(&mut self, v: bool) {
        self.manualSelector = ::std::option::Option::Some(v);
    }

    pub fn get_manualSelector(&self) -> bool {
        self.manualSelector.unwrap_or(false)
    }

    fn get_manualSelector_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.manualSelector
    }

    fn mut_manualSelector_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.manualSelector
    }

    // optional .k8s.io.api.core.v1.PodTemplateSpec template = 6;

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

impl ::protobuf::Message for JobSpec {
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
                    self.parallelism = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.completions = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.activeDeadlineSeconds = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.selector)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.manualSelector = ::std::option::Option::Some(tmp);
                },
                6 => {
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
        if let Some(v) = self.parallelism {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.completions {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.activeDeadlineSeconds {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.selector.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.manualSelector {
            my_size += 2;
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
        if let Some(v) = self.parallelism {
            os.write_int32(1, v)?;
        }
        if let Some(v) = self.completions {
            os.write_int32(2, v)?;
        }
        if let Some(v) = self.activeDeadlineSeconds {
            os.write_int64(3, v)?;
        }
        if let Some(ref v) = self.selector.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.manualSelector {
            os.write_bool(5, v)?;
        }
        if let Some(ref v) = self.template.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for JobSpec {
    fn new() -> JobSpec {
        JobSpec::new()
    }

    fn descriptor_static(_: ::std::option::Option<JobSpec>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "parallelism",
                    JobSpec::get_parallelism_for_reflect,
                    JobSpec::mut_parallelism_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "completions",
                    JobSpec::get_completions_for_reflect,
                    JobSpec::mut_completions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "activeDeadlineSeconds",
                    JobSpec::get_activeDeadlineSeconds_for_reflect,
                    JobSpec::mut_activeDeadlineSeconds_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::LabelSelector>>(
                    "selector",
                    JobSpec::get_selector_for_reflect,
                    JobSpec::mut_selector_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "manualSelector",
                    JobSpec::get_manualSelector_for_reflect,
                    JobSpec::mut_manualSelector_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::PodTemplateSpec>>(
                    "template",
                    JobSpec::get_template_for_reflect,
                    JobSpec::mut_template_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JobSpec>(
                    "JobSpec",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JobSpec {
    fn clear(&mut self) {
        self.clear_parallelism();
        self.clear_completions();
        self.clear_activeDeadlineSeconds();
        self.clear_selector();
        self.clear_manualSelector();
        self.clear_template();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JobSpec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JobSpec {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JobStatus {
    // message fields
    conditions: ::protobuf::RepeatedField<JobCondition>,
    startTime: ::protobuf::SingularPtrField<super::generated::Time>,
    completionTime: ::protobuf::SingularPtrField<super::generated::Time>,
    active: ::std::option::Option<i32>,
    succeeded: ::std::option::Option<i32>,
    failed: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JobStatus {}

impl JobStatus {
    pub fn new() -> JobStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JobStatus {
        static mut instance: ::protobuf::lazy::Lazy<JobStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JobStatus,
        };
        unsafe {
            instance.get(JobStatus::new)
        }
    }

    // repeated .k8s.io.api.batch.v1.JobCondition conditions = 1;

    pub fn clear_conditions(&mut self) {
        self.conditions.clear();
    }

    // Param is passed by value, moved
    pub fn set_conditions(&mut self, v: ::protobuf::RepeatedField<JobCondition>) {
        self.conditions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_conditions(&mut self) -> &mut ::protobuf::RepeatedField<JobCondition> {
        &mut self.conditions
    }

    // Take field
    pub fn take_conditions(&mut self) -> ::protobuf::RepeatedField<JobCondition> {
        ::std::mem::replace(&mut self.conditions, ::protobuf::RepeatedField::new())
    }

    pub fn get_conditions(&self) -> &[JobCondition] {
        &self.conditions
    }

    fn get_conditions_for_reflect(&self) -> &::protobuf::RepeatedField<JobCondition> {
        &self.conditions
    }

    fn mut_conditions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<JobCondition> {
        &mut self.conditions
    }

    // optional .k8s.io.apimachinery.pkg.apis.meta.v1.Time startTime = 2;

    pub fn clear_startTime(&mut self) {
        self.startTime.clear();
    }

    pub fn has_startTime(&self) -> bool {
        self.startTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_startTime(&mut self, v: super::generated::Time) {
        self.startTime = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_startTime(&mut self) -> &mut super::generated::Time {
        if self.startTime.is_none() {
            self.startTime.set_default();
        }
        self.startTime.as_mut().unwrap()
    }

    // Take field
    pub fn take_startTime(&mut self) -> super::generated::Time {
        self.startTime.take().unwrap_or_else(|| super::generated::Time::new())
    }

    pub fn get_startTime(&self) -> &super::generated::Time {
        self.startTime.as_ref().unwrap_or_else(|| super::generated::Time::default_instance())
    }

    fn get_startTime_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::Time> {
        &self.startTime
    }

    fn mut_startTime_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::Time> {
        &mut self.startTime
    }

    // optional .k8s.io.apimachinery.pkg.apis.meta.v1.Time completionTime = 3;

    pub fn clear_completionTime(&mut self) {
        self.completionTime.clear();
    }

    pub fn has_completionTime(&self) -> bool {
        self.completionTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_completionTime(&mut self, v: super::generated::Time) {
        self.completionTime = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_completionTime(&mut self) -> &mut super::generated::Time {
        if self.completionTime.is_none() {
            self.completionTime.set_default();
        }
        self.completionTime.as_mut().unwrap()
    }

    // Take field
    pub fn take_completionTime(&mut self) -> super::generated::Time {
        self.completionTime.take().unwrap_or_else(|| super::generated::Time::new())
    }

    pub fn get_completionTime(&self) -> &super::generated::Time {
        self.completionTime.as_ref().unwrap_or_else(|| super::generated::Time::default_instance())
    }

    fn get_completionTime_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::Time> {
        &self.completionTime
    }

    fn mut_completionTime_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::Time> {
        &mut self.completionTime
    }

    // optional int32 active = 4;

    pub fn clear_active(&mut self) {
        self.active = ::std::option::Option::None;
    }

    pub fn has_active(&self) -> bool {
        self.active.is_some()
    }

    // Param is passed by value, moved
    pub fn set_active(&mut self, v: i32) {
        self.active = ::std::option::Option::Some(v);
    }

    pub fn get_active(&self) -> i32 {
        self.active.unwrap_or(0)
    }

    fn get_active_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.active
    }

    fn mut_active_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.active
    }

    // optional int32 succeeded = 5;

    pub fn clear_succeeded(&mut self) {
        self.succeeded = ::std::option::Option::None;
    }

    pub fn has_succeeded(&self) -> bool {
        self.succeeded.is_some()
    }

    // Param is passed by value, moved
    pub fn set_succeeded(&mut self, v: i32) {
        self.succeeded = ::std::option::Option::Some(v);
    }

    pub fn get_succeeded(&self) -> i32 {
        self.succeeded.unwrap_or(0)
    }

    fn get_succeeded_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.succeeded
    }

    fn mut_succeeded_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.succeeded
    }

    // optional int32 failed = 6;

    pub fn clear_failed(&mut self) {
        self.failed = ::std::option::Option::None;
    }

    pub fn has_failed(&self) -> bool {
        self.failed.is_some()
    }

    // Param is passed by value, moved
    pub fn set_failed(&mut self, v: i32) {
        self.failed = ::std::option::Option::Some(v);
    }

    pub fn get_failed(&self) -> i32 {
        self.failed.unwrap_or(0)
    }

    fn get_failed_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.failed
    }

    fn mut_failed_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.failed
    }
}

impl ::protobuf::Message for JobStatus {
    fn is_initialized(&self) -> bool {
        for v in &self.conditions {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.startTime {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.completionTime {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.startTime)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.completionTime)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.active = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.succeeded = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.failed = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.startTime.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.completionTime.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.active {
            my_size += ::protobuf::rt::value_size(4, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.succeeded {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.failed {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
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
        if let Some(ref v) = self.startTime.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.completionTime.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.active {
            os.write_int32(4, v)?;
        }
        if let Some(v) = self.succeeded {
            os.write_int32(5, v)?;
        }
        if let Some(v) = self.failed {
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

impl ::protobuf::MessageStatic for JobStatus {
    fn new() -> JobStatus {
        JobStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<JobStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<JobCondition>>(
                    "conditions",
                    JobStatus::get_conditions_for_reflect,
                    JobStatus::mut_conditions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::Time>>(
                    "startTime",
                    JobStatus::get_startTime_for_reflect,
                    JobStatus::mut_startTime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::Time>>(
                    "completionTime",
                    JobStatus::get_completionTime_for_reflect,
                    JobStatus::mut_completionTime_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "active",
                    JobStatus::get_active_for_reflect,
                    JobStatus::mut_active_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "succeeded",
                    JobStatus::get_succeeded_for_reflect,
                    JobStatus::mut_succeeded_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "failed",
                    JobStatus::get_failed_for_reflect,
                    JobStatus::mut_failed_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JobStatus>(
                    "JobStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JobStatus {
    fn clear(&mut self) {
        self.clear_conditions();
        self.clear_startTime();
        self.clear_completionTime();
        self.clear_active();
        self.clear_succeeded();
        self.clear_failed();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JobStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JobStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n#k8s.io/api/batch/v1/generated.proto\x12\x13k8s.io.api.batch.v1\x1a\"k\
    8s.io/api/core/v1/generated.proto\x1a4k8s.io/apimachinery/pkg/apis/meta/\
    v1/generated.proto\x1a/k8s.io/apimachinery/pkg/runtime/generated.proto\
    \x1a6k8s.io/apimachinery/pkg/runtime/schema/generated.proto\x1a3k8s.io/a\
    pimachinery/pkg/util/intstr/generated.proto\"\xbd\x01\n\x03Job\x12L\n\
    \x08metadata\x18\x01\x20\x01(\x0b20.k8s.io.apimachinery.pkg.apis.meta.v1\
    .ObjectMetaR\x08metadata\x120\n\x04spec\x18\x02\x20\x01(\x0b2\x1c.k8s.io\
    .api.batch.v1.JobSpecR\x04spec\x126\n\x06status\x18\x03\x20\x01(\x0b2\
    \x1e.k8s.io.api.batch.v1.JobStatusR\x06status\"\x9a\x02\n\x0cJobConditio\
    n\x12\x12\n\x04type\x18\x01\x20\x01(\tR\x04type\x12\x16\n\x06status\x18\
    \x02\x20\x01(\tR\x06status\x12P\n\rlastProbeTime\x18\x03\x20\x01(\x0b2*.\
    k8s.io.apimachinery.pkg.apis.meta.v1.TimeR\rlastProbeTime\x12Z\n\x12last\
    TransitionTime\x18\x04\x20\x01(\x0b2*.k8s.io.apimachinery.pkg.apis.meta.\
    v1.TimeR\x12lastTransitionTime\x12\x16\n\x06reason\x18\x05\x20\x01(\tR\
    \x06reason\x12\x18\n\x07message\x18\x06\x20\x01(\tR\x07message\"\x85\x01\
    \n\x07JobList\x12J\n\x08metadata\x18\x01\x20\x01(\x0b2..k8s.io.apimachin\
    ery.pkg.apis.meta.v1.ListMetaR\x08metadata\x12.\n\x05items\x18\x02\x20\
    \x03(\x0b2\x18.k8s.io.api.batch.v1.JobR\x05items\"\xbd\x02\n\x07JobSpec\
    \x12\x20\n\x0bparallelism\x18\x01\x20\x01(\x05R\x0bparallelism\x12\x20\n\
    \x0bcompletions\x18\x02\x20\x01(\x05R\x0bcompletions\x124\n\x15activeDea\
    dlineSeconds\x18\x03\x20\x01(\x03R\x15activeDeadlineSeconds\x12O\n\x08se\
    lector\x18\x04\x20\x01(\x0b23.k8s.io.apimachinery.pkg.apis.meta.v1.Label\
    SelectorR\x08selector\x12&\n\x0emanualSelector\x18\x05\x20\x01(\x08R\x0e\
    manualSelector\x12?\n\x08template\x18\x06\x20\x01(\x0b2#.k8s.io.api.core\
    .v1.PodTemplateSpecR\x08template\"\xba\x02\n\tJobStatus\x12A\n\nconditio\
    ns\x18\x01\x20\x03(\x0b2!.k8s.io.api.batch.v1.JobConditionR\nconditions\
    \x12H\n\tstartTime\x18\x02\x20\x01(\x0b2*.k8s.io.apimachinery.pkg.apis.m\
    eta.v1.TimeR\tstartTime\x12R\n\x0ecompletionTime\x18\x03\x20\x01(\x0b2*.\
    k8s.io.apimachinery.pkg.apis.meta.v1.TimeR\x0ecompletionTime\x12\x16\n\
    \x06active\x18\x04\x20\x01(\x05R\x06active\x12\x1c\n\tsucceeded\x18\x05\
    \x20\x01(\x05R\tsucceeded\x12\x16\n\x06failed\x18\x06\x20\x01(\x05R\x06f\
    ailedB\x04Z\x02v1\
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
