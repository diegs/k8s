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
pub struct CronJob {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    spec: ::protobuf::SingularPtrField<CronJobSpec>,
    status: ::protobuf::SingularPtrField<CronJobStatus>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CronJob {}

impl CronJob {
    pub fn new() -> CronJob {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CronJob {
        static mut instance: ::protobuf::lazy::Lazy<CronJob> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CronJob,
        };
        unsafe {
            instance.get(CronJob::new)
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

    // optional .k8s.io.api.batch.v2alpha1.CronJobSpec spec = 2;

    pub fn clear_spec(&mut self) {
        self.spec.clear();
    }

    pub fn has_spec(&self) -> bool {
        self.spec.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spec(&mut self, v: CronJobSpec) {
        self.spec = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_spec(&mut self) -> &mut CronJobSpec {
        if self.spec.is_none() {
            self.spec.set_default();
        }
        self.spec.as_mut().unwrap()
    }

    // Take field
    pub fn take_spec(&mut self) -> CronJobSpec {
        self.spec.take().unwrap_or_else(|| CronJobSpec::new())
    }

    pub fn get_spec(&self) -> &CronJobSpec {
        self.spec.as_ref().unwrap_or_else(|| CronJobSpec::default_instance())
    }

    fn get_spec_for_reflect(&self) -> &::protobuf::SingularPtrField<CronJobSpec> {
        &self.spec
    }

    fn mut_spec_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CronJobSpec> {
        &mut self.spec
    }

    // optional .k8s.io.api.batch.v2alpha1.CronJobStatus status = 3;

    pub fn clear_status(&mut self) {
        self.status.clear();
    }

    pub fn has_status(&self) -> bool {
        self.status.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: CronJobStatus) {
        self.status = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status(&mut self) -> &mut CronJobStatus {
        if self.status.is_none() {
            self.status.set_default();
        }
        self.status.as_mut().unwrap()
    }

    // Take field
    pub fn take_status(&mut self) -> CronJobStatus {
        self.status.take().unwrap_or_else(|| CronJobStatus::new())
    }

    pub fn get_status(&self) -> &CronJobStatus {
        self.status.as_ref().unwrap_or_else(|| CronJobStatus::default_instance())
    }

    fn get_status_for_reflect(&self) -> &::protobuf::SingularPtrField<CronJobStatus> {
        &self.status
    }

    fn mut_status_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CronJobStatus> {
        &mut self.status
    }
}

impl ::protobuf::Message for CronJob {
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

impl ::protobuf::MessageStatic for CronJob {
    fn new() -> CronJob {
        CronJob::new()
    }

    fn descriptor_static(_: ::std::option::Option<CronJob>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    CronJob::get_metadata_for_reflect,
                    CronJob::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CronJobSpec>>(
                    "spec",
                    CronJob::get_spec_for_reflect,
                    CronJob::mut_spec_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CronJobStatus>>(
                    "status",
                    CronJob::get_status_for_reflect,
                    CronJob::mut_status_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CronJob>(
                    "CronJob",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CronJob {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_spec();
        self.clear_status();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CronJob {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CronJob {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CronJobList {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ListMeta>,
    items: ::protobuf::RepeatedField<CronJob>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CronJobList {}

impl CronJobList {
    pub fn new() -> CronJobList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CronJobList {
        static mut instance: ::protobuf::lazy::Lazy<CronJobList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CronJobList,
        };
        unsafe {
            instance.get(CronJobList::new)
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

    // repeated .k8s.io.api.batch.v2alpha1.CronJob items = 2;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<CronJob>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<CronJob> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<CronJob> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[CronJob] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<CronJob> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<CronJob> {
        &mut self.items
    }
}

impl ::protobuf::Message for CronJobList {
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

impl ::protobuf::MessageStatic for CronJobList {
    fn new() -> CronJobList {
        CronJobList::new()
    }

    fn descriptor_static(_: ::std::option::Option<CronJobList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ListMeta>>(
                    "metadata",
                    CronJobList::get_metadata_for_reflect,
                    CronJobList::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CronJob>>(
                    "items",
                    CronJobList::get_items_for_reflect,
                    CronJobList::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CronJobList>(
                    "CronJobList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CronJobList {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CronJobList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CronJobList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CronJobSpec {
    // message fields
    schedule: ::protobuf::SingularField<::std::string::String>,
    startingDeadlineSeconds: ::std::option::Option<i64>,
    concurrencyPolicy: ::protobuf::SingularField<::std::string::String>,
    suspend: ::std::option::Option<bool>,
    jobTemplate: ::protobuf::SingularPtrField<JobTemplateSpec>,
    successfulJobsHistoryLimit: ::std::option::Option<i32>,
    failedJobsHistoryLimit: ::std::option::Option<i32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CronJobSpec {}

impl CronJobSpec {
    pub fn new() -> CronJobSpec {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CronJobSpec {
        static mut instance: ::protobuf::lazy::Lazy<CronJobSpec> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CronJobSpec,
        };
        unsafe {
            instance.get(CronJobSpec::new)
        }
    }

    // optional string schedule = 1;

    pub fn clear_schedule(&mut self) {
        self.schedule.clear();
    }

    pub fn has_schedule(&self) -> bool {
        self.schedule.is_some()
    }

    // Param is passed by value, moved
    pub fn set_schedule(&mut self, v: ::std::string::String) {
        self.schedule = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_schedule(&mut self) -> &mut ::std::string::String {
        if self.schedule.is_none() {
            self.schedule.set_default();
        }
        self.schedule.as_mut().unwrap()
    }

    // Take field
    pub fn take_schedule(&mut self) -> ::std::string::String {
        self.schedule.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_schedule(&self) -> &str {
        match self.schedule.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_schedule_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.schedule
    }

    fn mut_schedule_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.schedule
    }

    // optional int64 startingDeadlineSeconds = 2;

    pub fn clear_startingDeadlineSeconds(&mut self) {
        self.startingDeadlineSeconds = ::std::option::Option::None;
    }

    pub fn has_startingDeadlineSeconds(&self) -> bool {
        self.startingDeadlineSeconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_startingDeadlineSeconds(&mut self, v: i64) {
        self.startingDeadlineSeconds = ::std::option::Option::Some(v);
    }

    pub fn get_startingDeadlineSeconds(&self) -> i64 {
        self.startingDeadlineSeconds.unwrap_or(0)
    }

    fn get_startingDeadlineSeconds_for_reflect(&self) -> &::std::option::Option<i64> {
        &self.startingDeadlineSeconds
    }

    fn mut_startingDeadlineSeconds_for_reflect(&mut self) -> &mut ::std::option::Option<i64> {
        &mut self.startingDeadlineSeconds
    }

    // optional string concurrencyPolicy = 3;

    pub fn clear_concurrencyPolicy(&mut self) {
        self.concurrencyPolicy.clear();
    }

    pub fn has_concurrencyPolicy(&self) -> bool {
        self.concurrencyPolicy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_concurrencyPolicy(&mut self, v: ::std::string::String) {
        self.concurrencyPolicy = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_concurrencyPolicy(&mut self) -> &mut ::std::string::String {
        if self.concurrencyPolicy.is_none() {
            self.concurrencyPolicy.set_default();
        }
        self.concurrencyPolicy.as_mut().unwrap()
    }

    // Take field
    pub fn take_concurrencyPolicy(&mut self) -> ::std::string::String {
        self.concurrencyPolicy.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_concurrencyPolicy(&self) -> &str {
        match self.concurrencyPolicy.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_concurrencyPolicy_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.concurrencyPolicy
    }

    fn mut_concurrencyPolicy_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.concurrencyPolicy
    }

    // optional bool suspend = 4;

    pub fn clear_suspend(&mut self) {
        self.suspend = ::std::option::Option::None;
    }

    pub fn has_suspend(&self) -> bool {
        self.suspend.is_some()
    }

    // Param is passed by value, moved
    pub fn set_suspend(&mut self, v: bool) {
        self.suspend = ::std::option::Option::Some(v);
    }

    pub fn get_suspend(&self) -> bool {
        self.suspend.unwrap_or(false)
    }

    fn get_suspend_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.suspend
    }

    fn mut_suspend_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.suspend
    }

    // optional .k8s.io.api.batch.v2alpha1.JobTemplateSpec jobTemplate = 5;

    pub fn clear_jobTemplate(&mut self) {
        self.jobTemplate.clear();
    }

    pub fn has_jobTemplate(&self) -> bool {
        self.jobTemplate.is_some()
    }

    // Param is passed by value, moved
    pub fn set_jobTemplate(&mut self, v: JobTemplateSpec) {
        self.jobTemplate = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_jobTemplate(&mut self) -> &mut JobTemplateSpec {
        if self.jobTemplate.is_none() {
            self.jobTemplate.set_default();
        }
        self.jobTemplate.as_mut().unwrap()
    }

    // Take field
    pub fn take_jobTemplate(&mut self) -> JobTemplateSpec {
        self.jobTemplate.take().unwrap_or_else(|| JobTemplateSpec::new())
    }

    pub fn get_jobTemplate(&self) -> &JobTemplateSpec {
        self.jobTemplate.as_ref().unwrap_or_else(|| JobTemplateSpec::default_instance())
    }

    fn get_jobTemplate_for_reflect(&self) -> &::protobuf::SingularPtrField<JobTemplateSpec> {
        &self.jobTemplate
    }

    fn mut_jobTemplate_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<JobTemplateSpec> {
        &mut self.jobTemplate
    }

    // optional int32 successfulJobsHistoryLimit = 6;

    pub fn clear_successfulJobsHistoryLimit(&mut self) {
        self.successfulJobsHistoryLimit = ::std::option::Option::None;
    }

    pub fn has_successfulJobsHistoryLimit(&self) -> bool {
        self.successfulJobsHistoryLimit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_successfulJobsHistoryLimit(&mut self, v: i32) {
        self.successfulJobsHistoryLimit = ::std::option::Option::Some(v);
    }

    pub fn get_successfulJobsHistoryLimit(&self) -> i32 {
        self.successfulJobsHistoryLimit.unwrap_or(0)
    }

    fn get_successfulJobsHistoryLimit_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.successfulJobsHistoryLimit
    }

    fn mut_successfulJobsHistoryLimit_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.successfulJobsHistoryLimit
    }

    // optional int32 failedJobsHistoryLimit = 7;

    pub fn clear_failedJobsHistoryLimit(&mut self) {
        self.failedJobsHistoryLimit = ::std::option::Option::None;
    }

    pub fn has_failedJobsHistoryLimit(&self) -> bool {
        self.failedJobsHistoryLimit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_failedJobsHistoryLimit(&mut self, v: i32) {
        self.failedJobsHistoryLimit = ::std::option::Option::Some(v);
    }

    pub fn get_failedJobsHistoryLimit(&self) -> i32 {
        self.failedJobsHistoryLimit.unwrap_or(0)
    }

    fn get_failedJobsHistoryLimit_for_reflect(&self) -> &::std::option::Option<i32> {
        &self.failedJobsHistoryLimit
    }

    fn mut_failedJobsHistoryLimit_for_reflect(&mut self) -> &mut ::std::option::Option<i32> {
        &mut self.failedJobsHistoryLimit
    }
}

impl ::protobuf::Message for CronJobSpec {
    fn is_initialized(&self) -> bool {
        for v in &self.jobTemplate {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.schedule)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int64()?;
                    self.startingDeadlineSeconds = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.concurrencyPolicy)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.suspend = ::std::option::Option::Some(tmp);
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.jobTemplate)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.successfulJobsHistoryLimit = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.failedJobsHistoryLimit = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.schedule.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.startingDeadlineSeconds {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.concurrencyPolicy.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        if let Some(v) = self.suspend {
            my_size += 2;
        }
        if let Some(ref v) = self.jobTemplate.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(v) = self.successfulJobsHistoryLimit {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.failedJobsHistoryLimit {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.schedule.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.startingDeadlineSeconds {
            os.write_int64(2, v)?;
        }
        if let Some(ref v) = self.concurrencyPolicy.as_ref() {
            os.write_string(3, &v)?;
        }
        if let Some(v) = self.suspend {
            os.write_bool(4, v)?;
        }
        if let Some(ref v) = self.jobTemplate.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(v) = self.successfulJobsHistoryLimit {
            os.write_int32(6, v)?;
        }
        if let Some(v) = self.failedJobsHistoryLimit {
            os.write_int32(7, v)?;
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

impl ::protobuf::MessageStatic for CronJobSpec {
    fn new() -> CronJobSpec {
        CronJobSpec::new()
    }

    fn descriptor_static(_: ::std::option::Option<CronJobSpec>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "schedule",
                    CronJobSpec::get_schedule_for_reflect,
                    CronJobSpec::mut_schedule_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt64>(
                    "startingDeadlineSeconds",
                    CronJobSpec::get_startingDeadlineSeconds_for_reflect,
                    CronJobSpec::mut_startingDeadlineSeconds_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "concurrencyPolicy",
                    CronJobSpec::get_concurrencyPolicy_for_reflect,
                    CronJobSpec::mut_concurrencyPolicy_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "suspend",
                    CronJobSpec::get_suspend_for_reflect,
                    CronJobSpec::mut_suspend_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<JobTemplateSpec>>(
                    "jobTemplate",
                    CronJobSpec::get_jobTemplate_for_reflect,
                    CronJobSpec::mut_jobTemplate_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "successfulJobsHistoryLimit",
                    CronJobSpec::get_successfulJobsHistoryLimit_for_reflect,
                    CronJobSpec::mut_successfulJobsHistoryLimit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "failedJobsHistoryLimit",
                    CronJobSpec::get_failedJobsHistoryLimit_for_reflect,
                    CronJobSpec::mut_failedJobsHistoryLimit_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CronJobSpec>(
                    "CronJobSpec",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CronJobSpec {
    fn clear(&mut self) {
        self.clear_schedule();
        self.clear_startingDeadlineSeconds();
        self.clear_concurrencyPolicy();
        self.clear_suspend();
        self.clear_jobTemplate();
        self.clear_successfulJobsHistoryLimit();
        self.clear_failedJobsHistoryLimit();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CronJobSpec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CronJobSpec {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CronJobStatus {
    // message fields
    active: ::protobuf::RepeatedField<super::generated::ObjectReference>,
    lastScheduleTime: ::protobuf::SingularPtrField<super::generated::Time>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CronJobStatus {}

impl CronJobStatus {
    pub fn new() -> CronJobStatus {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CronJobStatus {
        static mut instance: ::protobuf::lazy::Lazy<CronJobStatus> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CronJobStatus,
        };
        unsafe {
            instance.get(CronJobStatus::new)
        }
    }

    // repeated .k8s.io.api.core.v1.ObjectReference active = 1;

    pub fn clear_active(&mut self) {
        self.active.clear();
    }

    // Param is passed by value, moved
    pub fn set_active(&mut self, v: ::protobuf::RepeatedField<super::generated::ObjectReference>) {
        self.active = v;
    }

    // Mutable pointer to the field.
    pub fn mut_active(&mut self) -> &mut ::protobuf::RepeatedField<super::generated::ObjectReference> {
        &mut self.active
    }

    // Take field
    pub fn take_active(&mut self) -> ::protobuf::RepeatedField<super::generated::ObjectReference> {
        ::std::mem::replace(&mut self.active, ::protobuf::RepeatedField::new())
    }

    pub fn get_active(&self) -> &[super::generated::ObjectReference] {
        &self.active
    }

    fn get_active_for_reflect(&self) -> &::protobuf::RepeatedField<super::generated::ObjectReference> {
        &self.active
    }

    fn mut_active_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::generated::ObjectReference> {
        &mut self.active
    }

    // optional .k8s.io.apimachinery.pkg.apis.meta.v1.Time lastScheduleTime = 4;

    pub fn clear_lastScheduleTime(&mut self) {
        self.lastScheduleTime.clear();
    }

    pub fn has_lastScheduleTime(&self) -> bool {
        self.lastScheduleTime.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lastScheduleTime(&mut self, v: super::generated::Time) {
        self.lastScheduleTime = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lastScheduleTime(&mut self) -> &mut super::generated::Time {
        if self.lastScheduleTime.is_none() {
            self.lastScheduleTime.set_default();
        }
        self.lastScheduleTime.as_mut().unwrap()
    }

    // Take field
    pub fn take_lastScheduleTime(&mut self) -> super::generated::Time {
        self.lastScheduleTime.take().unwrap_or_else(|| super::generated::Time::new())
    }

    pub fn get_lastScheduleTime(&self) -> &super::generated::Time {
        self.lastScheduleTime.as_ref().unwrap_or_else(|| super::generated::Time::default_instance())
    }

    fn get_lastScheduleTime_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::Time> {
        &self.lastScheduleTime
    }

    fn mut_lastScheduleTime_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::Time> {
        &mut self.lastScheduleTime
    }
}

impl ::protobuf::Message for CronJobStatus {
    fn is_initialized(&self) -> bool {
        for v in &self.active {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.lastScheduleTime {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.active)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.lastScheduleTime)?;
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
        for value in &self.active {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.lastScheduleTime.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.active {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.lastScheduleTime.as_ref() {
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

impl ::protobuf::MessageStatic for CronJobStatus {
    fn new() -> CronJobStatus {
        CronJobStatus::new()
    }

    fn descriptor_static(_: ::std::option::Option<CronJobStatus>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectReference>>(
                    "active",
                    CronJobStatus::get_active_for_reflect,
                    CronJobStatus::mut_active_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::Time>>(
                    "lastScheduleTime",
                    CronJobStatus::get_lastScheduleTime_for_reflect,
                    CronJobStatus::mut_lastScheduleTime_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CronJobStatus>(
                    "CronJobStatus",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CronJobStatus {
    fn clear(&mut self) {
        self.clear_active();
        self.clear_lastScheduleTime();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CronJobStatus {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CronJobStatus {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JobTemplate {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    template: ::protobuf::SingularPtrField<JobTemplateSpec>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JobTemplate {}

impl JobTemplate {
    pub fn new() -> JobTemplate {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JobTemplate {
        static mut instance: ::protobuf::lazy::Lazy<JobTemplate> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JobTemplate,
        };
        unsafe {
            instance.get(JobTemplate::new)
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

    // optional .k8s.io.api.batch.v2alpha1.JobTemplateSpec template = 2;

    pub fn clear_template(&mut self) {
        self.template.clear();
    }

    pub fn has_template(&self) -> bool {
        self.template.is_some()
    }

    // Param is passed by value, moved
    pub fn set_template(&mut self, v: JobTemplateSpec) {
        self.template = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_template(&mut self) -> &mut JobTemplateSpec {
        if self.template.is_none() {
            self.template.set_default();
        }
        self.template.as_mut().unwrap()
    }

    // Take field
    pub fn take_template(&mut self) -> JobTemplateSpec {
        self.template.take().unwrap_or_else(|| JobTemplateSpec::new())
    }

    pub fn get_template(&self) -> &JobTemplateSpec {
        self.template.as_ref().unwrap_or_else(|| JobTemplateSpec::default_instance())
    }

    fn get_template_for_reflect(&self) -> &::protobuf::SingularPtrField<JobTemplateSpec> {
        &self.template
    }

    fn mut_template_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<JobTemplateSpec> {
        &mut self.template
    }
}

impl ::protobuf::Message for JobTemplate {
    fn is_initialized(&self) -> bool {
        for v in &self.metadata {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.metadata)?;
                },
                2 => {
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
        if let Some(ref v) = self.metadata.as_ref() {
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
        if let Some(ref v) = self.metadata.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.template.as_ref() {
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

impl ::protobuf::MessageStatic for JobTemplate {
    fn new() -> JobTemplate {
        JobTemplate::new()
    }

    fn descriptor_static(_: ::std::option::Option<JobTemplate>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    JobTemplate::get_metadata_for_reflect,
                    JobTemplate::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<JobTemplateSpec>>(
                    "template",
                    JobTemplate::get_template_for_reflect,
                    JobTemplate::mut_template_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JobTemplate>(
                    "JobTemplate",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JobTemplate {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_template();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JobTemplate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JobTemplate {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct JobTemplateSpec {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    spec: ::protobuf::SingularPtrField<super::generated::JobSpec>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for JobTemplateSpec {}

impl JobTemplateSpec {
    pub fn new() -> JobTemplateSpec {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static JobTemplateSpec {
        static mut instance: ::protobuf::lazy::Lazy<JobTemplateSpec> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const JobTemplateSpec,
        };
        unsafe {
            instance.get(JobTemplateSpec::new)
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
    pub fn set_spec(&mut self, v: super::generated::JobSpec) {
        self.spec = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_spec(&mut self) -> &mut super::generated::JobSpec {
        if self.spec.is_none() {
            self.spec.set_default();
        }
        self.spec.as_mut().unwrap()
    }

    // Take field
    pub fn take_spec(&mut self) -> super::generated::JobSpec {
        self.spec.take().unwrap_or_else(|| super::generated::JobSpec::new())
    }

    pub fn get_spec(&self) -> &super::generated::JobSpec {
        self.spec.as_ref().unwrap_or_else(|| super::generated::JobSpec::default_instance())
    }

    fn get_spec_for_reflect(&self) -> &::protobuf::SingularPtrField<super::generated::JobSpec> {
        &self.spec
    }

    fn mut_spec_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::generated::JobSpec> {
        &mut self.spec
    }
}

impl ::protobuf::Message for JobTemplateSpec {
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

impl ::protobuf::MessageStatic for JobTemplateSpec {
    fn new() -> JobTemplateSpec {
        JobTemplateSpec::new()
    }

    fn descriptor_static(_: ::std::option::Option<JobTemplateSpec>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    JobTemplateSpec::get_metadata_for_reflect,
                    JobTemplateSpec::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::JobSpec>>(
                    "spec",
                    JobTemplateSpec::get_spec_for_reflect,
                    JobTemplateSpec::mut_spec_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<JobTemplateSpec>(
                    "JobTemplateSpec",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for JobTemplateSpec {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_spec();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for JobTemplateSpec {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for JobTemplateSpec {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n)k8s.io/api/batch/v2alpha1/generated.proto\x12\x19k8s.io.api.batch.v2a\
    lpha1\x1a#k8s.io/api/batch/v1/generated.proto\x1a\"k8s.io/api/core/v1/ge\
    nerated.proto\x1a4k8s.io/apimachinery/pkg/apis/meta/v1/generated.proto\
    \x1a/k8s.io/apimachinery/pkg/runtime/generated.proto\x1a6k8s.io/apimachi\
    nery/pkg/runtime/schema/generated.proto\x1a3k8s.io/apimachinery/pkg/util\
    /intstr/generated.proto\"\xd5\x01\n\x07CronJob\x12L\n\x08metadata\x18\
    \x01\x20\x01(\x0b20.k8s.io.apimachinery.pkg.apis.meta.v1.ObjectMetaR\x08\
    metadata\x12:\n\x04spec\x18\x02\x20\x01(\x0b2&.k8s.io.api.batch.v2alpha1\
    .CronJobSpecR\x04spec\x12@\n\x06status\x18\x03\x20\x01(\x0b2(.k8s.io.api\
    .batch.v2alpha1.CronJobStatusR\x06status\"\x93\x01\n\x0bCronJobList\x12J\
    \n\x08metadata\x18\x01\x20\x01(\x0b2..k8s.io.apimachinery.pkg.apis.meta.\
    v1.ListMetaR\x08metadata\x128\n\x05items\x18\x02\x20\x03(\x0b2\".k8s.io.\
    api.batch.v2alpha1.CronJobR\x05items\"\xf1\x02\n\x0bCronJobSpec\x12\x1a\
    \n\x08schedule\x18\x01\x20\x01(\tR\x08schedule\x128\n\x17startingDeadlin\
    eSeconds\x18\x02\x20\x01(\x03R\x17startingDeadlineSeconds\x12,\n\x11conc\
    urrencyPolicy\x18\x03\x20\x01(\tR\x11concurrencyPolicy\x12\x18\n\x07susp\
    end\x18\x04\x20\x01(\x08R\x07suspend\x12L\n\x0bjobTemplate\x18\x05\x20\
    \x01(\x0b2*.k8s.io.api.batch.v2alpha1.JobTemplateSpecR\x0bjobTemplate\
    \x12>\n\x1asuccessfulJobsHistoryLimit\x18\x06\x20\x01(\x05R\x1asuccessfu\
    lJobsHistoryLimit\x126\n\x16failedJobsHistoryLimit\x18\x07\x20\x01(\x05R\
    \x16failedJobsHistoryLimit\"\xa4\x01\n\rCronJobStatus\x12;\n\x06active\
    \x18\x01\x20\x03(\x0b2#.k8s.io.api.core.v1.ObjectReferenceR\x06active\
    \x12V\n\x10lastScheduleTime\x18\x04\x20\x01(\x0b2*.k8s.io.apimachinery.p\
    kg.apis.meta.v1.TimeR\x10lastScheduleTime\"\xa3\x01\n\x0bJobTemplate\x12\
    L\n\x08metadata\x18\x01\x20\x01(\x0b20.k8s.io.apimachinery.pkg.apis.meta\
    .v1.ObjectMetaR\x08metadata\x12F\n\x08template\x18\x02\x20\x01(\x0b2*.k8\
    s.io.api.batch.v2alpha1.JobTemplateSpecR\x08template\"\x91\x01\n\x0fJobT\
    emplateSpec\x12L\n\x08metadata\x18\x01\x20\x01(\x0b20.k8s.io.apimachiner\
    y.pkg.apis.meta.v1.ObjectMetaR\x08metadata\x120\n\x04spec\x18\x02\x20\
    \x01(\x0b2\x1c.k8s.io.api.batch.v1.JobSpecR\x04specB\nZ\x08v2alpha1\
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
