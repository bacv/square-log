print("hello from global")

local test = function()
    print("hello from local")
end

-- This function needs to be global because it's recursive.
print_r = function(t, indent)
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

sqrt_call_fn = function(uri)
    test()
    print_r(sqrt_log:fetch_json(uri))
end
