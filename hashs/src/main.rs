use std::collections::HashMap;

fn main() {
    // let mut h = HashMap::new();
    // h.insert(String::from("ch"), 10);
    // println!("{:?}", h);

    // let teams = vec!["xi", "ban"];
    // let scores = vec![10, 12];
    // let sh : HashMap<_, _> = teams.iter().zip(scores.iter()).collect();
    // println!("{:?}", sh);

    // let fi = String::from("fi");
    // let fv = String::from("b");
    // let mut hs = HashMap::new();
    // hs.insert(fi, fv);
    // println!("{:?}", sh);
    // println!("{}",fi); // fi is owned by sh

    // let fi = String::from("fi");
    // let fv = String::from("b");
    // let mut hs = HashMap::new();
    // hs.insert(&fi, &fv);
    // println!("{:?}", sh);
    // println!("{}",fi); // fi is not owned by sh

    // let mut hs = HashMap::new();
    // hs.insert("fos", 10);
    // hs.insert("fos", 11); // update k or insert k
    // let e = hs.entry(("foss"));
    // println!("{:?}",e);
    // e.or_insert(50);
    // hs.entry(("foss")).or_insert(100);

    // println!("{:?}",hs);
    // let v = hs.get("fos");
    // println!("{:?}", v);

    // match v {
    //     Some(vv) => println!("{}", vv),
    //     None => println!("nonw")
    // };

    // for (k , v) in &hs {
    //     println!("{} {}", k, v);
    // }

    let text = "hello world world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map);
}
