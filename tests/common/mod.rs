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

pub fn create_md_test_file_with_5_todos_3_open_headers() -> io::Result<TempDir> {
    let dir = TempDir::new("modo_integrationtests")?;
    md_test_file_creator::one_simple_file_5_todos_3_open_with_headings(&dir, "file1.md")?;
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
    use std::fs;
    use std::io;
    use tempdir::TempDir;

    pub fn one_simple_file_1_open_todo(dir: &TempDir, filename: &str) -> io::Result<()> {
        let file_path = dir.path().join(filename);
        fs::write(file_path, "- [ ] A single open todo!")?;
        Ok(())
    }
    pub fn one_simple_file_5_todos_4_open(dir: &TempDir, filename: &str) -> io::Result<()> {
        let file_path = dir.path().join(filename);
        fs::write(file_path, "- [ ] A open todo!\r\n- [ ] A open todo!\r\n- [x] A closed todo!\r\n- [ ] A open todo!\r\n- [ ] A open todo!\r\n")?;
        Ok(())
    }

    pub fn one_simple_file_5_todos_3_open_with_headings(
        dir: &TempDir,
        filename: &str,
    ) -> io::Result<()> {
        let file_path = dir.path().join(filename);
        let md = "## Heading
                # Heading2
                - [ ] Todo with Heading
                - [ ] A open todo!
                
                ### Heading Done
                - [x] A done todo!
                Some text that
                does not really 000
                matter.
                Todos with tabs and spaces infront:
                    - [x] A done todo!
                    - [ ] A open todo!";

        fs::write(file_path, md)?;
        Ok(())
    }

    // TODO: pub fn three_complex_files_with_10_todos_5_open(
}
