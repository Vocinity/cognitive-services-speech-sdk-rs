use std::fmt;

/// SpeechSynthesisOutputFormat defines the possible speech synthesis output audio formats.
#[derive(Debug)]
pub enum SpeechSynthesisOutputFormat {
    /// Raw8Khz8BitMonoMULaw stands for raw-8khz-8bit-mono-mulaw
    Raw8Khz8BitMonoMULaw = 1,

    /// Riff16Khz16KbpsMonoSiren stands for riff-16khz-16kbps-mono-siren
    Riff16Khz16KbpsMonoSiren = 2,

    /// Audio16Khz16KbpsMonoSiren stands for audio-16khz-16kbps-mono-siren
    Audio16Khz16KbpsMonoSiren = 3,

    /// Audio16Khz32KBitRateMonoMp3 stands for audio-16khz-32kbitrate-mono-mp3
    Audio16Khz32KBitRateMonoMp3 = 4,

    /// Audio16Khz128KBitRateMonoMp3 stands for audio-16khz-128kbitrate-mono-mp3
    Audio16Khz128KBitRateMonoMp3 = 5,

    /// Audio16Khz64KBitRateMonoMp3 stands for audio-16khz-64kbitrate-mono-mp3
    Audio16Khz64KBitRateMonoMp3 = 6,

    /// Audio24Khz48KBitRateMonoMp3 stands for audio-24khz-48kbitrate-mono-mp3
    Audio24Khz48KBitRateMonoMp3 = 7,

    /// Audio24Khz96KBitRateMonoMp3 stands for audio-24khz-96kbitrate-mono-mp3
    Audio24Khz96KBitRateMonoMp3 = 8,

    /// Audio24Khz160KBitRateMonoMp3 stands for audio-24khz-160kbitrate-mono-mp3
    Audio24Khz160KBitRateMonoMp3 = 9,

    /// Raw16Khz16BitMonoTrueSilk stands for raw-16khz-16bit-mono-truesilk
    Raw16Khz16BitMonoTrueSilk = 10,

    /// Riff16Khz16BitMonoPcm stands for riff-16khz-16bit-mono-pcm
    Riff16Khz16BitMonoPcm = 11,

    /// Riff8Khz16BitMonoPcm stands for riff-8khz-16bit-mono-pcm
    Riff8Khz16BitMonoPcm = 12,

    /// Riff24Khz16BitMonoPcm stands for riff-24khz-16bit-mono-pcm
    Riff24Khz16BitMonoPcm = 13,

    /// Riff8Khz8BitMonoMULaw stands for riff-8khz-8bit-mono-mulaw
    Riff8Khz8BitMonoMULaw = 14,

    /// Raw16Khz16BitMonoPcm stands for raw-16khz-16bit-mono-pcm
    Raw16Khz16BitMonoPcm = 15,

    /// Raw24Khz16BitMonoPcm stands for raw-24khz-16bit-mono-pcm
    Raw24Khz16BitMonoPcm = 16,

    /// Raw8Khz16BitMonoPcm stands for raw-8khz-16bit-mono-pcm
    Raw8Khz16BitMonoPcm = 17,

    /// Ogg16Khz16BitMonoOpus stands for ogg-16khz-16bit-mono-opus
    Ogg16Khz16BitMonoOpus = 18,

    /// Ogg24Khz16BitMonoOpus stands for ogg-24khz-16bit-mono-opus
    Ogg24Khz16BitMonoOpus = 19,

    /// Raw48Khz16BitMonoPcm stands for raw-48khz-16bit-mono-pcm
    Raw48Khz16BitMonoPcm = 20,

    /// Riff48Khz16BitMonoPcm stands for riff-48khz-16bit-mono-pcm
    Riff48Khz16BitMonoPcm = 21,

    /// Audio48Khz96KBitRateMonoMp3 stands for audio-48khz-96kbitrate-mono-mp3
    Audio48Khz96KBitRateMonoMp3 = 22,

    /// Audio48Khz192KBitRateMonoMp3 stands for audio-48khz-192kbitrate-mono-mp3
    Audio48Khz192KBitRateMonoMp3 = 23,

