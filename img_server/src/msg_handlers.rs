use crate::structs::GetImageRequest;
use crate::State;
use crate::helpers;
use crate::kiprintln;
use crate::save_state;
use crate::URI;

pub fn get_img(get_image_request: GetImageRequest, state: &mut State) -> anyhow::Result<Vec<u8>> {
    let uri = get_image_request.uri;
    Ok(state.images.get(&uri).cloned().ok_or(anyhow::anyhow!("Image not found"))?)
}

pub fn upload_img(state: &mut State) -> anyhow::Result<URI> {
    let jpeg_bytes = helpers::get_jpeg_bytes()?;
    let hash_hex = helpers::calculate_sha256_hash(&jpeg_bytes);

    kiprintln!("SHA-256 hash of JPEG: {}", hash_hex);
    state.images.insert(hash_hex.clone(), jpeg_bytes);
    save_state(state)?;
    Ok(hash_hex)
}

// TODO: Responses