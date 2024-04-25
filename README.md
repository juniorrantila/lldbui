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

Passing an executable as a positional argument prepares lldbui to debug the given executable. To disambiguate between arguments passed to lldbui and arguments passed to the debugged executable, arguments starting with a `-` must be passed after `--`. Eg. `lldbui someprog -- --progarg1 --progarg2=foo`.

## Build

TODO

## Rationale

TODO

## Credits

This project wouldn't have been possible without:

*  [Emil Ernerfeldts](https://github.com/emilk/) awesome [egui](https://www.egui.rs/) library 
*  [Bruce Mitcheners](https://github.com/waywardmonkeys) [Rust bindings for the lldb C++ API](https://docs.rs/lldb/latest/lldb/)

I drew lots of inspiration from Vadim Chugunov VSCode plugin [codelldb](https://github.com/vadimcn/codelldb) and [lldbg](https://github.com/zmeadows/lldbg/).

## TODO

- add build instructions
- breakpoints
  - delete
  - add
- watchpoints
  - list
  - delete
  - all
- proper language mappings for the syntax highlighting
- ability to view coredumps
- ...
