use clap::ArgAction as ClapArgAction;
use serde::{Deserialize, Serialize};

enum_de!(ClapArgAction, ArgAction,
    #[derive(Deserialize, Serialize, Clone, Copy)]
    {
        Set,
        Append,
        SetTrue,
        SetFalse,
        Count,
        Help,
        HelpShort,
        HelpLong,
        Version,
    }
);
