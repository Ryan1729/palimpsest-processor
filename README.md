#What is this?

TODO

## Installation for Compilation

This program relies on `libBearLibTerminal.so` so that should be copied into `usr/local/lib` or another folder indicated by this command: `ldconfig -v 2>/dev/null | grep -v ^$'\t'`

then you should run `sudo ldconfig` to complete the installation.

Then the executable should run correctly

##TODO List
-> re-fill hand with generated cards that use whole range of instructions
-> break execution button
-> pause execution button
-> more instructions
  -> display Instruction Register
    -> let's make it a separate thing for now, we can make it a normal
       register later if we want.
  -> jumps
    -> Jump to immediate if Register is 0
      -> naming: JAZ -> Jump if A is Zero?
    -> Jump to immediate if Register is not 0
    -> Jump to Register 1 if Register 2 is 0
    -> Jump to Register 1 if Register 2 is not 0
-> Copy playfield to clipboard button
