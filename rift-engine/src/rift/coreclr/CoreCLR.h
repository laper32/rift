#ifndef RIFT_ENGINE_CORECLR_H
#define RIFT_ENGINE_CORECLR_H

namespace rift::coreclr {
int  Bootstrap(void* natives);
void Shutdown();
bool Init();
// bool  CreateNative(const char* name, void* func);
void* GetManagedFunction(const char* name);
template <typename T>
T GetManagedFunction(const char* name)
{
    return reinterpret_cast<T>(GetManagedFunction(name));
}

} // namespace rift

#endif // !RIFT_ENGINE_CORECLR_H
