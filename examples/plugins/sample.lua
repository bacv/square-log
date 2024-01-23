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

sqrt_call_fn = function(source)
    test()
    print_r(sqrt_log:fetch_json(source.url))
end
