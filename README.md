# woppleblox

No, not the fictional company - a micro-blogging app with a  focus on being lightweight, easy to setup / install, and open.

## Principles

 - **Lightweight:** Heavyweight dependencies should be avoided if possible, and the overall size kept to a minimum. If some features are optional, their dependencies should be as well. Hardware requirements should be minimal (hence no big database).
 - **Open and Transparent:** It should be easy to see how and where woppleblox is storing its data. It should also be easy to export data from woppleblox in a variety of standard formats that are accepted elsewhere. Importing would be nice too, but this is probably a nice-to-have that's some way off (and challenging to test).
 - **Easy to install:** It should be very easy to install and setup for the first time, such that even novices can get started using it

## Other versions
Before we got comfortable with Rust, we started implementing woppleblox in Node.js - but didn't get very far. That version exists [here](https://git.starbeamrainbowlabs.com/sbrl/woppleblox-js) for reference (it was responsible for some nice patterns and code snippets), but you'll probably need to ask for permission to access it.
