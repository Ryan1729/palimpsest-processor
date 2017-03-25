#What is this?

Playing a card game where the cards change the rules like *Fluxx* or *Magic the Gathering* is essentially equivalent to evaluating a programming language. [*Magic* is even turing complete](http://www.toothycat.net/~hologram/Turing/) upto an asterisk. So I thought it might be interesting to make a game/gamish thing that was more explicit about that fact.

![demo gif](/demo.gif?raw=true "Demo")

## Installation for Compilation

This program relies on `libBearLibTerminal.so` so that should be copied into `usr/local/lib` or another folder indicated by this command: `ldconfig -v 2>/dev/null | grep -v ^$'\t'`

then you should run `sudo ldconfig` to complete the installation.

Then the executable should run correctly.

Alternately if your OS has a package for BearLibTerminal, that may work as well.

## Why is this in 3 crates? And why is the game crate loaded as a dynamic library?

So I can make a change to the game crate's code and recompile just that crate and see the effects of the changes live, without restarting the application. Note that the main crate holds all the state in order to allow that. THe common crate exists in order to holdthings that are common (as you might expect from the name) to the other two crates.

#Current Status/Future

It's an interesting bit of mutable state to play with for a while, but I'm having a hard time coming up with an interesting goal. I thought "make an infinite loop" might work but a single card in the right place can solve that, and I haven't yet come up with an instruction set that makes loops difficult but not impossible. Maybe this will remain in the back of my mind and I'll come back to this later with more ideas, but maybe not.
