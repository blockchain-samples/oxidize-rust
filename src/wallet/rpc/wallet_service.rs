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
pub struct Account {
    // message fields
    address: ::protobuf::SingularField<::std::string::String>,
    spendable: ::std::option::Option<u64>,
    transactions: ::protobuf::RepeatedField<super::blockchain_entities::Transaction>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Account {}

impl Account {
    pub fn new() -> Account {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Account {
        static mut instance: ::protobuf::lazy::Lazy<Account> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Account,
        };
        unsafe {
            instance.get(Account::new)
        }
    }

    // required string address = 1;

    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    pub fn has_address(&self) -> bool {
        self.address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::string::String) {
        self.address = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut ::std::string::String {
        if self.address.is_none() {
            self.address.set_default();
        }
        self.address.as_mut().unwrap()
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::string::String {
        self.address.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_address(&self) -> &str {
        match self.address.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_address_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.address
    }

    fn mut_address_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.address
    }

    // required uint64 spendable = 3;

    pub fn clear_spendable(&mut self) {
        self.spendable = ::std::option::Option::None;
    }

    pub fn has_spendable(&self) -> bool {
        self.spendable.is_some()
    }

    // Param is passed by value, moved
    pub fn set_spendable(&mut self, v: u64) {
        self.spendable = ::std::option::Option::Some(v);
    }

    pub fn get_spendable(&self) -> u64 {
        self.spendable.unwrap_or(0)
    }

    fn get_spendable_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.spendable
    }

    fn mut_spendable_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.spendable
    }

    // repeated .wire.Transaction transactions = 5;

    pub fn clear_transactions(&mut self) {
        self.transactions.clear();
    }

