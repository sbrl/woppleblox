# woppleblox

No, not the fictional company - a micro-blogging app with a focus on being lightweight, easy to setup / install, and open.

From the developer that brings you [Pepperminty Wiki](https://github.com/sbrl/Pepperminty-Wiki.git) and reviews [tldr-pages PRs](https://github.com/tldr/tldr-pages), I now bring you _Woppleblox_ (well I _will_ be, when it works :P).

**Warning:** Woppleblox is is pre-alpha quality. In other words, it's _not finished yet_. I've open-sourced it early, as [someone asked](https://www.reddit.com/r/selfhosted/comments/fy6aib/tumblr_alternative_for_bloggingmicroblogging/fn35xpy?utm_source=share&utm_medium=web2x) to see the code. Suggestions are welcome, but don't expect it to actually work or for binaries to be available yet.

**Follow along by clicking watch → watch releases in the top centre-right!**

I'll occasionally be tweeting about it on my Twitter account [@SBRLabs](https://twitter.com/SBRLabs) if you want to follow me there too.


## Principles
 - **Lightweight:** Heavyweight dependencies should be avoided if possible, and the overall size kept to a minimum. If some features are optional, their dependencies should be as well. Hardware requirements should be minimal (hence no big database).
 - **Open and Transparent:** It should be easy to see how and where woppleblox is storing its data. It should also be easy to export data from woppleblox in a variety of standard formats that are accepted elsewhere. Importing would be nice too, but this is probably a nice-to-have that's some way off (and challenging to test).
 - **Easy to install:** It should be very easy to install and setup for the first time, such that even novices can get started using it


## News
**Current estimate:** first usable build in ~6 months (as of April 2020)

 - **2020-04-11:** Most recently we've got the SQLite database connection & repos setup and the translation engine up and running (we think; untested as of yet), so next up we need to tackle the views, controllers, HTTP routing, and work towards a basic login system.


## Getting started
You want to try running it! Wow, you're brave! Remember that it's not finished and it's not yet completed.

### System requirements
 - `rustc`, `cargo` - the rust compiler & build tool
 - Linux (Windows & macOS _might_ work, but I don't have either of these and I'm far too focused on making it actually work in the first place to support them. PRs welcome :P)
<!--  - `node`, `npm` - Node.js & npm for building the client-side stuff -->

### Compiling
Clone this repository, and then run the build script:

```bash
git clone https://github.com/sbrl/woppleblox.git
cd woppleblox;
./build run
# Or, for release mode (faster execution & *much* smaller binary, but no debugging output if it crashes and even slower compilation times):
SBRL_CARGO_FLAGS="--release" ./build run
```

This will take a while, 'cause the Rust compiler is slow. It will automatically start the server after compilation is completed. Binaries are written somewhere in the `target`, depending on whether you're compiling in debug or release mode.


## Other versions
Before we got comfortable (I am? News to me :P) with Rust, we started implementing woppleblox in Node.js - but didn't get very far. That version exists [here](https://git.starbeamrainbowlabs.com/sbrl/woppleblox-js) for reference (it was responsible for some nice patterns and code snippets), but you'll need to ask for permission to access it.


## Contributing
Contributions are welcome! Because of the early stage that this project is at, please [open an issue first](https://github.com/sbrl/woppleblox/issues/new) before doing any serious work, 'cause things are changing quite quickly at the moment.

All contributions must be stated to be under the _Mozilla Public Licence 2.0_.


## Licence
This project is licenced under the _Mozilla Public Licence 2.0_, the full text for which can be found in the `LICENSE` file in the root of this repository.

See also [tldr legal](https://tldrlegal.com/license/mozilla-public-license-2.0-(mpl-2)), which has a nice short summary of what you can and can't do.
