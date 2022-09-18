/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 * You can obtain source code at httsp://github.com/gmflex/gm_bson
 * 
 * (C) kithf, 2022
 */
pub mod from;
pub mod to;

use gmod::lua::State;
use mongodb::bson::Bson;
use crate::get_bson;

#[lua_function]
unsafe fn objectid_tostring(lua: State) -> i32 {
  let oid = get_bson!(lua);

  if let Bson::ObjectId(oid) = oid {
    lua.push_string(&oid.to_hex());
    1
  } else {
    0
  }
}

pub unsafe fn objectid_new(lua: State, oid: Option<String>) {
  let oid = match oid {
    Some(oid) => mongodb::bson::oid::ObjectId::parse_str(&oid).unwrap(),
    None => mongodb::bson::oid::ObjectId::new()
  };

  lua.new_metatable(lua_string!("objectid"));

  lua.push_function(objectid_tostring);
  lua.set_field(-2, lua_string!("__tostring"));

  lua.new_userdata(Bson::ObjectId(oid), Some(-1));
}

#[lua_function]
pub unsafe fn objectid(lua: State) -> i32 {
  let oid = match lua.lua_type(1) {
    gmod::lua::LUA_TSTRING => Some(lua.check_string(1).to_string()),
    _ => None
  };

  objectid_new(lua, oid);
  1
}

#[lua_function]
unsafe fn datetime_tostring(lua: State) -> i32 {
  let dt = get_bson!(lua);

  if let Bson::DateTime(dt) = dt {
    lua.push_string(&dt.try_to_rfc3339_string().unwrap_or_else(|_| "Invalid DateTime".to_string()));
    1
  } else {
    0
  }
}

pub unsafe fn datetime_new(lua: State, ms: Option<i64>) {
  let dt = match ms {
    Some(ms) => mongodb::bson::DateTime::from_millis(ms),
    None => mongodb::bson::DateTime::from_millis(chrono::Utc::now().timestamp_millis())
  };

  lua.new_metatable(lua_string!("datetime"));

  lua.push_function(datetime_tostring);
  lua.set_field(-2, lua_string!("__tostring"));

  lua.new_userdata(Bson::DateTime(dt), Some(-1));
}

#[lua_function]
pub unsafe fn datetime(lua: State) -> i32 {
  let ms: i64 = lua.check_integer(1).try_into().unwrap_or(0);

  datetime_new(lua, Some(ms));
  1
}

#[lua_function]
unsafe fn timestamp_tostring(lua: State) -> i32 {
  let ts = get_bson!(lua);

  if let Bson::Timestamp(ts) = ts {
    lua.push_string(&format!("{}", ts));
    1
  } else {
    0
  }
}

pub unsafe fn timestamp_new(lua: State, time: u32, increment: Option<u32>) {
  let ts = mongodb::bson::Timestamp {
    time,
    increment: increment.unwrap_or(0)
  };

  lua.new_metatable(lua_string!("timestamp"));

  lua.push_function(timestamp_tostring);
  lua.set_field(-2, lua_string!("__tostring"));

  lua.new_userdata(Bson::Timestamp(ts), Some(-1));
}

#[lua_function]
pub unsafe fn timestamp(lua: State) -> i32 {
  let time: u32 = lua.check_integer(1).try_into().unwrap_or(0);
  let increment: u32 = lua.check_integer(2).try_into().unwrap_or(0);

  timestamp_new(lua, time, Some(increment));
  1
}

#[lua_function]
unsafe fn binary_tostring(lua: State) -> i32 {
  let bin = get_bson!(lua);

  if let Bson::Binary(bin) = bin {
    lua.push_string(&format!("Binary({})", String::from_utf8(bin.bytes).unwrap()));
    1
  } else {
    0
  }
}

pub unsafe fn binary_new(lua: State, bin: Vec<u8>) {
  let bin = mongodb::bson::Binary {
    subtype: mongodb::bson::spec::BinarySubtype::Generic,
    bytes: bin
  };

  lua.new_metatable(lua_string!("binary"));

  lua.push_function(binary_tostring);
  lua.set_field(-2, lua_string!("__tostring"));

  lua.new_userdata(Bson::Binary(bin), Some(-1));
}

#[lua_function]
pub unsafe fn binary(lua: State) -> i32 {
  let str = lua.check_string(1).as_bytes().to_vec();

  binary_new(lua, str);
  1
}

