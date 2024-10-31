// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

#include "rift/bridge/natives/CoreNatives.h"

#include "rift/bridge/InteropService.h"

#include "rift/fundamental/PrimitivePath.h"
#include "rift/fundamental/String.h"

namespace rift::bridge::core::natives {

const char* ExecutablePath()
{
#if _WIN32
    const auto executable_path = GetExecutablePath();
    const auto narrowed_executable_path = Narrow(executable_path.c_str());

    auto ret = std::make_unique<char[]>(narrowed_executable_path.length() + 1);
    narrowed_executable_path.copy(ret.get(), narrowed_executable_path.length());
    return ret.release();

#else
#error Not Implemented.
#endif
}

void Init() { CreateNative("Core.GetExecutablePath", ExecutablePath); }

}

// namespace rift::bridge::core::natives
