mod lua_skim;
mod lua_skim_opts;
mod my_macro;
mod skim_mpsc;

use crate::lua_skim::*;
use mlua::prelude::*;

#[mlua::lua_module]
fn skim(lua: &Lua) -> LuaResult<LuaTable<'_>> {
	let exports = lua.create_table()?;
	exports.set("new", lua.create_function(LuaSkim::init)?)?;
	exports.set(
		"strip_ansi",
		lua.create_function(|_, text: String| Ok(LuaSkim::strip_ansi(text)))?,
	)?;
	Ok(exports)
}
