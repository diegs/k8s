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
pub struct AdmissionHookClientConfig {
    // message fields
    service: ::protobuf::SingularPtrField<ServiceReference>,
    caBundle: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AdmissionHookClientConfig {}

impl AdmissionHookClientConfig {
    pub fn new() -> AdmissionHookClientConfig {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AdmissionHookClientConfig {
        static mut instance: ::protobuf::lazy::Lazy<AdmissionHookClientConfig> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AdmissionHookClientConfig,
        };
        unsafe {
            instance.get(AdmissionHookClientConfig::new)
        }
    }

    // optional .k8s.io.api.admissionregistration.v1alpha1.ServiceReference service = 1;

    pub fn clear_service(&mut self) {
        self.service.clear();
    }

    pub fn has_service(&self) -> bool {
        self.service.is_some()
    }

    // Param is passed by value, moved
    pub fn set_service(&mut self, v: ServiceReference) {
        self.service = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_service(&mut self) -> &mut ServiceReference {
        if self.service.is_none() {
            self.service.set_default();
        }
        self.service.as_mut().unwrap()
    }

    // Take field
    pub fn take_service(&mut self) -> ServiceReference {
        self.service.take().unwrap_or_else(|| ServiceReference::new())
    }

    pub fn get_service(&self) -> &ServiceReference {
        self.service.as_ref().unwrap_or_else(|| ServiceReference::default_instance())
    }

    fn get_service_for_reflect(&self) -> &::protobuf::SingularPtrField<ServiceReference> {
        &self.service
    }

    fn mut_service_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ServiceReference> {
        &mut self.service
    }

    // optional bytes caBundle = 2;

    pub fn clear_caBundle(&mut self) {
        self.caBundle.clear();
    }

    pub fn has_caBundle(&self) -> bool {
        self.caBundle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_caBundle(&mut self, v: ::std::vec::Vec<u8>) {
        self.caBundle = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_caBundle(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.caBundle.is_none() {
            self.caBundle.set_default();
        }
        self.caBundle.as_mut().unwrap()
    }

    // Take field
    pub fn take_caBundle(&mut self) -> ::std::vec::Vec<u8> {
        self.caBundle.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_caBundle(&self) -> &[u8] {
        match self.caBundle.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_caBundle_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.caBundle
    }

    fn mut_caBundle_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.caBundle
    }
}

impl ::protobuf::Message for AdmissionHookClientConfig {
    fn is_initialized(&self) -> bool {
        for v in &self.service {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.service)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.caBundle)?;
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
        if let Some(ref v) = self.service.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.caBundle.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.service.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.caBundle.as_ref() {
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

impl ::protobuf::MessageStatic for AdmissionHookClientConfig {
    fn new() -> AdmissionHookClientConfig {
        AdmissionHookClientConfig::new()
    }

    fn descriptor_static(_: ::std::option::Option<AdmissionHookClientConfig>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ServiceReference>>(
                    "service",
                    AdmissionHookClientConfig::get_service_for_reflect,
                    AdmissionHookClientConfig::mut_service_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "caBundle",
                    AdmissionHookClientConfig::get_caBundle_for_reflect,
                    AdmissionHookClientConfig::mut_caBundle_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AdmissionHookClientConfig>(
                    "AdmissionHookClientConfig",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AdmissionHookClientConfig {
    fn clear(&mut self) {
        self.clear_service();
        self.clear_caBundle();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AdmissionHookClientConfig {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AdmissionHookClientConfig {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ExternalAdmissionHook {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    clientConfig: ::protobuf::SingularPtrField<AdmissionHookClientConfig>,
    rules: ::protobuf::RepeatedField<RuleWithOperations>,
    failurePolicy: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ExternalAdmissionHook {}

impl ExternalAdmissionHook {
    pub fn new() -> ExternalAdmissionHook {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExternalAdmissionHook {
        static mut instance: ::protobuf::lazy::Lazy<ExternalAdmissionHook> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExternalAdmissionHook,
        };
        unsafe {
            instance.get(ExternalAdmissionHook::new)
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

    // optional .k8s.io.api.admissionregistration.v1alpha1.AdmissionHookClientConfig clientConfig = 2;

    pub fn clear_clientConfig(&mut self) {
        self.clientConfig.clear();
    }

    pub fn has_clientConfig(&self) -> bool {
        self.clientConfig.is_some()
    }

    // Param is passed by value, moved
    pub fn set_clientConfig(&mut self, v: AdmissionHookClientConfig) {
        self.clientConfig = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_clientConfig(&mut self) -> &mut AdmissionHookClientConfig {
        if self.clientConfig.is_none() {
            self.clientConfig.set_default();
        }
        self.clientConfig.as_mut().unwrap()
    }

    // Take field
    pub fn take_clientConfig(&mut self) -> AdmissionHookClientConfig {
        self.clientConfig.take().unwrap_or_else(|| AdmissionHookClientConfig::new())
    }

    pub fn get_clientConfig(&self) -> &AdmissionHookClientConfig {
        self.clientConfig.as_ref().unwrap_or_else(|| AdmissionHookClientConfig::default_instance())
    }

    fn get_clientConfig_for_reflect(&self) -> &::protobuf::SingularPtrField<AdmissionHookClientConfig> {
        &self.clientConfig
    }

    fn mut_clientConfig_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<AdmissionHookClientConfig> {
        &mut self.clientConfig
    }

    // repeated .k8s.io.api.admissionregistration.v1alpha1.RuleWithOperations rules = 3;

    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }

    // Param is passed by value, moved
    pub fn set_rules(&mut self, v: ::protobuf::RepeatedField<RuleWithOperations>) {
        self.rules = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rules(&mut self) -> &mut ::protobuf::RepeatedField<RuleWithOperations> {
        &mut self.rules
    }

    // Take field
    pub fn take_rules(&mut self) -> ::protobuf::RepeatedField<RuleWithOperations> {
        ::std::mem::replace(&mut self.rules, ::protobuf::RepeatedField::new())
    }

    pub fn get_rules(&self) -> &[RuleWithOperations] {
        &self.rules
    }

    fn get_rules_for_reflect(&self) -> &::protobuf::RepeatedField<RuleWithOperations> {
        &self.rules
    }

    fn mut_rules_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RuleWithOperations> {
        &mut self.rules
    }

    // optional string failurePolicy = 4;

    pub fn clear_failurePolicy(&mut self) {
        self.failurePolicy.clear();
    }

    pub fn has_failurePolicy(&self) -> bool {
        self.failurePolicy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_failurePolicy(&mut self, v: ::std::string::String) {
        self.failurePolicy = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_failurePolicy(&mut self) -> &mut ::std::string::String {
        if self.failurePolicy.is_none() {
            self.failurePolicy.set_default();
        }
        self.failurePolicy.as_mut().unwrap()
    }

    // Take field
    pub fn take_failurePolicy(&mut self) -> ::std::string::String {
        self.failurePolicy.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_failurePolicy(&self) -> &str {
        match self.failurePolicy.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_failurePolicy_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.failurePolicy
    }

    fn mut_failurePolicy_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.failurePolicy
    }
}

impl ::protobuf::Message for ExternalAdmissionHook {
    fn is_initialized(&self) -> bool {
        for v in &self.clientConfig {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.clientConfig)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rules)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.failurePolicy)?;
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
        if let Some(ref v) = self.clientConfig.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.rules {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.failurePolicy.as_ref() {
            my_size += ::protobuf::rt::string_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.clientConfig.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.rules {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.failurePolicy.as_ref() {
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

impl ::protobuf::MessageStatic for ExternalAdmissionHook {
    fn new() -> ExternalAdmissionHook {
        ExternalAdmissionHook::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExternalAdmissionHook>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ExternalAdmissionHook::get_name_for_reflect,
                    ExternalAdmissionHook::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AdmissionHookClientConfig>>(
                    "clientConfig",
                    ExternalAdmissionHook::get_clientConfig_for_reflect,
                    ExternalAdmissionHook::mut_clientConfig_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RuleWithOperations>>(
                    "rules",
                    ExternalAdmissionHook::get_rules_for_reflect,
                    ExternalAdmissionHook::mut_rules_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "failurePolicy",
                    ExternalAdmissionHook::get_failurePolicy_for_reflect,
                    ExternalAdmissionHook::mut_failurePolicy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExternalAdmissionHook>(
                    "ExternalAdmissionHook",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExternalAdmissionHook {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_clientConfig();
        self.clear_rules();
        self.clear_failurePolicy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ExternalAdmissionHook {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExternalAdmissionHook {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ExternalAdmissionHookConfiguration {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    externalAdmissionHooks: ::protobuf::RepeatedField<ExternalAdmissionHook>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ExternalAdmissionHookConfiguration {}

impl ExternalAdmissionHookConfiguration {
    pub fn new() -> ExternalAdmissionHookConfiguration {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExternalAdmissionHookConfiguration {
        static mut instance: ::protobuf::lazy::Lazy<ExternalAdmissionHookConfiguration> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExternalAdmissionHookConfiguration,
        };
        unsafe {
            instance.get(ExternalAdmissionHookConfiguration::new)
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

    // repeated .k8s.io.api.admissionregistration.v1alpha1.ExternalAdmissionHook externalAdmissionHooks = 2;

    pub fn clear_externalAdmissionHooks(&mut self) {
        self.externalAdmissionHooks.clear();
    }

    // Param is passed by value, moved
    pub fn set_externalAdmissionHooks(&mut self, v: ::protobuf::RepeatedField<ExternalAdmissionHook>) {
        self.externalAdmissionHooks = v;
    }

    // Mutable pointer to the field.
    pub fn mut_externalAdmissionHooks(&mut self) -> &mut ::protobuf::RepeatedField<ExternalAdmissionHook> {
        &mut self.externalAdmissionHooks
    }

    // Take field
    pub fn take_externalAdmissionHooks(&mut self) -> ::protobuf::RepeatedField<ExternalAdmissionHook> {
        ::std::mem::replace(&mut self.externalAdmissionHooks, ::protobuf::RepeatedField::new())
    }

    pub fn get_externalAdmissionHooks(&self) -> &[ExternalAdmissionHook] {
        &self.externalAdmissionHooks
    }

    fn get_externalAdmissionHooks_for_reflect(&self) -> &::protobuf::RepeatedField<ExternalAdmissionHook> {
        &self.externalAdmissionHooks
    }

    fn mut_externalAdmissionHooks_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ExternalAdmissionHook> {
        &mut self.externalAdmissionHooks
    }
}

impl ::protobuf::Message for ExternalAdmissionHookConfiguration {
    fn is_initialized(&self) -> bool {
        for v in &self.metadata {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.externalAdmissionHooks {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.externalAdmissionHooks)?;
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
        for value in &self.externalAdmissionHooks {
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
        for v in &self.externalAdmissionHooks {
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

impl ::protobuf::MessageStatic for ExternalAdmissionHookConfiguration {
    fn new() -> ExternalAdmissionHookConfiguration {
        ExternalAdmissionHookConfiguration::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExternalAdmissionHookConfiguration>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    ExternalAdmissionHookConfiguration::get_metadata_for_reflect,
                    ExternalAdmissionHookConfiguration::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ExternalAdmissionHook>>(
                    "externalAdmissionHooks",
                    ExternalAdmissionHookConfiguration::get_externalAdmissionHooks_for_reflect,
                    ExternalAdmissionHookConfiguration::mut_externalAdmissionHooks_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExternalAdmissionHookConfiguration>(
                    "ExternalAdmissionHookConfiguration",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExternalAdmissionHookConfiguration {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_externalAdmissionHooks();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ExternalAdmissionHookConfiguration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExternalAdmissionHookConfiguration {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ExternalAdmissionHookConfigurationList {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ListMeta>,
    items: ::protobuf::RepeatedField<ExternalAdmissionHookConfiguration>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ExternalAdmissionHookConfigurationList {}

impl ExternalAdmissionHookConfigurationList {
    pub fn new() -> ExternalAdmissionHookConfigurationList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ExternalAdmissionHookConfigurationList {
        static mut instance: ::protobuf::lazy::Lazy<ExternalAdmissionHookConfigurationList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ExternalAdmissionHookConfigurationList,
        };
        unsafe {
            instance.get(ExternalAdmissionHookConfigurationList::new)
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

    // repeated .k8s.io.api.admissionregistration.v1alpha1.ExternalAdmissionHookConfiguration items = 2;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<ExternalAdmissionHookConfiguration>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<ExternalAdmissionHookConfiguration> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<ExternalAdmissionHookConfiguration> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[ExternalAdmissionHookConfiguration] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<ExternalAdmissionHookConfiguration> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<ExternalAdmissionHookConfiguration> {
        &mut self.items
    }
}

impl ::protobuf::Message for ExternalAdmissionHookConfigurationList {
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

impl ::protobuf::MessageStatic for ExternalAdmissionHookConfigurationList {
    fn new() -> ExternalAdmissionHookConfigurationList {
        ExternalAdmissionHookConfigurationList::new()
    }

    fn descriptor_static(_: ::std::option::Option<ExternalAdmissionHookConfigurationList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ListMeta>>(
                    "metadata",
                    ExternalAdmissionHookConfigurationList::get_metadata_for_reflect,
                    ExternalAdmissionHookConfigurationList::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ExternalAdmissionHookConfiguration>>(
                    "items",
                    ExternalAdmissionHookConfigurationList::get_items_for_reflect,
                    ExternalAdmissionHookConfigurationList::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ExternalAdmissionHookConfigurationList>(
                    "ExternalAdmissionHookConfigurationList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ExternalAdmissionHookConfigurationList {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ExternalAdmissionHookConfigurationList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ExternalAdmissionHookConfigurationList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Initializer {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    rules: ::protobuf::RepeatedField<Rule>,
    failurePolicy: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Initializer {}

impl Initializer {
    pub fn new() -> Initializer {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Initializer {
        static mut instance: ::protobuf::lazy::Lazy<Initializer> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Initializer,
        };
        unsafe {
            instance.get(Initializer::new)
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

    // repeated .k8s.io.api.admissionregistration.v1alpha1.Rule rules = 2;

    pub fn clear_rules(&mut self) {
        self.rules.clear();
    }

    // Param is passed by value, moved
    pub fn set_rules(&mut self, v: ::protobuf::RepeatedField<Rule>) {
        self.rules = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rules(&mut self) -> &mut ::protobuf::RepeatedField<Rule> {
        &mut self.rules
    }

    // Take field
    pub fn take_rules(&mut self) -> ::protobuf::RepeatedField<Rule> {
        ::std::mem::replace(&mut self.rules, ::protobuf::RepeatedField::new())
    }

    pub fn get_rules(&self) -> &[Rule] {
        &self.rules
    }

    fn get_rules_for_reflect(&self) -> &::protobuf::RepeatedField<Rule> {
        &self.rules
    }

    fn mut_rules_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Rule> {
        &mut self.rules
    }

    // optional string failurePolicy = 3;

    pub fn clear_failurePolicy(&mut self) {
        self.failurePolicy.clear();
    }

    pub fn has_failurePolicy(&self) -> bool {
        self.failurePolicy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_failurePolicy(&mut self, v: ::std::string::String) {
        self.failurePolicy = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_failurePolicy(&mut self) -> &mut ::std::string::String {
        if self.failurePolicy.is_none() {
            self.failurePolicy.set_default();
        }
        self.failurePolicy.as_mut().unwrap()
    }

    // Take field
    pub fn take_failurePolicy(&mut self) -> ::std::string::String {
        self.failurePolicy.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_failurePolicy(&self) -> &str {
        match self.failurePolicy.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_failurePolicy_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.failurePolicy
    }

    fn mut_failurePolicy_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.failurePolicy
    }
}

impl ::protobuf::Message for Initializer {
    fn is_initialized(&self) -> bool {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rules)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.failurePolicy)?;
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
        for value in &self.rules {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.failurePolicy.as_ref() {
            my_size += ::protobuf::rt::string_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.name.as_ref() {
            os.write_string(1, &v)?;
        }
        for v in &self.rules {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.failurePolicy.as_ref() {
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

impl ::protobuf::MessageStatic for Initializer {
    fn new() -> Initializer {
        Initializer::new()
    }

    fn descriptor_static(_: ::std::option::Option<Initializer>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    Initializer::get_name_for_reflect,
                    Initializer::mut_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Rule>>(
                    "rules",
                    Initializer::get_rules_for_reflect,
                    Initializer::mut_rules_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "failurePolicy",
                    Initializer::get_failurePolicy_for_reflect,
                    Initializer::mut_failurePolicy_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Initializer>(
                    "Initializer",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Initializer {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_rules();
        self.clear_failurePolicy();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Initializer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Initializer {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct InitializerConfiguration {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ObjectMeta>,
    initializers: ::protobuf::RepeatedField<Initializer>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InitializerConfiguration {}

impl InitializerConfiguration {
    pub fn new() -> InitializerConfiguration {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InitializerConfiguration {
        static mut instance: ::protobuf::lazy::Lazy<InitializerConfiguration> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InitializerConfiguration,
        };
        unsafe {
            instance.get(InitializerConfiguration::new)
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

    // repeated .k8s.io.api.admissionregistration.v1alpha1.Initializer initializers = 2;

    pub fn clear_initializers(&mut self) {
        self.initializers.clear();
    }

    // Param is passed by value, moved
    pub fn set_initializers(&mut self, v: ::protobuf::RepeatedField<Initializer>) {
        self.initializers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_initializers(&mut self) -> &mut ::protobuf::RepeatedField<Initializer> {
        &mut self.initializers
    }

    // Take field
    pub fn take_initializers(&mut self) -> ::protobuf::RepeatedField<Initializer> {
        ::std::mem::replace(&mut self.initializers, ::protobuf::RepeatedField::new())
    }

    pub fn get_initializers(&self) -> &[Initializer] {
        &self.initializers
    }

    fn get_initializers_for_reflect(&self) -> &::protobuf::RepeatedField<Initializer> {
        &self.initializers
    }

    fn mut_initializers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Initializer> {
        &mut self.initializers
    }
}

impl ::protobuf::Message for InitializerConfiguration {
    fn is_initialized(&self) -> bool {
        for v in &self.metadata {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.initializers {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.initializers)?;
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
        for value in &self.initializers {
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
        for v in &self.initializers {
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

impl ::protobuf::MessageStatic for InitializerConfiguration {
    fn new() -> InitializerConfiguration {
        InitializerConfiguration::new()
    }

    fn descriptor_static(_: ::std::option::Option<InitializerConfiguration>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ObjectMeta>>(
                    "metadata",
                    InitializerConfiguration::get_metadata_for_reflect,
                    InitializerConfiguration::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Initializer>>(
                    "initializers",
                    InitializerConfiguration::get_initializers_for_reflect,
                    InitializerConfiguration::mut_initializers_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InitializerConfiguration>(
                    "InitializerConfiguration",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InitializerConfiguration {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_initializers();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for InitializerConfiguration {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InitializerConfiguration {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct InitializerConfigurationList {
    // message fields
    metadata: ::protobuf::SingularPtrField<super::generated::ListMeta>,
    items: ::protobuf::RepeatedField<InitializerConfiguration>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InitializerConfigurationList {}

impl InitializerConfigurationList {
    pub fn new() -> InitializerConfigurationList {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InitializerConfigurationList {
        static mut instance: ::protobuf::lazy::Lazy<InitializerConfigurationList> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InitializerConfigurationList,
        };
        unsafe {
            instance.get(InitializerConfigurationList::new)
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

    // repeated .k8s.io.api.admissionregistration.v1alpha1.InitializerConfiguration items = 2;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<InitializerConfiguration>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<InitializerConfiguration> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<InitializerConfiguration> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[InitializerConfiguration] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<InitializerConfiguration> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<InitializerConfiguration> {
        &mut self.items
    }
}

impl ::protobuf::Message for InitializerConfigurationList {
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

impl ::protobuf::MessageStatic for InitializerConfigurationList {
    fn new() -> InitializerConfigurationList {
        InitializerConfigurationList::new()
    }

    fn descriptor_static(_: ::std::option::Option<InitializerConfigurationList>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::generated::ListMeta>>(
                    "metadata",
                    InitializerConfigurationList::get_metadata_for_reflect,
                    InitializerConfigurationList::mut_metadata_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<InitializerConfiguration>>(
                    "items",
                    InitializerConfigurationList::get_items_for_reflect,
                    InitializerConfigurationList::mut_items_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InitializerConfigurationList>(
                    "InitializerConfigurationList",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InitializerConfigurationList {
    fn clear(&mut self) {
        self.clear_metadata();
        self.clear_items();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for InitializerConfigurationList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InitializerConfigurationList {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Rule {
    // message fields
    apiGroups: ::protobuf::RepeatedField<::std::string::String>,
    apiVersions: ::protobuf::RepeatedField<::std::string::String>,
    resources: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Rule {}

impl Rule {
    pub fn new() -> Rule {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Rule {
        static mut instance: ::protobuf::lazy::Lazy<Rule> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Rule,
        };
        unsafe {
            instance.get(Rule::new)
        }
    }

    // repeated string apiGroups = 1;

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

    // repeated string apiVersions = 2;

    pub fn clear_apiVersions(&mut self) {
        self.apiVersions.clear();
    }

    // Param is passed by value, moved
    pub fn set_apiVersions(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.apiVersions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_apiVersions(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.apiVersions
    }

    // Take field
    pub fn take_apiVersions(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.apiVersions, ::protobuf::RepeatedField::new())
    }

    pub fn get_apiVersions(&self) -> &[::std::string::String] {
        &self.apiVersions
    }

    fn get_apiVersions_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.apiVersions
    }

    fn mut_apiVersions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.apiVersions
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
}

impl ::protobuf::Message for Rule {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.apiGroups)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.apiVersions)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.resources)?;
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
        for value in &self.apiGroups {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.apiVersions {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.resources {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.apiGroups {
            os.write_string(1, &v)?;
        };
        for v in &self.apiVersions {
            os.write_string(2, &v)?;
        };
        for v in &self.resources {
            os.write_string(3, &v)?;
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

impl ::protobuf::MessageStatic for Rule {
    fn new() -> Rule {
        Rule::new()
    }

    fn descriptor_static(_: ::std::option::Option<Rule>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "apiGroups",
                    Rule::get_apiGroups_for_reflect,
                    Rule::mut_apiGroups_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "apiVersions",
                    Rule::get_apiVersions_for_reflect,
                    Rule::mut_apiVersions_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "resources",
                    Rule::get_resources_for_reflect,
                    Rule::mut_resources_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Rule>(
                    "Rule",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Rule {
    fn clear(&mut self) {
        self.clear_apiGroups();
        self.clear_apiVersions();
        self.clear_resources();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Rule {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Rule {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RuleWithOperations {
    // message fields
    operations: ::protobuf::RepeatedField<::std::string::String>,
    rule: ::protobuf::SingularPtrField<Rule>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RuleWithOperations {}

impl RuleWithOperations {
    pub fn new() -> RuleWithOperations {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RuleWithOperations {
        static mut instance: ::protobuf::lazy::Lazy<RuleWithOperations> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RuleWithOperations,
        };
        unsafe {
            instance.get(RuleWithOperations::new)
        }
    }

    // repeated string operations = 1;

    pub fn clear_operations(&mut self) {
        self.operations.clear();
    }

    // Param is passed by value, moved
    pub fn set_operations(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.operations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_operations(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.operations
    }

    // Take field
    pub fn take_operations(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.operations, ::protobuf::RepeatedField::new())
    }

    pub fn get_operations(&self) -> &[::std::string::String] {
        &self.operations
    }

    fn get_operations_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.operations
    }

    fn mut_operations_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.operations
    }

    // optional .k8s.io.api.admissionregistration.v1alpha1.Rule rule = 2;

    pub fn clear_rule(&mut self) {
        self.rule.clear();
    }

    pub fn has_rule(&self) -> bool {
        self.rule.is_some()
    }

    // Param is passed by value, moved
    pub fn set_rule(&mut self, v: Rule) {
        self.rule = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_rule(&mut self) -> &mut Rule {
        if self.rule.is_none() {
            self.rule.set_default();
        }
        self.rule.as_mut().unwrap()
    }

    // Take field
    pub fn take_rule(&mut self) -> Rule {
        self.rule.take().unwrap_or_else(|| Rule::new())
    }

    pub fn get_rule(&self) -> &Rule {
        self.rule.as_ref().unwrap_or_else(|| Rule::default_instance())
    }

    fn get_rule_for_reflect(&self) -> &::protobuf::SingularPtrField<Rule> {
        &self.rule
    }

    fn mut_rule_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Rule> {
        &mut self.rule
    }
}

impl ::protobuf::Message for RuleWithOperations {
    fn is_initialized(&self) -> bool {
        for v in &self.rule {
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
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.operations)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.rule)?;
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
        for value in &self.operations {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if let Some(ref v) = self.rule.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.operations {
            os.write_string(1, &v)?;
        };
        if let Some(ref v) = self.rule.as_ref() {
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

impl ::protobuf::MessageStatic for RuleWithOperations {
    fn new() -> RuleWithOperations {
        RuleWithOperations::new()
    }

    fn descriptor_static(_: ::std::option::Option<RuleWithOperations>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "operations",
                    RuleWithOperations::get_operations_for_reflect,
                    RuleWithOperations::mut_operations_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Rule>>(
                    "rule",
                    RuleWithOperations::get_rule_for_reflect,
                    RuleWithOperations::mut_rule_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RuleWithOperations>(
                    "RuleWithOperations",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RuleWithOperations {
    fn clear(&mut self) {
        self.clear_operations();
        self.clear_rule();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RuleWithOperations {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RuleWithOperations {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ServiceReference {
    // message fields
    namespace: ::protobuf::SingularField<::std::string::String>,
    name: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ServiceReference {}

impl ServiceReference {
    pub fn new() -> ServiceReference {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ServiceReference {
        static mut instance: ::protobuf::lazy::Lazy<ServiceReference> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ServiceReference,
        };
        unsafe {
            instance.get(ServiceReference::new)
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

    // optional string name = 2;

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

impl ::protobuf::Message for ServiceReference {
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
        if let Some(ref v) = self.name.as_ref() {
            my_size += ::protobuf::rt::string_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.namespace.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.name.as_ref() {
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

impl ::protobuf::MessageStatic for ServiceReference {
    fn new() -> ServiceReference {
        ServiceReference::new()
    }

    fn descriptor_static(_: ::std::option::Option<ServiceReference>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "namespace",
                    ServiceReference::get_namespace_for_reflect,
                    ServiceReference::mut_namespace_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "name",
                    ServiceReference::get_name_for_reflect,
                    ServiceReference::mut_name_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ServiceReference>(
                    "ServiceReference",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ServiceReference {
    fn clear(&mut self) {
        self.clear_namespace();
        self.clear_name();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ServiceReference {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ServiceReference {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n9k8s.io/api/admissionregistration/v1alpha1/generated.proto\x12)k8s.io.\
    api.admissionregistration.v1alpha1\x1a4k8s.io/apimachinery/pkg/apis/meta\
    /v1/generated.proto\x1a/k8s.io/apimachinery/pkg/runtime/generated.proto\
    \x1a6k8s.io/apimachinery/pkg/runtime/schema/generated.proto\x1a3k8s.io/a\
    pimachinery/pkg/util/intstr/generated.proto\"\x8e\x01\n\x19AdmissionHook\
    ClientConfig\x12U\n\x07service\x18\x01\x20\x01(\x0b2;.k8s.io.api.admissi\
    onregistration.v1alpha1.ServiceReferenceR\x07service\x12\x1a\n\x08caBund\
    le\x18\x02\x20\x01(\x0cR\x08caBundle\"\x90\x02\n\x15ExternalAdmissionHoo\
    k\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12h\n\x0cclientConfig\
    \x18\x02\x20\x01(\x0b2D.k8s.io.api.admissionregistration.v1alpha1.Admiss\
    ionHookClientConfigR\x0cclientConfig\x12S\n\x05rules\x18\x03\x20\x03(\
    \x0b2=.k8s.io.api.admissionregistration.v1alpha1.RuleWithOperationsR\x05\
    rules\x12$\n\rfailurePolicy\x18\x04\x20\x01(\tR\rfailurePolicy\"\xec\x01\
    \n\"ExternalAdmissionHookConfiguration\x12L\n\x08metadata\x18\x01\x20\
    \x01(\x0b20.k8s.io.apimachinery.pkg.apis.meta.v1.ObjectMetaR\x08metadata\
    \x12x\n\x16externalAdmissionHooks\x18\x02\x20\x03(\x0b2@.k8s.io.api.admi\
    ssionregistration.v1alpha1.ExternalAdmissionHookR\x16externalAdmissionHo\
    oks\"\xd9\x01\n&ExternalAdmissionHookConfigurationList\x12J\n\x08metadat\
    a\x18\x01\x20\x01(\x0b2..k8s.io.apimachinery.pkg.apis.meta.v1.ListMetaR\
    \x08metadata\x12c\n\x05items\x18\x02\x20\x03(\x0b2M.k8s.io.api.admission\
    registration.v1alpha1.ExternalAdmissionHookConfigurationR\x05items\"\x8e\
    \x01\n\x0bInitializer\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12E\
    \n\x05rules\x18\x02\x20\x03(\x0b2/.k8s.io.api.admissionregistration.v1al\
    pha1.RuleR\x05rules\x12$\n\rfailurePolicy\x18\x03\x20\x01(\tR\rfailurePo\
    licy\"\xc4\x01\n\x18InitializerConfiguration\x12L\n\x08metadata\x18\x01\
    \x20\x01(\x0b20.k8s.io.apimachinery.pkg.apis.meta.v1.ObjectMetaR\x08meta\
    data\x12Z\n\x0cinitializers\x18\x02\x20\x03(\x0b26.k8s.io.api.admissionr\
    egistration.v1alpha1.InitializerR\x0cinitializers\"\xc5\x01\n\x1cInitial\
    izerConfigurationList\x12J\n\x08metadata\x18\x01\x20\x01(\x0b2..k8s.io.a\
    pimachinery.pkg.apis.meta.v1.ListMetaR\x08metadata\x12Y\n\x05items\x18\
    \x02\x20\x03(\x0b2C.k8s.io.api.admissionregistration.v1alpha1.Initialize\
    rConfigurationR\x05items\"d\n\x04Rule\x12\x1c\n\tapiGroups\x18\x01\x20\
    \x03(\tR\tapiGroups\x12\x20\n\x0bapiVersions\x18\x02\x20\x03(\tR\x0bapiV\
    ersions\x12\x1c\n\tresources\x18\x03\x20\x03(\tR\tresources\"y\n\x12Rule\
    WithOperations\x12\x1e\n\noperations\x18\x01\x20\x03(\tR\noperations\x12\
    C\n\x04rule\x18\x02\x20\x01(\x0b2/.k8s.io.api.admissionregistration.v1al\
    pha1.RuleR\x04rule\"D\n\x10ServiceReference\x12\x1c\n\tnamespace\x18\x01\
    \x20\x01(\tR\tnamespace\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04nameB\n\
    Z\x08v1alpha1\
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
