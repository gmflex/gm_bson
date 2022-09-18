# gm_bson
bson for gmod

## Docs

### bson.from_lua(value: `any`): `Bson`
Converts any lua type into Bson.<br>
**Unsupported types: `function`, `lightuserdata`, `thread`**
```lua
local data = bson.from_lua {
  _id = bson.objectid(),
  name = "John",
  age = 31,
}
```

### bson.to_lua(value: `Bson`): `any`
Converts Bson value into lua.
```lua
local table = bson.to_lua(data)
-- _id = objectid(smth),
-- name = "John",
-- age = 31,
```

### bson.objectid(oid: `string | nil`): `objectid`
Creates new ObjectID from given Base64 value or generates a new one.
```lua
local oid = bson.objectid()
-- or
local oid = bson.objectid("stmth")
```

### bson.datetime(ms: `integer | nil`): `datetime`
Creates new Datetime from given timestamp or from current time.
```lua
local dt = bson.datetime(1)
-- or
local dt = bson.datetime()
```

### bson.timestamp(time: `integer`, increment: `integer | nil`): `timestamp`
Creates new Timestamp from given time and increment, if present.
```lua
local ts = bson.timestamp(1, 1)
-- or
local ts = bson.timestamp(1)
```

### bson.binary(bin: `string`): `binary`
Creates new Generic Binary from given string.
```lua
local bin = bson.binary "123"
```

### bson.regex(pattern: `string`, options: `string | nil`): `regex`
Creates new Regex from given pattern and options, if present.
**Patterns must be in ECMAScript format.**
```lua
local rx = bson.regex("\w*", "i")
-- or 
local rx = bson.regex("\d+")
```

### bson.code(code: `string`): `code`
Creates new Code from given string.
**Code must be written in JavaScript**
```lua
local code = bson.code("console.log()")
```

### bson.minkey(): `minkey`
Creates new MinKey
```lua
local mnk = bson.minkey()
```

### bson.maxkey(): `maxkey`
Creates new MaxKey
```lua
local mxk = bson.maxkey()
```

### bson.decimal128(dcml: `string`): `decimal128`
Creates new Decimal128 from given string.
**String must be a valid number**
```lua
local dcml = bson.decimal128 "123"
```