#[lua_function]
unsafe fn regex_tostring(lua: State) -> i32 {
  let regex = get_bson!(lua);

  if let Bson::RegularExpression(regex) = regex {
    lua.push_string(&format!("Regex({}, {})", regex.pattern, regex.options));
    1
  } else {
    0
  }
}

pub unsafe fn regex_new(lua: State, pattern: String, options: String) {
  let regex = mongodb::bson::Regex {
    pattern,
    options
  };

  lua.new_metatable(lua_string!("regex"));

  lua.push_function(regex_tostring);
  lua.set_field(-2, lua_string!("__tostring"));

  lua.new_userdata(Bson::RegularExpression(regex), Some(-1));
}

#[lua_function]
pub unsafe fn regex(lua: State) -> i32 {
  let pattern = lua.check_string(1).to_string();
  let options = lua.get_string(2)
    .and_then(|s| Some(s.to_string()))
    .unwrap_or_else(|| "".to_string());

  regex_new(lua, pattern, options);
  1
}

#[lua_function]
unsafe fn code_tostring(lua: State) -> i32 {
  let code = get_bson!(lua);

  if let Bson::JavaScriptCode(code) = code {
    lua.push_string(&format!("Code({})", code));
    1
  } else {
    0
  }
}

pub unsafe fn code_new(lua: State, code: String) {
  lua.new_metatable(lua_string!("code"));

  lua.push_function(code_tostring);
  lua.set_field(-2, lua_string!("__tostring"));

  lua.new_userdata(Bson::JavaScriptCode(code), Some(-1));
}

#[lua_function]
pub unsafe fn code(lua: State) -> i32 {
  let str = lua.check_string(1).to_string();

  code_new(lua, str);
  1
}

#[lua_function]
unsafe fn minkey_tostring(lua: State) -> i32 {
  lua.push_string("MinKey");
  1
}

pub unsafe fn minkey_new(lua: State) {
  lua.new_metatable(lua_string!("minkey"));

  lua.push_function(minkey_tostring);
  lua.set_field(-2, lua_string!("__tostring"));

  lua.new_userdata(Bson::MinKey, Some(-1));
}

#[lua_function]
pub unsafe fn minkey(lua: State) -> i32 {
  minkey_new(lua);
  1
}

#[lua_function]
unsafe fn maxkey_tostring(lua: State) -> i32 {
  lua.push_string("MaxKey");
  1
}

pub unsafe fn maxkey_new(lua: State) {
  lua.new_metatable(lua_string!("maxkey"));

  lua.push_function(maxkey_tostring);
  lua.set_field(-2, lua_string!("__tostring"));

  lua.new_userdata(Bson::MaxKey, Some(-1));
}

#[lua_function]
pub unsafe fn maxkey(lua: State) -> i32 {
  maxkey_new(lua);
  1
}

#[lua_function]
unsafe fn decimal128_tostring(lua: State) -> i32 {
  let dec = get_bson!(lua);

  if let Bson::Decimal128(dec) = dec {
    lua.push_string(&format!("{}", dec));
    1
  } else {
    0
  }
}

pub unsafe fn decimal128_new(lua: State, bytes: [u8; 16]) {
  let dec = mongodb::bson::Decimal128::from_bytes(bytes);

  lua.new_metatable(lua_string!("decimal128"));

  lua.push_function(decimal128_tostring);
  lua.set_field(-2, lua_string!("__tostring"));

  lua.new_userdata(Bson::Decimal128(dec), Some(-1));
}

#[lua_function]
pub unsafe fn decimal128(lua: State) -> i32 {
  let str = lua.check_binary_string(1);

  let mut bytes = [0u8; 16];
  for (&x, p) in str.iter().zip(bytes.iter_mut()) {
    *p = x;
  }

  decimal128_new(lua, bytes);
  1
}


#[macro_export]
macro_rules! get_bson {
  ($lua:ident) => {
    match $lua.lua_type(1) {
      gmod::lua::LUA_TUSERDATA => match ($lua.to_userdata(1) as *const mongodb::bson::Bson).as_ref() {
        Some(bson) => bson.clone(),
        None => $lua.error("Invalid BSON userdata")
      },
      _ => $lua.error("Failed to convert to Bson")
    }
  }
}