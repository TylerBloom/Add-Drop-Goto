## Goal
Add-Drop-Goto, or `adg` for short, seeks provide way to store shortcuts for the names of directories that works across active terminals. Moreover, it wraps the `cd` command so that you can largely replace its usage with `adg`, even when moving between directories that do not have shortcuts.

## About
Quickly moving between projects or around a directory tree can be challenging. Often, the easiest solutions to this are terse directory names, limiting subdirectory trees, and/or relying on your shell's command autocompletion. Providing shortcuts allows for quickly moving between directories.

There are two types of shortcuts that `adg` provides.

First of these are refered to as "locales". These are shortcuts that expand to an absolute path. Project directories are prime canadites for this type of shortcut.

The second type of shortcut is refered to a "site". These are shortcuts that expand to a relative path. These shortcuts are meant to express directories that are common between locales.

## Example
Consider the following directory tree.
```
HOME
|
└───projectOne
|  └───build
|  │   └───subfolder1
|  └───docs
|
└───projectTwo
    └───build
```
With `adg` the above directories could be shortcut to `one: HOME/projectOne` and `two: HOME/projectTwo`. Since both projects have a `build` directory, a site could be added such as `bld: build`. This allows the directory `HOME/projectOne/build` to be expressed as `one/bld`. Similarly, `HOME/projectOne/docs` can be written as `one/docs`.

## Configuring
To start, run the `install.sh` bash script. This will create the required directories and copy the `adg` executable where it needs to go.

Next, you will need a shell script that wraps the `adg` executable. Since communicating with the parent terminal isn't possible, a shell script is needed. Currently, both bash and fish shell are supported. Copy the shell script somewhere in your `$PATH` (such as `~/bin`) and alias the script to be sourced. For Bash users, this will look like `alias adg=". adg.sh"`. This is required in order to move directories.

After that, you are all set up!

## How to Use
By default, `adg` will expand your arguments into a path unless your specify any of it's three arguments.
```
$ pwd
>> HOME
$ adg one/bld
$ pwd
>> HOME/projectOne/build
```

To add a shortcut, use `add`. In order to specify a new locale, you must give the path (relative or absolute) to a directory and a name for this shortcut. For locales, the order doesn't matter. Starting from the home directory in the example, we can do this by:
```
$ adg add one HOME/projectOne
$ adg add two projectTwo
```
To add a site, you must give the name of the shortcut followed by the path. Again, starting from the home directory in the example, we can the `build` shortcut by:
```
$ adg add bld build
```

To remove a shortcut, use `--drop` followed by the name of the shortcut:
```
$ adg drop one
$ adg drop bld
```

To see your stored use `--list`:
```
$ adg list
>> Locales:
>>	 one : HOME/projectOne
>>	 two : HOME/projectTwo
>>
>> Sites:
>>	 bld : build
```

## Notes and Planned Changes
Ideally, the shell script wrapper would be removed. The main issue there is that it is impossible (or at least very hard) to communicate with your parent shell session in Rust. Until such time, shell scripts for other shells, such as zsh, will be added.

In prior iterators of this project, tab-completion was supported as standard tab-completion breaks down when you use a shortcut. This is planned to make a return, but will need reworked in Rust and for each supported shell type.
