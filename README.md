Aurora.rs
=========

Aurora.rs is a framework (with some DNA from Aurora.js, with some additional sprinkles of Media Foundation) that makes writing media pipelines in Rust easier.

It will handle common media tasks like binary data streams and building media decoding topologies. It doesn't support any of the high-level features of Aurora.js yet (and probably won't) and most of the low-level stuff is broken, but it does something Aurora.js doesn't, and that is that it supports encoders as well as decoders.

It is also intended to support video, which isn't really in the realm of JS performance right now, and Images, which we just didn't feel was a focus in Aurora.js.


What is working
---------------

 - WAV mux, demux (mostly, but it is brittle)
 - PCM transcoder (always round-trips via double, so while accurate, it might be slow)


What is not working (but is planned in the short term)
------------------------------------------------------

To finish my thesis, I will need to implement at least

 - Short-time Fourier Transforms (for generating spectrograms).
 - MDCT.
 - BMP encoder.


What is not working (and hopefully fixed in the long term)
----------------------------------------------------------

 - Some orchestration of topologies, now you need to control each element manually.
 - AIFF, MPEG-1, MPEG-2, Ogg, QuickTime, CAF, and MPEG-4 demuxers.
 - MP3, AAC, Vorbis, and FLAC decoders.
 - BMP, TIFF, JPEG, GIF and PNG decoders.
 - Automatic setting of stream types, based on what is supported by a transform.
 

Goal
----

It would be really cool if this could be used in Servo for audio / video / img tags, but I doubt it will, mainly due to licensing reasons. It is also meant to support my thesis, since I am getting tired of working with Python.


Building
--------

You can `make aurora` to build the library, or `make all` to also build the example applications.


Demo
----

You should be able to run `make examples` to build the examples, then run `bin/wav-to-wav-s16` and if you have a `media/wav/test-float.wav` around, it'll create a `output.wav` with 16-bit samples.


Authors
-------

Aurora.rs was written by [@jensnockert](https://github.com/jensnockert), and you should check out the [Audiocogs](https://github.com/audiocogs/) Github, where there is a lot of code related to media.

In addition Aurora.js is currently being developed by [@devongovett](https://github.com/devongovett) and he is really awesome (and also a member of Audiocogs).

If you want to contact me about it, either tweet or poke me on #audiocogs (Freenode IRC) or #rust (Mozilla IRC).


License
-------

Aurora.rs is released under the MIT license.