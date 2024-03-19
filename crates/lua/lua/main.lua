-- Print contents of `tbl`, with indentation.
-- `indent` sets the initial level of indentation.
function tprint (tbl, indent)
  if not indent then indent = 0 end
  for k, v in pairs(tbl) do
    formatting = string.rep("  ", indent) .. k .. ": "
    if type(v) == "table" then
      print(formatting)
      tprint(v, indent+1)
    elseif type(v) == 'boolean' then
      print(formatting .. tostring(v))
    elseif type(v) == 'function' then
      print(formatting .. 'function')
    else
      print(formatting .. v)
    end
  end
end

--- Handler for the home page
function home()
    return "<h1>Home Page</h1>"
end

--- Handler for the /hello page
function hello()
    return "<h1>Hello</h1>"
end

return function()
    local server = http.Server
    local response = http.Client:get("https://httpbin.org")

    Server:get("/", home)
    Server:serve("127.0.0.1:3000")
end
