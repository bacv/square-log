print("Initializing wines plugin")

local item = {
    wine = "Chardonnay",
    location = "Napa Valley",
    rating = {
        average = "4.5"
    },
    image = "http://example.com/wine/chardonnay.png",
    winery = "Silver Oak",
    id = 12345
}

local record = {
    title = item.wine,
    description = "Wine from " .. item.location,
    tags = { "wine", "rating:" .. item.rating.average },
    link = item.image,
    extended = { winery = item.winery },
    hash = tostring(item.id),
    origin_timestamp = os.time(),
    pull_timestamp = os.time(),
}

sq_pull_fn = function(source)
    -- local res = sq_log:fetch_json(source.url)
    -- for _, item in ipairs(res) do
    --     local record = {
    --         title = item.wine,
    --         description = "Wine from " .. item.location,
    --         tags = { "wine", "rating:" .. item.rating.average },
    --         link = item.image,
    --         extended = { winery = item.winery },
    --         hash = tostring(item.id),
    --         origin_timestamp = os.time(),
    --         pull_timestamp = os.time(),
    --     }
    --     sq_log:insert_data(record)
    -- end
    sq_log.db:insert_data(record)
end
