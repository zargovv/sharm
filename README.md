# Charm Shell

> The appropriate shell for Microsoft Windows.

## State

This is a very simple project intended for personal use. **The author does not
guarantee anything yet**: speed, maintenance, neither safety.

The development is currently in its early stages. However, it has not yet been
decided how far it might be taken.

By the time of this commit, the shell is only being tested on **Windows 11 using
[Windows Terminal (v1.18.10301.0)](https://github.com/microsoft/terminal)**

## Motivation

I enjoy using Debian and its simple yet powerful shell. Incorporating excellent
shells such as [fish](https://github.com/fish-shell/fish-shell) turns your
workflow into a pleasure. Unfortunately, I have not found anything comparable to
this experience as it comes to Windows. At times, I am required to use Windows.
Meanwhile, MinGW or WSL may not always be relevant.

## Phylosophy

Very simple project implies simple implementation. This project doesn't require
many external dependencies, only the kernel functions to interact with the
console. As of its current state, there is no need for any unnecessary bloating.

## Shortcut Table

Shortcut | Description
-------- | ---------------------
Ctrl+K   | Clear entire buffer
Ctrl+L   | Clear screen
Ctrl+W   | Delete word backwards
Ctrl+C   | Terminate
Ctrl+D   | Exit

More shortcuts will be introduced soon.

## Todos

- [ ] Split commands on `&&`, `;` and `|`.
  - [ ] Handle appropriately on execution.
- [ ] Probably make `Ctrl+W` delete until dash (`-`).
  - And maybe some special symbols.
- [ ] Highlight paths in cmd and arguments (underline).
- [ ] Proper handling for newlines within single command.
  - [ ] Either hide current time once command length reaches it, or break until.
  - [ ] Proper backspace and Ctrl+W handling.
  - [ ] Consider breaking line to prompt length instead of console begin.
- [ ] History file.
  - [ ] Limit buffer by 4096 bytes.
  - [ ] Arrow Up should traverse through previous commands.
    - [ ] Only show previous results based on command prefix (user prompt).
  - [ ] Ctrl+R shortcut.
  - [ ] Inline suggestions.
- [ ] Task runner.
  - [ ] Absolute and relative paths as "commands" (cd).
  - [ ] HashMap with internal commands (check them first for efficiency).
  - [ ] Actually run commands.
    - [ ] Treat strings as single paramter ("" and '') when passing to command.
- [ ] Path intellisense.
  - [ ] Check possible values on `TAB`.
     - [ ] Display possible variants using subwindow.
- [ ] Subwindow (path intellisense and Ctrl+R).
- [ ] Tokenization.
  - [ ] Highlight strings.

## Licensing

The project is available under the `GPLv3` license. Please refer to the
[`LICENSE`](LICENSE) file for the complete license text.
