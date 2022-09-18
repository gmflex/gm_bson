/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 * You can obtain source code at httsp://github.com/gmflex/gm_bson
 * 
 * (C) kithf, 2022
 */
use gmod::lua::State;
use mongodb::bson::Bson;
use crate::get_bson;

unsafe fn null_to_lua(lua: State) -> i32 {
  lua.push_nil();
  1
}

unsafe fn boolean_to_lua(lua: State, value: bool) -> i32 {
  lua.push_boolean(value);
  1
}

unsafe fn int32_to_lua(lua: State, value: i32) -> i32 {
  lua.push_integer(value as isize);
  1
}

unsafe fn int64_to_lua(lua: State, value: i64) -> i32 {
  lua.push_integer(value as isize);
  1
}


unsafe fn double_to_lua(lua: State, value: f64) -> i32 {
  lua.push_number(value);
  1
}

unsafe fn string_to_lua(lua: State, value: String) -> i32 {
  lua.push_string(&value);
  1
}

unsafe fn array_to_lua(lua: State, value: Vec<Bson>) -> i32 {
  lua.create_table(value.len().try_into().unwrap(), 0);
  let mut j = 1;
  for (_i, v) in value.iter().enumerate() {
    bson_to_lua(lua, v.clone());
    lua.raw_seti(-2, j);
    j += 1;
  }
  1
}

unsafe fn document_to_lua(lua: State, value: mongodb::bson::Document) -> i32 {
  lua.create_table(0, value.len().try_into().unwrap());
  for (k, v) in value.iter() {
    lua.push_string(k);
    bson_to_lua(lua, v.clone());
    lua.set_table(-3);
  }
  1
}

unsafe fn objectid_to_lua(lua: State, value: mongodb::bson::oid::ObjectId) -> i32 {
  crate::bson::objectid_new(lua, Some(value.to_hex()));
  1
}

unsafe fn datetime_to_lua(lua: State, value: mongodb::bson::DateTime) -> i32 {
  crate::bson::datetime_new(lua, Some(value.timestamp_millis()));
  1
}

unsafe fn timestamp_to_lua(lua: State, value: mongodb::bson::Timestamp) -> i32 {
  crate::bson::timestamp_new(lua, value.time, Some(value.increment));
  1
}

unsafe fn binary_to_lua(lua: State, value: mongodb::bson::Binary) -> i32 {
  crate::bson::binary_new(lua, value.bytes);
  1
}

unsafe fn regex_to_lua(lua: State, value: mongodb::bson::Regex) -> i32 {
  crate::bson::regex_new(lua, value.pattern, value.options);
  1
}

unsafe fn code_to_lua(lua: State, value: String) -> i32 {
  crate::bson::code_new(lua, value);
  1
}

unsafe fn minkey_to_lua(lua: State) -> i32 {
  crate::bson::minkey_new(lua);
  1
}

unsafe fn maxkey_to_lua(lua: State) -> i32 {
  crate::bson::maxkey_new(lua);
  1
}

//unsafe fn null_to_lua(lua: State) -> i32 {
//  crate::bson::null(lua)
//}

unsafe fn decimal128_to_lua(lua: State, value: mongodb::bson::Decimal128) -> i32 {
  crate::bson::decimal128_new(lua, value.bytes());
  1
}


pub unsafe fn bson_to_lua(lua: State, value: Bson) -> i32 {
  match value {
    // lua types
    Bson::Null => null_to_lua(lua),
    Bson::Boolean(boolean) => boolean_to_lua(lua, boolean),
    Bson::Int32(int) => int32_to_lua(lua, int),
    Bson::Int64(int) => int64_to_lua(lua, int),
    Bson::Double(dbl) => double_to_lua(lua, dbl),
    Bson::String(str) => string_to_lua(lua, str),
    Bson::Array(arr) => array_to_lua(lua, arr),
    Bson::Document(doc) => document_to_lua(lua, doc),
    // non-lua types(userdata)
    Bson::ObjectId(oid) => objectid_to_lua(lua, oid),
    Bson::DateTime(dtm) => datetime_to_lua(lua, dtm),
    Bson::Binary(bin) => binary_to_lua(lua, bin),
    Bson::RegularExpression(rgx) => regex_to_lua(lua, rgx),
    Bson::JavaScriptCode(jsc) => code_to_lua(lua, jsc),
    Bson::Timestamp(tsp) => timestamp_to_lua(lua, tsp),
    Bson::Decimal128(dcm) => decimal128_to_lua(lua, dcm),
    Bson::MaxKey => maxkey_to_lua(lua),
    Bson::MinKey => minkey_to_lua(lua),
    _ => 0
  }
}

#[lua_function]
pub unsafe fn bson(lua: State) -> i32 {
  let bson = get_bson!(lua);

  bson_to_lua(lua, bson);
  1
}