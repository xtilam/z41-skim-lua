use std::env;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::PathBuf;

fn main() {
	let profile = env::var("PROFILE").unwrap();
	// Get absolute path to the project root
	let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("Failed to get manifest dir");
	let project_root = PathBuf::from(manifest_dir);

	// Source: Absolute path to the actual compiled library
	let mut src = project_root.clone();
	src.push("target");
	src.push(profile);
	src.push("libluaskim.so");

	// Destination: Absolute path to the symlink in your lua folder
	let mut dst = project_root.clone();
	dst.push("lua");
	dst.push("skim.so");

	// Clean up old link/file if it exists
	if dst.exists() || fs::symlink_metadata(&dst).is_ok() {
		let _ = fs::remove_file(&dst);
	}

	// Create the symlink using absolute paths
	if let Err(e) = symlink(&src, &dst) {
		println!("cargo:warning=Symlink failed: {}", e);
	} else {
		// This will show up in your terminal if you use -vv
		println!(
			"cargo:warning=Symlink created: {} -> {}",
			dst.display(),
			src.display()
		);
	}

	// Re-run if any file in src/ changes
	println!("cargo:rerun-if-changed=src");
}
