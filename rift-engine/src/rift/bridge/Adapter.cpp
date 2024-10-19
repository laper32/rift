#include "rift/bridge/Adapter.h"

#include "rift/tier1/CUtlVector.h"

#include <print>


namespace rift::bridge {


namespace {
CUtlVector<RuntimeNative*> g_RuntimeNatives;
}

inline size_t StrCopy(char* dst, size_t dsize, const char* src)
{
    const char* osrc = src;
    size_t nleft = dsize;

    /* Copy as many bytes as will fit. */
    if (nleft != 0)
    {
        while (--nleft != 0)
        {
            if ((*dst++ = *src++) == '\0')
                break;
        }
    }

    /* Not enough room in dst, add NUL and traverse rest of src. */
    if (nleft == 0)
    {
        if (dsize != 0)
            *dst = '\0'; /* NUL-terminate dst */
        while (*src++)
            ;
    }

    return (src - osrc - 1); /* count does not include NUL */
}

void CreateNative(const char* name, void* func)
{
    const auto item = new RuntimeNative();
    StrCopy(item->name, sizeof(item->name), name);
    item->func = func;
    g_RuntimeNatives.AddToTail(item);
}

void* GetNatives() { return &g_RuntimeNatives; }


namespace natives {

void ExampleNative()
{
    std::println("rift::bridge::natives::ExampleNative invoked.");

}

void InitNatives()
{
    CreateNative("Core.ExampleNative", reinterpret_cast<void*>(ExampleNative));
}

} // namespace natives

} // namespace rift::bridge
