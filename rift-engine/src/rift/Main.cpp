#include "rift/bridge/Adapter.h"
#include "rift/coreclr/CoreCLR.h"

#include <print>

namespace rift {
bool Init()
{
    coreclr::Init();

    bridge::natives::InitNatives();

    const auto clr = !!coreclr::Bootstrap(bridge::GetNatives());

    return clr;
}

void Shutdown()
{
    coreclr::Shutdown();
    std::println("Rift.Runtime.Shutdown");
}
} // namespace rift
