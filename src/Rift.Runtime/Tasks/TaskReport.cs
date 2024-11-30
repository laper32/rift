using System.Collections;
using Rift.Runtime.Abstractions.Tasks;

namespace Rift.Runtime.Tasks;

public class TaskReport : ITaskReport, IEnumerable<ITaskReportRecipe>
{
    private readonly List<ITaskReportRecipe> _reports = [];


    public void Add(ITaskReportRecipe recipe)
    {
        _reports.Add(recipe);
    }


    public IEnumerator<ITaskReportRecipe> GetEnumerator()
    {
        return _reports.GetEnumerator();
    }

    IEnumerator IEnumerable.GetEnumerator()
    {
        return GetEnumerator();
    }
}