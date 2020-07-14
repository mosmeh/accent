# accent

[![build](https://github.com/mosmeh/accent/workflows/build/badge.svg)](https://github.com/mosmeh/accent/actions)

Minimal audio reverberator

## Installation

Clone this repository and run:

```sh
cargo install --path .
```

## Usage

To add reverberation to `input.wav` using JCRev algorithm and output to `output.wav`, run:

```sh
accent jcrev input.wav -o output.wav
```

You can also provide parameters to algorithms:

```sh
accent freeverb input.wav -o output.wav --roomsize=0.5 --damp=0.2
```

Following algorithms are available:

| Algorithm | Description                                                                             |
|-----------|-----------------------------------------------------------------------------------------|
| jcrev     | [original JCRev](https://ccrma.stanford.edu/~jos/pasp/Schroeder_Reverberators.html)     |
| satrev    | [SATREV](https://ccrma.stanford.edu/~jos/pasp/Example_Schroeder_Reverberators.html)     |
| stk-jcrev | [JCRev in Synthesis ToolKit](https://github.com/thestk/stk/blob/master/include/JCRev.h) |
| prcrev    | [PRCRev](https://github.com/thestk/stk/blob/master/include/PRCRev.h)                    |
| nrev      | [NRev](https://github.com/thestk/stk/blob/master/include/NRev.h)                        |
| freeverb  | [Freeverb](https://ccrma.stanford.edu/~jos/pasp/Freeverb.html)                          |

## Options

```
-g, --gain <gain>    Final gain in dB [default: 0]
-o <output>          Output WAV file [default: out.wav]
```

### `stk-jcrev`, `prcrev`, and `nrev`

```
--t60 <t60>       [default: 1]
```

### `freeverb`

```
--damp <damp>             [default: 0.1]
--dry <dry>               [default: 0]
--roomsize <roomsize>     [default: 0.1]
--wet <wet>               [default: 1]
--width <width>           [default: 1]
```
