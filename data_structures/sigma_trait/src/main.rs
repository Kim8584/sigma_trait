pub trait Permutable {
    fn permute(&self) -> PermutedList;
}

pub struct PermutedList {
    original: Vec<i32>,
    permuted: Vec<i32>,
}

impl PermutedList {
    fn sigma(&self, index: usize) -> usize {
        let original_index = self.original[index];
        self.permuted
            .iter()
            .position(|&x| x == original_index)
            .unwrap()
    }
}

impl Permutable for Vec<i32> {
    fn permute(&self) -> PermutedList {
        let mut permuted = self.clone();
        permuted.rotate_left(1);
        PermutedList {
            original: self.clone(),
            permuted,
        }
    }
}

fn main() {
    let original_list = vec![1, 2, 3, 4, 5];
    let permuted_list = original_list.permute();

    println!("Original list: {:?}", original_list);
    println!("Permuted list: {:?}", permuted_list.permuted);

    println!("Sigma(0) = {}", permuted_list.sigma(0));
    println!("Sigma(1) = {}", permuted_list.sigma(1));
    println!("Sigma(2) = {}", permuted_list.sigma(2));
    println!("Sigma(3) = {}", permuted_list.sigma(3));
    println!("Sigma(4) = {}", permuted_list.sigma(4));
}
