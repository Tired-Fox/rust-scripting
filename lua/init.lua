print("Before:", config)
config.paths = {
	projects = "D:/Repo",
	download = "D:/Downloads",
	build = "D:/Repo/test-project/build",
}
config.features.show_docker_logs = true
v.print("After:", config)
require("utils").print_sep("-")

require("nested.file_io")
require("plugins")
