using System.Runtime.InteropServices;

namespace Rift.Runtime.Fundamental.Tier1;

// ReSharper disable ConvertToAutoPropertyWhenPossible
// ReSharper disable ConvertToAutoPropertyWithPrivateSetter
// ReSharper disable UnusedMember.Global

[StructLayout(LayoutKind.Sequential)]
public unsafe struct CUtlMemory<T> : IDisposable
    where T : unmanaged
{
    // For memory alignment. MUST NOT MODIFY IT!
    private T* _memory;

    // For memory alignment. MUST NOT MODIFY IT!
    private int _allocationCount;

    // For memory alignment. MUST NOT MODIFY IT!
    private int _growSize;

    private const int ExternalConstBufferMarker = 1 << 30;
    private const int ExternalBufferMarker = 1 << 31;

    public int AllocationCount => _allocationCount;
    public bool IsReadOnly => (_growSize & ExternalConstBufferMarker) != 0;
    public bool IsExternallyAllocated => (_growSize & (ExternalConstBufferMarker | ExternalBufferMarker)) != 0;

    public CUtlMemory(int growSize, int initAllocationCount)
    {
        _allocationCount = initAllocationCount;
        _growSize = growSize & ~(ExternalConstBufferMarker | ExternalBufferMarker);

        if (_allocationCount > 0)
        {
            _memory = (T*)NativeMemory.Alloc((nuint)(_allocationCount * sizeof(T)));
        }
    }

    public void Dispose()
    {
        NativeMemory.Free(_memory);
    }

    public ref T this[long index] => ref _memory[index];

    private int CalcNewAllocationCount(int nAllocationCount, int nGrowSize, int nNewSize, int nBytesItem)
    {
        if (nGrowSize > 0)
        {
            nAllocationCount = (1 + (nNewSize - 1) / nGrowSize) * nGrowSize;
        }
        else
        {
            if (nAllocationCount == 0)
            {
                // Compute an allocation which is at least as big as a cache line...
                nAllocationCount = (31 + nBytesItem) / nBytesItem;

                // If the requested amount is larger then compute an allocation which
                // is exactly the right size. Otherwise we can end up with wasted memory
                // when CUtlVector::EnsureCount(n) is called.
                if (nAllocationCount < nNewSize)
                {
                    nAllocationCount = nNewSize;
                }
            }

            while (nAllocationCount < nNewSize)
            {
                var nNewAllocationCount = nAllocationCount * 9 / 8; // 12.5 %

                if (nNewAllocationCount > nAllocationCount)
                {
                    nAllocationCount = nNewAllocationCount;
                }
                else
                {
                    nAllocationCount *= 2;
                }
            }
        }

        return nAllocationCount;
    }

    public void Grow(int num)
    {
        if (num <= 0)
        {
            throw new ArgumentOutOfRangeException(nameof(num));
        }

        if (IsReadOnly)
        {
            throw new InvalidOperationException("Cannot grow a read-only buffer");
        }

        if ((long)_allocationCount + num > int.MaxValue)
        {
            throw new OutOfMemoryException("Cannot allocate more than 2GB of memory");
        }

        // Make sure we have at least numallocated + num allocations.
        // Use the grow rules specified for this memory (in m_nGrowSize)
        var nAllocationRequested = _allocationCount + num;

        var nNewAllocationCount = CalcNewAllocationCount(_allocationCount,
                                                         _growSize
                                                         & ~(ExternalConstBufferMarker | ExternalBufferMarker),
                                                         nAllocationRequested,
                                                         sizeof(T));

        _allocationCount = nNewAllocationCount;

        if (_memory is not null)
        {
            _memory = (T*)NativeMemory.Realloc(_memory, (nuint)(_allocationCount * sizeof(T)));
        }
        else
        {
            _memory = (T*)NativeMemory.Alloc((nuint)(_allocationCount * sizeof(T)));
        }

        if (_memory == null)
        {
            throw new OutOfMemoryException("Failed to allocate memory");
        }
    }
}
