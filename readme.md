# Core-latency

This is a simple tool to try and measure the intercore latency between two cpus.

## How to run

You will need cargo and rust installed. The easy way to do so is to via [rustup](https://rustup.rs/).

`cargo run --release`

## What do these numbers mean?
The numbers represent the time it takes to do a ping-pong between two cores. Both cores write a number to a memory
location and the they look for the other core's write to be observed. They then try and overwrite the value they just 
read with the next one in the sequence. In effect the two cores are both swapping who is in charge of writing to a
cache line.

This is done in a square pattern between each possible core. So each line of the file that is printed is the 
latency between one core and every other one.

