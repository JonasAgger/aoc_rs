pub trait Denominators {
    type Item;

    fn gcd(&self, other: Self::Item) -> Self::Item;
    fn lcm(&self, other: Self::Item) -> Self::Item;
}

impl Denominators for usize {
    type Item = Self;

    fn gcd(&self, other: Self::Item) -> Self::Item {
        let mut a = *self;
        let mut b = other;

        while b != 0 {
            let tmp = b;
            b = a % b;
            a = tmp;
        }

        a
    }

    fn lcm(&self, other: Self::Item) -> Self::Item {
        self * other / self.gcd(other)
    }
}

pub fn lcm_multiple<Item: Denominators<Item = Item> + Copy>(items: &[Item]) -> Item {
    let first = items[0].lcm(items[1]);
    items.iter().skip(2).fold(first, |acc, next| next.lcm(acc))
}
