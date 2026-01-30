local function notifyerror(message)
	ya.notify({
		title = "Error",
		content = message,
		level = "error",
		timeout = 5,
	})
end
local write = ya.sync(function(state, path, content)
	local f = io.open(path, "w")
	if f then
		f:write(content)
		f:close()
	end
end)
local get_out = ya.sync(function(state)
	return state.out
end)
local function finish(content)
	local out = get_out()

	ya.dbg("printing: " .. tostring(content) .. "to" .. tostring(out))

	-- write("test.txt", tostring(content))
	if out and content then
		local f = io.open(out, "w")
		if f then
			f:write(tostring(content))
			f:flush()
			f:close()
			ya.manager_emit("quit", {})
		end
	end
end
local function setup(state, opts)
	state.Enter      = opts.Enter
	state.ShiftEnter = opts.ShiftEnter

	local raw_mode   = os.getenv("YAZI_PICKER_MODE")
	local raw_out    = os.getenv("YAZI_PICKER_OUT")



	if raw_out then
		os.execute("unset YAZI_PICKER_OUT")
	else
		return
	end
	if raw_mode then
		os.execute("unset YAZI_PICKER_MODE")
	else
		raw_mode = "1"
	end

	state.out = raw_out
	state.mode = tonumber(raw_mode)
	if not state.mode then
		state.mode = 1
		notifyerror("cant convert " .. state.mode .. " to number, defaulting to mode 1")
	end


	Status:children_add(function()
		local labels = {
			[1] = "FILE",
			[2] = "SAVE AS",
			[4] = "DIR",
			[3] = "FILES+",
			[5] = "DIRS+",
			[6] = "MIXED"
		}

		local label = labels[state.mode]
		if label then
			return ui.Span("  PICKER: " .. label .. " ")
				:fg("black")
				:bg("yellow")
				:style(ui.Style():bold())
		end
	end, 500, Status.RIGHT)
end

local get_state = ya.sync(function(state)
	if not state.out then
		return {
			normal = state.Enter,
			shift = state.ShiftEnter,
		}
	end

	local state_out = {}
	state_out.mode = state.mode
	state_out.out = state.out
	state_out.cwd = cx.active.current.cwd


	local hov = cx.active.current.hovered
	local sel = cx.active.selected

	if #sel == 0 then
		if hov then
			state_out.url = hov.url
		else
			state_out.empty = true
		end
	elseif #sel == 1 then
		local url
		for _, f in pairs(sel) do
			url = f
			break
		end
		state_out.url = url
	else
		state_out.urls = {}
		for _, f in pairs(sel) do
			table.insert(state_out.urls, f)
		end
	end



	return state_out
end)

local function is_dir(url)
	return fs.cha(url).is_dir
end
local function is_file(url)
	return not is_dir(url)
end

local function entry(_, job)
	local state = get_state()
	local shiftmode = job.args.shift

	if not state.out then
		if shiftmode then
			ya.dbg("enter+shift " .. (state.shift or "[not assigned]"))
			if state.shift then
				ya.emit(state.shift)
			end
		else
			ya.dbg("enter " .. (state.normal or "[not assigned]"))
			if state.normal then
				ya.emit(state.normal)
			end
		end
		return
	end

	local mode = state.mode
	local url = state.url
	local urls = state.urls
	local empty = state.empty

	if empty then
		notifyerror("nothing selected")
	end

	if mode == 1 then -- single file
		if url then
			if is_file(url) then
				finish(url)
			else
				notifyerror("expected file got directory")
			end
		else
			notifyerror("too many")
		end
	elseif mode == 2 then -- single file save
		if url then
			if is_file(url) then
				local name, event = ya.input({ -- ask to override not name
					title = "Filename: ",
					-- value = "", -- preffered name
					position = { "center", w = 50 },
				})
				-- finish todo
			else
				notifyerror("cant override directory")
			end
		else
			notifyerror("too many")
		end
	elseif mode == 3 then -- single dir
		if url then
			if is_dir(url) then
				finish(url)
			else
				notifyerror("expected directory got file ")
			end
		else
			notifyerror("too many")
		end
	elseif mode == 4 then -- multi file
		if url then
			if is_file(url) then
				finish(url)
			else
				notifyerror("expected file got directory")
			end
		elseif urls then
			local paths = ""
			for _, v in pairs(urls) do
				ya.dbg("urls 2")
				if is_file(v) then
					ya.dbg("urls 3")
					paths = paths .. tostring(v) .. '\n'
					ya.dbg("urls 4")
				else
					notifyerror("expected file got directory")
				end
			end
			finish(paths)
		else
			notifyerror("ble ble 1")
		end
	elseif mode == 5 then -- multi dir
		if url then
			if is_dir(url) then
				finish(url)
			else
				notifyerror("expected directory got file ")
			end
		elseif urls then
			local paths = ""
			for _, v in pairs(urls) do
				if is_dir(v) then
					paths = paths .. '\n' .. v
				else
					notifyerror("expected directory got file ")
				end
			end
			finish(paths)
		else
			notifyerror("ble ble 2")
		end
	elseif mode == 6 then -- mixed
		if url then
			finish(url)
		elseif urls then
			local paths = ""
			for _, v in pairs(urls) do
				paths = paths .. '\n' .. v
			end
			finish(paths)
		else
			notifyerror("ble ble 3")
		end
	end
end

return {
	entry = entry,
	setup = setup,
}
