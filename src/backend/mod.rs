pub mod controllers;

pub static controller_list = vec![
    Box::new(self::controllers::article::ArticleControler),
];