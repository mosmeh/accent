# accent 
Minimal audio reverberator

## Usage
To add reverberation to `input.wav` using Freeverb algorithm and place the output into `output.wav`, run:
```
cargo run freeverb input.wav -o output.wav
```

Following algorithms are supported:
 - `jcrev` (original JCRev)
 - `stk-jcrev` (JCRev in Synthesis ToolKit)
 - `satrev` (SATREV)
 - `freeverb` (Freeverb)
