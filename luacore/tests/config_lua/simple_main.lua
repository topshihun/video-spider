function search(key)
  local detail_datas = {}
  detail_datas[1] = "This is a data"
  detail_datas[2] = "This is a data2"
  return detail_datas
end

function get_detail(data)
  local video = {}
  video["name"] = "video_name"
  video["description"] = "description"
  video["image"] = "http://localhost/simple.png"
  video["episodes"] = {}
  video["episodes"][1] = { ["name"] = "1", ["addr"] = "http://localhost/simple1.mp4" }
  video["episodes"][2] = { ["name"] = "2", ["addr"] = "http://localhost/simple2.mp4" }
  return video
end
