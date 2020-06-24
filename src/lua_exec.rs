
use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use rlua::Lua;
use std::path::Path;

fn load_code(str_path: &str) -> Vec<u8> {
    let file_path = Path::new(str_path);
    let f = File::open(&file_path).unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).unwrap();
    //let s = ::std::str::from_utf8(&buf).unwrap();
    buf
}


pub fn exec() -> Result<(), rlua::Error> {
    // let lua_file = Path::new("./lua_code/monster.lua");
    let monster_file_str_path = "./lua_code/monster.lua";
    // let weapon_file_str_path = "./lua_code/weapon.lua";

    let lua = Lua::new();
    // let plus_fn = eval(&lua, &lua_file, Some("Weapon")).unwrap();

    let monster_vec = load_code(monster_file_str_path);
    let monster_code = std::str::from_utf8(&monster_vec).unwrap();

    // let weapon_vec = load_code(weapon_file_str_path);
    // let weapon_code = std::str::from_utf8(&weapon_vec).unwrap();

    // let s = ::std::str::from_utf8(&buf).unwrap();
    // println!("s: {:?}", s);
    lua.context(|lua_context| {
        lua_context.load(monster_code).exec()
    })?;

    Ok(())
}