use std::collections::HashMap;

pub fn count_provinces() -> String {
    let mut result: Vec<String> = vec![];
    for district in parse_json().iter() {
        result.push(count(district.clone()).to_string());
    }
    result.join(",")
}

fn count(district: HashMap<String, Vec<String>>) -> i32 {
    let mut cnt = 0;
    let mut deleted: Vec<String> = vec![];
    loop {
        let mut done = true;
        for (city, _) in district.iter() {
            if deleted.contains(city) {
                continue;
            }
            done = false;
            for node in dfs(&district, city.clone()) {
                if !deleted.contains(&node) {
                    deleted.push(node);
                }
            }
            break;
        }
        if done {
            break;
        } else {
            cnt += 1;
        }
    }
    cnt
}

fn dfs(district: &HashMap<String, Vec<String>>, node: String) -> Vec<String> {
    let mut nodes: Vec<String> = vec![];
    let mut stack: Vec<String> = vec![node];
    while let Some(node) = stack.pop() {
        if nodes.contains(&node) {
            continue;
        }
        nodes.push(node.clone());
        if let Some(neighbors) = district.get(&node) {
            for neighbor in neighbors {
                if stack.contains(neighbor) {
                    continue;
                }
                stack.push(neighbor.clone());
            }
        }
    }
    nodes
}

fn parse_json() -> Vec<HashMap<String, Vec<String>>> {
    let mut district: Vec<HashMap<String, Vec<String>>> = vec![];
    let file = std::fs::read_to_string("./district.json").unwrap();
    let file = file.strip_suffix("}\n").unwrap();
    for block in file.split("}") {
        let mut district_tmp: HashMap<String, Vec<String>> = HashMap::new();
        for line in block.lines() {
            let mut city_k = String::new();
            for city in line.split(":") {
                match city.find("[") {
                    Some(_) => {
                        if city_k.eq("") {
                            continue;
                        }
                        let v2 = district_tmp
                            .entry(String::from(city_k.clone()))
                            .or_insert(vec![]);
                        for k in city.split(",") {
                            let kk = k
                                .trim()
                                .trim_matches('[')
                                .trim_matches(']')
                                .trim_matches('"');
                            if kk.eq("") || v2.contains(&String::from(kk)) {
                                continue;
                            }
                            v2.push(String::from(kk));
                        }
                        for k in city.split(",") {
                            let kk = k
                                .trim()
                                .trim_matches('[')
                                .trim_matches(']')
                                .trim_matches('"');
                            if kk.eq("") {
                                continue;
                            }
                            let v3 = district_tmp.entry(String::from(kk)).or_insert(vec![]);
                            if v3.contains(&city_k) {
                                continue;
                            }
                            v3.push(String::from(city_k.clone()));
                        }
                    }
                    None => {
                        city_k = String::from(city.trim().trim_matches('"'));
                    }
                }
            }
        }
        if district_tmp.len() > 0 {
            district.push(district_tmp);
        }
    }

    district
}
