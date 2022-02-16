use std::io;
use tempdir::TempDir;

pub fn create_md_test_files_1() -> io::Result<TempDir> {
    let dir = TempDir::new("modo_integrationtests")?;
    md_test_file_creator::one_file_one_open_todo(&dir, "file1.md")?;
    Ok(dir)
}

pub fn create_md_test_files_2() -> io::Result<TempDir> {
    let dir = TempDir::new("modo_integrationtests")?;
    md_test_file_creator::one_file_one_open_todo(&dir, "file1.md")?;
    md_test_file_creator::one_file_one_open_todo(&dir, "file2.md")?;
    md_test_file_creator::one_file_one_open_todo(&dir, "file3.md")?;
    Ok(dir)
}

pub fn create_md_test_files_3() -> io::Result<TempDir> {
    let dir = TempDir::new("modo_integrationtests")?;
    md_test_file_creator::one_file_five_todos_4_open(&dir, "file1.md")?;
    Ok(dir)
}

pub fn create_md_test_files_4() -> io::Result<TempDir> {
    let dir = TempDir::new("modo_integrationtests")?;
    md_test_file_creator::one_file_five_todos_4_open(&dir, "file1.md")?;
    md_test_file_creator::one_file_five_todos_4_open(&dir, "file2.md")?;
    md_test_file_creator::one_file_one_open_todo(&dir, "file3.md")?;
    Ok(dir)
}

mod md_test_file_creator {
    use std::fs::File;
    use std::io::{self, Write};
    use tempdir::TempDir;

    pub fn one_file_one_open_todo(dir: &TempDir, filename: &str) -> io::Result<()> {
        let file_path = dir.path().join(filename);
        let mut f = File::create(file_path)?;
        f.write_all(b"- [ ] A single open todo!")?;
        f.sync_all()?;
        Ok(())
    }
    pub fn one_file_five_todos_4_open(dir: &TempDir, filename: &str) -> io::Result<()> {
        let file_path = dir.path().join(filename);
        let mut f = File::create(file_path)?;
        f.write_all(b"- [ ] A open todo!\r\n- [ ] A open todo!\r\n- [x] A closed todo!\r\n- [ ] A open todo!\r\n- [ ] A open todo!\r\n")?;
        f.sync_all()?;
        Ok(())
    }
}
