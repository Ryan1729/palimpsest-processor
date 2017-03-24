#What is this?

TODO

## Installation for Compilation

This program relies on `libBearLibTerminal.so` so that should be copied into `usr/local/lib` or another folder indicated by this command: `ldconfig -v 2>/dev/null | grep -v ^$'\t'`

then you should run `sudo ldconfig` to complete the installation.

Then the executable should run correctly

##TODO List
-> "Run" button that inverts colours when clicked
-> when button is clicked invert each instruction in playfield in turn
-> actually run instructions.
  -> simple register display, init to zero
-> Copy playfield to clipboard button
