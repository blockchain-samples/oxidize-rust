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
pub struct GetBestHeaderRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetBestHeaderRequest {}

impl GetBestHeaderRequest {
    pub fn new() -> GetBestHeaderRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetBestHeaderRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetBestHeaderRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetBestHeaderRequest,
        };
        unsafe {
            instance.get(GetBestHeaderRequest::new)
        }
    }
}

impl ::protobuf::Message for GetBestHeaderRequest {
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

impl ::protobuf::MessageStatic for GetBestHeaderRequest {
    fn new() -> GetBestHeaderRequest {
        GetBestHeaderRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetBestHeaderRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetBestHeaderRequest>(
                    "GetBestHeaderRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetBestHeaderRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetBestHeaderRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetBestHeaderRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetBestHeaderResponse {
    // message fields
    header: ::protobuf::SingularPtrField<super::blockchain_entities::BlockHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetBestHeaderResponse {}

impl GetBestHeaderResponse {
    pub fn new() -> GetBestHeaderResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetBestHeaderResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetBestHeaderResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetBestHeaderResponse,
        };
        unsafe {
            instance.get(GetBestHeaderResponse::new)
        }
    }

    // required .wire.BlockHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: super::blockchain_entities::BlockHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut super::blockchain_entities::BlockHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> super::blockchain_entities::BlockHeader {
        self.header.take().unwrap_or_else(|| super::blockchain_entities::BlockHeader::new())
    }

