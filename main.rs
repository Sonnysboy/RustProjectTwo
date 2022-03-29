use std::collections::HashMap;
use std::io;
fn main() {

    let mut map = HashMap::new();

    loop 
	{
        let mut key = String::new();
        
		io::stdin().read_line(&mut key).expect("Error");

        let spl: Vec<&str> = key.trim().split(" ").collect();

        let mut name = Option::None;
        
		let mut department = Option::None;

		
		for (i, entry) in spl.iter().enumerate() {
            if i > 0usize && spl[i - 1].to_lowercase() == "add" 
			{
                name = Some(entry.to_string());
                // name found.
            }
        
			if name.is_some() {
                // if name exists then we should have a "To"
                if spl[i - 1].to_lowercase() == "to" {
                    department = Some(entry.to_string()); // the dept.
                }
            }
			
            if i > 0usize && spl[i - 1].to_lowercase() == "get" {
                let dept = Some(entry.to_string());
                match dept {
                    Some(x) => {
						if x.to_string().contains("*") {
							// this is the only way i could figure it out.
							println!("{:?}", map);
						}	else {
                        let entry = &mut map.entry(x.to_string()).or_insert(vec![]);
                        println!("{} : {:?}", x, entry);
						}
                    }
                    _ => (),
                };
                // println!("{:?}", &map.entry(&dept).or_insert(vec![]));
            }
        }
        match name {
            Some(name) => {
                println!("Name: {}", &name);
                if department.is_none() {
                    panic!("Something went wrong with department")
                }
                let dept = department.unwrap();
                println!("Department: {}", &dept);
                let people = &mut map.entry(dept).or_insert(vec![]);
                // add person to department.
                // using pointers we add to the vector.
                (*people).push(name);
            }
            _ => ()
        };
    }
}
