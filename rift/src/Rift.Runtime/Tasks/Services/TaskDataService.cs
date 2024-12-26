namespace Rift.Runtime.Tasks.Services;

internal class TaskDataService
{
    private readonly Dictionary<Type, object> _data = [];

    public TData Get<TData>() where TData : class
    {
        if (!_data.TryGetValue(typeof(TData), out var data))
        {
            throw new InvalidOperationException("The context data has not been initialized.");
        }

        if (data is TData typedData)
        {
            return typedData;
        }

        throw new InvalidOperationException(
            $"Context data exists but is of the wrong type ({data.GetType().FullName}).");
    }

    public void Add<TData>(TData data) where TData : class
    {
        if (_data.ContainsKey(typeof(TData)))
        {
            throw new InvalidOperationException(
                $"Context data of type '{typeof(TData).FullName}' has already been registered."
            );
        }

        _data.Add(typeof(TData), data);
    }
}