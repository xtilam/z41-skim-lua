local skim = require("skim")

local sk = skim.new({
	cmd = "zellij ls",
	ansi = true,
	multi = false,
	-- preview = "echo Hello from preview!",
})

sk:bind("ctrl-t", function(self)
	self:set_header("-- Ctrl-T Pressed --")
	return {
		"reload(fd)",
	}
end)

sk:bind("ctrl-a", function(self)
  self:set_header("-- Ctrl-A Pressed --")
	-- print("equal?", app == sk)
	-- app:set_header("-- Ctrl-A Pressed --")
	-- if err then
	-- 	print("Error setting header:", err)
	-- end
	return {
		"reload(zellij ls)",
	}
end)

local rs = sk:start()
print(table.concat(rs or { "No result!" }, "\n"))

-- skim.run({
-- 	cmd = "zellij ls",
-- 	ansi = true,
-- 	multi = false,
-- 	preview = "echo Hello from preview!",
-- })
-- local sk = skim.init()

-- sk.cmd = "zellij ls"
-- sk:run()
-- config.ansi = true
-- config.multi = true

-- skim.run(config)
-- sk.run(config)
-- sk.run(config)
-- sk:cmd("zellij ls")
-- sk:ansi(true)
-- sk:multi(true)
-- sk:bind("ctrl-t", function(e)
-- 	-- e.change_header("-- Ctrl-T Pressed --")
-- 	print("-- Ctrl-T Pressed --")
-- end)
-- local rs = sk:run()
-- print(table.concat(rs or {"No result!"}, "\n"))
--
