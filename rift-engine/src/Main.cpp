#include "rift.engine.export.h"

namespace rift {
extern bool Init();
extern void Shutdown();
} // namespace rift

extern "C" {

RIFT_API bool RiftEngineInit() { return rift::Init(); }

RIFT_API void RiftEngineShutdown() { return rift::Shutdown(); }
}
