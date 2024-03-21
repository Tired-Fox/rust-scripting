local _, err = pcall(plugins.new_plugin, {
    name = "test-plugin",
    version = "0.1.0",
    author = "Tired Fox",
    description = "Test plugin to use for testing lua state",
    setup = function (info)
        print(string.format("[SETUP] %s %s by %s\n  %s", info.name, info.version, info.author, info.description))
    end,
})

if err then
    print("Failed to create plugin test-plugin: " .. (err or "Unknown reason"))
end

-- Globally provided pprint where rust code will pretty print and format what is passed in, including tables.
pprint(plugins.PLUGINS[1])

plugins.new_plugin {
    name = "something",
    author = "Tired Fox",
    description = "Something to put here for type checking",
    version = "0.0.0",
}