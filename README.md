# extip
Get external ip from command line

_Written primarily to get up to speed with Rust. Turned out kind of useful!_

Examples:
```
./extip -h
extip 0.1.1
Magnus Wallin <magnuswallin@tutanota.com>
Gets external ip address

USAGE:
    extip [FLAGS]

FLAGS:
    -h, --help       Prints help information
    -u, --urls       Show the urls used
    -V, --version    Prints version information

./extip
Your external ipv4 address is: 194.109.14.3
Source: http://ipinfo.io/ip

./extip -u
We check against these urls:
https://myexternalip.com/raw
http://ipinfo.io/ip
https://ipv4.icanhazip.com/
http://ipecho.net/plain
https://canihazip.com/s
The first proper result from any of these urls are used.
```
