//! Temporary helper until we have a proper wallet.

use anoma_shared::types::key::ed25519::{Keypair, PublicKey};

pub fn adrian_keypair() -> Keypair {
    let bytes = [
        46, 197, 87, 225, 97, 76, 234, 148, 246, 211, 140, 73, 88, 77, 8, 28,
        16, 101, 140, 48, 80, 66, 111, 92, 101, 202, 137, 122, 135, 97, 144,
        252, 229, 150, 2, 227, 109, 48, 92, 128, 71, 179, 63, 73, 106, 183,
        163, 113, 143, 41, 76, 10, 17, 124, 210, 252, 123, 63, 54, 50, 127,
        209, 91, 12,
    ];
    Keypair::from_bytes(&bytes).unwrap()
}
pub fn alberto_keypair() -> Keypair {
    let bytes = [
        48, 254, 50, 93, 93, 56, 35, 136, 154, 211, 104, 190, 69, 179, 53, 236,
        120, 248, 51, 51, 122, 51, 96, 68, 132, 166, 103, 189, 100, 170, 30,
        124, 69, 93, 237, 69, 173, 75, 79, 149, 3, 101, 49, 169, 71, 161, 1,
        77, 66, 79, 136, 175, 59, 77, 213, 246, 186, 143, 211, 255, 38, 88,
        105, 149,
    ];
    Keypair::from_bytes(&bytes).unwrap()
}
pub fn ash_keypair() -> Keypair {
    let bytes = [
        14, 142, 126, 177, 63, 134, 48, 249, 247, 222, 178, 47, 6, 150, 220,
        109, 183, 220, 139, 13, 170, 95, 235, 1, 99, 87, 183, 174, 96, 63, 169,
        30, 27, 245, 179, 197, 0, 61, 69, 127, 190, 157, 213, 45, 173, 61, 37,
        162, 112, 213, 127, 121, 43, 82, 141, 98, 133, 21, 214, 199, 186, 217,
        165, 194,
    ];
    Keypair::from_bytes(&bytes).unwrap()
}
pub fn awa_keypair() -> Keypair {
    let bytes = [
        122, 224, 26, 240, 40, 175, 69, 139, 78, 11, 49, 60, 133, 89, 93, 56,
        76, 24, 195, 183, 247, 0, 56, 7, 160, 167, 89, 86, 15, 192, 189, 222,
        181, 204, 69, 171, 196, 3, 168, 67, 184, 4, 116, 40, 174, 106, 29, 12,
        45, 103, 37, 131, 139, 78, 26, 101, 138, 87, 164, 150, 90, 67, 36, 18,
    ];
    Keypair::from_bytes(&bytes).unwrap()
}
pub fn celso_keypair() -> Keypair {
    let bytes = [
        116, 123, 231, 91, 245, 21, 36, 210, 31, 12, 51, 99, 47, 236, 178, 93,
        112, 221, 239, 64, 58, 6, 246, 119, 131, 137, 183, 30, 115, 9, 137, 1,
        68, 193, 194, 136, 8, 74, 43, 142, 150, 226, 17, 199, 213, 197, 223,
        183, 160, 221, 230, 222, 71, 119, 120, 152, 227, 208, 62, 97, 177, 27,
        49, 221,
    ];
    Keypair::from_bytes(&bytes).unwrap()
}
pub fn chris_keypair() -> Keypair {
    let bytes = [
        93, 216, 81, 16, 6, 221, 205, 207, 199, 32, 26, 92, 43, 73, 79, 223,
        207, 85, 185, 148, 28, 51, 173, 132, 48, 41, 58, 243, 83, 205, 155,
        240, 136, 53, 203, 143, 122, 196, 42, 220, 230, 194, 209, 127, 224,
        164, 212, 235, 227, 8, 123, 253, 146, 34, 113, 202, 160, 207, 151, 82,
        179, 207, 197, 53,
    ];
    Keypair::from_bytes(&bytes).unwrap()
}
pub fn gabriella_keypair() -> Keypair {
    let bytes = [
        215, 143, 59, 43, 144, 206, 98, 52, 195, 175, 45, 110, 80, 157, 147,
        84, 153, 31, 166, 76, 126, 166, 249, 211, 155, 48, 80, 75, 230, 20, 69,
        51, 95, 37, 77, 142, 234, 183, 214, 92, 154, 5, 148, 81, 90, 60, 181,
        31, 90, 175, 6, 121, 27, 109, 221, 27, 221, 211, 34, 102, 119, 209, 19,
        3,
    ];
    Keypair::from_bytes(&bytes).unwrap()
}
pub fn gianmarco_keypair() -> Keypair {
    let bytes = [
        230, 126, 34, 102, 84, 215, 201, 105, 2, 214, 202, 205, 124, 183, 34,
        46, 231, 187, 138, 69, 201, 15, 205, 71, 148, 139, 84, 184, 177, 74,
        119, 33, 14, 26, 135, 56, 103, 242, 1, 42, 11, 193, 148, 241, 192, 25,
        200, 170, 102, 200, 199, 159, 216, 17, 100, 196, 123, 3, 198, 112, 206,
        4, 57, 187,
    ];
    Keypair::from_bytes(&bytes).unwrap()
}
pub fn joe_keypair() -> Keypair {
    let bytes = [
        48, 183, 6, 16, 86, 21, 0, 175, 39, 187, 61, 30, 159, 38, 48, 225, 49,
        172, 118, 14, 199, 62, 76, 240, 205, 255, 91, 107, 9, 55, 41, 142, 215,
        178, 242, 182, 97, 216, 110, 217, 241, 33, 105, 148, 91, 195, 84, 76,
        51, 43, 90, 98, 214, 200, 10, 95, 89, 146, 25, 48, 93, 39, 85, 175,
    ];
    Keypair::from_bytes(&bytes).unwrap()
}
pub fn nat_keypair() -> Keypair {
    let bytes = [
        168, 202, 83, 107, 217, 80, 95, 175, 248, 140, 143, 111, 235, 30, 231,
        30, 250, 154, 58, 160, 124, 141, 101, 184, 52, 154, 240, 113, 33, 122,
        187, 219, 179, 202, 238, 235, 134, 202, 64, 90, 82, 223, 12, 34, 79,
        55, 234, 161, 1, 131, 118, 174, 221, 222, 74, 115, 2, 142, 207, 66,
        129, 230, 42, 48,
    ];
    Keypair::from_bytes(&bytes).unwrap()
}
pub fn simon_keypair() -> Keypair {
    let bytes = [
        228, 46, 32, 49, 87, 253, 60, 248, 100, 201, 43, 170, 138, 139, 243,
        77, 190, 214, 77, 71, 17, 109, 230, 102, 228, 105, 51, 123, 80, 106,
        113, 189, 209, 45, 229, 77, 30, 154, 254, 21, 82, 188, 40, 190, 87,
        180, 236, 117, 247, 248, 145, 42, 125, 14, 110, 131, 178, 2, 166, 190,
        33, 44, 212, 143,
    ];
    Keypair::from_bytes(&bytes).unwrap()
}
pub fn sylvain_keypair() -> Keypair {
    let bytes = [
        169, 57, 53, 27, 78, 225, 171, 85, 143, 215, 151, 117, 170, 194, 17,
        71, 120, 206, 67, 106, 154, 211, 189, 118, 30, 253, 210, 218, 80, 15,
        119, 118, 66, 156, 197, 204, 90, 195, 59, 68, 41, 253, 250, 221, 92,
        195, 30, 231, 140, 232, 61, 108, 255, 183, 39, 166, 72, 17, 54, 176,
        92, 150, 235, 66,
    ];
    Keypair::from_bytes(&bytes).unwrap()
}
pub fn tomas_keypair() -> Keypair {
    let bytes = [
        114, 65, 33, 33, 5, 254, 103, 8, 82, 142, 19, 43, 45, 5, 101, 159, 19,
        13, 102, 249, 206, 65, 245, 78, 166, 65, 6, 202, 62, 184, 46, 253, 249,
        250, 251, 32, 59, 210, 91, 140, 85, 199, 7, 202, 129, 188, 184, 204,
        37, 37, 185, 68, 154, 196, 24, 105, 148, 231, 133, 140, 204, 84, 13,
        175,
    ];
    Keypair::from_bytes(&bytes).unwrap()
}
pub fn yuji_keypair() -> Keypair {
    let bytes = [
        10, 181, 53, 121, 14, 60, 134, 42, 222, 250, 61, 129, 192, 32, 128,
        226, 218, 4, 166, 42, 25, 63, 229, 141, 224, 244, 139, 186, 206, 127,
        52, 14, 18, 58, 239, 63, 213, 15, 120, 61, 253, 244, 147, 162, 170,
        221, 59, 144, 58, 147, 151, 158, 180, 48, 160, 104, 170, 117, 91, 97,
        18, 78, 185, 6,
    ];
    Keypair::from_bytes(&bytes).unwrap()
}