    /// Ogg48Khz16BitMonoOpus stands for ogg-48khz-16bit-mono-opus
    Ogg48Khz16BitMonoOpus = 24,

    /// Webm16Khz16BitMonoOpus stands for webm-16khz-16bit-mono-opus
    Webm16Khz16BitMonoOpus = 25,

    /// Webm24Khz16BitMonoOpus stands for webm-24khz-16bit-mono-opus
    Webm24Khz16BitMonoOpus = 26,

    /// Raw24Khz16BitMonoTrueSilk stands for raw-24khz-16bit-mono-truesilk
    Raw24Khz16BitMonoTrueSilk = 27,

    /// Raw8Khz8BitMonoALaw stands for raw-8khz-8bit-mono-alaw
    Raw8Khz8BitMonoALaw = 28,

    /// Riff8Khz8BitMonoALaw stands for riff-8khz-8bit-mono-alaw
    Riff8Khz8BitMonoALaw = 29,

    /// webm-24khz-16bit-24kbps-mono-opus
    /// Audio compressed by OPUS codec in a WebM container, with bitrate of 24kbps, optimized for IoT scenario.
    /// (Added in 1.19.0)
    Webm24Khz16Bit24KbpsMonoOpus = 30,

    /// audio-16khz-16bit-32kbps-mono-opus
    /// Audio compressed by OPUS codec without container, with bitrate of 32kbps.
    /// (Added in 1.20.0)
    Audio16Khz16Bit32KbpsMonoOpus = 31,

    /// audio-24khz-16bit-48kbps-mono-opus
    /// Audio compressed by OPUS codec without container, with bitrate of 48kbps.
    /// (Added in 1.20.0)
    Audio24Khz16Bit48KbpsMonoOpus = 32,

    /// audio-24khz-16bit-24kbps-mono-opus
    /// Audio compressed by OPUS codec without container, with bitrate of 24kbps.
    /// (Added in 1.20.0)
    Audio24Khz16Bit24KbpsMonoOpus = 33,

    /// raw-22050hz-16bit-mono-pcm
    /// Raw PCM audio at 22050Hz sampling rate and 16-bit depth.
    /// (Added in 1.22.0)
    Raw22050Hz16BitMonoPcm = 34,

    /// riff-22050hz-16bit-mono-pcm
    /// PCM audio at 22050Hz sampling rate and 16-bit depth, with RIFF header.
    /// (Added in 1.22.0)
    Riff22050Hz16BitMonoPcm = 35,

    /// raw-44100hz-16bit-mono-pcm
    /// Raw PCM audio at 44100Hz sampling rate and 16-bit depth.
    /// (Added in 1.22.0)
    Raw44100Hz16BitMonoPcm = 36,

    /// riff-44100hz-16bit-mono-pcm
    /// PCM audio at 44100Hz sampling rate and 16-bit depth, with RIFF header.
    /// (Added in 1.22.0)
    Riff44100Hz16BitMonoPcm = 37,
}

