use b_plus_tree::BPlusTree;

fn main() {
    println!("=== B+ Tree 테스트 ===");
    let tree: BPlusTree<i32, String> = BPlusTree::new(3);
    println!("B+ 트리가 생성되었습니다");
} 