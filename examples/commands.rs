use ltnt::Parsable;

ltnt::ltnt!(
    // A set of commands that, when used itself as a command, accepts the arguments of 'Root'
    pub enum Manager in ManagerArgs {
        // Basic command named 'help', takes no arguments, can be a full path;
        // if no command is passed, this is the default
        ~Help,

        // Nested command called with 'ls'
        Ls = Ls,

        // Nested command called with `dev` (*not* `info`)
        Info = Info as "dev",
    };

    // A set of arguments being used as the arguments for 'Manager' (but can be used anywhere)
    pub struct ManagerArgs {
        // A required argument with a short form
        pub name('n'): String,

        // An argument with a default value, that therefore doesn't need to be passed
        pub verbose: bool = false,
    };

    // A set of arguments that's being used in 'Manager' as a command
    struct Ls {
        pub all('a'): bool = true,

        // An optional argument (is really `Option<String>`, defaults to `None`)
        pub filter('f') ?: String,
    };

    #[derive(Debug)]
    pub struct Info {
        // An optional argument set with `--kinds`
        pub device_kinds as "kinds" ?: Vec<String>,
    };
);

#[rustfmt::skip]
const DEVICES: &[(&str, &str)] = &[
    (
        "Omega Keyboard",
        "keys: 450, latency: 0.73ms, color: sRGB",
    ),
    (
        "Quik Mouse 3.5",
        "dpi: 1000000, latency: 0.01ms, color: RGB",
    ),
    (
        "SoundBlasters",
        "volume: 150%, latency: 0.58ms, color: none",
    ),
    (
        "UltraRes Retina Display",
        "brightness: 75%, latency: 2.3ms, color: sRGB",
    ),
];

fn main() {
    let (cmd, args, rest) = Manager::parse().unwrap();

    if args.verbose {
        println!("Greetings, my dear friend, {}! Welcome!", args.name);
    } else {
        println!("Hello, {}!", args.name);
    }

    match cmd {
        Manager::Help => println!("TODO: help message"),
        Manager::Ls(ls) => {
            let list = if ls.all { DEVICES } else { &DEVICES[2..] };

            if let Some(filter) = ls.filter {
                for (item, _) in list {
                    if item.contains(&filter) {
                        println!("{item}");
                    }
                }
            } else {
                for (item, _) in list {
                    println!("{item}");
                }
            }
        }

        Manager::Info(info) => {
            if let Some(kinds) = info.device_kinds {
                if kinds.contains(&"keyboard".into()) {
                    let (device, description) = DEVICES[0];
                    println!("{device} : {description}");
                }

                if kinds.contains(&"mouse".into()) {
                    let (device, description) = DEVICES[1];
                    println!("{device} : {description}");
                }

                if kinds.contains(&"speaker".into()) || kinds.contains(&"headphones".into()) {
                    let (device, description) = DEVICES[2];
                    println!("{device} : {description}");
                }

                if kinds.contains(&"display".into()) || kinds.contains(&"monitor".into()) {
                    let (device, description) = DEVICES[3];
                    println!("{device} : {description}");
                }
            } else {
                for (device, description) in DEVICES {
                    println!("{device} : {description}");
                }
            }
        }
    }

    for item in rest {
        println!("we don't support or provide {item}");
    }
}
