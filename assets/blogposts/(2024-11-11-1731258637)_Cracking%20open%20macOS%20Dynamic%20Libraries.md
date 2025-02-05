# Cracking open macOS Dynamic Libraries 🍏🔍

Hey! Let’s take a dive into something that’s been haunting me for a while—macOS dynamic libraries (.dylib). It’s a wild ride through Mach-O binaries, weird @rpath issues, and Apple’s funky file organization system. If you’ve ever wondered why theres no `/usr/lib` on OSX, this one’s for you. Buckle up, we’re going to unravel this mystery together! Because of work, I'd been forced to become more intimate with the macOS filesystem than I'd like. 

## The problem

I was trying to write a Wireshark plugin on macOS, and I couldn’t get it to load. I was compiling my plugin code against `libwireshark.dylib` by hand and even basic examples were not loading.

With some hacky renaming from `.dylib` to `.so` and some `install_name_tool` magic to fix some absolute paths to `@rpaths`, I got it to work. But it got me thinking—why was macOS being so finicky with my `.dylib` files? That’s when I decided to dig deeper into how macOS handles dynamic libraries.


<div style="text-align: center;">
    <img src="/images/dogcow.png" style="width: 50%;" alt="Apparently searching for a macOS mascot returns this">
</div>


## So, What’s a .dylib Anyway? Whats a plugin?

Let’s kick things off by clearing up what exactly a .dylib is, and how it compares to .so on Linux and .dll on Windows. 

### The TL;DR version:
•	Linux: Everything is a “shared object” (.so).
•	macOS: Uses Mach-O binaries, where the distinction lies between .dylib and .bundle.

In the Linux world, shared libraries (.so) can be linked both statically or dynamically at runtime. But on macOS, the Mach-O binary format gets fancy. It distinguishes between:
•	.dylib: Traditional shared libraries.
•	.bundle: Plugins or modules that are dynamically loaded at runtime.

Essentially, .bundle is macOS’ equivalent of Linux’s .so used for plugins. It’s this distinction that led me down the rabbit hole when my Wireshark plugins weren’t loading correctly—turns out, Wireshark expects .so, not .dylib. I found that simply renaming my compiled files did the trick! 🧙‍♂️

## Getting Under the Hood

### Checking Your Library’s Dependencies: `otool -L`

Ever wondered what dependencies your .dylib has? Enter `otool`:

`otool -L my_plugin.dylib`

This command spits out the linked libraries and paths, so you can make sure all dependencies are being resolved correctly. If you see any weird @rpath entries, don’t panic—we’ll get to that in a sec.

### What’s With @rpath?

Apple loves abstracting file paths, especially with the introduction of System Integrity Protection (SIP). That’s where @rpath comes in. It’s a placeholder that resolves to different directories based on your environment.

If you’re seeing errors like:

`dyld: Library not loaded: @rpath/libfoo.dylib`

It means your system can’t resolve that path. You can use `install_name_tool` to remedy that:

```bash
install_name_tool -change @rpath/libfoo.dylib /actual/path/libfoo.dylib my_plugin.dylib
```

This came in handy when I was debugging my Wireshark plugin. For some reason, macOS Wireshark wasn’t recognizing my `.dylib` even though everything looked good in `otool`. Adjusting the @rpath paths fixed it.

## Symbol Hunting with nm

If you’re trying to figure out what symbols your .dylib exports, nm is your friend:

nm -gU my_plugin.dylib

This command lists all globally available symbols. If you’re missing expected symbols or exporting unnecessary ones, this is where you’d spot it.

## Frameworks and Bundles: Why macOS Is Special 

Unlike Linux, which keeps libraries under /usr/lib, macOS prefers everything to be neatly bundled into frameworks. It’s all about modularity. Apple wants developers to package everything needed into a single directory (think /Applications/YourApp.app/Contents/Frameworks).

This is partly because macOS uses “SDKs” instead of relying on system-wide shared libraries. So, don’t be surprised if you don’t see anything in /usr/lib that you’d expect on Linux.

## A Note on macOS Versioning 📅

macOS handles library versioning differently. When you inspect a .dylib, you’ll see multiple version numbers:

```bash
otool -L /usr/lib/libz.dylib
/usr/lib/libz.dylib:
    /usr/lib/libz.1.dylib (compatibility version 1.0.0, current version 1.2.11)
```

•	Compatibility version: Minimum version required to maintain backward compatibility.
•	Current version: Actual version of the library.

This is crucial when linking libraries, especially if your application expects a specific version.

Bringing It All Together: My Wireshark Plugin Journey 🦈

Why did I bother learning all of this? Well, it all started when I tried to write a Wireshark plugin. After compiling it against libwireshark.dylib, I couldn’t get Wireshark to recognize it.

Turns out, the problem boiled down to macOS not being happy with .dylib files for plugins. Wireshark was hard-coded to look for .so files. Simply renaming my .dylib to .so worked like magic! ✨

But that quick hack opened up more questions about how macOS handles dynamic libraries, leading me down the rabbit hole of Mach-O formats, @rpath, and Apple’s weird preference for frameworks over traditional library structures.

Final Thoughts 💭

If you’re like me, coming from a Linux background, macOS’ dynamic library system can be pretty alien. But once you wrap your head around Mach-O binaries, @rpath, and Apple’s obsession with bundles, it all starts to make sense.

Until next time, happy hacking! 🔧🍏

