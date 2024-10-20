#include "rift/bridge/InteropService.h"

#include "rift/bridge/natives/CoreNatives.h"

#include "rift/fundamental/String.h"
#include "rift/tier1/CUtlVector.h"

#include <print>


namespace rift::bridge {


namespace {
CUtlVector<RuntimeNative*> g_RuntimeNatives;
}

void CreateNativeInternal(const char* name, void* func)
{
    const auto item = new RuntimeNative();
    StrCopy(item->name, sizeof(item->name), name);
    item->func = func;
    g_RuntimeNatives.AddToTail(item);
}

void* GetNatives() { return &g_RuntimeNatives; }

namespace natives {

void ExampleNative() { std::println("rift::bridge::natives::ExampleNative invoked."); }

void Init() { CreateNative("Core.ExampleNative", ExampleNative); }

} // namespace natives

void InitNatives()
{
    natives::Init();
    core::natives::Init();
}

} // namespace rift::bridge
