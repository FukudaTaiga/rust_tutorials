// Sec 8
use std::collections::HashMap;

fn main() {
    vector();
    string();
    hashmap();
    exercise1_vec_training();
    exercise2_string_train();
    exercise3_hashmap_train();
}

fn vector() {
    //variable length list
    println!("vector collection");
    //initialise with two way
    //First
    let _v: Vec<i32> = Vec::new(); // type annotation needed for empty
                                   //Second
    let _v = vec![1, 2, 3];

    let mut v: Vec<i32> = Vec::new();
    v.push(50);
    //_v.push(2); of corse `mut` necessary
    v.push(5);

    //access
    let _first = v.get(0); // Some(&value) or None
    let first = &v[0]; // out of index -> panic
                       //let first = v[0]; this return i32 - not &i32

    println!("v first element: {}", first);
    v.push(1);
    v.push(100);
    println!(
        "popped {}",
        match v.pop() {
            //pop - return Option(last element) and delete it
            Some(i) => i,
            None => 0,
        }
    );
    print!("v ");
    // all
    for el in &v {
        print!("{} ", el);
    }
    println!();
    // all change - need mutate
    for el in &mut v {
        *el += 100;
    }
    print!("mutated v ");
    for el in &v {
        print!("{} ", el); //need excluding ref
    }
    println!();

    //multi-typed vector
    enum Vtype {
        I(i32),
        F(f64),
        S(String),
    }
    impl Vtype {
        fn print(&self, br: bool) {
            if br {
                match self {
                    Vtype::I(i) => println!("{} ", i),
                    Vtype::F(f) => println!("{} ", f),
                    Vtype::S(s) => println!("{} ", s),
                }
            } else {
                match self {
                    Vtype::I(i) => print!("{} ", i),
                    Vtype::F(f) => print!("{} ", f),
                    Vtype::S(s) => print!("{} ", s),
                }
            }
        }
    }
    let mut v: Vec<Vtype> = Vec::new();
    v.push(Vtype::I(4));
    v.push(Vtype::F(3.14));
    v.push(Vtype::S(String::from("text")));
    print!("multi-type v ");
    for el in &v {
        el.print(false);
    }
    println!();
    println!("----------------------------------------");
}

fn string() {
    //String - owned, &str - borrowed
    println!("string collection");

    //initialize
    let _s1 = String::new(); //empty String
    let s2 = "initialized".to_string();
    let s3 = String::from(" by String::from");

    //update
    let mut hell = "hell".to_string();
    hell.push('o'); // add char
    hell.push_str(" world"); // add string
    println!("hell: {}", hell);
    let s = s2 + &s3; //s2 will never read cuz moved
                      //this is abbrevigation of add(self, other: &T - there &str)
                      //Rust do force-typed String to &str (&s -> &s[..])
                      //println!("{}", s2); compile error
    println!("s: {}", s);
    let concat = format!(
        "{}{}{}",
        "hell".to_string(),
        "o ".to_string(),
        "world".to_string(),
    ); //format macro
       // Rust forbid random char access for internal expression of String, Vec<u8>
       // so "Здравствуйте".to_string().len() return 24
       // there are the wrong when String including Unicode
       // for random access,
    println!("concat first char {}", &concat[0..1]);
    //above is still not good cuz Rust panic if it is not the char separate

    //all
    for c in "नमस्त".chars() {
        //with char
        print!("{} ", c);
    }
    println!();

    for c in "नमस्त".bytes() {
        //with byte
        print!("{} ", c);
    }
    println!();
    println!("----------------------------------------");
}

fn hashmap() {
    //called dictionary in python and elm, object in js, map in c++ and scala
    // HashMap in Rust
    println!("hashmap collection");

    //initialize two way
    //First
    let mut scores = HashMap::new();
    // you may see error still this line, but Rust infer its type from below codes
    scores.insert("Blue".to_string(), 10);
    scores.insert("Yellow".to_string(), 500);
    //Second
    let keys = vec!["key1", "key2", "keyn"];
    let values = vec![
        "value1".to_string(),
        "value2".to_string(),
        "valuen".to_string(),
    ];
    let mut hashm: HashMap<_, _> = keys.iter().zip(values.iter()).collect();
    // collect() seems to be interesting to me
    // aimed Collection must explicitly indicated
    //ownership
    let color1 = "Red".to_string();
    let score1 = 2000;
    scores.insert(color1, score1);
    //println!("{}", color1); // String doesnt have Copy trait, so the value moved (insert(&mut self, k: K, v: V))
    println!("{}", score1); // i32 has Copy trait, so it didn't

    //access
    //all
    println!("hahsm:");
    for (k, v) in &hashm {
        println!("{}, {}", k, v);
    }
    //by get(&K) -> Option(&V)
    let v_exist = hashm.get(&"key1");
    let v_not_exist = hashm.get(&"key3");
    println!("hahsm(key1): {:?}", v_exist);
    println!("hahsm(key3): {:?}", v_not_exist);

    //update
    // overwrite
    let new_v1 = "value1 overwrited".to_string();
    hashm.insert(&"key1", &new_v1);
    println!("hashm:");
    println!("{:#?}", hashm);
    // write if does not exist, else do nothig
    let new_v1 = "value1 overwrited and overwrited".to_string();
    let v3 = "value3".to_string();
    //entry method return enum Entry and you should use a method on it
    let checked_v = hashm.entry(&"key1").or_insert(&new_v1);
    println!("{}", checked_v);
    let checked_v = hashm.entry(&"key3").or_insert(&v3);
    println!("{}", checked_v);
    println!("hashm:");
    println!("{:#?}", hashm);
    //update with old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        //or_insert method return the value if exist, inserted if not
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("map:");
    println!("{:#?}", map);

    println!("----------------------------------------");
}

