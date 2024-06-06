return {
	--- Print a separator to stdout repeated 25 times inline
	--- @param sep string
	print_sep = function(sep)
		print("\n" .. string.rep(sep, 25) .. "\n")
	end,
}
