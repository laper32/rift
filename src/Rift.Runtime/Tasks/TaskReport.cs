// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Collections;

namespace Rift.Runtime.Tasks;


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
}