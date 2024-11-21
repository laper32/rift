// ===========================================================================
// Rift
// Copyright (C) 2024 - Present laper32.
// All Rights Reserved
// ===========================================================================

using Rift.Runtime.API.Commands;
using Rift.Runtime.API.Fundamental;
using Rift.Runtime.Task;

namespace Rift.Runtime.Commands;

internal class CommandManagerInternal : CommandManager, IInitializable
{
    private readonly  List<UserCommand> _commands            = [];
    private readonly  List<UserCommand> _pendingLoadCommands = [];


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

    public void ToUserCommands()
    {
        var userDefinedCommands = TaskManagerInternal.Instance.GetUserDefinedCommands();
        TransformToUserCommands(userDefinedCommands);
        AddSubcommands(userDefinedCommands);
        LinkParentCommands(userDefinedCommands);
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

            userCommand.Subcommands = new List<UserCommand>();
            udc.Subcommands.ForEach(subCommandStr =>
            {
                if (_pendingLoadCommands.Find(x => x.Name.Equals(subCommandStr, StringComparison.OrdinalIgnoreCase)) is
                    not { } subCommand)
                {
                    return;
                }
                userCommand.Subcommands.Add(subCommand);

                _pendingLoadCommands.Remove(subCommand);
            });
        });
    }

    private void LinkParentCommands(List<UserDefinedCommand> userDefinedCommands)
    {
        userDefinedCommands.ForEach(udc =>
        {
            if (udc.Parent is not { } parentCmdStr)
            {
                return;
            }

            if (_pendingLoadCommands.Find(x => x.Name.Equals(udc.Name, StringComparison.OrdinalIgnoreCase)) is not
                { } userCommand)
            {
                return;
            }

            if (_pendingLoadCommands.Find(x => x.Name.Equals(parentCmdStr, StringComparison.OrdinalIgnoreCase)) is not
                { } parentCommand)
            {
                return;
            }

            parentCommand.Subcommands ??= [];
            parentCommand.Subcommands.Add(userCommand);
            _pendingLoadCommands.Remove(userCommand);
        });
    }
}