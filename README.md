# hex2rgb & rgb2hex

### Compile

You need rustc compiler and ln command, run this:

```bash
rustc -o hex2rgb hex2rgb.rs && command ln -s hex2rgb rgb2hex
```

### Usage

Run without argument to show the help screen.

#### rgb2hex
`rgb2hex` requires you to enter three arguments, which are red, green and blue.

Unlike the `hex2rgb`, the `rgb2hex` can only produce one result at a time.

#### hex2rgb
`hex2rgb` can accept two kind of argument value, a value with a pawn sign `#` or without a pawn sign.

If there is a pawn sign in the argument value, you need to quote it, or else the shell may interpret it as a comment not as a value.

`hex2rgb` can accept more than one argument value and produce more than one result at a time.

Check the help screen for some example.
