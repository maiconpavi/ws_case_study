use std::{marker::PhantomData, sync::OnceLock};

static CONFIG: OnceLock<Config> = OnceLock::new();

pub struct Config {
    pub aws_region: Box<str>,
    pub api_id: Box<str>,
    pub table_name: Box<str>,
    phantom: PhantomData<()>,
}

pub fn get() -> &'static Config {
    CONFIG.get_or_init(|| Config {
        aws_region: std::env::var("AWS_REGION")
            .expect("AWS_REGION is not set")
            .into_boxed_str(),
        api_id: std::env::var("API_ID")
            .expect("API_ID is not set")
            .into_boxed_str(),
        table_name: std::env::var("TABLE_NAME")
            .expect("TABLE_NAME is not set")
            .into_boxed_str(),
        phantom: PhantomData::default(),
    })
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_example() {
        // let v1 = vec![1_i32];
        // let v2 = &v1[..];
        let s1 = String::from("hello");
        let s2 = &s1[..2];
        // s1.push_str(" world");
        assert_eq!(s2, "he");
    }
}
