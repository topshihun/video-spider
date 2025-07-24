function main(key)
  local video = {}
  video["name"] = "video_name" .. key
  video["description"] = "description"
  video["image"] = "http://localhost/simple.png"
  video["episodes"] = {}
  video["episodes"]["1"] = "http://localhost/simple1.mp4"
  video["episodes"]["2"] = "http://localhost/simple2.mp4"
  return video
end
