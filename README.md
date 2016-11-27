Last Layer Algs
===============

Source code powering [@lastlayeralgs](https://twitter.com/lastlayeralgs).

## Usage

### `following <alg>`

Running the binary with the command `following <alg>` will cause the program to output the first last layer algorithm that comes after `<alg>` and then exit:

```
$ cargo run following "R' U' R U' R' U2 R"
R' F2 L F L' F R
```

### `tweet`

The program will read the file `last` to determine the last alg it tweeted, tweet the next one, and update `last`.
It will look in the file `creds` to find the Twitter credentials it should use.
The format of the credentials file should be

```
consumer key
consumer secret
access key
access secret
```
