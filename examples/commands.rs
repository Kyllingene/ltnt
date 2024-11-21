use ltnt::Parsable;

ltnt::ltnt!(
    // A set of commands that, when used itself as a command, accepts the arguments of 'Root'
    pub enum Manager in ManagerArgs {
        // Basic command named 'help', can be a full path to the type. If no
        // command is passed, this is the default
        ~Help = Help,

        // Nested command called with 'ls'
        Ls = Ls,

        // Nested command called with `dev` (*not* `info`)
        Info = Info as "dev",
    };

    // A set of arguments being used as the arguments for 'Manager' (but can be used anywhere)
    pub struct ManagerArgs {
        // A required argument with a short form
        pub name('n'): String,

        // An argument with a built-in default value (`false`), that therefore doesn't need to be passed
        pub verbose: bool,
    };

    // A set of arguments that's being used in 'Manager' as a command
    struct Help {
        // An argument with a given default value, that therefore doesn't need to be passed
        pub courteous: bool = true,
    };

    // A set of arguments that's being used in 'Manager' as a command
    struct Ls {
        pub all('a'): bool = true,

        // An optional argument (is really `Option<String>`, defaults to `None`)
        pub filter('f') ?: String,
    };

    #[derive(Debug)]
    pub struct Info {
        // An optional argument set with `--type`
        pub device_type as "type" ?: Vec<String>,
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
        Manager::Help(help) => {
            if help.courteous {
                println!("Please read the manual online.");
            } else {
                panic!("Read the docs, doofus.");
            }
        }
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
            if let Some(types) = info.device_type {
                if types.contains(&"keyboard".into()) {
                    let (device, description) = DEVICES[0];
                    println!("{device} : {description}");
                }

                if types.contains(&"mouse".into()) {
                    let (device, description) = DEVICES[1];
                    println!("{device} : {description}");
                }

                if types.contains(&"speaker".into()) || types.contains(&"headphones".into()) {
                    let (device, description) = DEVICES[2];
                    println!("{device} : {description}");
                }

                if types.contains(&"display".into()) || types.contains(&"monitor".into()) {
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
        println!("We don't support or provide {item}");
    }
}
