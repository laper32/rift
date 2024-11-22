// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

mod command;
mod de;
mod engine;
mod errors;

use command::CommandManager;
use errors::RiftResult;

fn main() -> RiftResult<()> {
    engine::init();
    /*
    绝大多数的指令必须加载workspace以后才能生效，而有些指令应当是无论如何都会生效的（如：new, init）。
    而clap的问题是，不支持conditional command。。。
     */
    engine::load();
    CommandManager::instance().get_user_commands();
    // CommandManager::instance().dump_user_commands();

    let _ = CommandManager::instance().exec_command();

    engine::shutdown();
    return Ok(());
}
