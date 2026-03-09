use rand::prelude::*;
const V: [char;6]= ['a', 'e', 'i', 'o', 'u', 'y'];
const C: [char;20] = ['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'z'];
enum State {
	StartV,
	StartC,
	StartC2,
}

fn generate(len: usize) -> String {
	let mut rng = rand::rng();
	let mut prevstate: Option<State> = None;
	let mut buf = String::with_capacity(len);
for _ in 0..len {
	if prevstate.is_none() {
	let is_v = rand::random_bool(0.5);
	let next_char = if is_v {
		V.choose(&mut rng)
	}
	else {
		C.choose(&mut rng)
	}.unwrap();
	let state = if is_v {
		State::StartV
	}
	else {
		State::StartC
	};
buf.push(*next_char);
prevstate = Some(state)
	}
	else {
		let state = match prevstate.unwrap() {
State::StartV => {
	let next_char = C.choose(&mut rng).unwrap();
	buf.push(*next_char);
	State::StartC
},
State::StartC => {
	let is_c = rng.random_bool(0.5);
	let next_char = if is_c {
		C.choose(&mut rng).unwrap()
	}
	else {
		V.choose(&mut rng).unwrap()
	};
	buf.push(*next_char);
if is_c {
	State::StartC2
}
else {
	State::StartV
}
},
State::StartC2 => {
	let next_char = V.choose(&mut rng).unwrap();
	buf.push(*next_char);
	State::StartV
}
		};
		prevstate = Some(state)
	}
}
buf
}
	fn main() {
let count: usize = std::env::args()
    .nth(1)
    .expect("Missing argument")
    .parse()
    .expect("Invalid number");
			let len: usize = std::env::args()
    .nth(2)
    .expect("Missing argument")
    .parse()
    .expect("Invalid number");
if len == 0 {
let target: String = std::env::args()
    .nth(3)
    .expect("Missing argument");
let stringlen = target.len();
let mut count: usize = 0;
let result = loop {
count += 1;
let string = generate(stringlen);
if string.len() == stringlen && string == target {
break string;
}
};
println!("the string {} was generated in {} attempts", result, count);
}
else {
	for i in 1..=count {
let string = generate(len);
println!("string number {} is: {}", i, string);
}
	}
}
