using System.Collections;

namespace Rift.Runtime.Tasks;

public interface ITaskReport
{
    void Add(ITaskReportRecipe recipe);
}


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