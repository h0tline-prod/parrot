# Parrot
This is *simple realization of Parrot* in **Rust Programming Language**.

## How it works
It's very simple.
* Struct for a parrot, that have three thins inside:
  + The name value(type: String);
  + The vector with famous frazes(type: String);
  + The function *live*.

Function works all time, because you don't stop an endless loop, that is a base for this function. The function takes from user two inputs:
* New famous fraze;
* Something, what you sad. *After this input parrot will interrept you and say a random famous frase.*

## Build
To build project you need to have installed **Cargo Project Manager** on your device(it usually installs with **Rust Compiler**). If you have it, you can build this project by run these command in terminal(CMD)
```
git clone https://github.com/shrug228/Parrot.git
cd Parrot
```
P.S.: *if you don't have installed git on your PC, you can just download .ZIP archive of this repository to your PC, unarchive it and run the command under this text*.
```
cargo build --release
```

Then you'll get a binary application file in *target/release* with name *Parrot*(or *Parrot.exe*, if you are on **Windows**).
