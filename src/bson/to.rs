use gmod::lua::{
  State, 
  LUA_TBOOLEAN,
  LUA_TNIL,
  LUA_TNONE,
  LUA_TNUMBER,
  LUA_TSTRING,
  LUA_TTABLE,
  LUA_TUSERDATA
};
use mongodb::bson::{Bson, Document};
use crate::get_bson;

unsafe fn table_len(lua: State) -> i32 {
  let len = lua.len(-1);
  if len > 0 {
    lua.push_integer(len as isize);
    if lua.next(-2) == 0 {
      return len
    } else {
      lua.pop_n(2);
    }
  }
  
  -1
}

unsafe fn coerce(lua: State) -> Bson {
  match lua.lua_type(-1) {
    LUA_TNIL | LUA_TNONE => Bson::Null,
    LUA_TBOOLEAN => Bson::Boolean(lua.get_boolean(-1)),
    LUA_TNUMBER => {
      let num = lua.to_number(-1);
      if num % 1.0 == 0.0 {
        let int = lua.to_integer(-1);
        i32::try_from(int).map(Bson::Int32).unwrap_or_else(|_| Bson::Int64(int as i64))
      } else {
        Bson::Double(num)
      }
    },
    LUA_TSTRING => Bson::String(lua.check_string(-1).to_string()),
    LUA_TTABLE => {
      let len = table_len(lua);
      if len > 0 {
        let mut vec = Vec::new();
        for i in 1..(len+1) {
          lua.raw_geti(-1, i);
          vec.push(coerce(lua));
          lua.pop();
        }

        Bson::Array(vec)
      } else {
        let mut doc = Document::new();
        lua.push_nil();
        while lua.next(-2) != 0 {
          let key = match lua.lua_type(-2) {
            LUA_TNUMBER | LUA_TSTRING => lua.check_string(-2),
            _ => lua.error("Invalid key type")
          };

          doc.insert(key, coerce(lua));
          lua.pop();
        }

        Bson::Document(doc)
      }
    },
    LUA_TUSERDATA => {
      let ud = match (lua.to_userdata(-1) as *const Bson).as_ref() {
        Some(ud) => ud.clone(),
        None => Bson::Null
      };

      ud
    }
    _ => Bson::Null
  }
}

#[lua_function]
unsafe fn bson_index(lua: State) -> i32 {
  let bson = get_bson!(lua);
  let key = lua.check_string(2);

  match key.as_ref() {
    "as_json" => {
      match std::panic::catch_unwind(|| {
        lua.push_string(&bson.into_relaxed_extjson().to_string());
      }) {
        Ok(_) => 1,
        Err(_) => lua.error("Failed to convert to JSON")
      }
    },
    "as_relaxed_json" => {
      match std::panic::catch_unwind(|| {
        lua.push_string(&bson.into_relaxed_extjson().to_string());
      }) {
        Ok(_) => 1,
        Err(_) => lua.error("Failed to convert to JSON")
      }
    },
    "as_canonical_json" => {
      match std::panic::catch_unwind(|| {
        lua.push_string(&bson.into_canonical_extjson().to_string());
      }) {
        Ok(_) => 1,
        Err(_) => lua.error("Failed to convert to JSON")
      }
    },
    "as_lua" => {
      crate::bson::from::bson_to_lua(lua, bson);
      1
    },
    _ => lua.error("Invalid key")
  }
}

#[lua_function]
unsafe fn bson_tostring(lua: State) -> i32 {
  let bson = get_bson!(lua);

  match std::panic::catch_unwind(|| {
    lua.push_string(&bson.into_relaxed_extjson().to_string());
  }) {
    Ok(_) => 1,
    Err(_) => lua.error("Failed to convert to JSON")
  }

  1
}

pub unsafe fn bson_new(lua: State, bson: Bson) {
  lua.new_table();

  lua.push_function(bson_index);
  lua.set_field(-2, lua_string!("__index"));

  lua.push_function(bson_tostring);
  lua.set_field(-2, lua_string!("__tostring"));

  lua.new_userdata(bson, Some(-1));
}

#[lua_function]
pub unsafe fn bson(lua: State) -> i32 {
  bson_new(lua, coerce(lua));
  1
}