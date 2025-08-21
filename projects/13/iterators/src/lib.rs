#[derive(PartialEq, Debug)]
pub struct Shoes {
    size: u32,
    style: String,
}

pub fn shoe_in_size(shoes: Vec<Shoes>, shoe_in_size: u32) -> Vec<Shoes> {
    shoes
        .into_iter()
        .filter(|s| s.size == shoe_in_size)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoes {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoes {
                size: 13,
                style: String::from("sandal"),
            },
            Shoes {
                size: 10,
                style: String::from("boot"),
            },
        ];
        let in_my_size = shoe_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoes {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoes {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        )
    }
}
