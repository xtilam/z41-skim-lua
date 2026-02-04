package = "luaskim"
version = "dev-1"
source = {
	url = "git://github.com/xtilam/luaskim",
}
description = {
	summary = "Skim binding for Lua",
	modules = {
		luaskim = "luaskim", -- Tên cái require("luaskim")
	},
}
dependencies = {
	"lua >= 5.1",
}
build = {
	type = "rust-mlua", -- Đây là điểm mấu chốt
	modules = {
		luaskim = {
			path = ".", -- Đường dẫn tới Cargo.toml
			-- Mày có thể chỉ định các features ở đây nếu cần
			default_features = false,
			features = { "use-luajit" },
		},
	},
}
