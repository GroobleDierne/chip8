# Chip-8 emulator

A chip-8 emulator written in Rust for Windows, may not work on other platforms.

## Building

Simply run
> cargo build

## Running

On your terminal simply run the previously obtained file, you can add the path to a ROM as an argument if you want, if no path is provided a pong ROM will be loaded.
> chip8.exe C:\path\to\a\ROM

You can also add the `--decompile` flag which will take the input file, decompile it and save it to `file.source`.
> chip8.exe C:\path\to\a\ROM --decompile
