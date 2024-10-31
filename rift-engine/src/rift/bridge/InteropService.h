// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

#ifndef RIFT_ENGINE_BRIDGE_INTEROP_SERVICE_H
#define RIFT_ENGINE_BRIDGE_INTEROP_SERVICE_H

#include "rift/fundamental/Char.h"

namespace rift::bridge {


struct RuntimeNative
{
    PrimitiveChar_t<char, 128> name;
    void* func;
};

template<typename F>
void CreateNative(const char* name, F func)
{
    extern void CreateNativeInternal(const char*, void*);
    CreateNativeInternal(name, reinterpret_cast<void*>(func));
}

void* GetNatives();

void InitNatives();

} // namespace rift::bridge

#endif
