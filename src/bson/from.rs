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

pub unsafe fn bson_to_lua(lua: State, value: Bson) -> i32 {
  match value {
    // lua types
    Bson::Null => lua.push_nil(),
    Bson::Boolean(bol) => lua.push_boolean(bol),
    Bson::Int32(int) => lua.push_integer(int as isize),
    Bson::Int64(int) => lua.push_integer(int as isize),
    Bson::Double(num) => lua.push_number(num),
    Bson::String(data) => lua.push_string(&data),
    Bson::Array(arr) => {
      lua.create_table(arr.len().try_into().unwrap(), 0);
      let mut j = 1;
      for (_i, v) in arr.iter().enumerate() {
        bson_to_lua(lua, v.clone());
        lua.raw_seti(-2, j);
        j += 1;
      }
    },
    Bson::Document(doc) => {
      lua.create_table(0, doc.len().try_into().unwrap());
      for (k, v) in doc.iter() {
        lua.push_string(k);
        bson_to_lua(lua, v.clone());
        lua.set_table(-3);
      }
    },
    // non-lua types(userdata)
    Bson::ObjectId(oid) => crate::bson::objectid_new(lua, Some(oid.to_hex())),
    Bson::DateTime(dtm) => crate::bson::datetime_new(lua, Some(dtm.timestamp_millis())),
    Bson::Binary(bin) => crate::bson::binary_new(lua, bin.bytes),
    Bson::RegularExpression(rgx) => crate::bson::regex_new(lua, rgx.pattern, rgx.options),
    Bson::JavaScriptCode(jsc) => crate::bson::code_new(lua, jsc),
    Bson::Timestamp(tsp) => crate::bson::timestamp_new(lua, tsp.time, Some(tsp.increment)),
    Bson::Decimal128(dcm) => crate::bson::decimal128_new(lua, dcm.bytes()),
    Bson::MaxKey => crate::bson::minkey_new(lua),
    Bson::MinKey => crate::bson::maxkey_new(lua),
    _ => return 0
  }
  1
}

#[lua_function]
pub unsafe fn bson(lua: State) -> i32 {
  let bson = get_bson!(lua);

  bson_to_lua(lua, bson);
  1
}
