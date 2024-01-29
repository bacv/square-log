print("Initializing coffees plugin")

local item = {
    title = "Caramel Macchiato",
    description = "Espresso with steamed milk and caramel drizzle",
    ingredients = { "espresso", "steamed milk", "caramel syrup", "vanilla syrup" },
    image = "http://example.com/coffee/caramel-macchiato.png",
    id = 54321
}

local record = {
    title = item.title,
    description = item.description,
    tags = item.ingredients,
    link = item.image,
    extended = { ext = "Extended information not provided." },
    hash = tostring(item.id),
    origin_timestamp = os.time(),
    pull_timestamp = os.time(),
}

local function printTable(tbl, indent)
    if not indent then indent = 0 end
    for k, v in pairs(tbl) do
        formatting = string.rep("  ", indent) .. k .. ": "
        if type(v) == "table" then
            print(formatting)
            printTable(v, indent + 1)
        else
            print(formatting .. tostring(v))
        end
    end
end

sq_pull_fn = function(source)
    print(">>>>>>>>>>>>>>")
    printTable(sq_log.db:get_latest())
    print(">>>>>>>>>>>>>>")
    -- local res = sq_log.http:fetch_json(source.url)
    -- for _, item in ipairs(res) do
    --     local record = {
    --         title = item.title,
    --         description = item.description,
    --         tags = item.ingredients,
    --         link = item.image,
    --         extended = { ext = "Extended information not provided." },
    --         hash = tostring(item.id),
    --         origin_timestamp = os.time(),
    --         pull_timestamp = os.time(),
    --     }
    --     sq_log:insert_data(record)
    -- end
    sq_log.db:insert_data(record)
end
