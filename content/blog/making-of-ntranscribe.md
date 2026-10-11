---
title: "Making of ntranscribe"
date: 2026-07-01
description: "The most close of the thing that i can find on internet is transcrybe which does the same thing as translation and caption but the two most prominent thing was"
---

## Why this?

The most close of the thing that i can find on internet is `transcrybe` which does the same thing as translation and caption but the two most prominent thing was

1.  Its not open source so not free (free as in freedom)
    
2.  Its not free (free as in beer here)
    

## Getting started

I researched the ways by which we can get the audio from the device and got to know about [blackhole](https://github.com/ExistentialAudio/BlackHole). So as to install it i have installed it using brew

```shell
brew install blackhole-2ch
```

Then i can made a multi output device to forward audio to blackhole as well as my computer system

![](https://cdn.hashnode.com/uploads/covers/696a6e4c5043d2bf87996284/a132079f-044c-4560-8d73-ccedf0a1e0e6.png)

Then i started working with the python but soon realized python is not the thing for the sub second latency so switched to rust for development

## Rust development journey

As python has sounddevice i need to figure out the libraries for handling audio devices in rust and later on i found out [`cpal`](https://docs.rs/cpal/) ([https://docs.rs/cpal/](https://docs.rs/cpal/))

It was capable of getting the audio so start developing in rust

i will explain you step by step what i have done

```rust
    let host = cpal::default_host();

    let device = host
        .input_devices()?
        .find(|device| device.to_string().contains("BlackHole"))
        .ok_or("Blackhole not found")?;
    println!("Using default input device: {}", &device);
    let config = device.default_input_config()?;
```

Here i get the default host and a device for audio capturing you must look [here](https://docs.rs/cpal/latest/cpal/index.html) to understand its the default thing needed for getting the device

Later on i figured out that the device config is this

```plaintext
SupportedStreamConfig { channels: 2, sample_rate: 48000, buffer_size: Range { min: 15, max: 4096 }, sample_format: F32 }
```

So the blackhole 2ch has 2 channels sampling rate is 48000Hz and sampling format is F32 (float32)

Then we created a streamconfig we will later see and a channel to transfer audio data between threads

```rust
let streamconfig: cpal::StreamConfig = config.into();
    let (tx,rx) = channel();
```

since we are getting the output of sound we will be using the inputstream method as this

```rust
let stream = device.build_input_stream(
        streamconfig,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let _ = tx.send(data.to_vec());
        },
        err_fn,
        None,
    )?;
```

As per docs of [build\_input\_stream](https://docs.rs/cpal/latest/cpal/traits/trait.DeviceTrait.html#method.build_input_stream) it needs a streamconfig callback function and error function and timeout

callback function gives the data and some timing information since we are only interested in data we are ignoring timing informating using the `_`

also as the data will be overwritten later as soon as the data came in the callback function we move it to the channel as a vector

then we start the stream using

```rust
    stream.play()?;
```

As of now i have done this

```rust
thread::spawn(move || {
        let mut audio_buffer: Vec<f32> = Vec::new();
        while let Ok(raw_audio_data) = rx.recv() {
            println!("Data received: {:?}", raw_audio_data);
        //     Downsampling the data (usually in 48Khz needed 16Khz)
        //     48000 - > 16000 Hz

        }
    });

    loop {
        thread::sleep(std::time::Duration::from_secs(1));
    }
```

spawning a thread and printing the data recieved but i will update it till then understand this

we are consuming the data using `rx.recv()` and printing it verbosely

now your question will be why this infinite loop then

its because if it exits the loop the program will stop all threads will be killed so this infinite loop is to prevent the killing of the thread and the program itself

### Integrating whisper in it

While searching for whisper i got to know that there is a rust binding whisper-rs which is the rust binding for whisper.cpp which we will be using as a dependency in rust project

The problem with whisper is its very picky about the audio chunks

It requires

*   16000 Hz frequency
    
*   Mono audio
    

But the blackhole provides

*   48000 Hz sampling rate
    
*   Stereo audio
    

So we will be doing this

![](https://cdn.hashnode.com/uploads/covers/696a6e4c5043d2bf87996284/febac14d-649c-42f5-940f-883e7f86c29e.png)

taking each pair and averaging it to get the mono audio and then taking the third one to get the 16000 Hz frequency as 16000\*3 = 48000

in rust

```rust
let mut mono_16khz = Vec::with_capacity(raw_audio_data.len() / 6);
            for chunk in raw_audio_data.chunks_exact(6) {
                // Downmix the first stereo frame of this group to mono
                let mono1 = (chunk[0] + chunk[1]) / 2.0;
                let mono2 = (chunk[2] + chunk[3]) / 2.0;
                let mono3 = (chunk[4] + chunk[5]) / 2.0;

                // Average the 3 mono samples to get 1 downsampled sample
                let final_sample = (mono1 + mono2 + mono3) / 3.0;


                mono_16khz.push(final_sample); // Decimates by 3 automatically by skipping the other 2 frames
            }
            audio_buffer.extend(mono_16khz);
```

making a mono\_16khz vector with capacity `raw_audiolen/6` 3 for above thing and /2 extra for stereo to mono

and then we are extending the audio\_buffer

Since we are extending the vector we must drain also somewhere to not comsume too much memory while running so the logic for comsuming memory can be written as

```rust
let mut audio_buffer: Vec<f32> = Vec::new();
        let sample_rate = 16000;
        let process_interval = sample_rate * 3;  // Process every 3 seconds of audio
        let max_window_size = sample_rate * 30;  // Keep max 30 seconds of context
        let mut last_processed_len = 0;

// ---- Some code snipper of storing in vector buffer -----
if audio_buffer.len() - last_processed_len >= process_interval {
                if audio_buffer.len() > max_window_size {
                    let drain_amount = audio_buffer.len() - max_window_size;
                    audio_buffer.drain(0..drain_amount);
                }
                last_processed_len = audio_buffer.len();
            };
```

Later i have implemented whisper-rs for translation of whatever language as according to [this](https://codeberg.org/tazz4843/whisper-rs/src/branch/master/examples/audio_transcription.rs) example but then i thought maybe i should polish the things so i used the library [rubato](https://docs.rs/rubato/latest/rubato/) for resampling the data into the 16 Khz.

Then i want the ggml model binary for speech to text recognition so i downloaded the multilingual ggml-small model from [here](https://huggingface.co/ggerganov/whisper.cpp/tree/main) .

Then i got a problem that the whisper.cpp is giving out verbose log

it can be solved using

```rust
whisper_rs::install_logging_hooks();
```

later you can view the whole script at my repository

Github url : [https://github.com/lsnnt/ntranscrybe](https://github.com/lsnnt/ntranscrybe)

## Demo

![](https://cdn.hashnode.com/uploads/covers/696a6e4c5043d2bf87996284/c62a93bf-d100-46a0-afe4-3fab9457f762.png)
