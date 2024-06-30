# Chapter 1: Skin Command Line Interface

The Skin Command Line Interface (CLI) provides a set of commands to manage and apply skinners to targets. It offers functionalities such as activating, deactivating, checking status, running commands, installing, configuring, and updating targets.

### Usage

To use the Skin CLI, you can run the following commands:

- `skin activate <target>`: Activate a specific target.
- `skin disactivate <target> [<skinner>: default is "R"]`: Disactivate a target with an optional skinner.
- `skin status <target>`: Check the status of a target.
- `skin run <target> [<extras>]`: Run a target with optional extra arguments.
- `skin install <url or path>`: Install a skinner from a specified URL/Path.

Where `<target>` is the command path or name, `<url or path>` is the URL or path to the skinner makefile.

### Examples

#### Disactivate

```Shell
skin disactivate /bin/tor # tor for example
```

The command disactivates the `/bin/tor` target with the default skinner, `R`.

When a specific path is allocated to be deactivated, the rest of command paths will remain activated and can be used except for the deactivated one.

For example, the `/sbin/tor` command can still be used if the `/bin/tor` path is allocated and disabled through the previous command.

To disable all command paths, you can insert the command name.

```Shell
skin disactivate tor # tor for example
```

This command disactivates all targets with the `tor` command name, `/bin/tor` and `/sbin/tor`, and others if any.

After disactivating all targets with the `tor` command name, you can't use the `tor` command anymore.

#### Activate

```Shell
skin activate /bin/tor # tor for example
```

No need to specify the skinner because it's auto-detected.

This command will reactivate the `/bin/tor`, if it was previously deactivated.

One target can be activated from several disabled targets, or all of them at the same time.

```Shell
skin activate tor # tor for example
```

#### Status

```Shell
skin status tor # tor for example
```

This command will show information and status of the `tor` target.

#### Run

```Shell
skin run tor # tor for example
```

The `run` command is only available for targets that have been deactivated.
Also you can specify a path to run as a target.

```Shell
skin run /bin/tor # tor for example
```

---

[Next Chapter: Skinners](./Chapter-02.md) | [Previous](./Index.md)