--- @meta

--- check if a value is in a table
--- @param t table
--- @param v any
--- @return boolean true if the value exists
function table.contains(t, v)
	-- print(pairs(t))
	for _, value in pairs(t) do
		if value == v then
			return true
		end
	end
	return false
end

--- check if a key is in a table
--- @param t table
--- @param k integer|string
--- @return boolean true if the key exists
function table.contains_key(t, k)
	for key, _ in pairs(t) do
		if key == k then
			return true
		end
	end
	return false
end
