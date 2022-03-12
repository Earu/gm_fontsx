#![feature(c_unwind)]

#[macro_use]
extern crate gmod;
extern crate font_loader as fonts;

use fonts::system_fonts;

#[lua_function]
unsafe fn get_installed_fonts(lua: gmod::lua::State) -> i32 {
	let sys_fonts = system_fonts::query_all();

	lua.new_table();
	for (i, font) in sys_fonts.iter().enumerate() {
		lua.push_integer((i + 1) as isize);
		lua.push_string(font.as_str());
		lua.set_table(-3);
	}

	1
}

#[lua_function]
unsafe fn font_exists(lua: gmod::lua::State) -> i32 {
	let sys_fonts = system_fonts::query_all();
	let input = lua.check_string(1);
	let res = sys_fonts.contains(&String::from(input.as_ref()));

	lua.push_boolean(res);
	1
}


#[gmod13_open]
unsafe fn gmod13_open(lua: gmod::lua::State) -> i32 {
	lua.new_table();

	lua.push_function(get_installed_fonts);
	lua.set_field(-2, lua_string!("GetAll"));

	lua.push_function(font_exists);
	lua.set_field(-2, lua_string!("Exists"));

	lua.set_global(lua_string!("fonts"));
    0
}

#[gmod13_close]
unsafe fn gmod13_close(_: gmod::lua::State) -> i32 {
    0
}