fn exercise1_vec_training() {
    println!("Vector training!");
    println!("type Integer and only enter to calc");
    let mut vec = Vec::<i32>::new();
    loop {
        let mut line = String::new();
        let ok = String::from("");
        std::io::stdin()
            .read_line(&mut line)
            .expect("unknown error");
        let insert = match line.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                if line.trim() == ok {
                    break;
                } else {
                    println!("type Integer or enter");
                    continue;
                }
            }
        };
        vec.push(insert);
    }
    println!("{:?}", vec);

    fn calc_mean(vec: &Vec<i32>) -> f64 {
        let mut sum = 0.0;

        for i in vec {
            sum += f64::from(*i); //cast equal to *i as f64
        }

        if vec.len() == 0 {
            0.0
        } else {
            sum / (vec.len() as f64) //cast usize to f64
        }
    }

    fn calc_median(vec: &Vec<i32>) -> f64 {
        let mut vec = vec.clone();

        vec.sort();

        if vec.len() == 0 {
            0.0
        } else if vec.len() % 2 == 0 {
            vec[vec.len() / 2] as f64
        } else {
            let m = vec.len() / 2; //i32 dividing value floored
            (vec[m] + vec[m + 1]) as f64 / 2.0
        }
    }

    fn calc_mode(vec: &Vec<i32>) -> f64 {
        let mut cnt_map = HashMap::<i32, u32>::new();
        let mut max = (0, 0);

        for i in vec {
            let cnt = cnt_map.entry(*i).or_insert(0);
            *cnt += 1;

            if max.1 < *cnt {
                max.0 = *i;
                max.1 = *cnt;
            }
        }

        max.0 as f64
    }

    if vec.len() == 0 {
        println!("warning!: given array is empty");
    }
    println!("mean {}", calc_mean(&vec));
    println!("median {}", calc_median(&vec));
    println!("mode {}", calc_mode(&vec));
    println!("----------------------------------------");
}

fn exercise2_string_train() {
    println!("String training!");
    println!("Big Latin, type string by one line");

    let mut line = String::new();

    std::io::stdin()
        .read_line(&mut line)
        .expect("unknown error");

    fn big_latin(s: String) -> String {
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        let mut res = String::new();
        let mut ay = String::new();

        if s.is_empty() {
            println!("warning: input string is empty");
            res.push_str("-ay");
        }

        for op in s.chars().enumerate() {
            let (i, c) = op;

            if i == 0 && vowels.contains(&c) {
                res.push(c);
                ay.push_str("-hay");
            } else if !vowels.contains(&c) && ay.is_empty() {
                // equal to && ay.len() == 0
                ay.push('-');
                ay.push(c);
                ay.push_str("ay");
            } else {
                res.push(c);
            }
        }

        res + &ay
    }
    println!("Big Latin: {}", big_latin(line.trim().to_string()));
    println!("----------------------------------------");
}

fn exercise3_hashmap_train() {
    println!("Hashmap training!");
    println!("your the personnel department, lets type command");

    let mut departments = HashMap::<String, Vec<String>>::new();

    loop {
        let mut line = String::new();
        std::io::stdin()
            .read_line(&mut line)
            .expect("unknown error");
        let commands: Vec<&str> = line.trim_end().split(' ').collect();

        match (*&commands[0], commands.get(1), commands.get(3)) {
            ("add", Some(person), Some(target)) => {
                let members = departments.entry(target.to_string()).or_insert(vec![]);
                members.push(person.to_string());
            }
            ("delete", Some(person), Some(target)) => {
                if let Some(members) = departments.get(*target) {
                    let new_mem: Vec<String> = members
                        .iter()
                        .filter(|m| *person != &m[..])
                        .map(|m| (&m[..]).to_string())
                        .collect();
                    departments.insert(target.to_string(), new_mem);
                } else {
                    println!("there is no department named {}", person);
                }
            }
            ("show-departments", None, None) => {
                println!("{:#?}", departments);
            }
            ("show-people", None, None) => {}
            ("quit", None, None) => break,
            ("help", None, None) => {
                println!(
                    "\
          commands showed below\n\
          - add $name to $department\n\
          - delete $name from $department\n\
          - show-departments\n\
          - show-people
          - quit\n\
          - help\
          "
                );
            }
            _ => {
                println!("invalid command, type help to see all commands");
            }
        }
    }

    println!("----------------------------------------");
}
