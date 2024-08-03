#ifndef RIFT_CORE_MODULE_H
#define RIFT_CORE_MODULE_H

#include <rift/core/Definitions.h>

#include <iostream>

namespace rift {

class IModule
{
public:
    virtual bool OnLoad() { return true; }
    virtual void OnAllLoaded() {}
    virtual void OnUnload() {}
};

} // namespace rift

#endif // !RIFT_CORE_MODULE_H
