// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

#include "rift/bridge/InteropService.h"
#include "rift/coreclr/CoreCLR.h"

#include "coreclr/CoreCLRDelegates.h"


namespace rift {


bool Init()
{
    coreclr::Init();

    bridge::InitNatives();

    const auto clr = !!coreclr::Bootstrap(bridge::GetNatives());

    return clr;
}

void Shutdown()
{
    coreclr::Shutdown();
}

const char* RuntimeGetTasks()
{
    using WorkspaceManagerGetTasksFn_t = const char*(CORECLR_DELEGATE_CALLTYPE*)();
    const auto get_tasks =
        coreclr::GetManagedFunction<WorkspaceManagerGetTasksFn_t>("Managers.WorkspaceManager.GetTasks");

    return get_tasks();
}

} // namespace rift
