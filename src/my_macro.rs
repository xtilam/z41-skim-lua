#[macro_export]
macro_rules! shield {
	($e:expr, $act: tt) => {
		match $e {
			Some(v) => v,
			None => $act,
		}
	};
}

#[macro_export]
macro_rules! lua_rs {
	($e:expr) => {
		$e.map_err(|err| mlua::Error::RuntimeError(err.to_string()))
	};
}

#[macro_export]
macro_rules! lua_err {
	($error: expr) => {
		mlua::Error::RuntimeError($error.to_string())
	};
}

#[macro_export]
macro_rules! lua_bridge {
    ($trait_name:ident, $struct_name:ident, { $($methods:tt)* }) => {
        pub trait $trait_name {
            lua_bridge!(@gen_trait $($methods)*);
        }

        impl mlua::UserData for $struct_name {
            fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
                lua_bridge!(@gen_methods methods; $($methods)*);
            }
        }
    };

    // --- Trait Generation ---
    (@gen_trait $name:ident : [ $($arg_name:ident : $arg_type:ty),* ] => $ret:ty , $($rest:tt)*) => {
        fn $name(&mut self, lua: &mlua::Lua, $($arg_name : $arg_type),*) -> mlua::Result<$ret>;
        lua_bridge!(@gen_trait $($rest)*);
    };
    (@gen_trait $name:ident : [ $($arg_name:ident : $arg_type:ty),* ] => $ret:ty $(,)?) => {
        fn $name(&mut self, lua: &mlua::Lua, $($arg_name : $arg_type),*) -> mlua::Result<$ret>;
    };
    (@gen_trait) => {};

    // --- Method Generation (UserData) ---
    (@gen_methods $m:ident ; $name:ident : [ $($arg_name:ident : $arg_type:ty),* ] => $ret:ty , $($rest:tt)*) => {
        lua_bridge!(@gen_method_add $m, $name ($($arg_name : $arg_type),*));
        lua_bridge!(@gen_methods $m ; $($rest)*);
    };
    (@gen_methods $m:ident ; $name:ident : [ $($arg_name:ident : $arg_type:ty),* ] => $ret:ty $(,)?) => {
        lua_bridge!(@gen_method_add $m, $name ($($arg_name : $arg_type),*));
    };
    (@gen_methods $m:ident ;) => {};

    // Helper add_method_mut: Bóc Tuple 'a' thành các tham số rời rạc
    (@gen_method_add $m:ident, $name:ident ( $($arg_name:ident : $arg_type:ty),* )) => {
        $m.add_method_mut(stringify!($name), |lua, this, ($($arg_name,)*): ($($arg_type,)*)| {
            this.$name(lua, $($arg_name),*)
        });
    };
}
