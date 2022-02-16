use std::fs::File;
use std::io::{self, Write};
use tempdir::TempDir;

pub fn create_md_test_files_1() -> io::Result<TempDir> {
    let dir = TempDir::new("modo_integrationtests")?;
    create_md_file_one_open_todo(&dir, "file1.md")?;
    Ok(dir)
}

pub fn create_md_test_files_3() -> io::Result<TempDir> {
    let dir = TempDir::new("modo_integrationtests")?;
    create_md_file_one_open_todo(&dir, "file1.md")?;
    create_md_file_one_open_todo(&dir, "file2.md")?;
    create_md_file_one_open_todo(&dir, "file3.md")?;
    Ok(dir)
}

fn create_md_file_one_open_todo(dir: &TempDir, filename: &str) -> io::Result<()> {
    let file_path = dir.path().join(filename);

    let mut f = File::create(file_path)?;
    f.write_all(b"- [ ] A single open todo!")?;
    f.sync_all()?;

    Ok(())
}
