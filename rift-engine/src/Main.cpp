// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

#include "rift.engine.export.h"

namespace rift {
extern bool Init();
extern void Load();
extern void Shutdown();
extern const char* RuntimeGetUserCommands();
} // namespace rift

extern "C" {

RIFT_API bool RiftEngineInit() { return rift::Init(); }

RIFT_API void RiftEngineShutdown() { return rift::Shutdown(); }

RIFT_API void RiftEngineLoad() { return rift::Load(); }

RIFT_API const char* RiftEngineGetUserCommands() { return rift::RuntimeGetUserCommands(); }
}
