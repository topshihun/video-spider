local base_url = "http://api.ffzyapi.com/api.php/provide/vod/?"

-- get_data_url
-- json_parse
-- string_spilt
-- url_decode

function main(keyword)
  local video_list_json = get_data_url(base_url .. "ac=list&wd=" .. keyword)
  local video_list = json_parse(video_list_json)
  local video_ids = ""
  for video in pairs(video_list["list"]) do
    video_ids = video["vod_id"] .. "," video_ids
  end
  local video_details_json = get_data_url(base_url .. "ac=detail&ids=" .. video_ids)
  local video_details = json_parse(video_details_json)
  local series_arr = {}
  for i, video_detail in ipairs(video_details["list"]) do
    series_arr[i] = {}
    series_arr[i]["name"] = url_decode(video_detail["vod_name"])
    series_arr[i]["description"] = url_decode(video_detail["vod_content"])
    series_arr[i]["image"] = video_detail["vod_pic"]
    -- video urls
    series_arr[i]["episodes"] = {}
    local episode_arr_str = string_spilt(video_detail["vod_play_url"], "$$$")
    for j = 1, #episode_arr_str do
      local episode_key_val = episode_arr_str[j].spilt("$")
      local episode_key_val = string_spilt(episode_arr_str[j], "$")
      episode_arr[i]["episodes"][episode_key_val[1]] = episode_key_val[2]
    end
  end
  return series_arr
end
