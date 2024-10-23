﻿// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

namespace Rift.Runtime.API.Manifest;

public record FolderManifest(
    string? Name,
    List<string>? Members,
    List<string>? Excludes
    );