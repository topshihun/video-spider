[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# VideoSpider

Watch TV series from custom platforms.

## How to use

- You have to define what you want to watch and then write a lua script.
- Optional, [videospidercli](videospidercli) is helpful to test your lua script.
- Run [videospidertui](videospidertui)(It need [mpv](https://mpv.io) to play).

## Lua script

- Input keyword to lua and return series information.
- Lua source file should be placed in the configuration path.

For example
```lua
function main(key)
  local video = {}
  video[1] = {}
  video[1]["name"] = "video_name1" .. key
  video[1]["description"] = "description"
  video[1]["image"] = "http://localhost/simple.png"
  video[1]["episodes"] = {}
  video[1]["episodes"][1] = { ["name"] = "1", ["addr"] = "http://localhost/simple1.mp4" }
  video[1]["episodes"][2] = { ["name"] = "2", ["addr"] = "http://localhost/simple2.mp4" }
  return video
end

## Lua extension
```

- http_get
```lua
utils.http_get = function(String)
  return String
end
```

- json_parse
```lua
utils.json_parse = function(String)
  return Table
end
```

- string_split
```lua
utils.string_split = function(String, String)
  return Array
end
```

- url_encode
```lua
utils.url_encode = function(String)
  return String
end
```

- url_decode
```lua
utils.url_decode = function(String)
  return String
end
```

- unicode_encode
```lua
utils.unicode_encode = function(String)
  return String
end
```

- unicode_decode
```lua
utils.unicode_decode = function(String)
  return String
end
```
