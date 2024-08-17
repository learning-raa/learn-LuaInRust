use std::fs;
use clap::Parser;
use mlua::prelude::*;
use mlua::Function;

fn main() {
    println!("parser intro..");
    {
        let args = CliArgs::parse();
        println!("..after parsing..");

        println!("\nlevel -> {}", args.level);
        let level_lua_code = fs::read_to_string(args.level).
            expect("impossible to open lua level file");

        let lua = Lua::new();
        match work_lua(&lua, &level_lua_code) {
            Ok(()) => println!("...Ok!"),
            Err(e) => {
                eprintln!("Lua: {}", e);
            },
        }
    }
}

#[derive(Parser)]
#[command(version, about)]
struct CliArgs {
    #[arg(short,long, default_value = "demo_level.lua")]
    level: String,
}



// // // // // // // //
fn work_lua( lua: &Lua, level_lua_code: &str ) -> mlua::Result<()> {
    let globals = lua.globals();
    globals.set("alfa", "omega")?;
    let rst_fn = lua.create_function( |_, ()| {
        println!("from LUAAAAAAAA");
        Ok(())
    })?;
    globals.set("invokeRust", rst_fn)?;

    lua.load( level_lua_code ).exec()?;
    let lua_b: String = globals.get("b")?;
    println!("lua B = {}", lua_b);

    let call_from_rust: Function = globals.get("fromRust")?;
    call_from_rust.call::<_, ()>(())?;

    let call_from_rust_2: Function = globals.get("fromRust2")?;
    let res: u32 = call_from_rust_2.call::<_, u32>("MegaKey")?;
    println!("from lua u32 = {}", res);

    Ok(())
}

