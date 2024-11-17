// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

#include "rift/fundamental/PrimitivePath.h"

#ifndef WIN32_LEAN_AND_MEAN
#define WIN32_LEAN_AND_MEAN
#endif

#if _WIN32
#include <windows.h>
#endif

namespace rift {

std::filesystem::path GetExecutablePath()
{
#if defined(_MSC_VER)
    wchar_t path[FILENAME_MAX] = {0};
    GetModuleFileNameW(nullptr, path, FILENAME_MAX);
    return {path};
#else
    char path[FILENAME_MAX];
    ssize_t count = readlink("/proc/self/exe", path, FILENAME_MAX);
    return std::filesystem::path(std::string(path, (count > 0) ? count : 0));
#endif
}

} // namespace rift
