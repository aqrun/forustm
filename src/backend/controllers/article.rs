use sapper::{SapperModule, SapperRouter, Response, Request, 
    Result as SapperResult};
use sapper_std::{render};
use super::super::super::{WebContext};


pub struct ArticleController;

impl ArticleController {
    //列表
    fn index(req: &mut Request) -> SapperResult<Response> {
        let web = req.ext().get::<WebContext>().unwrap().clone();

        res_html!("backend/article/index.html", web)
    }
/*
    // 详情
    fn view(req: &mut Request) -> SapperResult<Response> {
        let web = req.ext().get::<WebContext>().unwrap().clone();

        res_html!("backend/article/view.html", web)
    }

    // add article
    fn add(req: &mut Request) -> SapperResult<Response> {
        let web = req.ext().get::<WebContext>().unwrap().clone();

        res.html!("backend/article/add.html", web)
    }

    // update article
    fn update(){
        let web = req.ext().get::<WebContext>().unwrap().clone();

        res.html!("backend/article/update.html", web)
    }

    // delete a article
    fn delete(){
        let web = req.ext().get::<WebContext>().unwrap().clone();

        res.html!("backend/article/delete.html", web)
    }*/

}

impl SapperModule for ArticleController {
    fn router(&self, router: &mut SapperRouter) -> SapperResult<()> {
        router.get("/backend/article", ArticleController::index);
        // router.get("/backend/article/add", ArticleController::add);
        // router.get("/backend/article/update", ArticleController::update);
        // router.get("/backend/article/delete", ArticleController::delete);
        Ok(())
    }
}