use std::fs;
use clap::Parser;
use mlua::prelude::*;

fn main() {
    println!("parser intro..");
    {
        let args = CliArgs::parse();
        println!("..after parsing..");

        println!("\nlevel -> {}", args.level);
        let level_lua_code = fs::read_to_string(args.level).
            expect("impossible to open lua level file");

        let lua = Lua::new();
        lua.globals().set("alfa", "omega");
        let res = lua.load( level_lua_code ).exec();
        match res {
            Ok(()) => println!("...Ok..."),
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




        //println!("{} -> {}", args.commandName, args.zalue);
        //println!("{} -> {}", args.uflA, args.wflBCDe);
        //println!("mOtp -> {}", args.mOptCam );

    // //#[arg(short,long)]
    //commandName: String,
    //#[arg(short,long, default_value_t = 0)]
    //zalue: u8,
    //#[arg(short,long, default_value_t = false)]
    //uflA: bool,
    //#[arg(short,long, default_value_t = false)]
    //wflBCDe: bool,
    //#[arg(short,long, default_value_t = false)]
    //mOptCam: bool,
