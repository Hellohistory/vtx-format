//! vtx-format：统一定义 .vtx 包格式（编码/解码）
//!
//! v1 格式：
//! - Header: 4 bytes = b"VTX\x01"
//! - Payload: component bytes (WebAssembly Component)

use thiserror::Error;

pub const VTX_PREFIX: [u8; 3] = [0x56, 0x54, 0x58]; // "VTX"
pub const VTX_VERSION_V1: u8 = 0x01;
pub const VTX_MAGIC_V1: [u8; 4] = [VTX_PREFIX[0], VTX_PREFIX[1], VTX_PREFIX[2], VTX_VERSION_V1];

#[derive(Debug, Error)]
pub enum VtxFormatError {
    #[error("vtx file too short")]
    TooShort,

    #[error("invalid vtx prefix (expected 'VTX')")]
    InvalidPrefix,

    #[error("unsupported vtx version: {0}")]
    UnsupportedVersion(u8),
}

/// 编码 v1：VTX_MAGIC_V1 + component bytes
pub fn encode_v1(component_bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(VTX_MAGIC_V1.len() + component_bytes.len());
    out.extend_from_slice(&VTX_MAGIC_V1);
    out.extend_from_slice(component_bytes);
    out
}

/// 解码：返回 (version, component_bytes_slice)
pub fn decode(bytes: &[u8]) -> Result<(u8, &[u8]), VtxFormatError> {
    if bytes.len() < 4 {
        return Err(VtxFormatError::TooShort);
    }
    if bytes[0..3] != VTX_PREFIX {
        return Err(VtxFormatError::InvalidPrefix);
    }

    let version = bytes[3];
    match version {
        VTX_VERSION_V1 => Ok((version, &bytes[4..])),
        other => Err(VtxFormatError::UnsupportedVersion(other)),
    }
}
