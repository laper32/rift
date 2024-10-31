// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

#ifndef RIFT_ENGINE_TYPES_H
#define RIFT_ENGINE_TYPES_H

#include <type_traits>

namespace rift {

template <typename T>
concept PrimitiveChar_c = std::is_same_v<T, char> || std::is_same_v<T, wchar_t>;

template <PrimitiveChar_c Char, size_t I>
using PrimitiveChar_t = Char[I];

}; // namespace rift


#endif // !RIFT_ENGINE_TYPES_H
