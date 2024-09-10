# ElevenLabs Rust API Client 🎙️🦀

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A Rust client library for the ElevenLabs API, providing easy access to state-of-the-art text-to-speech capabilities.

## Features 🚀

- Easy-to-use Rust interface for ElevenLabs API
- Asynchronous API using `tokio`
- Comprehensive error handling 🔜
- Support for all major ElevenLabs endpoints including 🚧:
  - Text-to-Speech generation 🚧 (Partial)
  - Voice management 🚧 (Partial)
    - List available voices ✅
  - User information retrieval 🚫
- TODO: Streaming audio support 🚫

## Installation 📦

Add this to your `Cargo.toml`:

```toml
[dependencies]
elevenlabs-rs = "0.1.0"
```

## Quick Start 🏁

```rust
use elevenlabs_rs::{ElevenLabsClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Create a new client
    let client = ElevenLabsClient::new("YOUR_API_KEY".to_string());

    // Generate speech
    let audio = client.text_to_speech("voice_id", "Hello, World!").await?;

    // Save the audio to a file
    std::fs::write("output.mp3", audio)?;

    Ok(())
}
```

## Usage Examples 📚

### List Available Voices

```rust
let voices = client.voices().await?;
for voice in voices {
    println!("Voice ID: {}, Name: {}", voice.voice_id, voice.name);
}
```

### Generate Speech with Custom Options

```rust
use elevenlabs_rs::TextToSpeechOptions;

let options = TextToSpeechOptions {
    stability: 0.5,
    similarity_boost: 0.8,
    ..Default::default()
};

let audio = client.text_to_speech_with_options("voice_id", "Custom TTS", options).await?;
```

### Stream Audio Response

```rust
use futures_util::StreamExt;

let mut stream = client.text_to_speech_stream("voice_id", "Streaming audio").await?;

while let Some(chunk) = stream.next().await {
    let audio_chunk = chunk?;
    // Process or play the audio chunk
}
```

## Documentation 📖

For more detailed information about the API and its usage, please refer to the [documentation](https://docs.rs/elevenlabs-rs).

## Contributing 🤝

Contributions are welcome! Please feel free to submit a Pull Request.

## License 📄

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements 🙏

- [ElevenLabs](https://elevenlabs.io) for their amazing text-to-speech API
- The Rust community for their invaluable resources and support

---

<p align="center">Made with ❤️ and 🦀</p>
