print("hello from global")

local test = function()
	print("hello from local")
end

sqrt_call_fn = function(uri)
	test()
	fetch_json(uri)
end
