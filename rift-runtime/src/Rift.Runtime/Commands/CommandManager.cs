﻿// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using System.Runtime.InteropServices;
using System.Text;
using System.Text.Json;
using System.Text.Json.Serialization;
using Rift.Runtime.API.Commands;
using Rift.Runtime.API.Fundamental;
using Rift.Runtime.Task;

namespace Rift.Runtime.Commands;

internal class CommandManagerInternal : CommandManager, IInitializable
{
    private readonly List<UserCommand> _commands = [];
    private readonly List<UserCommand> _pendingLoadCommands = [];
    private readonly List<UserCommand> _pendingRemoveChildCommands = [];

    public new static CommandManagerInternal Instance { get; private set; } = null!;

    public override void CallOnce()
    {
        Console.WriteLine("Command.Call.Once");
    }

    public CommandManagerInternal()
    {
        Instance = this;
    }

    public bool Init()
    {
        return true;
    }

    public void Shutdown()
    {
    }

    public List<UserCommand> GetUserCommands()
    {
        var userDefinedCommands = TaskManagerInternal.Instance.GetUserDefinedCommands();

        TransformToUserCommands(userDefinedCommands);
        AddSubcommands(userDefinedCommands);
        LinkParentCommands(userDefinedCommands);

        _pendingRemoveChildCommands.ForEach(x =>
        {
            _pendingLoadCommands.Remove(x);
        });

        MoveToCommands();
        return _commands;
    }

    private void TransformToUserCommands(List<UserDefinedCommand> userDefinedCommands)
    {
        userDefinedCommands.ForEach(udc =>
        {
            var command = new UserCommand(udc.Name, udc.PackageName)
            {
                About      = udc.About,
                BeforeHelp = udc.BeforeHelp,
                AfterHelp  = udc.AfterHelp
            };
            if (udc.Args.Count > 0)
            {
                var args = new List<UserCommandArg>();

                udc.Args.ForEach(arg =>
                {
                    var userCommandArg = new UserCommandArg(arg.Name)
                    {
                        Short = arg.Short,
                        Description = arg.Description,
                        Default = arg.Default,
                        ConflictWith = arg.ConflictWith,
                        Heading = arg.Heading
                    };
                    args.Add(userCommandArg);
                });

                command.Args = args;
            }

            // 这个阶段不能处理subcommand。

            _pendingLoadCommands.Add(command);
        });
    }

    private void AddSubcommands(List<UserDefinedCommand> userDefinedCommands)
    {
        var pendingRemoveCommands = new List<UserCommand>();
        userDefinedCommands.ForEach(udc =>
        {
            if (udc.Subcommands.Count == 0)
            {
                return;
            }

            if (_pendingLoadCommands.Find(x => x.Name.Equals(udc.Name, StringComparison.OrdinalIgnoreCase)) is not
                { } userCommand)
            {
                return;
            }

            userCommand.Subcommands ??= [];
            udc.Subcommands.ForEach(subCommandStr =>
            {
                if (_pendingLoadCommands.Find(x => x.Name.Equals(subCommandStr, StringComparison.OrdinalIgnoreCase)) is
                    not { } subCommand)
                {
                    return;
                }
                userCommand.Subcommands.Add(subCommand);
                pendingRemoveCommands.Add(subCommand);
            });
        });

        pendingRemoveCommands.ForEach(x =>
        {
            _pendingLoadCommands.Remove(x);
        });
    }

    private void LinkParentCommands(IEnumerable<UserDefinedCommand> userDefinedCommands)
    {
        var allParentCommands = userDefinedCommands.Where(x => x.Parent is not null).ToArray();

        foreach (var userDefinedCommand in allParentCommands)
        {
            var parentCommand = userDefinedCommand.Parent!;

            var selectedCommand = _pendingLoadCommands.Find(x =>
                x.Name.Equals(userDefinedCommand.Name, StringComparison.OrdinalIgnoreCase));

            SetChildCommandToParent(parentCommand, selectedCommand);
        }
    }

    private void SetChildCommandToParent(string parentCommandStr, UserCommand? selectedCommand)
    {
        if (selectedCommand is null)
        {
            return;
        }

        if (_pendingLoadCommands.Find(x => x.Name.Equals(parentCommandStr, StringComparison.OrdinalIgnoreCase)) is
            { } parentCommand)
        {
            parentCommand.Subcommands ??=
            [
                selectedCommand
            ];
            _pendingRemoveChildCommands.Add(selectedCommand);
        }
        else
        {
            SearchNestedCommands(parentCommandStr, selectedCommand);
        }

    }

    private void SearchNestedCommands(string parentCommandStr, UserCommand selectedCommand)
    {
        _pendingLoadCommands.ForEach(cmd =>
        {
            cmd.Subcommands?.ForEach(x =>
            {
                if (x.Name.Equals(parentCommandStr, StringComparison.OrdinalIgnoreCase))
                {
                    x.Subcommands ??= [selectedCommand];
                    _pendingRemoveChildCommands.Add(selectedCommand);
                }
                else
                {
                    SearchNestedCommands(parentCommandStr, selectedCommand);
                }
            });
        });
    }

    private void MoveToCommands()
    {
        _commands.AddRange(_pendingLoadCommands);
    }

    [UnmanagedCallersOnly]
    public static unsafe sbyte* GetUserCommandsExport()
    {
        var commands = Instance.GetUserCommands();
        var commandsStr = JsonSerializer.Serialize(commands, new JsonSerializerOptions
        {
            PropertyNamingPolicy = JsonNamingPolicy.SnakeCaseLower,
            Converters =
            {
                new JsonStringEnumConverter<ArgAction>(JsonNamingPolicy.SnakeCaseLower)
            }
        });

        var bytes = Encoding.UTF8.GetBytes(commandsStr);
        var sBytes = Array.ConvertAll(bytes, Convert.ToSByte);

        fixed (sbyte* p = sBytes)
        {
            return p;
        }
    }
}