Aurora.rs
=========

Aurora.rs is a framework (with some DNA from Aurora.js, with some additional sprinkles of Media Foundation) that makes writing media pipelines in Rust easier.

It will handle common media tasks like binary data streams and building media decoding topologies. It doesn't support any of the high-level features of Aurora.js yet (and probably won't) and most of the low-level stuff is broken, but it does something Aurora.js doesn't, and that is that it supports audio encoders as well as decoders.

Currently it ships with a WAV source, a WAV sink and well… nothing more. I'll need a bit more to be able to port my thesis work to it, so you can expect a PCM converter, short-time fourier transforms, MDCT, and a BMP writer at least, which should prove it a bit.


Goal
----

It would be really cool if this could be used in Servo for audio / video / img tags, but I doubt it will, mainly due to licensing reasons. It is also meant to support my thesis, since I am getting tired of working with Python.


Demo
----

You should be able to run aurora.rs if you have some `.wav` files in the same directory, but it doesn't actually do anything right now.


Authors
-------

Aurora.rs was written by [@jensnockert](https://github.com/jensnockert), and you should check out the [Audiocogs](https://github.com/audiocogs/) Github, where there is a lot of code related to media.

In addition Aurora.js is currently being developed by [@devongovett](https://github.com/devongovett) and he is really awesome (and also a member of Audiocogs).

If you want to contact me about it, either tweet or poke me on #audiocogs (Freenode IRC) or #rust (Mozilla IRC).


Building
--------

You can `rust run aurora.rs` or build it as a library, both should work.


License
-------

Aurora.rs is released under the MIT license.