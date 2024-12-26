// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.Tasks.Fundamental;
using System.Collections;

namespace Rift.Runtime.Tasks.Reporting;

public class TaskReport : IEnumerable<TaskReportRecipe>
{
    private readonly List<TaskReportRecipe> _reports = [];


    public IEnumerator<TaskReportRecipe> GetEnumerator()
    {
        return _reports.GetEnumerator();
    }

    IEnumerator IEnumerable.GetEnumerator()
    {
        return GetEnumerator();
    }


    public void Add(TaskReportRecipe recipe)
    {
        _reports.Add(recipe);
    }

    public void Add(string taskName, TimeSpan elapsed)
    {
        Add(new TaskReportRecipe(taskName, string.Empty, elapsed, RiftTaskExecutionStatus.Executed));
    }

    public void AddFailed(string taskName, TimeSpan elapsed)
    {
        Add(new TaskReportRecipe(taskName, string.Empty, elapsed, RiftTaskExecutionStatus.Failed));
    }
}