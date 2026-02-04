extern crate skim;
use crate::{
	lua_bridge, lua_err, lua_rs,
	lua_skim_opts::*,
	skim_mpsc::{self, Msg, Request, Response},
};
use crossterm::event::KeyEvent;
use mlua::prelude::*;
use skim::{
	Skim,
	binds::parse_key,
	prelude::SkimOptionsBuilder,
	tui::{
		App, Event,
		event::{Action, ActionCallback, parse_action},
	},
};
use std::{sync::mpsc, thread};
use strip_ansi_escapes::strip;

type LResult<T = ()> = Result<T, mlua::Error>;
pub struct LuaSkim {
	pub opts: LuaSkimOpts,
	pub app: Option<usize>,
	pub is_running: bool,
	pub actions: mlua::RegistryKey,
	pub actions_skim: Vec<(KeyEvent, Action)>,
	pub sender: mpsc::Sender<Msg>,
	pub receiver: mpsc::Receiver<Msg>,
}

impl LuaSkim {
	pub fn init(lua: &Lua, config: LuaTable) -> LResult<Self> {
		let (tx, rx) = skim_mpsc::server();
		Ok(Self {
			app: None,
			actions: lua.create_registry_value(lua.create_table()?)?,
			is_running: false,
			opts: LuaSkimOpts::new(&config),
			actions_skim: Vec::new(),
			receiver: rx,
			sender: tx,
		})
	}
	fn no_run(&self) -> LResult {
		match self.is_running {
			false => Ok(()),
			true => Err(lua_err!("Skim is already running")),
		}
	}
	fn lua_callback_handler(&mut self, lua: &Lua) -> () {
		let table_actions = lua.registry_value::<mlua::Table>(&self.actions).ok();
		while let Ok(msg) = self.receiver.recv() {
			match msg.data {
				Request::CallLuaAction((lua_fn_index, app_ptr)) => {
					self.app = Some(app_ptr);
					let rs: mlua::Result<Vec<Event>> = (|| {
						let table = table_actions
							.as_ref()
							.ok_or_else(|| lua_err!("Lua actions table is missing"))?;
						let fn_callback = table.raw_get::<_, mlua::Function>(lua_fn_index)?;
						let events: Vec<String> = lua.scope(|s| {
							fn_callback.call::<_, Vec<String>>(s.create_userdata_ref_mut(self)?)
						})?;

						Ok(events
							.iter()
							.filter_map(|s| parse_action(&s).map(Event::Action))
							.collect())
					})();
					msg.done(Response::Actions(rs.unwrap_or_else(|_e| Vec::new())))
						.ok();
					continue;
				}
				Request::Done() => {
					println!("Lua skim done");
					break;
				}
			}
		}
		self.app = None;
	}
	fn get_app(&self) -> LResult<&mut App<'_>> {
		self.app
			.ok_or_else(|| lua_err!("Skim app is not running"))?;
		match self.is_running {
			true => unsafe { Ok(&mut *(self.app.unwrap() as *mut App)) },
			false => Err(lua_err!("No skim event dispatched")),
		}
	}
	pub fn strip_ansi(str: String) -> String {
		let plain_bytes = strip(str);
		String::from_utf8(plain_bytes).unwrap()
	}
	fn dispatch_done<T>(sender: mpsc::Sender<Msg>, rs: T) -> T {
		sender
			.send(Msg {
				data: Request::Done(),
				reply: None,
			})
			.ok();
		rs
	}
}

lua_bridge!(LuaSkimActions, LuaSkim, {
	config: [config: mlua::Table] => (),
	bind: [keymap: String, callback: mlua::Function] => (),
  start: [] => Vec<String>,
	set_header: [header: String] => (),
});

impl LuaSkimActions for LuaSkim {
	fn config(&mut self, _lua: &mlua::Lua, config: mlua::Table) -> mlua::Result<()> {
		self.no_run()?;
		self.opts = LuaSkimOpts::new(&config);
		Ok(())
	}
	fn set_header(&mut self, _lua: &mlua::Lua, header: String) -> mlua::Result<()> {
		let app = self.get_app()?;
		app.header.header = header.clone();
		app.options.header = Some(header.clone());
		Ok(())
	}
	fn start(&mut self, lua: &mlua::Lua) -> mlua::Result<Vec<String>> {
		self.no_run()?;
		self.is_running = true;
		let opts = self.opts.clone();
		let sender = self.sender.clone();
		let keymaps = self.actions_skim.clone();
		let handle = thread::spawn(move || {
			Self::dispatch_done(sender, {
				let mut opts = lua_rs!(SkimOptionsBuilder::from(opts.clone()).build())?;
				opts.keymap
					.extend(keymaps.iter().map(|(k, v)| (k.clone(), vec![v.clone()])));
				Ok(lua_rs!(Skim::run_with(opts, None))?
					.selected_items
					.iter()
					.map(|item| Self::strip_ansi(item.output().to_string()))
					.collect::<Vec<String>>())
			})
		});

		self.lua_callback_handler(lua);
		let rs_thread = handle.join();
		self.is_running = false;
		match rs_thread {
			Ok(rs) => rs,
			Err(e) => Err(lua_err!(format!("Skim thread panicked: {:?}", e))),
		}
	}
	fn bind(
		&mut self,
		lua: &mlua::Lua,
		keymap: String,
		callback: mlua::Function,
	) -> mlua::Result<()> {
		self.no_run()?;
		let keymap = lua_rs!(parse_key(&keymap))?;
		let actions_table: mlua::Table = lua.registry_value(&self.actions)?;
		actions_table.push(callback)?;
		let len = actions_table.raw_len();
		let client = skim_mpsc::Client::new(self.sender.clone());
		let skim_act = ActionCallback::new(move |app| {
			client
				.call_lua(len, app as *const App as usize)
				.ok_or("Lua call failed".into())
		});
		self.actions_skim.push((keymap, Action::Custom(skim_act)));
		Ok(())
	}
}
