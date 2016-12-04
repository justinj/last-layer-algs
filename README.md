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

## Future

The current cube representation (storing each sticker) has a lot of benefits:

* it's simple,
* just one component, doesn't require a separate orientation and permutation and corners and edges,
* can be very easily used to generate images, since we have each sticker.

But it has a bunch of downsides as well:

* it's not a very compact format, so we do more work than necessary when we apply a move,
* it's difficult to optimize and prune the search tree because it doesn't really represent the structure of the cube very well,
* since it's all one component, we can't speed it up in small chunks by replacing components with lookup tables.

So this is just to say I eventually plan to change the current representation to one that stores the
permutation and orientation of corners and edges independently.
