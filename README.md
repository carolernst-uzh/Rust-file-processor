# Word counter
-----

A simple rust application to read a file and count the number of words.

## Steps to build
-----

1. Clone the repo
2. Compile the package to generate the release build
    ```bash
    cargo build --release
    ```
3. Run the command by passing the filepath
    ```bash
    ./target/release/word-counter <file-path relative or absolute>
    ```
    
## Performance
----

Compared with `wc` 

```bash
time ./target/release/word-counter input.txt
Parsing the file input.txt
199279
./target/release/word-counter input.txt  0.01s user 0.00s system 83% cpu 0.019 total

---------------

time wc -w input.txt
  199279 input.txt
wc -w input.txt  0.01s user 0.00s system 83% cpu 0.011 total
```