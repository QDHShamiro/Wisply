use std::path::Path;
use std::process::Stdio;

use crate::config::Config;
use crate::spawn;

pub fn decode_args(video: &Path, start: Option<f64>, end: Option<f64>, rate: u32) -> Vec<String> {
    let mut args: Vec<String> = ["-v", "error", "-nostdin"].map(String::from).to_vec();
    if let Some(start) = start {
        args.push("-ss".into());
        args.push(format!("{start}"));
    }
    if let Some(end) = end {
        args.push("-to".into());
        args.push(format!("{end}"));
    }
    args.push("-i".into());
    args.push(video.display().to_string());
    for part in ["-vn", "-ac", "1", "-ar"] {
        args.push(part.into());
    }
    args.push(rate.to_string());
    for part in ["-f", "f32le", "-"] {
        args.push(part.into());
    }
    args
}

pub fn parse_f32le(bytes: &[u8]) -> Vec<f32> {
    let (chunks, _) = bytes.as_chunks::<4>();
    chunks.iter().map(|c| f32::from_le_bytes(*c)).collect()
}

pub fn decode(config: &Config, video: &Path, start: Option<f64>, end: Option<f64>) -> Result<Vec<f32>, String> {
    let args = decode_args(video, start, end, config.audio.sample_rate);
    let output = spawn::quiet(&config.paths.ffmpeg)
        .args(&args)
        .stdin(Stdio::null())
        .output()
        .map_err(|e| config.message("ffmpeg_failed", &[("detail", &e.to_string())]))?;
    if !output.status.success() {
        let detail = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(config.message("ffmpeg_failed", &[("detail", &detail)]));
    }
    let samples = parse_f32le(&output.stdout);
    if samples.is_empty() {
        let from = start.unwrap_or(0.0).to_string();
        let to = end.map(|e| e.to_string()).unwrap_or_default();
        return Err(config.message("no_audio", &[("start", &from), ("end", &to)]));
    }
    Ok(samples)
}

pub fn wav_bytes(samples: &[f32], rate: u32) -> Vec<u8> {
    let data_len = (samples.len() * 2) as u32;
    let mut out = Vec::with_capacity(44 + data_len as usize);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data_len).to_le_bytes());
    out.extend_from_slice(b"WAVEfmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&rate.to_le_bytes());
    out.extend_from_slice(&(rate * 2).to_le_bytes());
    out.extend_from_slice(&2u16.to_le_bytes());
    out.extend_from_slice(&16u16.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_len.to_le_bytes());
    for sample in samples {
        let value = (sample.clamp(-1.0, 1.0) * 32767.0).round() as i16;
        out.extend_from_slice(&value.to_le_bytes());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_little_endian_floats() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&0.5f32.to_le_bytes());
        bytes.extend_from_slice(&(-1.0f32).to_le_bytes());
        bytes.push(7);
        assert_eq!(parse_f32le(&bytes), vec![0.5, -1.0]);
    }

    #[test]
    fn wav_has_header_and_clamps() {
        let wav = wav_bytes(&[0.0, 2.0, -2.0], 16000);
        assert_eq!(&wav[0..4], b"RIFF");
        assert_eq!(wav.len(), 44 + 6);
        assert_eq!(i16::from_le_bytes([wav[46], wav[47]]), 32767);
        assert_eq!(i16::from_le_bytes([wav[48], wav[49]]), -32767);
    }

    #[test]
    fn decode_args_place_seek_before_input() {
        let args = decode_args(Path::new("a.mp4"), Some(5.0), Some(9.5), 16000);
        let seek = args.iter().position(|a| a == "-ss").unwrap();
        let input = args.iter().position(|a| a == "-i").unwrap();
        assert!(seek < input);
        assert!(args.contains(&"9.5".to_string()));
        assert_eq!(args.last().unwrap(), "-");
    }

    #[test]
    fn decode_args_without_range() {
        let args = decode_args(Path::new("a.mp4"), None, None, 16000);
        assert!(!args.contains(&"-ss".to_string()));
        assert!(!args.contains(&"-to".to_string()));
    }
}
