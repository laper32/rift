#include "rift/bridge/Adapter.h"
#include "rift/coreclr/CoreCLR.h"

#include <print>

#include "coreclr/CoreCLRDelegates.h"


namespace rift {


bool Init()
{
    coreclr::Init();

    bridge::natives::InitNatives();

    const auto clr = !!coreclr::Bootstrap(bridge::GetNatives());

    std::println("{}", clr);

    return clr;
}

void Shutdown()
{
    coreclr::Shutdown();
    std::println("Rift.Runtime.Shutdown");
}

const char* RuntimeGetTasks()
{
    using WorkspaceManagerGetTasksFn_t = const char* (CORECLR_DELEGATE_CALLTYPE*)();
    const auto get_tasks = coreclr::GetManagedFunction<WorkspaceManagerGetTasksFn_t>(
        "Managers.WorkspaceManager.GetTasks");

    return get_tasks();
}

} // namespace rift
