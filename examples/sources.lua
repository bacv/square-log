-- Example sample plugin configuration.
local sample_config = {
    secret_key = "sample_secret_key",
    badge_url = "http://dud.dud",
}

-- A sample plugin sources. Every plugin is free to define what data
-- needs to be provided per entry.
-- `id` and `interval` are mandatory fields for every source of any plugin.
local wine_sources = {
    {
        id = "red_wines",
        interval = "10s",
        url = "https://api.sampleapis.com/wines/reds",
        config = sample_config,
    },
    {
        id = "white_wines",
        interval = "10s",
        url = "https://api.sampleapis.com/wines/whites",
        config = sample_config,
    },
}

local coffee_sources = {
    {
        id = "hot_coffees",
        interval = "20s",
        url = "https://api.sampleapis.com/coffee/hot",
        token = "auth_token",
    }
}

-- A global function that the square-log will call to gather sources
-- per plugin when loading.
-- The table has to have a plugin name as key and the list of sources
-- as the value.
function sq_sources_fn()
    print("hello from sources")
    return {
        wines = wine_sources,
        coffees = coffee_sources,
    }
end
