using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;

namespace Rift.Runtime.Tier1;


[StructLayout(LayoutKind.Sequential)]
public unsafe struct CUtlVector<T> : IDisposable where T : unmanaged
{
    private int           _size;
    private CUtlMemory<T> _memory;
    private T* _elements;

    public CUtlVector(int growSize = 0, int initSize = 0)
        => _memory = new CUtlMemory<T>(growSize, initSize);

    public void Dispose()
    {
        _memory.Dispose();
    }

    public ref T this[long index] => ref _memory[index];

    public IEnumerator<T> GetEnumerator()
    {
        for (var i = 0; i < _size; i++)
        {
            yield return this[i];
        }
    }

    private void GrowVector(int num)
    {
        if (_size + num > _memory.AllocationCount)
        {
            _memory.Grow((_size + num) - _memory.AllocationCount);
        }

        _size += num;
    }

    public void Add(T item)
        => Add(ref item);

    public void Add(ref readonly T item)
        => Insert(_size, in item);

    private void Insert(int index, T item)
        => Insert(index, ref item);

    private void Insert(int index, ref readonly T item)
    {
        if (index < 0 || index > _size)
        {
            throw new IndexOutOfRangeException();
        }

        GrowVector(1);
        ShiftElementsRight(index);
        _memory[index] = item;
    }

    private void ShiftElementsRight(int index, int num = 1)
    {
        if (_size == 0)
        {
            throw new Exception();
        }

        if (num == 0)
        {
            throw new Exception();
        }

        var numToMove = _size - index - num;

        if (numToMove > 0 && num > 0)
        {
            NativeMemory.Copy(Unsafe.AsPointer(ref _memory[index]),
                Unsafe.AsPointer(ref _memory[index + num]),
                (nuint) (numToMove * sizeof(T)));
        }
    }

    public int Count => _size;
}
