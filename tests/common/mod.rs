use std::io;
use tempdir::TempDir;

pub fn create_md_test_file_with_1_open_todo(file_amount: i32) -> io::Result<TempDir> {
    let dir = TempDir::new("modo_integrationtests")?;
    for elem in 0..file_amount {
        md_test_file_creator::one_simple_file_1_open_todo(&dir, &format!("file{}.md", elem))?;
    }
    Ok(dir)
}

pub fn create_md_test_file_with_5_todos_4_open(file_amount: i32) -> io::Result<TempDir> {
    let dir = TempDir::new("modo_integrationtests")?;
    for elem in 0..file_amount {
        md_test_file_creator::one_simple_file_5_todos_4_open(&dir, &format!("file{}.md", elem))?;
    }
    Ok(dir)
}

pub fn create_folder_with_2_files_10_todos_8_open() -> io::Result<TempDir> {
    let dir = TempDir::new("modo_integrationtests")?;
    md_test_file_creator::one_simple_file_5_todos_4_open(&dir, "file1.md")?;
    md_test_file_creator::one_simple_file_5_todos_4_open(&dir, "file2.md")?;
    Ok(dir)
}

pub fn create_folder_with_2_files_10_todos_8_open_in(dir: &TempDir) -> io::Result<TempDir> {
    let dir = TempDir::new_in(dir, "modo_integrationtests")?;
    md_test_file_creator::one_simple_file_5_todos_4_open(&dir, "file1.md")?;
    md_test_file_creator::one_simple_file_5_todos_4_open(&dir, "file2.md")?;
    Ok(dir)
}

mod md_test_file_creator {
    use std::fs::File;
    use std::io::{self, Write};
    use tempdir::TempDir;

    pub fn one_simple_file_1_open_todo(dir: &TempDir, filename: &str) -> io::Result<()> {
        let file_path = dir.path().join(filename);
        let mut f = File::create(file_path)?;
        f.write_all(b"- [ ] A single open todo!")?;
        f.sync_all()?;
        Ok(())
    }
    pub fn one_simple_file_5_todos_4_open(dir: &TempDir, filename: &str) -> io::Result<()> {
        let file_path = dir.path().join(filename);
        let mut f = File::create(file_path)?;
        f.write_all(b"- [ ] A open todo!\r\n- [ ] A open todo!\r\n- [x] A closed todo!\r\n- [ ] A open todo!\r\n- [ ] A open todo!\r\n")?;
        f.sync_all()?;
        Ok(())
    }

    // TODO: pub fn three_complex_files_with_10_todos_5_open(
}