pub fn matchmaker_keypair() -> Keypair {
    // generated from [`tests::temp_gen_keypair`]
    let bytes = [
        91, 67, 244, 37, 241, 33, 157, 218, 37, 172, 191, 122, 75, 2, 44, 219,
        28, 123, 44, 34, 9, 240, 244, 49, 112, 192, 180, 98, 142, 160, 182, 14,
        244, 254, 3, 176, 211, 19, 15, 7, 126, 77, 81, 204, 119, 72, 186, 172,
        153, 135, 80, 71, 107, 239, 153, 74, 10, 115, 172, 78, 125, 24, 49,
        104,
    ];
    Keypair::from_bytes(&bytes).unwrap()
}

pub fn matchmaker_pk() -> PublicKey {
    PublicKey::from(matchmaker_keypair().public)
}

pub fn key_of(name: impl AsRef<str>) -> Keypair {
    match name.as_ref() {
        "a1qq5qqqqqxgeyzdeng4zrw33sxez5y3p3xqerz3psx5e5x32rg3rryw2ygc6yy3p4xpq5gvfnw3nwp8" => adrian_keypair(),
        "a1qq5qqqqq8yerw3jxx565y333gfpnjwzygcc5zd6xxarr2dzzgcm5xv3kxazrjve589p5vv34vl0yy3" => alberto_keypair(),
        "a1qq5qqqqqxue5vs69xc6nwvfcgdpyy3pnxv6rxw2zx3zryv33gyc5xdekxaryydehgvunsvzz2hjedu" => ash_keypair(),
        "a1qq5qqqqqg565zv34gcc52v3nxumr23z9gezrj3pnx56rwse4xc6yg3phgcun2d33xyenqv2x4xyw62" => awa_keypair(),
        "a1qq5qqqqq8qmrwsjyxcerqwzpx9pnzve3gvc5xw29gdqnvv2yx5mrvsjpxgcrxv6pg5engvf5hgjscj" => celso_keypair(),
        "a1qq5qqqqqgye5xwpcxqu5z3p4g5ens3zr8qm5xv69xfznvwzzx4p5xwpkxc6n2v6x8yc5gdpeezdqc4" => chris_keypair(),
        "a1qq5qqqqq8ycn2djrxqmnyd3sxcunsv2zgyeyvwzpgceyxdf3xyu5gv2pgeprxdfe8ycrzwzzkezpcp" => gabriella_keypair(),
        "a1qq5qqqqq89prqsf38qcrzd6zxym5xdfjg4pyg3pjg3pyx32zg5u5y3jpgc65zdej8pznwwf3jqzsws" => gianmarco_keypair(),
        "a1qq5qqqqqgvuyv335g9z5v32xgdz523zxgsuy23fjxazrjve5g4pnydphxyu5v33cxarrzd692045xh" => joe_keypair(),
        "a1qq5qqqqq89rygsejx9q5yd6pxpp5x3f38ymyydp3xcu523zzx4prw3fc8qu5vvjpxyeyydpnfha6qt" => nat_keypair(),
        "a1qq5qqqqqgfqnqdecxcurq33hxcey2sf4g5mygdjyxfrrjse4xyc52vpjxyenwve4gv6njsecz4tzen" => simon_keypair(),
        "a1qq5qqqqqgccnyvp3gyergvp5xgmr2s3s8yung3f4gdq52wzpxvurysfhgycnwd29xfryxvekfwc00t" => sylvain_keypair(),
        "a1qq5qqqqqggcrzsfj8ym5g3psxuurxv2yxseyxwpsxdpy2s35gsc5zdzpx9pyxde48ppnqd3cnzlava" => tomas_keypair(),
        "a1qq5qqqqqgvcrz3f5x4prssj9x5enydecxfznzdj9g5cnj3fcxarrxdjpx5cnwv69xye5vvfeva4z85" => yuji_keypair(),

        "a1qq5qqqqqxu6rvdzpxymnqwfkxfznvsjxggunyd3jg5erg3p3geqnvv35gep5yvzxx5m5x3fsfje8td" => matchmaker_keypair(),
        other => {
            panic!("Dont' have keys for: {}", other)
        }
    }
}

#[cfg(test)]
mod tests {
    use anoma_shared::types::key::ed25519::Keypair;
    use rand::prelude::ThreadRng;
    use rand::thread_rng;

    /// Run `cargo test temp_gen_keypair -- --nocapture` to generate a keypair.
    #[test]
    fn temp_gen_keypair() {
        let mut rng: ThreadRng = thread_rng();
        for _ in 0..14 {
            let keypair = Keypair::generate(&mut rng);
            println!("keypair {:?}", keypair.to_bytes());
        }
    }
}
