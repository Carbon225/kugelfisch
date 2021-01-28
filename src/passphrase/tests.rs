use super::*;

#[test]
fn derive_1234() {
    let key = derive_key("1234");
    let expected = [2769720036, 2049480382, 2835870842, 3476027172, 1390849281, 4111826761, 1262358725, 1457655305, 25635442, 3929043875, 4289544898, 2391540125, 121315033, 3602004245];
    assert_eq!(key, expected);
}
