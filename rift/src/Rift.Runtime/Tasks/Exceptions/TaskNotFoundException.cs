// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.Tasks.Exceptions;

public class TaskNotFoundException(string message = "") : Exception(message);