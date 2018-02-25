<p align="center">
  <img height="400" width="400" src="rusty-tor.png">
</p>

**rusty-tor** is a proof-of-concept utility for accessing internet content and hidden service content (.onion) via tor routers.

This utility is :
* developing on Rust.
* targeted platforms Linux and Windows.
* compilable to executable and/or dynamic library.

Should be able to access:
* onion services
* clear web(maybe in future)

Should support next protocols :
* http
* https(maybe in future)
* other(maybe in future)

## existing cheat sheet projects on Python :
* [Stem](https://www.torproject.org/getinvolved/volunteer.html.en#pythonTorClient) \[Python\] - links: [official site](https://stem.torproject.org/), [repository](https://gitweb.torproject.org/stem.git/). Consist of 3 core parts:
  * Tor Controller (stem.control)- for Tor's ControlPort protocol.
  * Tor Directory facilities (stem.descriptor) - for Tor's DirPort protocol.
  * Tor Client (stem.client) - for Tor's ORPort protocol.
* [endosome](https://github.com/teor2345/endosome) \[Python\] -  is a proof-of-concept Tor cell construction kit.
* [TinyTor](https://github.com/Marten4n6/TinyTor) \[Python\] - access to onion services only.
## existing cheat sheet projects on other languages :
* [mini-tor](https://github.com/wbenny/mini-tor) \[C++17\] - access to clear web and onion services.
* [GoTor](https://github.com/TvdW/gotor) \[Go\] - attempt at implementing a full Tor relay in the Go language. Links: [blog post](https://medium.com/interplanetary-social-network/a-pinch-of-privacy-tor-from-within-go-fc4b09986120)
## unimplemented similar project on Rust:
* [Tor crate](https://crates.io/crates/tor) \[Rust\] - A library enabling the usage of the Tor network without having Tor installed. Links: [repository](https://github.com/sahithyen/tor).

## rusty-tor implement support of :
1. Tor's DirPort protocol. Other implementations to keep in mind:
    * [TinyTor](https://github.com/Marten4n6/TinyTor).
    * [Stem tor directory facilities (stem.descriptor)](https://stem.torproject.org/), 
2. Tor's ORPort protocol. Other implementations to keep in mind:
    * [endosome](https://github.com/teor2345/endosome)
    * [Stem tor client (stem.client)](https://stem.torproject.org/)

_It's still a work in progress and is nowhere near complete or secure. 
Any one who wanna impove his skills on Rust, networking, cryptography can join to development._
