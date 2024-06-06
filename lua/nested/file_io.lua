print("Config has features: " .. tostring(table.contains_key(config, "features")))
print("Config features has a toggle that is true: " .. tostring(table.contains(config.features, true)))
require("utils").print_sep("-")

local file = io.open("test.txt", "r")
if file ~= nil then
	local data = file:read("*a")
	file:close()
	print(data)
else
	print("Failed to open file `test.txt`")
end
require("utils").print_sep("-")
