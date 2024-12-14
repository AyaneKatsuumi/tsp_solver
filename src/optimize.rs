pub fn opt2(src: &Vec<usize>, dist: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut res = src.to_vec();
    loop {
        let mut short = 0;
        for i in 0..src.len() - 3 {
            for j in i + 2..src.len() - 1 {
                let diff: isize = dist[res[i] - 1][res[i + 1] - 1] as isize
                    + dist[res[j] - 1][res[j + 1] - 1] as isize
                    - (dist[res[i] - 1][res[j] - 1] as isize
                        + dist[res[i + 1] - 1][res[j + 1] - 1] as isize);
                if diff > 0 {
                    opt2swap(&mut res, i + 1, j);
                    short += diff;
                }
            }
        }
        if short == 0 {
            break;
        }
    }
    res
}

fn opt2swap(src: &mut Vec<usize>, v1: usize, v2: usize) {
    let mut pt1 = src[0..v1].to_owned();
    let mut pt2 = src[v1..=v2].to_owned();
    let mut pt3 = src[v2 + 1..].to_owned();

    pt2.reverse();
    src.clear();
    src.append(&mut pt1);
    src.append(&mut pt2);
    src.append(&mut pt3);
}

pub fn or1opt(src: &Vec<usize>, dist: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut res = src.to_vec();
    loop {
        let mut short = 0;
        for i in 1..res.len() - 2
        // 入れ替え元
        {
            for j in i + 1..res.len() - 1
            // 入れ替え先
            {
                let before = dist[res[i - 1] - 1][res[i] - 1]
                    + dist[res[i] - 1][res[i + 1] - 1]
                    + dist[res[j] - 1][res[j + 1] - 1];
                let after = dist[res[i - 1] - 1][res[i + 1] - 1]
                    + dist[res[i] - 1][res[j] - 1]
                    + dist[res[i] - 1][res[j + 1] - 1];
                if before > after {
                    let tmp = res.remove(i);
                    res.insert(j, tmp);
                    short += before - after;
                }
            }
        }
        if short == 0 {
            break;
        }
    }
    res
}

pub fn or2opt(src: &Vec<usize>, dist: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut res = src.to_vec();
    loop {
        let mut short = 0;

        for i in 1..res.len() - 2 {
            for j in i + 2..res.len() - 1 {
                let before = dist[res[i - 1] - 1][res[i] - 1]
                    + dist[res[i + 1] - 1][res[i + 2] - 1]
                    + dist[res[j] - 1][res[j + 1] - 1];
                let after = dist[res[i - 1] - 1][res[i + 2] - 1]
                    + dist[res[i] - 1][res[j] - 1]
                    + dist[res[i + 1] - 1][res[j + 1] - 1];
                if before > after {
                    let tmp1 = res.remove(i);
                    let tmp2 = res.remove(i);
                    res.insert(j - 1, tmp2);
                    res.insert(j - 1, tmp1);
                    short += before - after;
                }
            }
        }

        if short == 0 {
            break;
        }
    }
    res
}
