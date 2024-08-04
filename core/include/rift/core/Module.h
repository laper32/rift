#ifndef RIFT_CORE_MODULE_H
#define RIFT_CORE_MODULE_H

#include <rift/core/Definitions.h>


/**
 * @brief Interface for modules, user for expand functionality of core.
 * All modules must derive this class, otherwise will not work.
 */
// ReSharper disable once CppClassCanBeFinal
class IModule
{
public:
    virtual ~IModule() = default;
    virtual bool OnLoad() { return true; }
    virtual void OnAllLoaded() {}
    virtual void OnUnload() {}
};

#endif // !RIFT_CORE_MODULE_H
