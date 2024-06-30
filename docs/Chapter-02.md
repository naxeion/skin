# Chapter 2: Skinners

## What are Skinners?

Skinners are a technique and method used to hide commands or targets. They provide a way to conceal specific functionalities from users, enhancing security and access control in command-line interfaces.

Skin tool provides some built-in skinners, but you can also create your own.

## Built-in Skinners

Skin provides some built-in skinners, such as:

- Skinner `L`: Hide the target with a random name.
- Skinner `R`: Hide the target with a random name.
- Skinner `X`: Hide the command by revoking execution permissions.

An example to apply a skinner with disactivate command:

```Shell
skin disactivate tor L # tor for example. Hidden with the skinner L
```

```Shell
skin disactivate tor X # tor for example. Hidden with the skinner X
```

As mentioned previously, the default skinner is `R`.
```Shell
skin disactivate tor # tor for example. Hidden with the skinner R
```

The built-in skinners are provided by the Skin tool, so no need to install anything.

## Creating a Skinner

Creating a new skinner is a simple process, all you need is to create a Makefile with the essential elements. This will be the focus of the next chapter.

---

[Next Chapter: Creating a Skinner](./Chapter-03.md) | [Previous](./Chapter-01.md)