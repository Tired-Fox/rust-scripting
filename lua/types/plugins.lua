--- @meta

--- The plugin information stripped of the event hooks
--- @class PluginInfo
--- @field name string
--- @field version string
--- @field author string
--- @field description string

--- A table of plugin information and event hooks
--- @class Plugin
--- @field name string
--- @field version string
--- @field author string
--- @field description string
--- @field setup? fun(plugin: PluginInfo)
Plugin = {}

--- Module for adding plugins
--- @class plugins
--- @field plugins Plugin[]
plugins = {}

--- Add a new plugin
--- @param plugin Plugin
function plugins.new_plugin(plugin) end

