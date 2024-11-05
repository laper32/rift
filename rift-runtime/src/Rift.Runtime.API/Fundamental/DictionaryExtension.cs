namespace Rift.Runtime.API.Fundamental;

public static class DictionaryExtension
{
    public static void ForEach<TKey, TValue>(this IDictionary<TKey, TValue> self, Action<TKey, TValue> predicate)
    {
        foreach (var (key, value) in self)
        {
            predicate(key, value);
        }
    }

    public static void ForEach<TKey, TValue>(this IDictionary<TKey, TValue> self,
        Action<KeyValuePair<TKey, TValue>> predicate)
    {
        foreach (var value in self)
        {
            predicate(value);
        }
    }
}