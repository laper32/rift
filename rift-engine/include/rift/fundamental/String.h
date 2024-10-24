#ifndef RIFT_ENGINE_FUNDAMENTAL_STRING_H
#define RIFT_ENGINE_FUNDAMENTAL_STRING_H

#include <sstream>
#include <string>

namespace rift {

inline std::wstring Widen(const std::string& str)
{
    std::wostringstream ret;
    const auto& facet = std::use_facet<std::ctype<wchar_t>>(ret.getloc());
    for (const char i : str)
    {
        ret << facet.widen(i);
    }
    return ret.str();
}

inline std::string Narrow(const std::wstring& str)
{
    std::ostringstream stm;

    const auto& facet = std::use_facet<std::ctype<wchar_t>>(stm.getloc());

    for (const wchar_t i : str)
    {
        stm << facet.narrow(i, 0);
    }
    return stm.str();
}

inline size_t StrCopy(char* dst, const size_t dst_size, const char* src)
{
    const char* original_src = src;
    size_t number_left = dst_size;

    /* Copy as many bytes as will fit. */
    if (number_left != 0)
    {
        while (--number_left != 0)
        {
            if ((*dst++ = *src++) == '\0')
                break;
        }
    }

    /* Not enough room in dst, add NUL and traverse rest of src. */
    if (number_left == 0)
    {
        if (dst_size != 0)
            *dst = '\0'; /* NUL-terminate dst */
        while (*src++);
    }

    return src - original_src - 1; /* count does not include NUL */
}

} // namespace rift

#endif // !RIFT_ENGINE_FUNDAMENTAL_STRING_H
