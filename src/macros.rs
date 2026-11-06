/// ```rust
/// # use ltnt::ltnt;
/// ltnt!(
///     // A set of commands that, when used itself as a command, accepts the arguments of 'Root'
///     pub enum Manager in ManagerArgs {
///         // Basic command named 'help', takes no arguments, can be a full path;
///         // if no command is passed, this is the default
///         ~Help,
///
///         // Nested command called with 'ls'
///         Ls = Ls,
///
///         // Nested command called with `dev` (*not* `info`)
///         Info = Info as "dev",
///     };
///
///     // A set of arguments being used as the arguments for 'Manager' (but can be used anywhere)
///     pub struct ManagerArgs {
///         // A required argument with a short form
///         pub name('n'): String,
///
///         // An argument with a default value, that therefore doesn't need to be passed
///         pub verbose: bool = false,
///     };
///
///     // A set of arguments that's being used in 'Manager' as a command
///     struct Ls {
///         pub all('a'): bool = false,
///
///         // An optional argument (is really `Option<String>`, defaults to `None`)
///         pub filter('f') ?: String,
///     };
///
///     #[derive(Debug)]
///     pub struct Info {
///         // An optional argument set with `--kinds`
///         pub device_kinds as "kinds" ?: Vec<String>,
///     };
/// );
/// ```
///
/// Example usages for above code, assuming `Manager` is the root and the binary
/// is called `manager`:
///
/// ```sh
/// # Both print help text
/// manager
/// manager help
///
/// # When supplying values, long-form arguments can optionally use an '=',
/// # while short-form ones cannot
/// manager ls --filter="foo"
/// manager ls -f "bar"
///
/// # Arguments come exclusively after the command they modify, e.g.
/// # `manager dev --verbose` is invalid
/// manager --verbose dev --kinds keyboard,monitor
/// ```
#[macro_export]
macro_rules! ltnt {
    ($(
        $(#[$attr:meta])*
        $( pub $(( $($vis_args:tt)* ))? )?

        $(
            enum $cmds:ident $( in $cmds_args:ty )? {$(
                $(#[$cmd_attr:meta])*
                $( ~ $(@ $cmd_default:tt )? )?
                $cmd:ident
                    $( = $cmd_inner:ty )?
                    $( as $cmd_rename:literal )?
            ),* $(,)?}
        )?

        $(
            struct $args:ident $( in $args_args:ident $( :: $args_args_path:ident )* )? {$(
                $(#[$arg_attr:meta])*
                $arg_vis:vis $arg:ident
                    $(( $arg_short:literal ))?
                    $( as $arg_rename:literal )?
                    $( ? $(@ $arg_optional:tt )? )?
                    : $arg_ty:ty
                    $( = $arg_default:expr )?
            ),* $(,)?}
        )?
    ;)+) => {$(
        $(#[$attr])*
        $( pub $(( $($vis_args)* ))? )?

        $(
            enum $cmds {$(
                $(#[$cmd_attr])*
                $cmd $(( $cmd_inner ))?
            ),*}

            impl $crate::Parsable for $cmds {
                type Output = $crate::__if_else!(
                    if { $(( Self, $cmds_args ))? }
                    else { Self }
                );

                type FullOutput<T> = (
                    Self,
                    $( $cmds_args, )?
                    T,
                );

                fn make_full_output<T>(output: Self::Output, rest: T) -> Self::FullOutput<T> {
                    $crate::__if_else!(
                        if ($( $cmds_args )?) {
                            (output.0, output.1, rest)
                        } else {
                            (output, rest)
                        }
                    )
                }

                fn parse_from<S>(stream: &mut $crate::Peekable<S>)
                    -> $crate::Result<<Self as $crate::Parsable>::Output>
                where
                    S: ::core::iter::Iterator,
                    S::Item: ::core::convert::AsRef<::core::primitive::str>
                        + ::core::convert::Into<::alloc::string::String>,
                {
                    let ret = $crate::__if_else!(
                        if {$({
                            let args = <$cmds_args as $crate::Parsable>::parse_from(stream)?;
                            move |x: Self| ::core::result::Result::Ok((x, args))
                        })?} else {
                            |x: Self| ::core::result::Result::Ok(x)
                        }
                    );

                    let cmd = stream.peek().ok_or($crate::Error::OutOfData);
                    $($crate::__if_else!(if ($(~ $($cmd_default)?)?) {
                        if cmd.is_err() {
                            $crate::__if_else!(
                                if {$(
                                    let cmd = <$cmd_inner as $crate::Parsable>::parse_from(stream)?;
                                    return ret(Self::$cmd(cmd));
                                )?}
                                else { return ret(Self::$cmd); }
                            );
                        }
                    });)*

                    let cmd = cmd?.as_ref();

                    $(
                        let target = $crate::__if_else!(if {
                            $( $cmd_rename )?
                        } else { $crate::__kebab!($cmd) });
                        if cmd == target {
                            stream.next();
                            $crate::__if_else!(
                                if {$(
                                    let cmd = <$cmd_inner as $crate::Parsable>::parse_from(stream)?;
                                    return ret(Self::$cmd(cmd));
                                )?} else { return ret(Self::$cmd); }
                            );
                        }
                    )*

                    #[allow(unreachable_code)]
                    {
                        $({$crate::__if_else!(if ($(~ $($cmd_default)?)?) {
                            $crate::__if_else!(
                                if {$(
                                    let cmd = <$cmd_inner as $crate::Parsable>::parse_from(stream)?;
                                        // .ok_or_else(|| $crate::Error::InvalidDefault(stringify!($cmd)))?;
                                    return ret(Self::$cmd(cmd));
                                )?}
                                else { return ret(Self::$cmd); }
                            );
                        });})*

                        return ::core::result::Result::Err(
                            $crate::Error::UnrecognizedCommand(cmd.into())
                        );
                    }
                }
            }
        )?

        $(
            struct $args {$(
                $(#[$arg_attr])*
                $arg_vis $arg: $crate::__opt_ty!($( ? $($arg_optional)? )? $arg_ty)
            ),*}

            impl $crate::Parsable for $args {
                type Output = Self;
                type FullOutput<T> = (Self, T);

                fn make_full_output<T>(output: Self::Output, rest: T) -> Self::FullOutput<T> {
                    (output, rest)
                }

                fn parse_from<S>(stream: &mut $crate::Peekable<S>) -> $crate::Result<Self>
                where
                    S: ::core::iter::Iterator,
                    S::Item: ::core::convert::AsRef<::core::primitive::str>
                        + ::core::convert::Into<::alloc::string::String>,
                {
                    $(
                        let mut $arg = ::core::option::Option::<$arg_ty>::None;
                    )*

                    fn missing_value(s: impl ::core::convert::Into<::alloc::string::String>)
                        -> impl ::core::ops::FnOnce($crate::Error) -> $crate::Error
                    {
                        move |e| match e {
                            $crate::Error::OutOfData => $crate::Error::MissingValue(s.into()),
                            e => e,
                        }
                    }

                    while let ::core::option::Option::Some(arg)
                        = stream.peek().map(|s| s.as_ref().trim())
                    {
                        if arg == "--" {
                            break;
                        } else if arg.starts_with("--") {
                            let arg = stream.next().unwrap();
                            let long = arg.as_ref().strip_prefix("--").unwrap();
                            let (long, val) = long.split_once('=').unwrap_or((long, ""));
                            $(
                                let target = $crate::__if_else!(if {
                                    $( $arg_rename )?
                                } else { $crate::__kebab!($arg) });
                                if long == target {
                                    let default = $crate::__if_else!(
                                        if ($( $arg_optional )?) {
                                            ::core::option::Option::None::<$arg_ty>
                                        } else {
                                            <$arg_ty as $crate::Parsable>
                                                ::default()
                                        }
                                    );

                                    let val = if val.is_empty() && default.is_none() {
                                        ::core::option::Option::Some(
                                            <$arg_ty as $crate::Parsable>::parse_from(stream)
                                                .map_err(missing_value(long))?
                                        )
                                    } else if !val.is_empty() {
                                        ::core::option::Option::Some(
                                            <$arg_ty as $crate::Parsable>::parse_from(
                                                &mut ::core::iter::once(val).peekable()
                                            )?
                                        )
                                    } else if !default.is_none() {
                                        default
                                    } else {
                                        ::core::option::Option::Some(
                                            <$arg_ty as $crate::Parsable>::parse_from(stream)
                                                .map_err(missing_value(long))?
                                        )
                                    }.ok_or_else(|| $crate::Error::MissingValue(long.to_string()))?;

                                    $arg = ::core::option::Option::Some(val);

                                    continue;
                                }
                            )*

                            return ::core::result::Result::Err(
                                $crate::Error::UnrecognizedArgument(long.to_string())
                            );
                        } else if arg.starts_with('-') {
                            let arg = stream.next().unwrap();
                            let short = arg.as_ref().strip_prefix('-').unwrap();


                            let mut chars = short.chars();
                            // let last = chars.next_back().ok_or($crate::Error::EmptyShortList)?;
                            let Some(last) = chars.next_back() else { continue };

                            for ch in chars {
                                $($(
                                    if ch == $arg_short {
                                        // only the final arg in a short chain is allowed to
                                        // consume a following value, the rest must rely on
                                        // defaults
                                        let val = <$arg_ty as $crate::Parsable>::default()
                                            .ok_or_else(|| $crate::Error::MissingValue(ch.to_string()))?;

                                        $arg = ::core::option::Option::Some(val);

                                        continue;
                                    }
                                )?)*

                                return ::core::result::Result::Err(
                                    $crate::Error::UnrecognizedArgument(ch.to_string())
                                );
                            }

                            $($(
                                if last == $arg_short {
                                    let val = <$arg_ty as $crate::Parsable>::default()
                                        .map(::core::result::Result::Ok)
                                        .unwrap_or_else(|| {
                                            <$arg_ty as $crate::Parsable>::parse_from(stream)
                                                .map_err(missing_value(short))
                                        })?;

                                    $arg = ::core::option::Option::Some(val);

                                    continue;
                                }

                            )?)*

                            return ::core::result::Result::Err(
                                $crate::Error::UnrecognizedArgument(last.to_string())
                            );
                        } else {
                            break; // arguments in a set must be contiguous
                        }
                    }

                    let mut missing = Vec::new();

                    $(
                        let $arg = $arg
                            $( .or_else(|| ::core::option::Option::Some($arg_default)) )?;

                        $crate::__if_else!(
                            if ($( ? $($arg_optional)? )?) {}
                            else {
                                if $arg.is_none() {
                                    let name = $crate::__if_else!(if {
                                        $( $arg_rename )?
                                    } else { $crate::__kebab!($arg) });
                                    missing.push(name.to_string());
                                }
                            }
                        );
                    )*

                    if !missing.is_empty() {
                        return ::core::result::Result::Err($crate::Error::MissingArguments(missing));
                    }

                    ::core::result::Result::Ok(Self {$(
                        $arg: $crate::__if_else!(
                            if ($( ? $($arg_optional)? )?) {
                                $arg
                            } else {
                                $arg.unwrap()
                            }
                        )
                    ),*})
                }
            }
        )?
    )+};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __opt_ty {
    ( ? $($t:tt)* ) => { ::core::option::Option<$($t)*> };
    ( $($t:tt)* ) => { $($t)* };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __if_else {
    (if () {$($_:tt)*} else {$($t:tt)*}) => {$($t)*};
    (if ($($_:tt)*) {$($t:tt)*} else {$($__:tt)*}) => {$($t)*};

    (if () {$($_:tt)*}) => {};
    (if ($($_:tt)*) {$($t:tt)*}) => {$($t)*};

    (if {} else {$($t:tt)*}) => {$($t)*};
    (if {$($t:tt)*} else {$($_:tt)*}) => {$($t)*};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __kebab {
    ($s:ident) => {
        $crate::__map_ascii_case!($crate::__Case::Kebab, stringify!($s))
    };
}
