# punter

Simple dotfile manager

## How to use

1. Get your dotfiles directory ready (e.g. by cloning it from git)
2. Generate the default config with `punter generate`
3. Make any changes you like to the generated `punter.toml`
4. Sync your dotfiles to your home directory with `punter sync`

You can add other dotfiles later with `punter add file`

### Config

By default `punter generate` will assume you want to symlink every file (and directory) in your dotfiles folder, and prefix it with a dot.

A default entry looks something like this `"bashrc" = ".bashrc"`

The default destination base is your home folder, but it can be specified.

Hidden files will not be added to your config automatically, you can add them yourself, or use the `--ignore-hidden` flag when generating. Another dot will of course _not_ be prefixed then.

An entry in a config is very flexible:

- the source can be any path relative to your dotfile folder
- the destination can be any path
- absolute destination paths will ignore the destination directory

You can update your config with `dotter add .`, which will ignore files already present.

#### Example config

```toml
[settings]
verbosity = "info"
destination = "/media/wschella/home"

[files]
"bashrc" = ".bashrc"
"absolute_file" = "/foo/bar"
".hidden_file" = ".absolute_file"
```
