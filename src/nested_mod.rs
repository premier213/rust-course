pub mod clean {
    //! nested mod
    pub fn clean_hdd() {
        println!("HDD Cleaned!")
    }
    pub mod files {
        pub fn clean_files() {
            println!("files removed !")
        }
    }
}
