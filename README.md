# Chip'n'Claw [WIP]
![Our mascot, Crambon](https://i.imgur.com/uvBWC4x.png)

Blazingly fast CHIP-8 interpreter written in Rust.

## TODO:
* Code CPU emulator
  * Finish writing OpCodes
  * Test all of them rigorously
  * Check if RAM works as intended
* Code Graphics
* Code Audio

## Build
*Beware, this is still highly unstable, and I'm not even sure it works.*
### Linux
Making sure you have `cargo` installed,
```bash
$ make
```
should add a file called `chip-n-claw` to your working directory.

```bash
$ ./chip-n-claw cowgod.ch8
```

executes `cowgod.ch8` as a CHIP-8 ROM.