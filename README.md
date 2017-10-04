# Nix `ptsname_r()` Shim

Using this shim's `ptsname_r()` over Nix's `ptsname_r()` on Linux will make your application portable to macOS with regards to `ptsname_r()`. The `ptsname_r()` exposed by this crate will simply reference Nix's original `ptsname_r()` on Linux and use its shim on macOS. The signature of the function is identical and the behavior is identical.

## Background

In POSIX, `ptsname()` is used on a file descriptor created by `posix_openpt` to get the name of the slave psuedoterminal.

Unfortunately, `ptsname()` modifies global variables in order to return a pointer to the modified variables with the name of the slave in it. In Rust's Nix, this is interpreted as unsafe. It is also not thread-safe and can cause issues during signal handling amongst other issues. Thus, its implementation Nix is marked as `unsafe`.

In response, at least on the Linux platform, there is a function called `ptsname_r()` which is a re-entrant and safer version of `ptsname()` as it writes to a buffer the caller provides. In Rust's Nix, it allocates a buffer and wraps it into an owned `String` to make it safe.

Unfortunately, macOS/Darwin does not include the `ptsname_r()` function. Alternatively, you are to use the `TIOCPTYGNAME` syscall with the master file descriptor and a buffer pointer as the argument. It's _almost_ like `ptsname_r()` but it's different nevertheless.

[It was determined that it was not in the scope of Nix to include shims such as this for platforms that do in fact have the functionality but the underlying calls are different.](https://github.com/nix-rust/nix/pull/742).

It's out of scope, but a shim like this is desired, as can be demonstrated with the following two examples who have implemented similar shims:

* This [guy](https://blog.tarq.io/ptsname-on-osx-with-rust/), who came up with most of this approach, was using it for what I'm assuming is something work related at some large hosting provider. I'm not sure what purpose he was exactly using it for but he had a need for `ptsname_r` functionality.  He's not using `nix`, but I can see that he `cfg` gates his similar `nix`-like wrappers to the OS. I'm guessing that he also wants to maintain a similar API between Linux and macOS. This version was the basis for the following reference.

* My other [example](https://github.com/philippkeller/rexpect/blob/a71dd02/src/process.rs#L67) is doing a pexpect clone. To do a pexpect clone, he has to open the slave terminal by its name which he had to find through `ptsname_r` and then assign them to the correct descriptors before calling exec in the child. Unlike the first reference, he uses `nix`, but polyfills in pretty much this implementation of `ptsname_r` for macOS where `nix`/macOS falls short. Like me, and the previous example, he also fills in the gap with a similar API for macOS that emulates the Linux version.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
