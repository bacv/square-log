print("Initializing wines plugin")

sq_pull_fn = function(source)
    local res = sq_log:fetch_json(source.url)
    for _, item in ipairs(res) do
        local record = {
            title = item.wine,
            description = "Wine from " .. item.location,
            tags = { "wine", "rating:" .. item.rating.average },
            link = item.image,
            extended = "Winery: " .. item.winery,
            hash = tostring(item.id),
            origin_timestamp = os.time(),
            pull_timestamp = os.time(),
        }
        sq_log:insert_data(record)
    end
end
