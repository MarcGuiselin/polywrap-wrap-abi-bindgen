use serde::{Deserialize, Serialize};
use polywrap_msgpack_serde::{
    from_slice,
    to_vec,
    wrappers::polywrap_json::JSONString,
    wrappers::polywrap_bigint::BigIntWrapper
};
use polywrap_wasm_rs::{
    BigInt,
    BigNumber,
    Map,
    JSON,
    wrap_load_env
};
use crate::{ModuleTrait, Module};
use crate::Arg;
use crate::Output;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ArgsMethod {
    pub arg: Arg,
}

pub fn method_wrapped(args: &[u8], env_size: u32) -> Vec<u8> {
    match from_slice::<ArgsMethod>(args) {
        Ok(args) => {
            let result = Module::method(ArgsMethod {
                arg: args.arg,
            });
            match result {
                Ok(res) => {
                    to_vec(&res).unwrap()
                }
                Err(e) => {
                    panic!("{}", e.to_string())
                }
            }
        }
        Err(e) => {
            panic!("{}", e.to_string())
        }
    }
}
