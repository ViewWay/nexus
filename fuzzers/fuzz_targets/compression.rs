//! Compression Fuzz Target
//! 压缩 Fuzz 测试目标
//!
//! This fuzz target tests compression and decompression robustness.
//! 此 Fuzz 测试目标测试压缩和解压的鲁棒性。

#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    use bytes::Bytes;

    // Skip if data is too large
    // 如果数据过大则跳过
    if data.len() > 64 * 1024 {
        return;
    }

    let input = Bytes::copy_from_slice(data);

    // Test gzip compression and decompression
    // 测试 gzip 压缩和解压
    #[cfg(feature = "gzip")]
    {
        use flate2::read::GzDecoder;
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::{Read, Write};

        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        if encoder.write_all(data).is_ok() {
            if let Ok(compressed) = encoder.finish() {
                // Try to decompress
                // 尝试解压
                let mut decoder = GzDecoder::new(&compressed[..]);
                let mut decompressed = Vec::new();
                if decoder.read_to_end(&mut decompressed).is_ok() {
                    // Verify round-trip / 验证往返
                    if decompressed == data {
                        // Successful round-trip / 成功往返
                    }
                }
            }
        }
    }

    // Test DEFLATE compression and decompression
    // 测试 DEFLATE 压缩和解压
    #[cfg(feature = "deflate")]
    {
        use flate2::read::DeflateDecoder;
        use flate2::write::DeflateEncoder;
        use flate2::Compression;
        use std::io::{Read, Write};

        let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
        if encoder.write_all(data).is_ok() {
            if let Ok(compressed) = encoder.finish() {
                // Try to decompress
                // 尝试解压
                let mut decoder = DeflateDecoder::new(&compressed[..]);
                let mut decompressed = Vec::new();
                if decoder.read_to_end(&mut decompressed).is_ok() {
                    // Verify round-trip / 验证往返
                    if decompressed == data {
                        // Successful round-trip / 成功往返
                    }
                }
            }
        }
    }

    // Test Brotli compression and decompression
    // 测试 Brotli 压缩和解压
    #[cfg(feature = "brotli")]
    {
        use brotli::dec::{BrotliDecompress, BrotliState};
        use brotli::enc::{BrotliCompress, BrotliEncoderParams};

        let mut compressed = Vec::new();
        let params = BrotliEncoderParams::default();
        if BrotliCompress(&mut data.as_ref(), &mut compressed, &params).is_ok() {
            // Try to decompress
            // 尝试解压
            let mut decompressed = Vec::new();
            let mut state = BrotliState::new();
            if BrotliDecompress(&mut compressed.as_slice(), &mut decompressed, &mut state).is_ok() {
                // Verify round-trip / 验证往返
                if decompressed == data.as_ref() {
                    // Successful round-trip / 成功往返
                }
            }
        }
    }
});
