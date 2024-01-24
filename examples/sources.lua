-- Example sample plugin configuration.
local sample_config = {
    secret_key = "sample_secret_key",
}

-- A sample plugin sources. Every plugin is free to define what data
-- needs to be provided per entry.
-- `url` and `interval` are mandatory fields for every source of any plugin.
local wine_sources = {
    {
        url = "https://api.sampleapis.com/wines/reds",
        interval = "10s",
        config = sample_config,
    },
    {
        url = "https://api.sampleapis.com/wines/whites",
        interval = "10s",
        config = sample_config,
    },
}

local coffee_sources = {
    {
        url = "https://api.sampleapis.com/coffee/hot",
        interval = "20s",
        config = sample_config,
    }
}

-- A global function that the sqrt-log will call to gather sources
-- per plugin when loading.
-- The table has to have a plugin name as key and the list of sources
-- as the value.
function sqrt_sources_fn()
    print("hello from sources")
    return {
        wines = wine_sources,
        coffees = coffee_sources,
    }
end
