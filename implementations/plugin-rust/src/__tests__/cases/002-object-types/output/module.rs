/// NOTE: This is an auto-generated file.
///       All modifications will be overwritten.

use std::sync::Arc;
use polywrap_core::invoke::Invoker;
use polywrap_plugin::{error::PluginError, module::PluginModule};
use polywrap_msgpack_serde::{
  to_vec,
  from_slice,
  BigIntWrapper,
  BigNumber,
  JSONString,
  bytes
};
use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};
use super::types::*;

pub trait Module: PluginModule {
}
