local M = {}
M.popenv = function(name)
	local v = os.getenv(name)
	if v then
		os.execute("unset " .. name)
		return v
	end
end
M.make_label = function(mode)
	local s = "  PICKER: " .. string.upper(mode.id) .. " "
	s = s .. (mode.directory == nil and "NIL" or mode.directory and "DIR" or "FILE") .. " "
	s = s .. (mode.multiple == nil and "NIL" or mode.multiple and "MULTI" or "SINGLE") .. " "
	return s
end
M.is_dir = function(url)
	return fs.cha(url).is_dir
end
M.verify_file_type = function(urls, opts)
	if not (urls and opts) then
		return false, "urls == nil or opts == nil"
	end
	local req_dir = opts.directory or opts.id == "SaveMulti"
	local req_single = not opts.multiple or opts.id == "SaveMulti"

	local len = #urls
	if len == 0 then
		return false, "nothing selected"
	elseif req_single and len ~= 1 then
		return false, "too many selected"
	end

	for _, v in pairs(urls) do
		if M.is_dir(v) ~= req_dir then
			return false,
				"expected " .. (req_dir and "directory" or "file") .. " but got " .. (req_dir and "file" or "directory")
		end
	end

	return true
end

M.mode_opts = {
	[1] = {
		mode = {
			id = "Open",
			directory = false,
			multiple = false
		}
	},
	[2] = {
		mode = { id = "Save" }
	},
	[3] = {
		mode = {
			id = "Open",
			directory = true,
			multiple = false
		},
	},
	[4] = {
		mode = {
			id = "Open",
			directory = false,
			multiple = true
		}
	},
	[5] = {
		mode = {
			id = "Open",
			directory = true,
			multiple = true
		},
	}
}
return M
