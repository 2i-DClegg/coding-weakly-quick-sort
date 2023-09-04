# Coding Weakly Quick Sort

A program that generates a list of 100,000 random numbers and sorts them in the quickest time possible over 100 iterations. Any sorting algorithm goes (aside from the in-built language ones, you gotta write it yourself, looking at you Python people), any programming language goes.

I have used rust to attempt this. My current solution generates all the number in each iteration, which needs to be optimised. I have attempted to implement the quick sort method. 

## Requirements
You will need Rust installed on your machine. This can be acquired here.
[https://www.rust-lang.org/tools/install]

You will then need to run the following commands in the root directory of the project:
1) cargo build
2) cargo run

## Future Development
Add in the function to generate the number once, and then pass it tot eh sort function, rather than generate each time.