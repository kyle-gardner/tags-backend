use crate::user_mode;

pub fn give_full_filesystem() {
	let fs = user_mode::get_filesystem().expect("Filesystem not found.");
	println!("{:?}", fs);
}


pub fn filter_tags(arguments: Vec<String>) {
	let fs = user_mode::get_filesystem().expect("Filesystem not found.");
	if let Ok(tag) = fs.filter(arguments) {
		println!("{:?}", tag);
	} else {
		println!("Something went wrong with filesystem::filter()");
	}
}