// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

#ifndef RIFT_ENGINE_CORECLR_H
#define RIFT_ENGINE_CORECLR_H

namespace rift::coreclr {
int  Bootstrap(void* natives);
void Shutdown();
void Load();
bool Init();

void* GetRuntimeFunction(const char* fn_name);
template <typename T>
T GetRuntimeFunction(const char* name)
{
    return reinterpret_cast<T>(GetRuntimeFunction(name));
}

} // namespace rift

#endif // !RIFT_ENGINE_CORECLR_H
