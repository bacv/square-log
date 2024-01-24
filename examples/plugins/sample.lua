print("hello from global")

local function test()
    print("hello from local")
end

local function print_r(t, indent)
    local indent = indent or ""
    for k, v in pairs(t) do
        io.write(indent, tostring(k))
        if type(v) == "table" then
            io.write(":\n")
            print_r(v, indent .. "  ")
        else
            io.write(": ", v == nil and "null" or tostring(v), "\n")
        end
    end
end

local record = {
    date = "2024-01-22",
    title = "Example Title",
    description = "This is a sample description.",
    tags = { "tag1", "tag2", "tag3" },
    link = "https://example.com",
    extended = "Extended information here.",
    hash = "abc123hash"
}

sqrt_call_fn = function(source)
    print_r(source)
    -- print_r(sqrt_log:fetch_json(source.url))
    local res = sqrt_log:fetch_json(source.url)
    record.date = os.date("%Y-%m-%d")
    sqrt_log:insert_data(record)
    print()
end
