#ifndef RIFT_ENGINE_BRIDGE_ADAPTER_H
#define RIFT_ENGINE_BRIDGE_ADAPTER_H
#include "rift/fundamental/Char.h"

namespace rift::bridge {


struct RuntimeNative
{
    PrimitiveChar_t<char, 128> name;
    void* func;
};

void CreateNative(const char* name, void* func);

void* GetNatives();

namespace natives {
void InitNatives();
}

} // namespace rift::bridge

#endif
