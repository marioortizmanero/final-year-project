# Dynamic loading of plugins in Rust in the absence of a stable Application Binary Interface

* **Author**: Mario Ortiz Manero
* **Mentors**: Matthias Wahl, Heinz N. Gies, and Darach Ennis
* **Academic director**: Francisco Javier Fabra Caro

This repository holds all the sources related to my Final Year Project in
University of Zaragoza. Abstract:

> Decision-making in many modern companies like Wayfair is based on the
> collection and analysis of data from their systems. To carry this out
> efficiently on a massive scale, it is essential to use high-performance tools
> such as Tremor, written with Rust, multi-threaded asynchronous programming,
> and SIMD.
>
> As Tremor evolves, its compilation times grow and, consequently, its
> development experience deteriorates. This can be alleviated by implementing a
> plugin system that divides its single binary into smaller,
> independently-compilable components. There are multiple available technologies
> for its development: interpreted languages, WebAssembly, eBPF, inter-process
> communication, or dynamic loading. However, many of them must be discarded for
> not meeting Tremor’s efficiency standards. Among the remaining alternatives,
> dynamic loading is chosen for being the most usable and popular one.
>
> Dynamic loading is impossible with types and functions declared with pure
> Rust, because its Application Binary Interface (ABI) is not stable. It is
> necessary to convert its types to C’s ABI, which is stable, and vice-versa. To
> facilitate the process, existing libraries and language tools such as
> procedural macros can be used.
> 
> Since dynamic loading is a very new ecosystem in Rust, a great amount of
> Tremor’s dependencies need to be contributed to in open source in order to
> implement the functionality needed for a plugin system.
> 
> The complexity of the project increases significantly compared to the original
> plan, so even if functional, the implementation does not attain some of the
> initial objectives, mainly related to performance. However, it serves as a
> great base for future versions of Tremor that do include it in production, and
> it will continue to evolve with the program.

The full document can be accessed
[here](https://github.com/marioortizmanero/final-year-project/blob/master/main.pdf).
Note that it's in Spanish, but if more information was needed, most of the
contents are available in my personal website, with a somewhat different format:
https://nullderef.com/series/rust-plugins/.

This project was done in collaboration with [Tremor](https://www.tremor.rs/),
and funded by [Wayfair](https://www.wayfair.com/).
