extern crate clap;

use clap::Arg;
use clap::App;

mod app;

//parse args and start the app
fn main() 
{
    let matches = App::new("byteread")
					.version("0.1")
					.about("Inspect the bytes of a file")
					.arg(Arg::with_name("INPUT")
						.help("Sets the input file")
						.required(true))
					.arg(Arg::with_name("format")
						.short("f")
						.long("format")
						.possible_values(&["hex","bin","u8","i8"])
						.default_value("hex")
						.takes_value(true)
						.help("Outputs are printed in the given format"))
					.arg(Arg::with_name("pageformat")
						.short("p")
						.long("pageformat")
						.takes_value(true)
						.default_value("1:8")
						.help("Format: <blocksize:cols>. Sets page format settings"))
					.arg(Arg::with_name("lines")
						.short("l")
						.long("lines")
						.help("Display byte numbers"))
					.arg(Arg::with_name("indent")
						.short("i")
						.long("indent")
						.takes_value(true)
						.default_value("2")
						.help("Set indentation width"))
					.arg(Arg::with_name("ascii")
						.short("a")
						.long("ascii")
						.help("Additionally print the corresponding ASCII characters"))
					.get_matches();
	let ascii = matches.occurrences_of("ascii") >= 1;
	let lines = matches.occurrences_of("lines") >= 1;
	let format = match matches.value_of("format").unwrap()
	{
		"hex" => Format::Hex,
		"bin" => Format::Bin,
		"u8" => Format::U8,
		"i8" => Format::I8,
		_ => return
	};
	let indent = match matches.value_of("indent").unwrap().parse::<usize>()
	{
		Ok(v) => v,
		Err(_) => {
			println!("indentation size is invalid");
			return;
		}
	};
	let file_path = matches.value_of("INPUT").unwrap().to_owned();
	let mut pageformat_raw = matches.value_of("pageformat").unwrap().split(":").into_iter();
	let pageformat_blocks = match pageformat_raw.next()
	{
		Some(v) => match v.parse::<usize>(){
				Ok(v1) => v1,
				Err(_) => {
					println!("invalid page format number:  {}", v);
					return;
				}
			},
		None => {
			println!("invalid page format (expected blocksize:cols)");
			return;
		}
	};
	//dup
	let pageformat_cols = match pageformat_raw.next()
	{
		Some(v) => match v.parse::<usize>(){
				Ok(v1) => v1,
				Err(_) => {
					println!("invalid page format number:  {}", v);
					return;
				}
			},
		None => {
			println!("invalid page format (expected blocksize:cols)");
			return;
		}
	};

	if pageformat_cols < pageformat_blocks
	{
		println!("block number is higher than the number of columns");
		return;
	}
	if pageformat_blocks == 0 || pageformat_cols == 0
	{
		println!("0 is an invalid value for a page format");
		return;
	}

	let args = AppArgs{
		ascii: ascii,
		lines: lines,
		format: format,
		indent: indent,
		file_path: file_path,
		pageformat_blocks: pageformat_blocks,
		pageformat_cols: pageformat_cols
	};

	app::start(args);
}

//holds the parsed commandline arguments
#[derive(Debug)]
pub struct AppArgs
{
	pub ascii: bool,
	pub lines: bool,
	pub format: Format,
	pub indent: usize,
	pub file_path: String,
	pub pageformat_blocks: usize,
	pub pageformat_cols: usize
}

//holds the format the data should be presented in
#[derive(Debug)]
pub enum Format
{
	Hex,
	Bin,
	U8,
	I8,
}