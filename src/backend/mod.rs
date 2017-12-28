use sapper::SapperApp;

pub mod controllers;
use self::controllers::*;

// init backend controller
pub fn init_controllers(app: &mut SapperApp) {
    app.add_module(Box::new(article::ArticleController));
}