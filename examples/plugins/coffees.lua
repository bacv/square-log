print("Initializing coffees plugin")

-- Sample record.
-- local record = {
--     date = "2024-01-22",
--     title = "Example Title",
--     description = "This is a sample description.",
--     tags = { "tag1", "tag2", "tag3" },
--     link = "https://example.com",
--     extended = "Extended information here.",
--     hash = "abc123hash"
-- }

sqrt_call_fn = function(source)
    local res = sqrt_log:fetch_json(source.url)
    for _, item in ipairs(res) do
        local record = {
            date = os.date("%Y-%m-%d"),
            title = item.title,
            description = item.description,
            tags = item.ingredients,
            link = item.image,
            extended = "Extended information not provided.",
            hash = tostring(item.id)
        }
        sqrt_log:insert_data(record)
    end
end
