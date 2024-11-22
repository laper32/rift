// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

#include "rift/bridge/InteropService.h"

#include "rift/coreclr/CoreCLR.h"
#include "rift/coreclr/CoreCLRDelegates.h"

namespace rift {


bool Init()
{
    coreclr::Init();

    bridge::InitNatives();

    const auto clr = !!coreclr::Bootstrap(bridge::GetNatives());

    return clr;
}

void Shutdown() { coreclr::Shutdown(); }

const char* RuntimeGetUserCommands()
{
    using CommandManagerGetUserCommandsFn_t = const char*(CORECLR_DELEGATE_CALLTYPE*)();
    const auto get_user_commands = coreclr::GetRuntimeFunction<CommandManagerGetUserCommandsFn_t>(
        "Commands.CommandManagerInternal.GetUserCommands");
    return get_user_commands();
}

void RuntimeProcessUserCommand(const char* serializedStr)
{
    using CommandManagerProcessUserCommandFn_t = void(CORECLR_DELEGATE_CALLTYPE*)(const char*);
    const auto process_user_commands = coreclr::GetRuntimeFunction<CommandManagerProcessUserCommandFn_t>("Commands.CommandManagerInternal.ProcessUserCommand");
    process_user_commands(serializedStr);
}

void Load() { coreclr::Load(); }

} // namespace rift
