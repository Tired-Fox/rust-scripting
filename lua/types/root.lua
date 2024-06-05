---@meta

--- Root namespace
--- @class v
--- @field print fun(...) Print all arguments to stdout
--- @field plugins plugins
v = {}

--- Application Configuration
--- @class Config
config = {
	--- Paths to search for files
	--- @class Paths
	--- @field projects string Path to the projects directory where all your cloned repositories live
	--- @field download string Path where external dependencies should be downloaded/installed
	--- @field build string Path where the build will occur
	paths = {},
	--- Application optional features
	--- @class Features
	--- @field show_docker_logs boolean show or hide docker command stdout responses
	features = {},
}