    // Param is passed by value, moved
    pub fn set_transactions(&mut self, v: ::protobuf::RepeatedField<super::blockchain_entities::Transaction>) {
        self.transactions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_transactions(&mut self) -> &mut ::protobuf::RepeatedField<super::blockchain_entities::Transaction> {
        &mut self.transactions
    }

    // Take field
    pub fn take_transactions(&mut self) -> ::protobuf::RepeatedField<super::blockchain_entities::Transaction> {
        ::std::mem::replace(&mut self.transactions, ::protobuf::RepeatedField::new())
    }

    pub fn get_transactions(&self) -> &[super::blockchain_entities::Transaction] {
        &self.transactions
    }

    fn get_transactions_for_reflect(&self) -> &::protobuf::RepeatedField<super::blockchain_entities::Transaction> {
        &self.transactions
    }

    fn mut_transactions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<super::blockchain_entities::Transaction> {
        &mut self.transactions
    }
}

impl ::protobuf::Message for Account {
    fn is_initialized(&self) -> bool {
        if self.address.is_none() {
            return false;
        }
        if self.spendable.is_none() {
            return false;
        }
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.address)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.spendable = ::std::option::Option::Some(tmp);
                },
                5 => {
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
        if let Some(ref v) = self.address.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(v) = self.spendable {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
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
        if let Some(ref v) = self.address.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(v) = self.spendable {
            os.write_uint64(3, v)?;
        }
        for v in &self.transactions {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Account {
    fn new() -> Account {
        Account::new()
    }

    fn descriptor_static(_: ::std::option::Option<Account>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "address",
                    Account::get_address_for_reflect,
                    Account::mut_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "spendable",
                    Account::get_spendable_for_reflect,
                    Account::mut_spendable_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::blockchain_entities::Transaction>>(
                    "transactions",
                    Account::get_transactions_for_reflect,
                    Account::mut_transactions_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Account>(
                    "Account",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Account {
    fn clear(&mut self) {
        self.clear_address();
        self.clear_spendable();
        self.clear_transactions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Account {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Account {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WireUnspentOutput {
    // message fields
    address: ::protobuf::SingularField<::std::string::String>,
    txHash: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    output: ::protobuf::SingularPtrField<super::blockchain_entities::Output>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WireUnspentOutput {}

impl WireUnspentOutput {
    pub fn new() -> WireUnspentOutput {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WireUnspentOutput {
        static mut instance: ::protobuf::lazy::Lazy<WireUnspentOutput> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WireUnspentOutput,
        };
        unsafe {
            instance.get(WireUnspentOutput::new)
        }
    }

    // required string address = 1;

    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    pub fn has_address(&self) -> bool {
        self.address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::string::String) {
        self.address = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut ::std::string::String {
        if self.address.is_none() {
            self.address.set_default();
        }
        self.address.as_mut().unwrap()
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::string::String {
        self.address.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_address(&self) -> &str {
        match self.address.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_address_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.address
    }

    fn mut_address_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.address
    }

    // required bytes txHash = 2;

    pub fn clear_txHash(&mut self) {
        self.txHash.clear();
    }

    pub fn has_txHash(&self) -> bool {
        self.txHash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_txHash(&mut self, v: ::std::vec::Vec<u8>) {
        self.txHash = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_txHash(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.txHash.is_none() {
            self.txHash.set_default();
        }
        self.txHash.as_mut().unwrap()
    }

    // Take field
    pub fn take_txHash(&mut self) -> ::std::vec::Vec<u8> {
        self.txHash.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_txHash(&self) -> &[u8] {
        match self.txHash.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_txHash_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.txHash
    }

    fn mut_txHash_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.txHash
    }

    // required .wire.Output output = 3;

    pub fn clear_output(&mut self) {
        self.output.clear();
    }

    pub fn has_output(&self) -> bool {
        self.output.is_some()
    }

    // Param is passed by value, moved
    pub fn set_output(&mut self, v: super::blockchain_entities::Output) {
        self.output = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_output(&mut self) -> &mut super::blockchain_entities::Output {
        if self.output.is_none() {
            self.output.set_default();
        }
        self.output.as_mut().unwrap()
    }

    // Take field
    pub fn take_output(&mut self) -> super::blockchain_entities::Output {
        self.output.take().unwrap_or_else(|| super::blockchain_entities::Output::new())
    }

    pub fn get_output(&self) -> &super::blockchain_entities::Output {
        self.output.as_ref().unwrap_or_else(|| super::blockchain_entities::Output::default_instance())
    }

    fn get_output_for_reflect(&self) -> &::protobuf::SingularPtrField<super::blockchain_entities::Output> {
        &self.output
    }

    fn mut_output_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::blockchain_entities::Output> {
        &mut self.output
    }
}

impl ::protobuf::Message for WireUnspentOutput {
    fn is_initialized(&self) -> bool {
        if self.address.is_none() {
            return false;
        }
        if self.txHash.is_none() {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.address)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.txHash)?;
                },
                3 => {
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
        if let Some(ref v) = self.address.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        }
        if let Some(ref v) = self.txHash.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
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
        if let Some(ref v) = self.address.as_ref() {
            os.write_string(1, &v)?;
        }
        if let Some(ref v) = self.txHash.as_ref() {
            os.write_bytes(2, &v)?;
        }
        if let Some(ref v) = self.output.as_ref() {
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

impl ::protobuf::MessageStatic for WireUnspentOutput {
    fn new() -> WireUnspentOutput {
        WireUnspentOutput::new()
    }

    fn descriptor_static(_: ::std::option::Option<WireUnspentOutput>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "address",
                    WireUnspentOutput::get_address_for_reflect,
                    WireUnspentOutput::mut_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "txHash",
                    WireUnspentOutput::get_txHash_for_reflect,
                    WireUnspentOutput::mut_txHash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::blockchain_entities::Output>>(
                    "output",
                    WireUnspentOutput::get_output_for_reflect,
                    WireUnspentOutput::mut_output_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WireUnspentOutput>(
                    "WireUnspentOutput",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WireUnspentOutput {
    fn clear(&mut self) {
        self.clear_address();
        self.clear_txHash();
        self.clear_output();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WireUnspentOutput {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WireUnspentOutput {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountRequest {
    // message fields
    addresses: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountRequest {}

impl AccountRequest {
    pub fn new() -> AccountRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountRequest {
        static mut instance: ::protobuf::lazy::Lazy<AccountRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountRequest,
        };
        unsafe {
            instance.get(AccountRequest::new)
        }
    }

    // repeated string addresses = 1;

    pub fn clear_addresses(&mut self) {
        self.addresses.clear();
    }

    // Param is passed by value, moved
    pub fn set_addresses(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.addresses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_addresses(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.addresses
    }

    // Take field
    pub fn take_addresses(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.addresses, ::protobuf::RepeatedField::new())
    }

    pub fn get_addresses(&self) -> &[::std::string::String] {
        &self.addresses
    }

    fn get_addresses_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.addresses
    }

    fn mut_addresses_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.addresses
    }
}

impl ::protobuf::Message for AccountRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.addresses)?;
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
        for value in &self.addresses {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.addresses {
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

impl ::protobuf::MessageStatic for AccountRequest {
    fn new() -> AccountRequest {
        AccountRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "addresses",
                    AccountRequest::get_addresses_for_reflect,
                    AccountRequest::mut_addresses_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountRequest>(
                    "AccountRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountRequest {
    fn clear(&mut self) {
        self.clear_addresses();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AccountResponse {
    // message fields
    accounts: ::protobuf::RepeatedField<Account>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AccountResponse {}

impl AccountResponse {
    pub fn new() -> AccountResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AccountResponse {
        static mut instance: ::protobuf::lazy::Lazy<AccountResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AccountResponse,
        };
        unsafe {
            instance.get(AccountResponse::new)
        }
    }

    // repeated .rpc.Account accounts = 1;

    pub fn clear_accounts(&mut self) {
        self.accounts.clear();
    }

    // Param is passed by value, moved
    pub fn set_accounts(&mut self, v: ::protobuf::RepeatedField<Account>) {
        self.accounts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_accounts(&mut self) -> &mut ::protobuf::RepeatedField<Account> {
        &mut self.accounts
    }

    // Take field
    pub fn take_accounts(&mut self) -> ::protobuf::RepeatedField<Account> {
        ::std::mem::replace(&mut self.accounts, ::protobuf::RepeatedField::new())
    }

    pub fn get_accounts(&self) -> &[Account] {
        &self.accounts
    }

    fn get_accounts_for_reflect(&self) -> &::protobuf::RepeatedField<Account> {
        &self.accounts
    }

    fn mut_accounts_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Account> {
        &mut self.accounts
    }
}

impl ::protobuf::Message for AccountResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.accounts {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.accounts)?;
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
        for value in &self.accounts {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.accounts {
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

impl ::protobuf::MessageStatic for AccountResponse {
    fn new() -> AccountResponse {
        AccountResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AccountResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Account>>(
                    "accounts",
                    AccountResponse::get_accounts_for_reflect,
                    AccountResponse::mut_accounts_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AccountResponse>(
                    "AccountResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AccountResponse {
    fn clear(&mut self) {
        self.clear_accounts();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AccountResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AccountResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UnspentOutputsRequest {
    // message fields
    addresses: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UnspentOutputsRequest {}

impl UnspentOutputsRequest {
    pub fn new() -> UnspentOutputsRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UnspentOutputsRequest {
        static mut instance: ::protobuf::lazy::Lazy<UnspentOutputsRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnspentOutputsRequest,
        };
        unsafe {
            instance.get(UnspentOutputsRequest::new)
        }
    }

    // repeated string addresses = 1;

    pub fn clear_addresses(&mut self) {
        self.addresses.clear();
    }

    // Param is passed by value, moved
    pub fn set_addresses(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.addresses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_addresses(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.addresses
    }

    // Take field
    pub fn take_addresses(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.addresses, ::protobuf::RepeatedField::new())
    }

    pub fn get_addresses(&self) -> &[::std::string::String] {
        &self.addresses
    }

    fn get_addresses_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.addresses
    }

    fn mut_addresses_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.addresses
    }
}

impl ::protobuf::Message for UnspentOutputsRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.addresses)?;
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
        for value in &self.addresses {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.addresses {
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

impl ::protobuf::MessageStatic for UnspentOutputsRequest {
    fn new() -> UnspentOutputsRequest {
        UnspentOutputsRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<UnspentOutputsRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "addresses",
                    UnspentOutputsRequest::get_addresses_for_reflect,
                    UnspentOutputsRequest::mut_addresses_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UnspentOutputsRequest>(
                    "UnspentOutputsRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UnspentOutputsRequest {
    fn clear(&mut self) {
        self.clear_addresses();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnspentOutputsRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnspentOutputsRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UnspentOutputsResponse {
    // message fields
    outputs: ::protobuf::RepeatedField<WireUnspentOutput>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UnspentOutputsResponse {}

impl UnspentOutputsResponse {
    pub fn new() -> UnspentOutputsResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UnspentOutputsResponse {
        static mut instance: ::protobuf::lazy::Lazy<UnspentOutputsResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UnspentOutputsResponse,
        };
        unsafe {
            instance.get(UnspentOutputsResponse::new)
        }
    }

    // repeated .rpc.WireUnspentOutput outputs = 1;

    pub fn clear_outputs(&mut self) {
        self.outputs.clear();
    }

    // Param is passed by value, moved
    pub fn set_outputs(&mut self, v: ::protobuf::RepeatedField<WireUnspentOutput>) {
        self.outputs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_outputs(&mut self) -> &mut ::protobuf::RepeatedField<WireUnspentOutput> {
        &mut self.outputs
    }

    // Take field
    pub fn take_outputs(&mut self) -> ::protobuf::RepeatedField<WireUnspentOutput> {
        ::std::mem::replace(&mut self.outputs, ::protobuf::RepeatedField::new())
    }

    pub fn get_outputs(&self) -> &[WireUnspentOutput] {
        &self.outputs
    }

    fn get_outputs_for_reflect(&self) -> &::protobuf::RepeatedField<WireUnspentOutput> {
        &self.outputs
    }

    fn mut_outputs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<WireUnspentOutput> {
        &mut self.outputs
    }
}

impl ::protobuf::Message for UnspentOutputsResponse {
    fn is_initialized(&self) -> bool {
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
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.outputs)?;
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
        for value in &self.outputs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.outputs {
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

impl ::protobuf::MessageStatic for UnspentOutputsResponse {
    fn new() -> UnspentOutputsResponse {
        UnspentOutputsResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<UnspentOutputsResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<WireUnspentOutput>>(
                    "outputs",
                    UnspentOutputsResponse::get_outputs_for_reflect,
                    UnspentOutputsResponse::mut_outputs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UnspentOutputsResponse>(
                    "UnspentOutputsResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UnspentOutputsResponse {
    fn clear(&mut self) {
        self.clear_outputs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UnspentOutputsResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UnspentOutputsResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ProposeTransactionRequest {
    // message fields
    transaction: ::protobuf::SingularPtrField<super::blockchain_entities::Transaction>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ProposeTransactionRequest {}

impl ProposeTransactionRequest {
    pub fn new() -> ProposeTransactionRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ProposeTransactionRequest {
        static mut instance: ::protobuf::lazy::Lazy<ProposeTransactionRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ProposeTransactionRequest,
        };
        unsafe {
            instance.get(ProposeTransactionRequest::new)
        }
    }

    // required .wire.Transaction transaction = 1;

    pub fn clear_transaction(&mut self) {
        self.transaction.clear();
    }

    pub fn has_transaction(&self) -> bool {
        self.transaction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transaction(&mut self, v: super::blockchain_entities::Transaction) {
        self.transaction = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transaction(&mut self) -> &mut super::blockchain_entities::Transaction {
        if self.transaction.is_none() {
            self.transaction.set_default();
        }
        self.transaction.as_mut().unwrap()
    }

    // Take field
    pub fn take_transaction(&mut self) -> super::blockchain_entities::Transaction {
        self.transaction.take().unwrap_or_else(|| super::blockchain_entities::Transaction::new())
    }

    pub fn get_transaction(&self) -> &super::blockchain_entities::Transaction {
        self.transaction.as_ref().unwrap_or_else(|| super::blockchain_entities::Transaction::default_instance())
    }

    fn get_transaction_for_reflect(&self) -> &::protobuf::SingularPtrField<super::blockchain_entities::Transaction> {
        &self.transaction
    }

    fn mut_transaction_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::blockchain_entities::Transaction> {
        &mut self.transaction
    }
}

impl ::protobuf::Message for ProposeTransactionRequest {
    fn is_initialized(&self) -> bool {
        if self.transaction.is_none() {
            return false;
        }
        for v in &self.transaction {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.transaction)?;
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
        if let Some(ref v) = self.transaction.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.transaction.as_ref() {
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

impl ::protobuf::MessageStatic for ProposeTransactionRequest {
    fn new() -> ProposeTransactionRequest {
        ProposeTransactionRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ProposeTransactionRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::blockchain_entities::Transaction>>(
                    "transaction",
                    ProposeTransactionRequest::get_transaction_for_reflect,
                    ProposeTransactionRequest::mut_transaction_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ProposeTransactionRequest>(
                    "ProposeTransactionRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ProposeTransactionRequest {
    fn clear(&mut self) {
        self.clear_transaction();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ProposeTransactionRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProposeTransactionRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ProposeTransactionResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ProposeTransactionResponse {}

impl ProposeTransactionResponse {
    pub fn new() -> ProposeTransactionResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ProposeTransactionResponse {
        static mut instance: ::protobuf::lazy::Lazy<ProposeTransactionResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ProposeTransactionResponse,
        };
        unsafe {
            instance.get(ProposeTransactionResponse::new)
        }
    }
}

impl ::protobuf::Message for ProposeTransactionResponse {
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

impl ::protobuf::MessageStatic for ProposeTransactionResponse {
    fn new() -> ProposeTransactionResponse {
        ProposeTransactionResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ProposeTransactionResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ProposeTransactionResponse>(
                    "ProposeTransactionResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ProposeTransactionResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ProposeTransactionResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ProposeTransactionResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14wallet_service.proto\x12\x03rpc\x1a\x20_proto/blockchain_entities.\
    proto\"x\n\x07Account\x12\x18\n\x07address\x18\x01\x20\x02(\tR\x07addres\
    s\x12\x1c\n\tspendable\x18\x03\x20\x02(\x04R\tspendable\x125\n\x0ctransa\
    ctions\x18\x05\x20\x03(\x0b2\x11.wire.TransactionR\x0ctransactions\"k\n\
    \x11WireUnspentOutput\x12\x18\n\x07address\x18\x01\x20\x02(\tR\x07addres\
    s\x12\x16\n\x06txHash\x18\x02\x20\x02(\x0cR\x06txHash\x12$\n\x06output\
    \x18\x03\x20\x02(\x0b2\x0c.wire.OutputR\x06output\".\n\x0eAccountRequest\
    \x12\x1c\n\taddresses\x18\x01\x20\x03(\tR\taddresses\";\n\x0fAccountResp\
    onse\x12(\n\x08accounts\x18\x01\x20\x03(\x0b2\x0c.rpc.AccountR\x08accoun\
    ts\"5\n\x15UnspentOutputsRequest\x12\x1c\n\taddresses\x18\x01\x20\x03(\t\
    R\taddresses\"J\n\x16UnspentOutputsResponse\x120\n\x07outputs\x18\x01\
    \x20\x03(\x0b2\x16.rpc.WireUnspentOutputR\x07outputs\"P\n\x19ProposeTran\
    sactionRequest\x123\n\x0btransaction\x18\x01\x20\x02(\x0b2\x11.wire.Tran\
    sactionR\x0btransaction\"\x1c\n\x1aProposeTransactionResponse2\xe7\x01\n\
    \rWalletService\x124\n\x07Account\x12\x13.rpc.AccountRequest\x1a\x14.rpc\
    .AccountResponse\x12I\n\x0eUnspentOutputs\x12\x1a.rpc.UnspentOutputsRequ\
    est\x1a\x1b.rpc.UnspentOutputsResponse\x12U\n\x12ProposeTransaction\x12\
    \x1e.rpc.ProposeTransactionRequest\x1a\x1f.rpc.ProposeTransactionRespons\
    e\
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
