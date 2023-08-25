# globreeks

it's globset but more dutch, because I made it for use in [electron_tasje](https://codeberg.org/selfisekai/electron_tasje). it tries to be the least broken globset. sorry, *reeks*.

glob order matters here. out of real examples, one can set the globs to `!**/node_modules/**/build/**` and then `node_modules/@signalapp/better-sqlite3/build/Release/better_sqlite3.node`, as to make an exception from the previously forbidden pattern.

it uses the glob implementation from ripgrep's `globset` (by BurntSushi). yes, it's the crate that holds a similar purpose to this one. however, it also includes the least broken rust implementation of a single glob.

copyright 2023 lauren n. liberda, if you must. usage allowed under the terms of `Unlicense`.
