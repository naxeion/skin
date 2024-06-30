# Chapter 4: Installing Skinners

When you publish your skinner publicly, people will be able to download and benefit from it. Additionally, if you want to download any skinner at any time, this option is available. All you need is a simple step.

## How to install a skinner

There is a command in Skin CLI, which is :

```Shell
skin install <url/path>
```

Where `<url/path>` is the location of the skinner makefile.

## How to apply the installed skinner

When you install a new skinner, it will be available to use alongside the existing skinners.

to deactivate a command with the new skinner, there is no difference.

```Shell
skin disactivate <target> <skinner-name>
```

Where `<target>` is the target you want to disactivate and `<skinner-name>` is the name of the skinner makefile.

```Shell
skin disactivate tor skinner-name # tor for example
```

`skinner-name` is the name defined in the Skinner Makefile

---

[Previous](./Chapter-03.md)