# lldbui

A lightweight native GUI for LLDB.

![](https://git.sr.ht/~dennis/lldbui/blob/main/resources/screenshots/screenshot_dark.png)

![](https://git.sr.ht/~dennis/lldbui/blob/main/resources/screenshots/screenshot_light.png)

## Usage

The goal is to be able to launch with the same command line options as lldb itself. Currently the following options are supported:

```
Usage: lldbui [OPTIONS] <EXECUTABLE|--attach-pid <ATTACH_PID>|--attach-name <ATTACH_NAME>> [ARGS]...

Arguments:
  [EXECUTABLE]
  [ARGS]...

Options:
  -p, --attach-pid <ATTACH_PID>    Tells the debugger to attach to a process with the given pid
  -n, --attach-name <ATTACH_NAME>  Tells the debugger to attach to a process with the given name
  -x, --no-lldbinit                Do not automatically parse any '.lldbinit' files
  -h, --help                       Print help
  -V, --version                    Print version
```

Passing an executable as a positional argument prepares lldbui to debug the given executable. To disambiguate between arguments passed to lldbui and arguments passed to the debugged executable, arguments starting with a `-` must be passed after `--`: `lldbui someprog -- --progarg1 --progarg2=foo`.

## Build

In addition to the Rust toolchain you need to fullfill the dependencies of:
* [egui](https://www.egui.rs) specifically [egui_glow](https://github.com/emilk/egui/tree/master/crates/egui_glow). On Linux that currently means something like: `sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev`. MacOS should work out of the box.
* [lldb.rs](https://docs.rs/lldb/latest/lldb/). Follow the instructions for [Linux](https://github.com/endoli/lldb.rs?tab=readme-ov-file#linux) or [macOS](https://github.com/endoli/lldb.rs?tab=readme-ov-file#macos).

On macOS I currenly install xcode, `brew install llvm` and set:
```
LLVM_CONFIG="/opt/homebrew/opt/llvm/bin/llvm-config"
LLVM_ROOT="/opt/homebrew/opt/llvm"
LLVM_BUILD_ROOT="/opt/homebrew/opt/llvm"
DYLD_FRAMEWORK_PATH="/Library/Developer/CommandLineTools/Library/PrivateFrameworks"
```

## Rationale

lldbui aims to be a middle ground between plain lldb and a full fledge debugger IDE. If you're already using an IDE like VSCode you probably should use that for debugging. But if you're a Vim/Emacs user that needs a bit more than plain lldb this project might be for you. The embedded lldb console gives you access to all lldb functions while the UI helps you to visually inspect the program state.

## Credits

This project wouldn't have been possible without:

*  [Emil Ernerfeldts](https://github.com/emilk/) awesome [egui](https://www.egui.rs/) library 
*  [Bruce Mitcheners](https://github.com/waywardmonkeys) [Rust bindings for the lldb C++ API](https://docs.rs/lldb/latest/lldb/)

I drew lots of inspiration from Vadim Chugunov VSCode plugin [codelldb](https://github.com/vadimcn/codelldb) and from [lldbg](https://github.com/zmeadows/lldbg/).

## TODO

- output lldb log in gui
- console history (empty submit repeats previous command)
- handle or prevent commands that require cli feedback (`break delete`)
- ability to view coredumps
- keyboard shortcuts
- use more idiomatic rust
- reduce amount of unwrap() etc.
- proper icons instead of text buttons
- ui love
- the grand prize: figure out how to distribute binaries
- ...
