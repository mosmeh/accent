# accent 
Minimal audio reverberator

## Usage
To add reverberation to `input.wav` using JCRev algorithm and place the output into `output.wav`, run:
```
cargo run jcrev input.wav -o output.wav
```

You can also provide parameters to algorithms:
```
cargo run freeverb input.wav -o output.wav --roomsize=0.5 --damp=0.2
```

Following algorithms are available:
 - `jcrev` (original JCRev)
 - `satrev` (SATREV)
 - `stk-jcrev` (JCRev in Synthesis ToolKit)
 - `prcrev` (PRCRev)
 - `nrev` (NRev)
 - `freeverb` (Freeverb)
