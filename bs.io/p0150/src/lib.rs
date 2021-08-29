use std::collections::HashSet;

fn root(ids: &mut Vec<usize>, mut i: usize) -> usize {
    while i != ids[i] {
        ids[i] = ids[ids[i]]; // path-compression
        i = ids[i];
    }
    i
}

fn connect(ids: &mut Vec<usize>, from: usize, to:usize) {
    let i = root(ids, from);
    let j = root(ids, to);
    ids[i] = j;
}

pub fn friend_groups(al: &Vec<Vec<usize>>) -> usize { 
    let n = al.len();
    let mut ids = (0..n).collect();
    for (i,fl) in al.iter().enumerate() {
        for &k in fl {
            if root(&mut ids, i) != root(&mut ids, k) { connect(&mut ids, i, k); }
        }
    }
    let mut fg = HashSet::new();
    for i in 0..n {
        fg.insert(root(&mut ids, i));
    }
    fg.len()
}

struct Graph {
    ids: Vec<usize>
}


#[test]
fn can_get_friend_groups() {
    let adjacency_list = vec![
        vec![1, 2],
        vec![],
        vec![0],
        vec![4],
        vec![]
    ];
    assert_eq!(friend_groups(&adjacency_list), 2);
}

#[test]
fn a_temp_test() {
    let adjacency_list = vec![
        vec![1, 2],
        vec![],
        vec![0],
        vec![4],
        vec![]
    ];
    let g = Graph::with_nodes(adjacency_list.len());
    for (i,fl) in adjacency_list.iter().enumerate() {
        for &k in fl {
            if g.root(i) != g.root(k) { g.connect(i, k); }
        }
    }
    assert_eq!(g.roots().len(), 2);
}
