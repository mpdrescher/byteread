//äää
use AppArgs;
use Format;

use std::io::Error as IOError;
use std::fs::File;
use std::io::Read;

type Data = Vec<u8>;
	
//read the file and start the print loop
pub fn start(args: AppArgs)
{
	let data: Data = match read_file(&args.file_path)
	{
		Ok(v) => v,
		Err(e) => {
			println!("error while reading file: {}", e);
			return;
		}
	};
	print(data, args);
}

fn print(data: Data, args: AppArgs)
{
	//buffer the ascii chars to be written if args.ascii is set
	let mut ascii_buffer = Vec::new();
	//the line position of the current byte
	//used to determine line wrap
	let mut line_byte = 0;
	//the block position of the current byte
	//used to determine block indent
	let mut block_byte = 0;
	//current line
	let mut line = 0;
	//the size the line number should be padded to (if args.lines set)
	let expected_line_no_len = format!("{}", data.len()).len();

	for byte in data
	{
		if line_byte == 0
		{
			print_line_start(line, &args, expected_line_no_len);
		}
		block_byte += 1;
		line_byte += 1;
		print_fmt(byte, &args.format, block_byte == args.pageformat_blocks);
		
		if args.ascii
		{
			ascii_buffer.push(byte);
		}

		if block_byte == args.pageformat_blocks && line_byte != args.pageformat_cols //new block
		{
			print_indent(args.indent);
			block_byte = 0;
		}
		if line_byte == args.pageformat_cols //new line
		{
			if args.ascii
			{
				print_ascii_buffer(&ascii_buffer, &args);
				ascii_buffer.clear();
			}

			block_byte = 0;
			line_byte = 0;
			line += 1;
			print_new_line();
		}
	}
	//end last line if print_new_line hasn't been called
	//occurs on data.len() % cols != 0
	if line_byte != 0
	{
		println!();
	}
}

fn print_line_start(line_no: usize, args: &AppArgs, str_len: usize)
{
	if args.lines
	{
		print!("{}", pad_left(format!("{}",line_no*args.pageformat_cols), str_len));
		print!("|");
		print_indent(args.indent);
	}
}

fn print_new_line()
{
	println!();
}

fn print_indent(size: usize)
{
	for _ in 0..size
	{
		print!(" ")
	}
}

fn print_ascii_buffer(buffer: &Data, args: &AppArgs)
{
	print_indent(args.indent*2);
	let mut char_counter = 0;
	for buffer_chunk in buffer.as_slice().chunks(args.pageformat_blocks)
	{
		for ch in buffer_chunk
		{
			if *ch > 32 && *ch < 127 //don't print whitespace and ctrl char
			{
				print!("{}", *ch as char);
			}
			else
			{
				print!(" ");
			}
			char_counter += 1;
		}
		if char_counter != buffer.len()
		{
			print_indent(args.indent/2);
		}
	}
}

fn print_fmt(byte: u8, format: &Format, last_in_block: bool)
{
	match *format
	{
		Format::Hex => {
			if !last_in_block
			{
				print!("{} ", show_hex(byte));
			}
			else 
			{
			    print!("{}", show_hex(byte));
			}
		},
		Format::Bin => {
			if !last_in_block
			{
				print!("{} ", show_bin(byte));
			}
			else 
			{
			    print!("{}", show_bin(byte));
			}
		},
		Format::U8 => {
			if !last_in_block
			{
				print!("{}", pad_left(format!("{} ", byte), 4));
			}
			else 
			{
			    print!("{}", pad_left(format!("{}", byte), 3));
			}
		},
		Format::I8 => {
			if !last_in_block
			{
				print!("{}", pad_left(format!("{} ", byte as i8), 4));
			}
			else 
			{
			    print!("{}", pad_left(format!("{}", byte as i8), 3));
			}
		}
	};
}

fn read_file(path: &String) -> Result<Data, IOError>
{
	let mut file = try!(File::open(path));
	let mut buffer = Vec::new();
	try!(file.read_to_end(&mut buffer));
	Ok(buffer)
}

fn pad_left(mut str: String, size: usize) -> String
{
	while str.len() < size
	{
		str.push(' ');
	}
	str
}

fn show_bin(byte: u8) -> String
{
	let mut result = String::new();
	for shift in 0..8
	{
		let rev_shift = 7-shift;
		if (byte & 1 << rev_shift) != 0
		{
			result.push('1');
		}
		else {
		    result.push('0');
		}
	}
	result
}

fn show_hex(byte: u8) -> String
{
	let mut result = String::new();
	let bin = show_bin(byte);
	let mut nibble = Vec::new();
	for bit in bin.chars()
	{
		nibble.push(bit);
		if nibble.len() == 4
		{
			let mut val_acc = 0;
			let mut bit_val = 8;
			for nibble_bit in &nibble
			{
				if *nibble_bit == '1'
				{
					val_acc += bit_val;
				}
				bit_val /= 2;
			}
			result.push(get_hex_digit(val_acc));
			nibble.clear();
		}
	}
	result
}

fn get_hex_digit(num: u8) -> char
{
	match num
	{
		0...9 => return (num+48) as char,
		_ => return (num+87) as char
	}
}