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

void Shutdown()
{
    coreclr::Shutdown();
}

const char* RuntimeGetTasks()
{
    using TaskManagerGetTasksFn_t = const char*(CORECLR_DELEGATE_CALLTYPE*)();
    const auto get_tasks =
        coreclr::GetRuntimeFunction<TaskManagerGetTasksFn_t>("Task.TaskManager.GetTasks");

    return get_tasks();
}

void Load()
{
    coreclr::Load();
}

} // namespace rift
