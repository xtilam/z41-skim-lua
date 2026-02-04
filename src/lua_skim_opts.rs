use mlua::prelude::*;
use skim::prelude::{SkimItemReader, SkimItemReaderOption, SkimOptionsBuilder};
use std::{cell::RefCell, rc::Rc};

macro_rules! create_struct { ($name:ident { $($field:ident: $type:ty),* }) => {
#[derive(Clone, Debug)]
pub struct $name { $($field: Option<$type>),* }
impl $name { pub fn new(config: &LuaTable) -> Self { $name { $($field: config.get::<_, $type>(stringify!($field)).ok()),* } } } }; }

create_struct!(LuaSkimOpts {
	cmd: String,
	ansi: bool,
	multi: bool,
	preview: String
});

impl From<LuaSkimOpts> for SkimOptionsBuilder {
	fn from(o: LuaSkimOpts) -> Self {
		let mut builder = SkimOptionsBuilder::default();
		macro_rules! set {
			($opts: ident, $check: expr, $method: ident, $val: expr) => {
				if o.$opts.is_some() && $check {
					builder.$method($val);
				}
			};
			($opts: ident, 0) => {
				set!($opts, true, $opts, o.$opts)
			};
			($opts: ident, 1) => {
				println!("setting option: {}", stringify!($opts));
				set!($opts, true, $opts, o.$opts.unwrap())
			};
		}
		builder.selector_icon("█".to_string());
		builder.multi_select_icon("█".to_string());
		builder.color(Some("molokai".to_string()));
		set!(cmd, 0);
		set!(multi, 1);
		set!(preview, 0);
		set!(
			ansi,
			o.ansi.unwrap(),
			cmd_collector,
			Rc::new(RefCell::new(SkimItemReader::new(
				SkimItemReaderOption::default().ansi(true),
			)))
		);
		return builder;
	}
}
