use qrcode::QrCode;
use image::Luma;

fn main() {
    // Encode some data into bits.
    let code = QrCode::new(b"http://petewall.net").unwrap();

    // Render the bits into an image.
    let image = code.render::<Luma<u8>>().build();

    // Save the image.
    image.save("qrcode.png").unwrap();
}
