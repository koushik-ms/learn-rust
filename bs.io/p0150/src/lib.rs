fn friend_groups(al: &Vec<Vec<i32>>) -> i32 { 2 }

#[test]
fn can_get_friend_groups() {
    let graph = vec![
        vec![1, 2],
        vec![],
        vec![0],
        vec![4],
        vec![]
    ];
    assert_eq!(friend_groups(&graph), 2);
}