    pub fn get_header(&self) -> &super::blockchain_entities::BlockHeader {
        self.header.as_ref().unwrap_or_else(|| super::blockchain_entities::BlockHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<super::blockchain_entities::BlockHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::blockchain_entities::BlockHeader> {
        &mut self.header
    }
}

impl ::protobuf::Message for GetBestHeaderResponse {
    fn is_initialized(&self) -> bool {
        if self.header.is_none() {
            return false;
        }
        for v in &self.header {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
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
        if let Some(ref v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.header.as_ref() {
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

impl ::protobuf::MessageStatic for GetBestHeaderResponse {
    fn new() -> GetBestHeaderResponse {
        GetBestHeaderResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetBestHeaderResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::blockchain_entities::BlockHeader>>(
                    "header",
                    GetBestHeaderResponse::get_header_for_reflect,
                    GetBestHeaderResponse::mut_header_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetBestHeaderResponse>(
                    "GetBestHeaderResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetBestHeaderResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetBestHeaderResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetBestHeaderResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetHeadersRequest {
    // message fields
    latestHash: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    latestIndex: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetHeadersRequest {}

impl GetHeadersRequest {
    pub fn new() -> GetHeadersRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetHeadersRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetHeadersRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetHeadersRequest,
        };
        unsafe {
            instance.get(GetHeadersRequest::new)
        }
    }

    // required bytes latestHash = 1;

    pub fn clear_latestHash(&mut self) {
        self.latestHash.clear();
    }

    pub fn has_latestHash(&self) -> bool {
        self.latestHash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_latestHash(&mut self, v: ::std::vec::Vec<u8>) {
        self.latestHash = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_latestHash(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.latestHash.is_none() {
            self.latestHash.set_default();
        }
        self.latestHash.as_mut().unwrap()
    }

    // Take field
    pub fn take_latestHash(&mut self) -> ::std::vec::Vec<u8> {
        self.latestHash.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_latestHash(&self) -> &[u8] {
        match self.latestHash.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_latestHash_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.latestHash
    }

    fn mut_latestHash_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.latestHash
    }

    // required uint64 latestIndex = 2;

    pub fn clear_latestIndex(&mut self) {
        self.latestIndex = ::std::option::Option::None;
    }

    pub fn has_latestIndex(&self) -> bool {
        self.latestIndex.is_some()
    }

    // Param is passed by value, moved
    pub fn set_latestIndex(&mut self, v: u64) {
        self.latestIndex = ::std::option::Option::Some(v);
    }

    pub fn get_latestIndex(&self) -> u64 {
        self.latestIndex.unwrap_or(0)
    }

    fn get_latestIndex_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.latestIndex
    }

    fn mut_latestIndex_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.latestIndex
    }
}

impl ::protobuf::Message for GetHeadersRequest {
    fn is_initialized(&self) -> bool {
        if self.latestHash.is_none() {
            return false;
        }
        if self.latestIndex.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.latestHash)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.latestIndex = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.latestHash.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(v) = self.latestIndex {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.latestHash.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(v) = self.latestIndex {
            os.write_uint64(2, v)?;
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

impl ::protobuf::MessageStatic for GetHeadersRequest {
    fn new() -> GetHeadersRequest {
        GetHeadersRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetHeadersRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "latestHash",
                    GetHeadersRequest::get_latestHash_for_reflect,
                    GetHeadersRequest::mut_latestHash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "latestIndex",
                    GetHeadersRequest::get_latestIndex_for_reflect,
                    GetHeadersRequest::mut_latestIndex_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetHeadersRequest>(
                    "GetHeadersRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetHeadersRequest {
    fn clear(&mut self) {
        self.clear_latestHash();
        self.clear_latestIndex();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetHeadersRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetHeadersRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetHeadersResponse {
    // message fields
    headerCount: ::std::option::Option<u32>,
    headers: ::protobuf::RepeatedField<super::blockchain_entities::BlockHeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetHeadersResponse {}

impl GetHeadersResponse {
    pub fn new() -> GetHeadersResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetHeadersResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetHeadersResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetHeadersResponse,
        };
        unsafe {
            instance.get(GetHeadersResponse::new)
        }
    }

    // required uint32 headerCount = 1;

    pub fn clear_headerCount(&mut self) {
        self.headerCount = ::std::option::Option::None;
    }

    pub fn has_headerCount(&self) -> bool {
        self.headerCount.is_some()
    }

    // Param is passed by value, moved
    pub fn set_headerCount(&mut self, v: u32) {
        self.headerCount = ::std::option::Option::Some(v);
    }

    pub fn get_headerCount(&self) -> u32 {
        self.headerCount.unwrap_or(0)
    }

    fn get_headerCount_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.headerCount
    }

    fn mut_headerCount_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.headerCount
    }

    // repeated .wire.BlockHeader headers = 2;

    pub fn clear_headers(&mut self) {
        self.headers.clear();
    }

    // Param is passed by value, moved
    pub fn set_headers(&mut self, v: ::protobuf::RepeatedField<super::blockchain_entities::BlockHeader>) {
        self.headers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_headers(&mut self) -> &mut ::protobuf::RepeatedField<super::blockchain_entities::BlockHeader> {
        &mut self.headers
    }

    // Take field
    pub fn take_headers(&mut self) -> ::protobuf::RepeatedField<super::blockchain_entities::BlockHeader> {
        ::std::mem::replace(&mut self.headers, ::protobuf::RepeatedField::new())
    }

    pub fn get_headers(&self) -> &[super::blockchain_entities::BlockHeader] {
        &self.headers
    }

    fn get_headers_for_reflect(&self) -> &::protobuf::RepeatedField<super::blockchain_entities::BlockHeader> {
        &self.headers
    }

    fn mut_headers_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::blockchain_entities::BlockHeader> {
        &mut self.headers
    }
}

impl ::protobuf::Message for GetHeadersResponse {
    fn is_initialized(&self) -> bool {
        if self.headerCount.is_none() {
            return false;
        }
        for v in &self.headers {
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
                    let tmp = is.read_uint32()?;
                    self.headerCount = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.headers)?;
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
        if let Some(v) = self.headerCount {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.headers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.headerCount {
            os.write_uint32(1, v)?;
        }
        for v in &self.headers {
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

impl ::protobuf::MessageStatic for GetHeadersResponse {
    fn new() -> GetHeadersResponse {
        GetHeadersResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetHeadersResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "headerCount",
                    GetHeadersResponse::get_headerCount_for_reflect,
                    GetHeadersResponse::mut_headerCount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::blockchain_entities::BlockHeader>>(
                    "headers",
                    GetHeadersResponse::get_headers_for_reflect,
                    GetHeadersResponse::mut_headers_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetHeadersResponse>(
                    "GetHeadersResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetHeadersResponse {
    fn clear(&mut self) {
        self.clear_headerCount();
        self.clear_headers();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetHeadersResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetHeadersResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetBlockRequest {
    // message fields
    hash: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    index: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetBlockRequest {}

impl GetBlockRequest {
    pub fn new() -> GetBlockRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetBlockRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetBlockRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetBlockRequest,
        };
        unsafe {
            instance.get(GetBlockRequest::new)
        }
    }

    // required bytes hash = 1;

    pub fn clear_hash(&mut self) {
        self.hash.clear();
    }

    pub fn has_hash(&self) -> bool {
        self.hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.hash = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.hash.is_none() {
            self.hash.set_default();
        }
        self.hash.as_mut().unwrap()
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::vec::Vec<u8> {
        self.hash.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_hash(&self) -> &[u8] {
        match self.hash.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_hash_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.hash
    }

    fn mut_hash_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.hash
    }

    // required uint64 index = 2;

    pub fn clear_index(&mut self) {
        self.index = ::std::option::Option::None;
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: u64) {
        self.index = ::std::option::Option::Some(v);
    }

    pub fn get_index(&self) -> u64 {
        self.index.unwrap_or(0)
    }

    fn get_index_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.index
    }
}

impl ::protobuf::Message for GetBlockRequest {
    fn is_initialized(&self) -> bool {
        if self.hash.is_none() {
            return false;
        }
        if self.index.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.hash)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.index = ::std::option::Option::Some(tmp);
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
        if let Some(ref v) = self.hash.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(v) = self.index {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.hash.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(v) = self.index {
            os.write_uint64(2, v)?;
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

impl ::protobuf::MessageStatic for GetBlockRequest {
    fn new() -> GetBlockRequest {
        GetBlockRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetBlockRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hash",
                    GetBlockRequest::get_hash_for_reflect,
                    GetBlockRequest::mut_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "index",
                    GetBlockRequest::get_index_for_reflect,
                    GetBlockRequest::mut_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetBlockRequest>(
                    "GetBlockRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetBlockRequest {
    fn clear(&mut self) {
        self.clear_hash();
        self.clear_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetBlockRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetBlockRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetBlockResponse {
    // message fields
    block: ::protobuf::SingularPtrField<super::blockchain_entities::Block>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetBlockResponse {}

impl GetBlockResponse {
    pub fn new() -> GetBlockResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetBlockResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetBlockResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetBlockResponse,
        };
        unsafe {
            instance.get(GetBlockResponse::new)
        }
    }

    // required .wire.Block block = 1;

    pub fn clear_block(&mut self) {
        self.block.clear();
    }

    pub fn has_block(&self) -> bool {
        self.block.is_some()
    }

    // Param is passed by value, moved
    pub fn set_block(&mut self, v: super::blockchain_entities::Block) {
        self.block = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_block(&mut self) -> &mut super::blockchain_entities::Block {
        if self.block.is_none() {
            self.block.set_default();
        }
        self.block.as_mut().unwrap()
    }

    // Take field
    pub fn take_block(&mut self) -> super::blockchain_entities::Block {
        self.block.take().unwrap_or_else(|| super::blockchain_entities::Block::new())
    }

    pub fn get_block(&self) -> &super::blockchain_entities::Block {
        self.block.as_ref().unwrap_or_else(|| super::blockchain_entities::Block::default_instance())
    }

    fn get_block_for_reflect(&self) -> &::protobuf::SingularPtrField<super::blockchain_entities::Block> {
        &self.block
    }

    fn mut_block_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::blockchain_entities::Block> {
        &mut self.block
    }
}

impl ::protobuf::Message for GetBlockResponse {
    fn is_initialized(&self) -> bool {
        if self.block.is_none() {
            return false;
        }
        for v in &self.block {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.block)?;
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
        if let Some(ref v) = self.block.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.block.as_ref() {
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

impl ::protobuf::MessageStatic for GetBlockResponse {
    fn new() -> GetBlockResponse {
        GetBlockResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetBlockResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::blockchain_entities::Block>>(
                    "block",
                    GetBlockResponse::get_block_for_reflect,
                    GetBlockResponse::mut_block_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetBlockResponse>(
                    "GetBlockResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetBlockResponse {
    fn clear(&mut self) {
        self.clear_block();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetBlockResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetBlockResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x18blockchain_service.proto\x12\x08blockrpc\x1a\x20_proto/blockchain_\
    entities.proto\"\x16\n\x14GetBestHeaderRequest\"B\n\x15GetBestHeaderResp\
    onse\x12)\n\x06header\x18\x01\x20\x02(\x0b2\x11.wire.BlockHeaderR\x06hea\
    der\"U\n\x11GetHeadersRequest\x12\x1e\n\nlatestHash\x18\x01\x20\x02(\x0c\
    R\nlatestHash\x12\x20\n\x0blatestIndex\x18\x02\x20\x02(\x04R\x0blatestIn\
    dex\"c\n\x12GetHeadersResponse\x12\x20\n\x0bheaderCount\x18\x01\x20\x02(\
    \rR\x0bheaderCount\x12+\n\x07headers\x18\x02\x20\x03(\x0b2\x11.wire.Bloc\
    kHeaderR\x07headers\";\n\x0fGetBlockRequest\x12\x12\n\x04hash\x18\x01\
    \x20\x02(\x0cR\x04hash\x12\x14\n\x05index\x18\x02\x20\x02(\x04R\x05index\
    \"5\n\x10GetBlockResponse\x12!\n\x05block\x18\x01\x20\x02(\x0b2\x0b.wire\
    .BlockR\x05block2\xeb\x01\n\x0bSyncService\x12P\n\rGetBestHeader\x12\x1e\
    .blockrpc.GetBestHeaderRequest\x1a\x1f.blockrpc.GetBestHeaderResponse\
    \x12G\n\nGetHeaders\x12\x1b.blockrpc.GetHeadersRequest\x1a\x1c.blockrpc.\
    GetHeadersResponse\x12A\n\x08GetBlock\x12\x19.blockrpc.GetBlockRequest\
    \x1a\x1a.blockrpc.GetBlockResponse\
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
