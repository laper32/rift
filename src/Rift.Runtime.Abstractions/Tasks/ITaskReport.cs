namespace Rift.Runtime.Abstractions.Tasks;

public interface ITaskReport
{
    void Add(ITaskReportRecipe recipe);
}
