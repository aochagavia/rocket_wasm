Rocket
======

*Note: this project is no longer being maintained and pull requests will be ignored. Feel free to fork if you think something is missing.*

> Rocket is a toy game written in Rust, using the Piston library. The code is thoroughly 
commented in order to help people to follow it easily.

## Screenshots

![Screenshot](screenshots/gameplay2.png)

You can find more screenshots in the [screenshots] directory.

[screenshots]: screenshots/

## How to play

As you can see in the screenshots below, you are the red rocket and have to save the world from 
the yellow invaders. To do so, you can use the following controls:

Keyboard                | Action
----------------------- | ------------
<kbd>&uparrow;</kbd>    | Boost
<kbd>&leftarrow;</kbd>  | Rotate left
<kbd>&rightarrow;</kbd> | Rotate right
<kbd>Space</kbd>        | Shoot

### Running it with Cargo

As always, it is a real pleasure to work with Cargo. You only need the following:

```
cargo run --release
```

## Why?

After having implemented some toy games in C++ using SDL and SFML, I thought it would be a 
good idea to try the same in Rust. Additionally, I had written a similar game in Haskell and 
wanted to port it to see the similarities and differences between Haskell and Rust. Another 
reason to program this game was to have an easy to follow Rust project that could be useful 
for people learning the language.
