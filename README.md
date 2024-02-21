# Drop

## Tom Jacobson

### Writeup

This assignment came with a few challenges, the first big one being finding out the name of the accelerometer chip. Once I found the lsm303agr and it's corresponding crate, the next big challenge was to find out how to access the lsm303 in code. After some trial and error (and some help from referencing https://github.com/pdx-cs-rust-embedded/mb2-thermometer/blob/main/src/main.rs) I was able to initialize the sensor and pull readings from it.

The next big hurdle was working with the interrupts. I am not confident that I did my interrupts in the intended way, as initially I tried to work with accelerometer interrupts but was unsuccessful in getting that up and running. Eventually I landed on having the display interupt handle the refreshing of the board and using my accelerometer readings to determine which image to show. I have a feeling that this is slightly wrong, and might have a small amount of lag as compared to implementing the show statements in the interrupt handler. My big struggle was understanding how often the display interrupt triggers and if I could control that at all. I was unable to determine whether or not I could change those factors.

Overall I think the assignment went well. It was certainly surprisingly challenging to work with the onboard hardware but once the syntax was figured out it went fairly smoothly. Interrupts are still a bit of a challenge to me and I don't feel as though I have a perfect understanding of how they work in the embedded environment. I am interested to see if my solution is implemented in a unique way or if it was simply implemented slightly incorrectly.

#### Sources

- https://github.com/pdx-cs-rust-embedded/mb2-grayscale
- https://github.com/pdx-cs-rust-embedded/mb2-thermometer/blob/main/src/main.rs
- https://github.com/pdx-cs-rust-embedded/hello-audio
