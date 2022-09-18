#![feature(c_unwind)]

#[macro_use] extern crate gmod;

mod bson;

#[gmod13_open]
unsafe fn gmod13_open(lua: gmod::lua::State) -> i32 {
  lua.new_table();

  lua.push_function(bson::objectid);
  lua.set_field(-2, lua_string!("objectid"));

  lua.push_function(bson::datetime);
  lua.set_field(-2, lua_string!("datetime"));

  lua.push_function(bson::timestamp);
  lua.set_field(-2, lua_string!("timestamp"));

  lua.push_function(bson::binary);
  lua.set_field(-2, lua_string!("binary"));
  
  lua.push_function(bson::regex);
  lua.set_field(-2, lua_string!("regex"));

  lua.push_function(bson::code);
  lua.set_field(-2, lua_string!("code"));

  lua.push_function(bson::minkey);
  lua.set_field(-2, lua_string!("minkey"));

  lua.push_function(bson::maxkey);
  lua.set_field(-2, lua_string!("maxkey"));

  lua.push_function(bson::decimal128);
  lua.set_field(-2, lua_string!("decimal128"));

  lua.push_function(bson::from::bson);
  lua.set_field(-2, lua_string!("to_lua"));

  lua.push_function(bson::to::bson);
  lua.set_field(-2, lua_string!("from_lua"));

  lua.set_global(lua_string!("bson"));
  0
}

#[gmod13_close]
unsafe fn gmod13_close(_lua: gmod::lua::State) -> i32 {
  0
}