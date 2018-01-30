enum Message {
    Hello { id: i32 },
}

fn main() {
	let msg = Message::Hello { id: 5 };

	match msg {
		Message::Hello { id: id_variable } if let 3..7 = id_variable {true} else {false} => {
			println!("Found an id in range: {}", id_variable)
		},
		Message::Hello { id: 10...12 } => {
			println!("Found an id in another range")
		},
		Message::Hello { id } => {
			println!("Found some other id: {}", id)
		},
	}
}
