local json = require(".json")
local utils = require(".utils")


local function notifyerror(message)
	ya.notify({
		title = "Error",
		content = message,
		level = "error",
		timeout = 5,
	})
end
local get_out = ya.sync(function(state)
	return state.out, state.json
end)
local function finish(urls)
	local out, makejson = get_out()

	ya.dbg("printing: " .. tostring(urls) .. " to " .. tostring(out))


	local f = io.open(out, "w")
	if f then
		if makejson then
			local files = {}
			for _, v in pairs(urls) do
				table.insert(files, tostring(v))
			end
			local R = {}
			R.files = files
			f:write(json.encode(R))
		else
			for _, v in pairs(urls) do
				f:write(tostring(v) .. '\n')
			end
		end
		f:flush()
		f:close()
		ya.emit("quit", {})
	end
end
local function status_texts(state)
	local old_layout = Root.layout
	local old_redraw = Root.redraw

	Root.layout = function(self)
		local original_area = self._area

		-- Split the screen: 1 line at top, rest at bottom
		local chunks = ui.Layout()
			:direction(ui.Layout.VERTICAL)
			:constraints({
				ui.Constraint.Length(1), -- Height of your top bar
				ui.Constraint.Fill(1),
			})
			:split(self._area)

		-- Trick Yazi into drawing standard interface in the bottom chunk
		self._area = chunks[2]
		old_layout(self)

		-- Restore area so 'render' sees the full screen
		self._area = original_area

		state.top = chunks[1]
	end

	Root.redraw = function(self)
		local rrr = old_redraw(self)
		table.insert(rrr, ui.Line(tostring(state.text_top)):area(state.top))
		return rrr
	end
end

local get_state = ya.sync(function(state)
	if not state.out then
		return {
			normal = state.Enter,
			shift = state.ShiftEnter,
		}
	end

	local state_out = {}
	state_out.opts = state.opts
	state_out.out = state.out
	state_out.cwd = cx.active.current.cwd


	local hov = cx.active.current.hovered
	local sel = cx.active.selected

	if #sel == 0 then
		if hov then
			state_out.urls = { hov.url }
		else
			state_out.urls = {}
		end
	else
		state_out.urls = {}
		for _, f in pairs(sel) do
			table.insert(state_out.urls, f)
		end
	end



	return state_out
end)

local M = {}
function M:setup(opts)
	self.Enter      = opts.Enter
	self.ShiftEnter = opts.ShiftEnter

	local raw_out   = utils.popenv("YAZI_PICKER_OUT")
	local raw_mode  = utils.popenv("YAZI_PICKER_MODE")
	local raw_in    = utils.popenv("YAZI_PICKER_IN")
	local raw_json  = utils.popenv("YAZI_PICKER_JSON")

	self.out        = raw_out
	local mode      = tonumber(raw_mode)
	self.json       = raw_json == "1"

	if not self.out then
		notifyerror("no output file specified")
		return
	end

	if raw_in and raw_in:len() > 0 then
		local f = io.open(raw_in, "r")
		if f then
			self.opts = json.decode(f:read("a"))
		else
			notifyerror("failed to open options file")
			return
		end
	elseif mode then
		ya.dbg("mode: " .. mode)
		self.opts = utils.mode_opts[mode]
	else
		notifyerror("no mode or options file specified")
		return
	end


	ya.dbg(self)


	status_texts(self)

	Header:children_add(function()
		return ui.Line {
			ui.Span("test2 "):fg("red"):bold(),
		}
	end, 1000, Header.RIGHT)

	Status:children_add(function()
		local label = utils.make_label(self.opts.mode or 1)
		return ui.Span(label)
			:fg("black")
			:bg("yellow")
			:style(ui.Style():bold())
	end, 500, Status.RIGHT)
end

function M:entry(job)
	local state = get_state()
	local shift = job.args.shift

	if not state.out then
		if shift then
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

	local opts = state.opts
	local urls = state.urls
	local cwd = state.cwd


	if not shift then
		local ok, err = utils.verify_file_type(urls, opts.mode)
		if not ok then
			notifyerror(err)
			return
		end

		-- TODO: ask if overriding
		-- TODO: dir + file mode (mode x or 5)
		finish(urls)

		-- local yes = ya.confirm {
		-- 	pos = { "center", w = 62, h = 10 },
		-- 	title = "Quit?",
		-- 	body = ui.Text("There are multiple tabs open. Are you sure you want to quit?"):wrap(ui.Wrap.YES),
		-- }

		-- local name, event = ya.input({ -- ask to override not name
		-- 	title = "Filename: ",
		-- 	value = "",             -- todo: preffered name
		-- 	position = { "center", w = 50 },
		-- })
	end
end

return M
