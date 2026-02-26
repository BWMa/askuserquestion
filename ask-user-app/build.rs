fn main() {
    #[cfg(target_os = "windows")]
    {
        let mut res = embed_resource::compile("askuserquestion.rc", embed_resource::NONE);
        if let Err(e) = res {
            println!("cargo:warning=Resource compilation failed: {}", e);
        }
    }
}