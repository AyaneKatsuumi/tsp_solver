/// 貪欲法でオイラー閉路を生成する.
/// ただし、始点と終点は必ず1つ目の点である.
///
/// * `n` - the count for points.
/// * `x` - X-coordinates of points.
/// * `y` - Y-coordinates of points.
pub fn greedy(n: usize, x: Vec<isize>, y: Vec<isize>) -> (Vec<usize>, Vec<Vec<usize>>) {
    let mut dist = vec![vec![0; n]; n];

    // 距離の2乗を算出
    for i in 0..n {
        for j in i + 1..n {
            let dist2 = ((x[i] - x[j]) * (x[i] - x[j]) + (y[i] - y[j]) * (y[i] - y[j])) as usize;
            dist[i][j] = dist2;
            dist[j][i] = dist2;
        }
    }

    let mut visited = vec![false; n];
    let mut route = Vec::<usize>::new();
    let mut dist4 = Vec::<usize>::new();
    let mut here = 0;
    route.push(1);
    dist4.push(0);

    for _ in 1..n {
        let mut dist3 = std::usize::MAX;
        let mut there = 0;
        for j in 1..n {
            if visited[j] == true {
                continue;
            }
            if dist[here][j] < dist3 {
                dist3 = dist[here][j];
                there = j;
            }
        }
        visited[there] = true;
        route.push(there + 1);
        dist4.push(dist3);
        here = there;
    }
    route.push(1);
    dist4.push(dist[here][0]);

    (route, dist)
}

/// Christofidesのアルゴリズムでオイラー閉路の経路を生成する.
/// 最小全域木の生成にはKruscal法を使用する.
/// ただし、始点と終点は必ず1つ目の点である.
///
/// * `n` - the count for points.
/// * `x` - X-coordinates of points.
/// * `y` - Y-coordinates of points.
pub fn christofides_kruscal(
    n: usize,
    x: Vec<isize>,
    y: Vec<isize>,
) -> (Vec<usize>, Vec<Vec<usize>>) {
    let mut dist = vec![vec![0; n]; n];
    let mut dist_i_j: Vec<(usize, usize, usize)> = Vec::<(usize, usize, usize)>::new();
    let mut unionfind: Vec<Vec<usize>> = Vec::<Vec<usize>>::new();

    // 距離の2乗を算出
    for i in 0..n {
        for j in i + 1..n {
            let dist2 = ((x[i] - x[j]) * (x[i] - x[j]) + (y[i] - y[j]) * (y[i] - y[j])) as usize;
            dist[i][j] = dist2;
            dist[j][i] = dist2;
            dist_i_j.push((dist2, i + 1, j + 1));
        }
    }
    dist_i_j.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut wood = Vec::<(usize, usize)>::new();
    // 最小全域木の導出
    for d in dist_i_j {
        let group1 = find(d.1, &unionfind);
        let group2 = find(d.2, &unionfind);
        if group1 == group2 && group1 != -1 {
            continue;
        }
        wood.push((d.1, d.2));
        union(d.1, d.2, group1, group2, &mut unionfind);
        if wood.len() == n - 1 {
            break;
        }
    }

    // 接続する辺の数が奇数の頂点の集合Tを取得
    let mut counts = vec![0; n];
    let mut odd_points = Vec::<usize>::new();
    for edge in &wood {
        counts[edge.0 - 1] += 1;
        counts[edge.1 - 1] += 1;
    }

    for c in counts.iter().enumerate() {
        if c.1 % 2 == 1 {
            odd_points.push(c.0);
        }
    }

    // Tを端点とする最小重み完全マッチングMを取得する
    let mut matching = Vec::<(usize, usize)>::new();
    let mut dist_i_j2: Vec<(usize, usize, usize)> = Vec::<(usize, usize, usize)>::new();
    for i in 0..odd_points.len() {
        for j in i + 1..odd_points.len() {
            let ii = odd_points[i];
            let jj = odd_points[j];
            dist_i_j2.push((dist[ii - 1][jj - 1], ii, jj));
        }
    }

    dist_i_j2.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let mut checked = Vec::<usize>::new();
    for d in dist_i_j2 {
        if checked.contains(&d.1) || checked.contains(&d.2) {
            continue;
        }
        matching.push((d.1, d.2));
        checked.push(d.1);
        checked.push(d.2);
        if checked.len() == odd_points.len() {
            break;
        }
    }

    wood.append(&mut matching);
    wood.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut route = Vec::<usize>::new();
    let mut visited = vec![false; n];
    route.push(1);
    let mut here = 1;
    let mut idx_ins = 1;
    visited[0] = true;
    loop {
        let mut next = 0;
        let mut idx = match wood.binary_search_by_key(&here, |&(a, _b)| a) {
            Ok(a) => a,
            Err(_) => usize::max_value(),
        };
        if idx == usize::max_value() {
            for (i, branch) in wood.iter().enumerate() {
                if branch.1 == here {
                    next = branch.0;
                    idx = i;
                    break;
                }
            }
            if idx == usize::max_value() {
                for branch in wood.iter().enumerate() {
                    for r in route.iter().enumerate() {
                        if *r.1 == branch.1 .0 {
                            next = branch.1 .1;
                            idx_ins = r.0 + 1;
                            idx = branch.0;
                            break;
                        } else if *r.1 == branch.1 .1 {
                            next = branch.1 .0;
                            idx_ins = r.0 + 1;
                            idx = branch.0;
                            break;
                        }
                    }
                    if idx != usize::max_value() {
                        break;
                    }
                }
            }
        } else {
            next = wood[idx].1;
        }
        wood.remove(idx);

        if !visited[next - 1] {
            route.insert(idx_ins, next);
            idx_ins += 1;
            visited[next - 1] = true;
        }

        if wood.len() == 0 {
            break;
        }

        here = next;
    }
    route.push(1);
    (route, dist)
}

fn find(index: usize, unionfind: &Vec<Vec<usize>>) -> isize {
    for (i, group) in unionfind.iter().enumerate() {
        if group.contains(&index) {
            return i as isize;
        }
    }
    -1
}

fn union(
    index1: usize,
    index2: usize,
    group1: isize,
    group2: isize,
    unionfind: &mut Vec<Vec<usize>>,
) {
    if group1 == -1 && group2 == -1 {
        let mut group = Vec::<usize>::new();
        group.push(index1);
        group.push(index2);
        unionfind.push(group);
    } else if group1 == -1 {
        unionfind[group2 as usize].push(index1);
    } else if group2 == -1 {
        unionfind[group1 as usize].push(index2);
    } else {
        for i in unionfind[group2 as usize].clone() {
            unionfind[group1 as usize].push(i);
        }
        unionfind.remove(group2 as usize);
    }
}
