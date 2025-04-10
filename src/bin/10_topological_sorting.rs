use std::io::stdin;

use code_run_rust::{pprinters, tokens};

fn main() {
    let mut t = tokens::Tokens::from_reader(stdin());

    let nodes_count: usize = t.next_token().unwrap().parse().unwrap();
    let edges_count: usize = t.next_token().unwrap().parse().unwrap();

    let mut get_node_num = || -> u32 {
        t.next_token()
            .unwrap()
            .parse::<u32>()
            .unwrap()
            .checked_sub(1)
            .unwrap()
    };

    let edges_reader = (0..edges_count).map(|_| (get_node_num(), get_node_num()));

    if let Some(topological_order) = solve(nodes_count, edges_reader) {
        pprinters::seq_println(topological_order.iter().map(|&x| x + 1), " ");
    } else {
        println!("-1");
    }
}

pub fn solve(nodes_count: usize, edges: impl IntoIterator<Item = (u32, u32)>) -> Option<Vec<u32>> {
    // topological order of nodes
    let mut t = Vec::with_capacity(nodes_count);
    // nodes incoming edges count
    let mut i = vec![0; nodes_count];
    // g[i] => (all linked nodes to node i)
    let mut g = vec![Vec::with_capacity(nodes_count); nodes_count];

    // fill "G" and "I"
    for (n1, n2) in edges {
        g[n1 as usize].push(n2);
        i[n2 as usize] += 1;
    }

    loop {
        let mut no_more_zero_nodes = true;

        for n in 0..nodes_count {
            if i[n] == 0 {
                no_more_zero_nodes = false;
                t.push(n as u32);
                i[n] = -1;

                for &n in &g[n] {
                    i[n as usize] -= 1;
                }
            }
        }

        if no_more_zero_nodes {
            break;
        }
    }

    (t.len() == nodes_count).then_some(t)
}
