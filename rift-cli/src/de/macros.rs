macro_rules! enum_de {
    ($basety : ident, $newty :ident,
        $(#[$derive_meta:meta])* {
            $(
                $( #[ $cfg_meta:meta ] )?
                $var: ident
            ,)*
        }
        $({
            $( $(
                #[ $cfg_meta_ex:meta ] )?
                $var_ex: ident $( { $( $(#[$cfg_v:meta])* $vx: ident : $vt: ty ),* } )?
                    => $to_ex: expr
            ,)*
        } )?
    ) => {

        $(#[$derive_meta])*
        pub(crate) enum $newty {
            $(  $(#[$cfg_meta])* $var , )*
            $($(  $(#[$cfg_meta_ex])* $var_ex $( { $( $( #[$cfg_v] )* $vx :  $vt ,)* } )* , )*)*
        }

        impl From<$newty> for $basety {
            fn from(s : $newty) -> $basety {
                match s {
                    $(
                        $(#[$cfg_meta])*
                        $newty::$var => $basety::$var,
                    )*
                    $($(
                        $(#[$cfg_meta_ex])*
                        $newty::$var_ex$({$( $vx ,)*})* => { $to_ex },
                    )*)*
                }
            }
        }
    };
}
