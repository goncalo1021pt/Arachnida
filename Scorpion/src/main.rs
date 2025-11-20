use clap::Parser;

#[derive(Parser)]
struct Args {
	/// Input file paths
	#[arg(required = true)]
	paths: Vec<String>,
	
	/// Strip all EXIF metadata from images
	#[arg(short = 's', long)]
	strip: bool,
}

fn read_metadata(path: &str) -> Result<(), Box<dyn std::error::Error>> {
	use::std::fs;

	let metadata = fs::metadata(path)?;

	println!("\n=== {} ===", path);
	println!("File size: {} bytes", metadata.len());

	if let Ok(modified) = metadata.modified() {
		println!("Modified: {:?}", modified);
	}

	match read_exif(path) {
		Ok(_) => {},
		Err(e) => println!("\nEXIF Data: None ({})", e),
	}
	
	read_image_info(path)?;
	
	Ok(())
}

fn read_exif(path: &str) -> Result<(), Box<dyn std::error::Error>> {
	use std::fs::File;
	use std::io::BufReader;
	
	let file = File::open(path)?;
	let mut bufreader = BufReader::new(&file);
	
	let exifreader = exif::Reader::new();
	let exif = exifreader.read_from_container(&mut bufreader)?;
	
	println!("\nEXIF Data:");
	
	for field in exif.fields() {
		println!("  {}: {}", field.tag, field.display_value());
	}
	
	Ok(())
}

fn strip_metadata(path: &str) -> Result<(), Box<dyn std::error::Error>> {
	use std::path::Path;
	
	let img = image::open(path)?;
	
	let path_obj = Path::new(path);
	let filename = path_obj.file_stem().unwrap().to_str().unwrap();
	let extension = path_obj.extension().and_then(|e| e.to_str()).unwrap_or("jpg");
	let output_path = format!("{}_no_exif.{}", filename, extension);
	
	img.save(&output_path)?;
	
	println!("✓ Stripped metadata: {} → {}", path, output_path);
	Ok(())
}

fn is_valid_image_file(path: &str) -> bool {
	let valid_extensions = [".jpg", ".jpeg", ".png", ".gif", ".bmp"];
	let lower_path = path.to_lowercase();
	valid_extensions.iter().any(|ext| lower_path.ends_with(ext))
}

fn read_image_info(path: &str) -> Result<(), Box<dyn std::error::Error>> {
	let img = image::open(path)?;
	
	println!("\nImage Info:");
	println!("  Dimensions: {}x{}", img.width(), img.height());
	println!("  Format: {:?}", img.color());
	
	Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = Args::parse();
	
	for path in args.paths {
		if !is_valid_image_file(&path) {
			eprintln!("Skipping {}: Not a supported image format (.jpg, .jpeg, .png, .gif, .bmp)", path);
			continue;
		}
		
		if args.strip {
			if let Err(e) = strip_metadata(&path) {
				eprintln!("Error stripping {}: {}", path, e);
			}
		} else {
			if let Err(e) = read_metadata(&path) {
				eprintln!("Error processing {}: {}", path, e);
			}
		}
	}
	
	Ok(())
}