impl fmt::Display for SpeechSynthesisOutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str =
            match self {
                SpeechSynthesisOutputFormat::Raw8Khz8BitMonoMULaw => "raw-8khz-8bit-mono-mulaw",
                SpeechSynthesisOutputFormat::Riff16Khz16KbpsMonoSiren => "riff-16khz-16kbps-mono-siren",
                SpeechSynthesisOutputFormat::Audio16Khz16KbpsMonoSiren => "audio-16khz-16kbps-mono-siren",
                SpeechSynthesisOutputFormat::Audio16Khz32KBitRateMonoMp3 => "audio-16khz-32kbitrate-mono-mp3",
                SpeechSynthesisOutputFormat::Audio16Khz128KBitRateMonoMp3 => "audio-16khz-128kbitrate-mono-mp3",
                SpeechSynthesisOutputFormat::Audio16Khz64KBitRateMonoMp3 => "audio-16khz-64kbitrate-mono-mp3",
                SpeechSynthesisOutputFormat::Audio24Khz48KBitRateMonoMp3 => "audio-24khz-48kbitrate-mono-mp3",
                SpeechSynthesisOutputFormat::Audio24Khz96KBitRateMonoMp3 => "audio-24khz-96kbitrate-mono-mp3",
                SpeechSynthesisOutputFormat::Audio24Khz160KBitRateMonoMp3 => "audio-24khz-160kbitrate-mono-mp3",
                SpeechSynthesisOutputFormat::Raw16Khz16BitMonoTrueSilk => "raw-16khz-16bit-mono-truesilk",
                SpeechSynthesisOutputFormat::Riff16Khz16BitMonoPcm => "riff-16khz-16bit-mono-pcm",
                SpeechSynthesisOutputFormat::Riff8Khz16BitMonoPcm => "riff-8khz-16bit-mono-pcm",
                SpeechSynthesisOutputFormat::Riff24Khz16BitMonoPcm => "riff-24khz-16bit-mono-pcm",
                SpeechSynthesisOutputFormat::Riff8Khz8BitMonoMULaw => "riff-8khz-8bit-mono-mulaw",
                SpeechSynthesisOutputFormat::Raw16Khz16BitMonoPcm => "raw-16khz-16bit-mono-pcm",
                SpeechSynthesisOutputFormat::Raw24Khz16BitMonoPcm => "raw-24khz-16bit-mono-pcm",
                SpeechSynthesisOutputFormat::Raw8Khz16BitMonoPcm => "raw-8khz-16bit-mono-pcm",
                SpeechSynthesisOutputFormat::Ogg16Khz16BitMonoOpus => "ogg-16khz-16bit-mono-opus",
                SpeechSynthesisOutputFormat::Ogg24Khz16BitMonoOpus => "ogg-24khz-16bit-mono-opus",
                SpeechSynthesisOutputFormat::Raw48Khz16BitMonoPcm => "raw-48khz-16bit-mono-pcm",
                SpeechSynthesisOutputFormat::Riff48Khz16BitMonoPcm => "riff-48khz-16bit-mono-pcm",
                SpeechSynthesisOutputFormat::Audio48Khz96KBitRateMonoMp3 => "audio-48khz-96kbitrate-mono-mp3",
                SpeechSynthesisOutputFormat::Audio48Khz192KBitRateMonoMp3 => "audio-48khz-192kbitrate-mono-mp3",
                SpeechSynthesisOutputFormat::Ogg48Khz16BitMonoOpus => "ogg-48khz-16bit-mono-opus",
                SpeechSynthesisOutputFormat::Webm16Khz16BitMonoOpus => "webm-16khz-16bit-mono-opus",
                SpeechSynthesisOutputFormat::Webm24Khz16BitMonoOpus => "webm-24khz-16bit-mono-opus",
                SpeechSynthesisOutputFormat::Raw24Khz16BitMonoTrueSilk => "raw-24khz-16bit-mono-truesilk",
                SpeechSynthesisOutputFormat::Raw8Khz8BitMonoALaw => "raw-8khz-8bit-mono-alaw",
                SpeechSynthesisOutputFormat::Riff8Khz8BitMonoALaw => "riff-8khz-8bit-mono-alaw",
                SpeechSynthesisOutputFormat::Webm24Khz16Bit24KbpsMonoOpus => "webm-24khz-16bit-24kbps-mono-opus",
                SpeechSynthesisOutputFormat::Audio16Khz16Bit32KbpsMonoOpus => "audio-16khz-16bit-32kbps-mono-opus",
                SpeechSynthesisOutputFormat::Audio24Khz16Bit48KbpsMonoOpus => "audio-24khz-16bit-48kbps-mono-opus",
                SpeechSynthesisOutputFormat::Audio24Khz16Bit24KbpsMonoOpus => "audio-24khz-16bit-24kbps-mono-opus",
                SpeechSynthesisOutputFormat::Raw22050Hz16BitMonoPcm => "raw-22050hz-16bit-mono-pcm",
                SpeechSynthesisOutputFormat::Riff22050Hz16BitMonoPcm => "riff-22050hz-16bit-mono-pcm",
                SpeechSynthesisOutputFormat::Raw44100Hz16BitMonoPcm => "raw-44100hz-16bit-mono-pcm",
                SpeechSynthesisOutputFormat::Riff44100Hz16BitMonoPcm => "riff-44100hz-16bit-mono-pcm",
            };
        write!(f, "{}", str)
    }
}
