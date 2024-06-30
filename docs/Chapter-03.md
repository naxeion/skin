# Chapter 3: Creating a Skinner

> NOTE: For a clearer understanding, you can take a look at the [example](#example) at the end before starting to read this chapter

Makefiles are scripts that automate the build process by specifying dependencies and commands. They are commonly used in compiling and managing software projects.

## What needs to create a skinner?

There are essential elements that must be included in the Makefile when creating a skinner. These elements ensure the basic operations for Skinners.

The first element is the `name` of the skinner, to define the name, you can use the `name` variable.

```Makefile
name = skinner-name
```

or name the file `skinner-name.makefile`.

Are you considering doing both? Smart move.
The program will take the skinner's name from the file name, even if a variable defining the skinner's name exists.

### Essential elements structure

#### Variables
| name | required | description |
| :---: | :---: | :---: |
| name | :white_check_mark: | The name of the skinner |
| description |  | The description of the skinner |
| version |  | The version of the skinner |
| license |  | The license of the skinner |
| author |  | The author of the skinner |
| maintainer |  | The maintainer of the skinner |

#### Targets
| name | required | description |
| :---: | :---: | :---: |
| do     | :white_check_mark: | Contains operations to execute the Skinner (deactivate a target) |
| undo   | :white_check_mark: | Unlike do, operations to activate a target |
| run    | :white_check_mark: | Execute the deactivated target |
| status | :white_check_mark: | Show the status of the Skinner |

When creating the target do, it is essential to print `do: success` if the deactivation process is successful.

When creating the target undo, it is essential to print `undo: success` if the activation process is successful.

## Example

The built-in Skinner, `x`, but in Makefile

```Makefile
name = "x"

.PHONY: all do undo run status

all:
	@echo "${name} v${version}"

do:
	@chmod -x ${TARGET}
	@echo "do: success"

undo:
	@chmod +x ${TARGET}
	@echo "undo: success"

run:
	@if [ ! -x ${TARGET} ]; then \
		$(MAKE) -f ${MAKEFILE_LIST} -s undo > /dev/null 2>&1; \
		${TARGET} ${ARGS}; \
		$(MAKE) -f ${MAKEFILE_LIST} -s do > /dev/null 2>&1; \
	fi

status:
	@if [ -x ${TARGET} ]; then \
		echo "Activated"; \
	else \
		echo "Disactivated"; \
	fi
```

Using `$(MAKE) -f ${MAKEFILE_LIST} -s` is safer than `$(MAKE)`, however, when you include the skinner's name in the file, it becomes mandatory. This ensures that the Makefile tool recognizes the path to your skinner file.

This was a simple example of creating a Skinner using a Makefile. Although it is short, there is a lot to learn to create a Skinner

---

[Next Chapter: Installing Skinners](./Chapter-04.md) | [Previous](./Chapter-02.md)