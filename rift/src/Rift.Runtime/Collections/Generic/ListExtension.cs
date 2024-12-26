namespace Rift.Runtime.Collections.Generic;

public static class ListExtension
{
    public static void ForEachCatchException<T>(
        this IReadOnlyCollection<T> list,
        Action<T>                   action,
        Action<Exception>           ex)
    {
        foreach (var item in list)
        {
            try
            {
                action(item);
            }
            catch (Exception e)
            {
                ex(e);
            }
        }
    }

    public static void ForEachIgnore<T>(this IReadOnlyCollection<T> list, Action<T> action)
    {
        foreach (var item in list)
        {
            action(item);
        }
    }
}