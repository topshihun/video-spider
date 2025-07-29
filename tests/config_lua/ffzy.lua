local base_url = "http://api.ffzyapi.com/api.php/provide/vod/?"

-- http_get
-- json_parse
-- string_spilt
-- url_decode

function main(keyword)
  local video_list_json = utils.http_get(base_url .. "ac=list&wd=" .. keyword)
  print(video_list_json)
  local video_list = utils.json_parse(video_list_json)
  video_list = video_list["list"]
  local video_ids = ""
  print(#video_list)
  for _, video in ipairs(video_list) do
    video_ids = video["vod_id"] .. "," .. video_ids
  end
  print("video_ids:" .. video_ids)
  local video_details_json = utils.http_get(base_url .. "ac=detail&ids=" .. video_ids)
  local video_details = utils.json_parse(video_details_json)
  local series_arr = {}
  for i, video_detail in ipairs(video_details["list"]) do
    series_arr[i] = {}
    series_arr[i]["name"] = utils.url_decode(video_detail["vod_name"])
    series_arr[i]["description"] = utils.url_decode(video_detail["vod_content"])
    series_arr[i]["image"] = video_detail["vod_pic"]
    -- video urls
    series_arr[i]["episodes"] = {}
    print(video_detail["vod_play_url"])
    local episode_arr_str = utils.string_split(video_detail["vod_play_url"], "#")
    print(#episode_arr_str)
    for _, episode_str in ipairs(episode_arr_str) do
      print(episode_str)
      local episode_key_val = utils.string_split(episode_str, "$")
      series_arr[i]["episodes"][episode_key_val[1]] = episode_key_val[2]
    end
  end
  return series_arr
end
