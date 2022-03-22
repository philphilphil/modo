pub mod md_test_file_creator {
    use std::fs;
    use std::io;
    use tempfile::TempDir;

    #[allow(dead_code)] // code only used in tests, shows up as warning
    pub fn simple_1_open_todo(dir: &TempDir, filename: &str) -> io::Result<()> {
        let file_path = dir.path().join(filename);
        fs::write(file_path, "- [ ] A single open todo!")?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn simple_5_todos_4_open(dir: &TempDir, filename: &str) -> io::Result<()> {
        let file_path = dir.path().join(filename);
        fs::write(file_path, "- [ ] A open todo!\r\n- [ ] A open todo!\r\n- [x] A closed todo!\r\n- [ ] A open todo!\r\n- [ ] A open todo!\r\n")?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn simple_5_todos_3_open_with_headings(dir: &TempDir, filename: &str) -> io::Result<()> {
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

    #[allow(dead_code)]
    pub fn complex_23_todos_15_open(dir: &TempDir, filename: &str) -> io::Result<()> {
        let file_path = dir.path().join(filename);
        let md = "# Complex note with 23 todos, 15 open
        ## Heading2
        - [ ] Todo with Heading
        - [ ] A open todo!
        
        ```rust
        // This is a comment, and is ignored by the compiler
        // You can test this code by clicking the \"Run\" button over there ->
        // or if you prefer to use your keyboard, you can use the \"Ctrl + Enter\" shortcut
        
        // This code is editable, feel free to hack it!
        // You can always return to the original code by clicking the \"Reset\" button ->
        // - [ ] Todo, should not be picked up!
        // This is the main function
        fn main() {
            // Statements here are executed when the compiled binary is called
        
            // Print text to the console
            println!(\"Hello World!\");
        }
        
        ```
        
        ### Heading Done
        - [x] A done todo!
        Some text that
        does not really 000
        matter.
        Todos with tabs and spaces infront:
        - [x] A done todo!
        - [ ] A open todo!
        
        ####
        - Abc
        - def
        - gfd
        
        - [ ] Todo
        - [x] todo 2
        
        [[temp]]
        
        - [ ] Todo
        
        [[p17_1.jpg]]
        - [ ] Todo
        
        
        ### Psipsum
        Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, - [ ] TODOSUM sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.
        
        # new area
        ## Work
        - [ ] Todo 1 o
        - [ ] Todo 2 c
            - [ ] Sub Todo
            - [ ] Sub Todo 2
            - [x] Sub Todo 3 c
        - [ ] Normal Todo again
            - [ ] a
                - [x] b
                    - [ ] c
                        - [x] d
                            - [ ] e
                                - [x] f
                                    - [ ] g
                                        - [x] h
        
        Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.
        
        ## Header
        ### Header
        ##### Header
        Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.";

        fs::write(file_path, md)?;
        Ok(())
    }
}
