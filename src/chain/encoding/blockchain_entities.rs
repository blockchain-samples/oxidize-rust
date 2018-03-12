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
pub struct Block {
    // message fields
    header: ::protobuf::SingularPtrField<BlockHeader>,
    transactions: ::protobuf::RepeatedField<Transaction>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Block {}

impl Block {
    pub fn new() -> Block {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Block {
        static mut instance: ::protobuf::lazy::Lazy<Block> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Block,
        };
        unsafe {
            instance.get(Block::new)
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
    pub fn set_header(&mut self, v: BlockHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut BlockHeader {
        if self.header.is_none() {
            self.header.set_default();
        }
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> BlockHeader {
        self.header.take().unwrap_or_else(|| BlockHeader::new())
    }

    pub fn get_header(&self) -> &BlockHeader {
        self.header.as_ref().unwrap_or_else(|| BlockHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<BlockHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<BlockHeader> {
        &mut self.header
    }

    // repeated .wire.Transaction transactions = 2;

    pub fn clear_transactions(&mut self) {
        self.transactions.clear();
    }

    // Param is passed by value, moved
    pub fn set_transactions(&mut self, v: ::protobuf::RepeatedField<Transaction>) {
        self.transactions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_transactions(&mut self) -> &mut ::protobuf::RepeatedField<Transaction> {
        &mut self.transactions
    }

    // Take field
    pub fn take_transactions(&mut self) -> ::protobuf::RepeatedField<Transaction> {
        ::std::mem::replace(&mut self.transactions, ::protobuf::RepeatedField::new())
    }

    pub fn get_transactions(&self) -> &[Transaction] {
        &self.transactions
    }

    fn get_transactions_for_reflect(&self) -> &::protobuf::RepeatedField<Transaction> {
        &self.transactions
    }

    fn mut_transactions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Transaction> {
        &mut self.transactions
    }
}

impl ::protobuf::Message for Block {
    fn is_initialized(&self) -> bool {
        if self.header.is_none() {
            return false;
        }
        for v in &self.header {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.transactions {
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
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.transactions)?;
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
        for value in &self.transactions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
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
        for v in &self.transactions {
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

impl ::protobuf::MessageStatic for Block {
    fn new() -> Block {
        Block::new()
    }

    fn descriptor_static(_: ::std::option::Option<Block>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<BlockHeader>>(
                    "header",
                    Block::get_header_for_reflect,
                    Block::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Transaction>>(
                    "transactions",
                    Block::get_transactions_for_reflect,
                    Block::mut_transactions_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Block>(
                    "Block",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Block {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_transactions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Block {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Block {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct BlockHeader {
    // message fields
    index: ::std::option::Option<u64>,
    previousHash: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    timestamp: ::std::option::Option<u64>,
    transactionsHash: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    nonce: ::std::option::Option<u64>,
    hash: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    difficulty: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BlockHeader {}

impl BlockHeader {
    pub fn new() -> BlockHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BlockHeader {
        static mut instance: ::protobuf::lazy::Lazy<BlockHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BlockHeader,
        };
        unsafe {
            instance.get(BlockHeader::new)
        }
    }

    // required uint64 index = 1;

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

    // required bytes previousHash = 2;

    pub fn clear_previousHash(&mut self) {
        self.previousHash.clear();
    }

    pub fn has_previousHash(&self) -> bool {
        self.previousHash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_previousHash(&mut self, v: ::std::vec::Vec<u8>) {
        self.previousHash = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_previousHash(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.previousHash.is_none() {
            self.previousHash.set_default();
        }
        self.previousHash.as_mut().unwrap()
    }

    // Take field
    pub fn take_previousHash(&mut self) -> ::std::vec::Vec<u8> {
        self.previousHash.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_previousHash(&self) -> &[u8] {
        match self.previousHash.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_previousHash_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.previousHash
    }

    fn mut_previousHash_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.previousHash
    }

    // required uint64 timestamp = 3;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None;
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: u64) {
        self.timestamp = ::std::option::Option::Some(v);
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp.unwrap_or(0)
    }

    fn get_timestamp_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.timestamp
    }

    // required bytes transactionsHash = 4;

    pub fn clear_transactionsHash(&mut self) {
        self.transactionsHash.clear();
    }

    pub fn has_transactionsHash(&self) -> bool {
        self.transactionsHash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transactionsHash(&mut self, v: ::std::vec::Vec<u8>) {
        self.transactionsHash = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transactionsHash(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.transactionsHash.is_none() {
            self.transactionsHash.set_default();
        }
        self.transactionsHash.as_mut().unwrap()
    }

    // Take field
    pub fn take_transactionsHash(&mut self) -> ::std::vec::Vec<u8> {
        self.transactionsHash.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_transactionsHash(&self) -> &[u8] {
        match self.transactionsHash.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_transactionsHash_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.transactionsHash
    }

    fn mut_transactionsHash_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.transactionsHash
    }

    // required uint64 nonce = 5;

    pub fn clear_nonce(&mut self) {
        self.nonce = ::std::option::Option::None;
    }

    pub fn has_nonce(&self) -> bool {
        self.nonce.is_some()
    }

    // Param is passed by value, moved
    pub fn set_nonce(&mut self, v: u64) {
        self.nonce = ::std::option::Option::Some(v);
    }

    pub fn get_nonce(&self) -> u64 {
        self.nonce.unwrap_or(0)
    }

    fn get_nonce_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.nonce
    }

    fn mut_nonce_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.nonce
    }

    // required bytes hash = 6;

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

    // required uint64 difficulty = 7;

    pub fn clear_difficulty(&mut self) {
        self.difficulty = ::std::option::Option::None;
    }

    pub fn has_difficulty(&self) -> bool {
        self.difficulty.is_some()
    }

    // Param is passed by value, moved
    pub fn set_difficulty(&mut self, v: u64) {
        self.difficulty = ::std::option::Option::Some(v);
    }

    pub fn get_difficulty(&self) -> u64 {
        self.difficulty.unwrap_or(0)
    }

    fn get_difficulty_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.difficulty
    }

    fn mut_difficulty_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.difficulty
    }
}

impl ::protobuf::Message for BlockHeader {
    fn is_initialized(&self) -> bool {
        if self.index.is_none() {
            return false;
        }
        if self.previousHash.is_none() {
            return false;
        }
        if self.timestamp.is_none() {
            return false;
        }
        if self.transactionsHash.is_none() {
            return false;
        }
        if self.nonce.is_none() {
            return false;
        }
        if self.hash.is_none() {
            return false;
        }
        if self.difficulty.is_none() {
            return false;
        }
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
                    let tmp = is.read_uint64()?;
                    self.index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.previousHash)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.timestamp = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.transactionsHash)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.nonce = ::std::option::Option::Some(tmp);
                },
                6 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.hash)?;
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.difficulty = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.previousHash.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(v) = self.timestamp {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.transactionsHash.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        if let Some(v) = self.nonce {
            my_size += ::protobuf::rt::value_size(5, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.hash.as_ref() {
            my_size += ::protobuf::rt::bytes_size(6, &v);
        }
        if let Some(v) = self.difficulty {
            my_size += ::protobuf::rt::value_size(7, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.index {
            os.write_uint64(1, v)?;
        }
        if let Some(ref v) = self.previousHash.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(v) = self.timestamp {
            os.write_uint64(3, v)?;
        }
        if let Some(ref v) = self.transactionsHash.as_ref() {
            os.write_bytes(4, &v)?;
        }
        if let Some(v) = self.nonce {
            os.write_uint64(5, v)?;
        }
        if let Some(ref v) = self.hash.as_ref() {
            os.write_bytes(6, &v)?;
        }
        if let Some(v) = self.difficulty {
            os.write_uint64(7, v)?;
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

impl ::protobuf::MessageStatic for BlockHeader {
    fn new() -> BlockHeader {
        BlockHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<BlockHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "index",
                    BlockHeader::get_index_for_reflect,
                    BlockHeader::mut_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "previousHash",
                    BlockHeader::get_previousHash_for_reflect,
                    BlockHeader::mut_previousHash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "timestamp",
                    BlockHeader::get_timestamp_for_reflect,
                    BlockHeader::mut_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "transactionsHash",
                    BlockHeader::get_transactionsHash_for_reflect,
                    BlockHeader::mut_transactionsHash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "nonce",
                    BlockHeader::get_nonce_for_reflect,
                    BlockHeader::mut_nonce_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hash",
                    BlockHeader::get_hash_for_reflect,
                    BlockHeader::mut_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "difficulty",
                    BlockHeader::get_difficulty_for_reflect,
                    BlockHeader::mut_difficulty_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BlockHeader>(
                    "BlockHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BlockHeader {
    fn clear(&mut self) {
        self.clear_index();
        self.clear_previousHash();
        self.clear_timestamp();
        self.clear_transactionsHash();
        self.clear_nonce();
        self.clear_hash();
        self.clear_difficulty();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for BlockHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BlockHeader {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Transaction {
    // message fields
    id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    inputs: ::protobuf::RepeatedField<SignedInput>,
    outputs: ::protobuf::RepeatedField<Output>,
    secret: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Transaction {}

impl Transaction {
    pub fn new() -> Transaction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Transaction {
        static mut instance: ::protobuf::lazy::Lazy<Transaction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Transaction,
        };
        unsafe {
            instance.get(Transaction::new)
        }
    }

    // required bytes id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.id.is_none() {
            self.id.set_default();
        }
        self.id.as_mut().unwrap()
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::vec::Vec<u8> {
        self.id.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_id(&self) -> &[u8] {
        match self.id.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_id_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.id
    }

    // repeated .wire.SignedInput inputs = 2;

    pub fn clear_inputs(&mut self) {
        self.inputs.clear();
    }

    // Param is passed by value, moved
    pub fn set_inputs(&mut self, v: ::protobuf::RepeatedField<SignedInput>) {
        self.inputs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_inputs(&mut self) -> &mut ::protobuf::RepeatedField<SignedInput> {
        &mut self.inputs
    }

    // Take field
    pub fn take_inputs(&mut self) -> ::protobuf::RepeatedField<SignedInput> {
        ::std::mem::replace(&mut self.inputs, ::protobuf::RepeatedField::new())
    }

    pub fn get_inputs(&self) -> &[SignedInput] {
        &self.inputs
    }

    fn get_inputs_for_reflect(&self) -> &::protobuf::RepeatedField<SignedInput> {
        &self.inputs
    }

    fn mut_inputs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<SignedInput> {
        &mut self.inputs
    }

    // repeated .wire.Output outputs = 3;

    pub fn clear_outputs(&mut self) {
        self.outputs.clear();
    }

    // Param is passed by value, moved
    pub fn set_outputs(&mut self, v: ::protobuf::RepeatedField<Output>) {
        self.outputs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_outputs(&mut self) -> &mut ::protobuf::RepeatedField<Output> {
        &mut self.outputs
    }

    // Take field
    pub fn take_outputs(&mut self) -> ::protobuf::RepeatedField<Output> {
        ::std::mem::replace(&mut self.outputs, ::protobuf::RepeatedField::new())
    }

    pub fn get_outputs(&self) -> &[Output] {
        &self.outputs
    }

    fn get_outputs_for_reflect(&self) -> &::protobuf::RepeatedField<Output> {
        &self.outputs
    }

    fn mut_outputs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Output> {
        &mut self.outputs
    }

    // required bytes secret = 4;

    pub fn clear_secret(&mut self) {
        self.secret.clear();
    }

    pub fn has_secret(&self) -> bool {
        self.secret.is_some()
    }

    // Param is passed by value, moved
    pub fn set_secret(&mut self, v: ::std::vec::Vec<u8>) {
        self.secret = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_secret(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.secret.is_none() {
            self.secret.set_default();
        }
        self.secret.as_mut().unwrap()
    }

    // Take field
    pub fn take_secret(&mut self) -> ::std::vec::Vec<u8> {
        self.secret.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_secret(&self) -> &[u8] {
        match self.secret.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_secret_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.secret
    }

    fn mut_secret_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.secret
    }
}

impl ::protobuf::Message for Transaction {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        }
        if self.secret.is_none() {
            return false;
        }
        for v in &self.inputs {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.outputs {
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
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.inputs)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.outputs)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.secret)?;
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
        if let Some(ref v) = self.id.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        for value in &self.inputs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.outputs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.secret.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.id.as_ref() {
            os.write_bytes(1, &v)?;
        }
        for v in &self.inputs {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.outputs {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.secret.as_ref() {
            os.write_bytes(4, &v)?;
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

impl ::protobuf::MessageStatic for Transaction {
    fn new() -> Transaction {
        Transaction::new()
    }

    fn descriptor_static(_: ::std::option::Option<Transaction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "id",
                    Transaction::get_id_for_reflect,
                    Transaction::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SignedInput>>(
                    "inputs",
                    Transaction::get_inputs_for_reflect,
                    Transaction::mut_inputs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Output>>(
                    "outputs",
                    Transaction::get_outputs_for_reflect,
                    Transaction::mut_outputs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "secret",
                    Transaction::get_secret_for_reflect,
                    Transaction::mut_secret_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Transaction>(
                    "Transaction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Transaction {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_inputs();
        self.clear_outputs();
        self.clear_secret();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Transaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Transaction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Output {
    // message fields
    index: ::std::option::Option<u32>,
    value: ::std::option::Option<u64>,
    publicKeyHash: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Output {}

impl Output {
    pub fn new() -> Output {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Output {
        static mut instance: ::protobuf::lazy::Lazy<Output> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Output,
        };
        unsafe {
            instance.get(Output::new)
        }
    }

    // required uint32 index = 1;

    pub fn clear_index(&mut self) {
        self.index = ::std::option::Option::None;
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: u32) {
        self.index = ::std::option::Option::Some(v);
    }

    pub fn get_index(&self) -> u32 {
        self.index.unwrap_or(0)
    }

    fn get_index_for_reflect(&self) -> &::std::option::Option<u32> {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut ::std::option::Option<u32> {
        &mut self.index
    }

    // required uint64 value = 2;

    pub fn clear_value(&mut self) {
        self.value = ::std::option::Option::None;
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: u64) {
        self.value = ::std::option::Option::Some(v);
    }

    pub fn get_value(&self) -> u64 {
        self.value.unwrap_or(0)
    }

    fn get_value_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.value
    }

    // required bytes publicKeyHash = 3;

    pub fn clear_publicKeyHash(&mut self) {
        self.publicKeyHash.clear();
    }

    pub fn has_publicKeyHash(&self) -> bool {
        self.publicKeyHash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_publicKeyHash(&mut self, v: ::std::vec::Vec<u8>) {
        self.publicKeyHash = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_publicKeyHash(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.publicKeyHash.is_none() {
            self.publicKeyHash.set_default();
        }
        self.publicKeyHash.as_mut().unwrap()
    }

    // Take field
    pub fn take_publicKeyHash(&mut self) -> ::std::vec::Vec<u8> {
        self.publicKeyHash.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_publicKeyHash(&self) -> &[u8] {
        match self.publicKeyHash.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_publicKeyHash_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.publicKeyHash
    }

    fn mut_publicKeyHash_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.publicKeyHash
    }
}

impl ::protobuf::Message for Output {
    fn is_initialized(&self) -> bool {
        if self.index.is_none() {
            return false;
        }
        if self.value.is_none() {
            return false;
        }
        if self.publicKeyHash.is_none() {
            return false;
        }
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
                    self.index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.value = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.publicKeyHash)?;
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
        if let Some(v) = self.index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(v) = self.value {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.publicKeyHash.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.index {
            os.write_uint32(1, v)?;
        }
        if let Some(v) = self.value {
            os.write_uint64(2, v)?;
        }
        if let Some(ref v) = self.publicKeyHash.as_ref() {
            os.write_bytes(3, &v)?;
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

impl ::protobuf::MessageStatic for Output {
    fn new() -> Output {
        Output::new()
    }

    fn descriptor_static(_: ::std::option::Option<Output>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "index",
                    Output::get_index_for_reflect,
                    Output::mut_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "value",
                    Output::get_value_for_reflect,
                    Output::mut_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "publicKeyHash",
                    Output::get_publicKeyHash_for_reflect,
                    Output::mut_publicKeyHash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Output>(
                    "Output",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Output {
    fn clear(&mut self) {
        self.clear_index();
        self.clear_value();
        self.clear_publicKeyHash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Output {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Output {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct OutputReference {
    // message fields
    id: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    output: ::protobuf::SingularPtrField<Output>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for OutputReference {}

impl OutputReference {
    pub fn new() -> OutputReference {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static OutputReference {
        static mut instance: ::protobuf::lazy::Lazy<OutputReference> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const OutputReference,
        };
        unsafe {
            instance.get(OutputReference::new)
        }
    }

    // required bytes id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.id = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.id.is_none() {
            self.id.set_default();
        }
        self.id.as_mut().unwrap()
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::vec::Vec<u8> {
        self.id.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_id(&self) -> &[u8] {
        match self.id.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_id_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.id
    }

    // required .wire.Output output = 2;

    pub fn clear_output(&mut self) {
        self.output.clear();
    }

    pub fn has_output(&self) -> bool {
        self.output.is_some()
    }

    // Param is passed by value, moved
    pub fn set_output(&mut self, v: Output) {
        self.output = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_output(&mut self) -> &mut Output {
        if self.output.is_none() {
            self.output.set_default();
        }
        self.output.as_mut().unwrap()
    }

    // Take field
    pub fn take_output(&mut self) -> Output {
        self.output.take().unwrap_or_else(|| Output::new())
    }

    pub fn get_output(&self) -> &Output {
        self.output.as_ref().unwrap_or_else(|| Output::default_instance())
    }

    fn get_output_for_reflect(&self) -> &::protobuf::SingularPtrField<Output> {
        &self.output
    }

    fn mut_output_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Output> {
        &mut self.output
    }
}

impl ::protobuf::Message for OutputReference {
    fn is_initialized(&self) -> bool {
        if self.id.is_none() {
            return false;
        }
        if self.output.is_none() {
            return false;
        }
        for v in &self.output {
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
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.output)?;
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
        if let Some(ref v) = self.id.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        }
        if let Some(ref v) = self.output.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.id.as_ref() {
            os.write_bytes(1, &v)?;
        }
        if let Some(ref v) = self.output.as_ref() {
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

impl ::protobuf::MessageStatic for OutputReference {
    fn new() -> OutputReference {
        OutputReference::new()
    }

    fn descriptor_static(_: ::std::option::Option<OutputReference>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "id",
                    OutputReference::get_id_for_reflect,
                    OutputReference::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Output>>(
                    "output",
                    OutputReference::get_output_for_reflect,
                    OutputReference::mut_output_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<OutputReference>(
                    "OutputReference",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for OutputReference {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_output();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for OutputReference {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for OutputReference {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SignedInput {
    // message fields
    reference: ::protobuf::SingularPtrField<OutputReference>,
    publicKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    signature: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SignedInput {}

impl SignedInput {
    pub fn new() -> SignedInput {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SignedInput {
        static mut instance: ::protobuf::lazy::Lazy<SignedInput> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SignedInput,
        };
        unsafe {
            instance.get(SignedInput::new)
        }
    }

    // required .wire.OutputReference reference = 1;

    pub fn clear_reference(&mut self) {
        self.reference.clear();
    }

    pub fn has_reference(&self) -> bool {
        self.reference.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reference(&mut self, v: OutputReference) {
        self.reference = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reference(&mut self) -> &mut OutputReference {
        if self.reference.is_none() {
            self.reference.set_default();
        }
        self.reference.as_mut().unwrap()
    }

    // Take field
    pub fn take_reference(&mut self) -> OutputReference {
        self.reference.take().unwrap_or_else(|| OutputReference::new())
    }

    pub fn get_reference(&self) -> &OutputReference {
        self.reference.as_ref().unwrap_or_else(|| OutputReference::default_instance())
    }

    fn get_reference_for_reflect(&self) -> &::protobuf::SingularPtrField<OutputReference> {
        &self.reference
    }

    fn mut_reference_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<OutputReference> {
        &mut self.reference
    }

    // required bytes publicKey = 2;

    pub fn clear_publicKey(&mut self) {
        self.publicKey.clear();
    }

    pub fn has_publicKey(&self) -> bool {
        self.publicKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_publicKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.publicKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_publicKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.publicKey.is_none() {
            self.publicKey.set_default();
        }
        self.publicKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_publicKey(&mut self) -> ::std::vec::Vec<u8> {
        self.publicKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_publicKey(&self) -> &[u8] {
        match self.publicKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_publicKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.publicKey
    }

    fn mut_publicKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.publicKey
    }

    // required bytes signature = 3;

    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    pub fn has_signature(&self) -> bool {
        self.signature.is_some()
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.signature = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.signature.is_none() {
            self.signature.set_default();
        }
        self.signature.as_mut().unwrap()
    }

    // Take field
    pub fn take_signature(&mut self) -> ::std::vec::Vec<u8> {
        self.signature.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_signature(&self) -> &[u8] {
        match self.signature.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_signature_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.signature
    }
}

impl ::protobuf::Message for SignedInput {
    fn is_initialized(&self) -> bool {
        if self.reference.is_none() {
            return false;
        }
        if self.publicKey.is_none() {
            return false;
        }
        if self.signature.is_none() {
            return false;
        }
        for v in &self.reference {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.reference)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.publicKey)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.signature)?;
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
        if let Some(ref v) = self.reference.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.publicKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        if let Some(ref v) = self.signature.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.reference.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.publicKey.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(ref v) = self.signature.as_ref() {
            os.write_bytes(3, &v)?;
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

impl ::protobuf::MessageStatic for SignedInput {
    fn new() -> SignedInput {
        SignedInput::new()
    }

    fn descriptor_static(_: ::std::option::Option<SignedInput>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OutputReference>>(
                    "reference",
                    SignedInput::get_reference_for_reflect,
                    SignedInput::mut_reference_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "publicKey",
                    SignedInput::get_publicKey_for_reflect,
                    SignedInput::mut_publicKey_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signature",
                    SignedInput::get_signature_for_reflect,
                    SignedInput::mut_signature_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SignedInput>(
                    "SignedInput",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SignedInput {
    fn clear(&mut self) {
        self.clear_reference();
        self.clear_publicKey();
        self.clear_signature();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SignedInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SignedInput {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UnsignedInput {
    // message fields
    reference: ::protobuf::SingularPtrField<OutputReference>,
    publicKey: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UnsignedInput {}

impl UnsignedInput {
    pub fn new() -> UnsignedInput {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UnsignedInput {
        static mut instance: ::protobuf::lazy::Lazy<UnsignedInput> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnsignedInput,
        };
        unsafe {
            instance.get(UnsignedInput::new)
        }
    }

    // required .wire.OutputReference reference = 1;

    pub fn clear_reference(&mut self) {
        self.reference.clear();
    }

    pub fn has_reference(&self) -> bool {
        self.reference.is_some()
    }

    // Param is passed by value, moved
    pub fn set_reference(&mut self, v: OutputReference) {
        self.reference = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_reference(&mut self) -> &mut OutputReference {
        if self.reference.is_none() {
            self.reference.set_default();
        }
        self.reference.as_mut().unwrap()
    }

    // Take field
    pub fn take_reference(&mut self) -> OutputReference {
        self.reference.take().unwrap_or_else(|| OutputReference::new())
    }

    pub fn get_reference(&self) -> &OutputReference {
        self.reference.as_ref().unwrap_or_else(|| OutputReference::default_instance())
    }

    fn get_reference_for_reflect(&self) -> &::protobuf::SingularPtrField<OutputReference> {
        &self.reference
    }

    fn mut_reference_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<OutputReference> {
        &mut self.reference
    }

    // required bytes publicKey = 2;

    pub fn clear_publicKey(&mut self) {
        self.publicKey.clear();
    }

    pub fn has_publicKey(&self) -> bool {
        self.publicKey.is_some()
    }

    // Param is passed by value, moved
    pub fn set_publicKey(&mut self, v: ::std::vec::Vec<u8>) {
        self.publicKey = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_publicKey(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.publicKey.is_none() {
            self.publicKey.set_default();
        }
        self.publicKey.as_mut().unwrap()
    }

    // Take field
    pub fn take_publicKey(&mut self) -> ::std::vec::Vec<u8> {
        self.publicKey.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_publicKey(&self) -> &[u8] {
        match self.publicKey.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_publicKey_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.publicKey
    }

    fn mut_publicKey_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.publicKey
    }
}

impl ::protobuf::Message for UnsignedInput {
    fn is_initialized(&self) -> bool {
        if self.reference.is_none() {
            return false;
        }
        if self.publicKey.is_none() {
            return false;
        }
        for v in &self.reference {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.reference)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.publicKey)?;
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
        if let Some(ref v) = self.reference.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.publicKey.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.reference.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.publicKey.as_ref() {
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

impl ::protobuf::MessageStatic for UnsignedInput {
    fn new() -> UnsignedInput {
        UnsignedInput::new()
    }

    fn descriptor_static(_: ::std::option::Option<UnsignedInput>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<OutputReference>>(
                    "reference",
                    UnsignedInput::get_reference_for_reflect,
                    UnsignedInput::mut_reference_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "publicKey",
                    UnsignedInput::get_publicKey_for_reflect,
                    UnsignedInput::mut_publicKey_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UnsignedInput>(
                    "UnsignedInput",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UnsignedInput {
    fn clear(&mut self) {
        self.clear_reference();
        self.clear_publicKey();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnsignedInput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnsignedInput {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x19blockchain_entities.proto\x12\x04wire\"i\n\x05Block\x12)\n\x06head\
    er\x18\x01\x20\x02(\x0b2\x11.wire.BlockHeaderR\x06header\x125\n\x0ctrans\
    actions\x18\x02\x20\x03(\x0b2\x11.wire.TransactionR\x0ctransactions\"\
    \xdb\x01\n\x0bBlockHeader\x12\x14\n\x05index\x18\x01\x20\x02(\x04R\x05in\
    dex\x12\"\n\x0cpreviousHash\x18\x02\x20\x02(\x0cR\x0cpreviousHash\x12\
    \x1c\n\ttimestamp\x18\x03\x20\x02(\x04R\ttimestamp\x12*\n\x10transaction\
    sHash\x18\x04\x20\x02(\x0cR\x10transactionsHash\x12\x14\n\x05nonce\x18\
    \x05\x20\x02(\x04R\x05nonce\x12\x12\n\x04hash\x18\x06\x20\x02(\x0cR\x04h\
    ash\x12\x1e\n\ndifficulty\x18\x07\x20\x02(\x04R\ndifficulty\"\x88\x01\n\
    \x0bTransaction\x12\x0e\n\x02id\x18\x01\x20\x02(\x0cR\x02id\x12)\n\x06in\
    puts\x18\x02\x20\x03(\x0b2\x11.wire.SignedInputR\x06inputs\x12&\n\x07out\
    puts\x18\x03\x20\x03(\x0b2\x0c.wire.OutputR\x07outputs\x12\x16\n\x06secr\
    et\x18\x04\x20\x02(\x0cR\x06secret\"Z\n\x06Output\x12\x14\n\x05index\x18\
    \x01\x20\x02(\rR\x05index\x12\x14\n\x05value\x18\x02\x20\x02(\x04R\x05va\
    lue\x12$\n\rpublicKeyHash\x18\x03\x20\x02(\x0cR\rpublicKeyHash\"G\n\x0fO\
    utputReference\x12\x0e\n\x02id\x18\x01\x20\x02(\x0cR\x02id\x12$\n\x06out\
    put\x18\x02\x20\x02(\x0b2\x0c.wire.OutputR\x06output\"~\n\x0bSignedInput\
    \x123\n\treference\x18\x01\x20\x02(\x0b2\x15.wire.OutputReferenceR\trefe\
    rence\x12\x1c\n\tpublicKey\x18\x02\x20\x02(\x0cR\tpublicKey\x12\x1c\n\ts\
    ignature\x18\x03\x20\x02(\x0cR\tsignature\"b\n\rUnsignedInput\x123\n\tre\
    ference\x18\x01\x20\x02(\x0b2\x15.wire.OutputReferenceR\treference\x12\
    \x1c\n\tpublicKey\x18\x02\x20\x02(\x0cR\tpublicKey\
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
