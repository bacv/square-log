print("Initializing coffees plugin")

sq_call_fn = function(source)
    local res = sq_log:fetch_json(source.url)
    for _, item in ipairs(res) do
        local record = {
            title = item.title,
            description = item.description,
            tags = item.ingredients,
            link = item.image,
            extended = "Extended information not provided.",
            hash = tostring(item.id),
            origin_timestamp = os.time(),
            pull_timestamp = os.time(),
        }
        sq_log:insert_data(record)
    end
end
