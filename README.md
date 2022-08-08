## Five 5-letter Word Search With No Shared Letters

#### &copy; Peter M. Steven 2022

twenty_five.rs is a Rust coding exercise for me inspired by Matt Parker's YouTube video on his Stand-Up Maths channel. ([Can you find: five five-letter words with twenty-five unique letters?](https://www.youtube.com/watch?v=_-AfhLQfb6w).) In the video, he said he had written a program (in Python) to find all the 5-word sets that didn't duplicate letters from a 370K-word dictionary. He explicitly said that the program was not optimized and he was sure it could be written to run faster, but he was interested the result, not so much the code. His program took 31 days to complete (his patience is laudable). After releasing the video a commentor said he had written a version that ran in 15 minutes. I wanted to replicate those results.

I am learning Rust and this is my first real Rust program, so please don't judge me too harshly. I am an _old_ C coder, so many of the concepts of Rust resonated with me and I was sensitive to at least some areas for optimization. I wrote this program and the initial test saw it complete the search on my M1 Mac in about 5 minutes. Though this seemed good, I was disappointed, as I expected Rust to run much faster than Python. I made a few tweaks and got the run time down to a little under 4 minutes. I was about to move on, but thought for comparison I should try a release version compiled with whatever default optimizations Rust does. The released version of the code ran in 7.3 seconds. That's more like it! I wrote a comment on the video to this effect and thought it was only fair to share the source code. This is a single module source file. Please go to Matt Parker's video to find the dictionary he used.
