/// Split the first 21 bits of [n] by 3.
/// Implementation according to Jeroen Beart's [blog post](https://www.forceflow.be/2013/10/07/morton-encodingdecoding-through-bit-interleaving-implementations/).
///
fn split_by_3(n: u32) -> u64 {
    // TODO: check if n exceeds the first 21 bits
    let n = n as u64;
    let mut x: u64 = n & 0x1fffff; // we only look at the first 21 bits
    x = (x | (x << 32)) & 0x1f00000000ffff; // shift left 32 bits, OR with self, and 00011111000000000000000000000000000000001111111111111111
    x = (x | (x << 16)) & 0x1f0000ff0000ff; // shift left 32 bits, OR with self, and 00011111000000000000000011111111000000000000000011111111
    x = (x | (x << 8)) & 0x100f00f00f00f00f; // shift left 32 bits, OR with self, and 0001000000001111000000001111000000001111000000001111000000000000
    x = (x | (x << 4)) & 0x10c30c30c30c30c3; // shift left 32 bits, OR with self, and 0001000011000011000011000011000011000011000011000011000100000000
    x = (x | (x << 2)) & 0x1249249249249249;

    x
}

pub fn morton_encode(x: u32, y: u32, z: u32) -> u64 {
    let mut answer: u64 = 0;
    answer |= split_by_3(x) | (split_by_3(y) << 1) | (split_by_3(z) << 2);

    answer
}

/*#[cfg(test)]
mod morton_encode_test {
    use crate::coords::morton::morton_encode;


    #[test]
    fn main() {
        let x = 5;
        let y = 1;
        let z = 0;
        let morton_index = morton_encode(x, y, z);
        println!("Morton index for ({:b}, {:b}, {:b}): {:b}", x, y, z, morton_index);
    }

    #[test]
    fn test_order() {
        let mut morton_codes: Vec<(u64, u32, u32, u32)> = Vec::new();

        let size = 3;
        for current_x in 0..size {
            for current_y in 0..size {
                for current_z in 0..size {
                    let encoded = morton_encode(current_x, current_y, current_z);
                    morton_codes.push((encoded, current_x, current_y, current_z));
                }
            }
        }

        morton_codes.sort_by_key(|k| k.0);
        for morton in morton_codes.iter() {
            println!("(x: {}, y: {}, z: {}) {}: bits {:b}", morton.1, morton.2, morton.3, morton.0, morton.0);
        }
    }
}*/
