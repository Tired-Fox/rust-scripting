local _, err = pcall(plugins.new_plugin, {
	name = "test-plugin",
	version = "0.1.0",
	author = "Tired Fox",
	description = "Test plugin to use for testing lua state",
	setup = function(info)
		print(string.format("%s %s by %s\n  %s", info.name, info.version, info.author, info.description))
	end,
})

if err then
	v.print("Failed to create plugin test-plugin:", err or "Unknown reason")
	return
end

-- Globally provided pprint where rust code will pretty print and format what is passed in, including tables.
local plugin = plugins.plugins[1]
v.print(plugin)

plugins.new_plugin({
	name = "something",
	author = "Tired Fox",
	description = "Something to put here for type checking",
	version = "0.0.0",
})

v.print("Hello, world!", { a = 1, b = 2 })
