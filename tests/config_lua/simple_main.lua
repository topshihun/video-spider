function main(key)
  local video = {}
  video[1] = {}
  video[1]["name"] = "video_name1" .. key
  video[1]["description"] = "description"
  video[1]["image"] = "http://localhost/simple.png"
  video[1]["episodes"] = {}
  video[1]["episodes"]["1"] = "http://localhost/simple1.mp4"
  video[1]["episodes"]["2"] = "http://localhost/simple2.mp4"

  video[2] = {}
  video[2]["name"] = "video_name2" .. key
  video[2]["description"] = "description"
  video[2]["image"] = "http://localhost/simple.png"
  video[2]["episodes"] = {}
  video[2]["episodes"]["1"] = "http://localhost/simple1.mp4"
  video[2]["episodes"]["2"] = "http://localhost/simple2.mp4"
  return video
end
