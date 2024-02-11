// https://leetcode.com/problems/flood-fill/

#[allow(dead_code)]
pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    let mut image_mut = image.clone();
    if sr < image.len() as i32 && sc < image[0].len() as i32 {
        // Save our starting pixel
        let scolor = image[sr as usize][sc as usize];

        flood_fill_inner(&mut image_mut, sr, sc, scolor, color);
    }
    image_mut
}

fn flood_fill_inner(image: &mut [Vec<i32>], sr: i32, sc: i32, scolor: i32, color: i32) {
    if image[sr as usize][sc as usize] != color {
        // Set our image color at this pixel
        image[sr as usize][sc as usize] = color;

        // Recurse into the 4-directional pixels

        // Top is sr - 1, sc
        if sr > 0 && image[(sr - 1) as usize][sc as usize] == scolor {
            flood_fill_inner(image, sr - 1, sc, scolor, color);
        }

        // Left is sr, sc - 1
        if sc > 0 && image[sr as usize][(sc - 1) as usize] == scolor {
            flood_fill_inner(image, sr, sc - 1, scolor, color);
        }

        // Bottom is sr + 1, sc
        if sr + 1 < image.len() as i32 && image[(sr + 1) as usize][sc as usize] == scolor {
            flood_fill_inner(image, sr + 1, sc, scolor, color);
        }

        // Right is sr, sc + 1
        if sc + 1 < image[0].len() as i32 && image[sr as usize][(sc + 1) as usize] == scolor {
            flood_fill_inner(image, sr, sc + 1, scolor, color);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::grind::flood_fill::flood_fill;

    #[test]
    fn test_flood_fill_case1() {
        let result = flood_fill(
            [[1, 1, 1].to_vec(), [1, 1, 0].to_vec(), [1, 0, 1].to_vec()].to_vec(),
            1,
            1,
            2,
        );
        assert_eq!(result, [[2, 2, 2], [2, 2, 0], [2, 0, 1]]);
    }

    #[test]
    fn test_flood_fill_case2() {
        let result = flood_fill([[0, 0, 0].to_vec(), [0, 0, 0].to_vec()].to_vec(), 0, 0, 0);
        assert_eq!(result, [[0, 0, 0], [0, 0, 0]]);
    }
}
