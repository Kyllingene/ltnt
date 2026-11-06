# ltnt

![lieutenant insigna](https://git.sr.ht/~kyllingene/ltnt/blob/master/ltnt.svg)

`ltnt` is a command-line arguments parsing library. It's a simple, efficient,
and flexible library. See the documentation for full examples, as well as the
`/examples` directory.

### Simple

It uses a powerful, but easy, syntax for establishing the structure of your
invocation:

```rust
ltnt::ltnt! {
    // A set of commands
    pub enum Root in RootArgs {
        ~Help,   // A default command without arguments
        Ls = Ls, // A command with arguments
    };

    // A set of arguments
    #[derive(Debug)]
    pub struct Ls {
        // An argument with long and short forms and a default value
        pub all('a'): bool = false,
        // An optional argument that's been renamed
        pub in_dir as "in" ?: String,
    };
}
```

### Efficient

It uses a declarative macro, has only one dependency, and avoids unnecessary
allocations. It's also available on `no_std` (with `no-default-features`),
though it still requires `alloc`.

### Flexible

Most of the enforced structure in `ltnt` is merely in how the macro implements
the core trait (`Parsable`) for your types. So long as you can implement
`Parsable`, you can use any types you want. The only hard requirements are that
your type is `Sized`, and that it can be parsed from a stream of strings with
only one item of lookahead.

In practice, you'll usually be interacting with three "kinds" of parsables:
commands, arguments, and values.

## Roadmap and non-goals

In the future, I'd like to add a system for generating documentation from your
structure, especially help text. I'd also like to support e.g. completion
generators, but I don't intend to write them myself.

I also don't plan to support arbitrary syntaxes, or a builder. The reason for
the latter is because builders require some kind of enforced structure, which I
don't want to force on users. One could, however, implement one on top of `ltnt`
- that would be awesome!

## Cloning

This repository is maintained on [Github](https://github.com/kyllingene/ltnt),
[sr.ht](https://git.sr.ht/~kyllingene/ltnt), and [Radicle](https://radicle.xyz)
(at `rad:zApjuUkQCBKSUaU6gzJozKsJvvRs`).

Note that issues, PRs, etc. cannot and will not be syncronized between all of
these. If you do not wish to check each platform before publishing an issue/PR,
that's okay; however, if it's redundant, I may close it and refer you to a
pre-existing issue/PR.
