use actix_web::http::{header, HttpTryFrom};
use actix_web::{App, HttpRequest, HttpResponse, Result};
use actix_web::http::header::HeaderValue;
use actix_web::middleware::{Middleware, Started, Response};
use actix_web::middleware::session::RequestSession;
use cookie::{Cookie, CookieJar, Key};
use chrono::Duration;

use common::state::AppState;
use common::lazy_static::CONFIG;

pub struct Remember;  

impl Middleware<AppState> for Remember {

    fn start(&self, req: &mut HttpRequest<AppState>) -> Result<Started> {
        
        Ok(Started::Done)
    }

    fn response(&self, req: &mut HttpRequest<AppState>, mut res: HttpResponse) -> Result<Response> {

        let _req = &*req;

        match _req.session().get::<bool>("remember") {

            Ok(data) => {

                    if (data.is_some()) {

                        let remember = data.unwrap();

                        if remember {

                            let redis_key = get_redis_key(_req).unwrap();

                            update_max_age(_req, &mut res);
                        }
                    }
                },
            Err(_) => ()
        }

        Ok(Response::Done(res))
    }
}

pub fn get_redis_key(req: &HttpRequest<AppState>) -> Option<String> {

    let cookies = req.cookies().unwrap();

    for cookie in cookies {

        if cookie.name() == &*CONFIG.cookie.key {

            let mut jar = CookieJar::new();
            jar.add_original(cookie.clone());

            if let Some(cookie) = jar.signed(&Key::from_master(&[0;32])).get(&*CONFIG.cookie.key) { 

                return Some(cookie.value().to_owned());
            }
        }
    }

    None
}

pub fn update_max_age(req: &HttpRequest<AppState>, res: &mut HttpResponse) {

    let cookies = req.cookies().unwrap();
    let mut temp = None;

    for cookie in cookies {

        if cookie.name() == &*CONFIG.cookie.key {

            let mut c = cookie.clone();

            c.set_http_only(true);
            c.set_path("/".to_owned());
            c.set_max_age(Duration::seconds(CONFIG.cookie.max_age as i64));

            temp = Some(c);
        }
    }

    if temp.is_some() {
        res.headers_mut().append(header::SET_COOKIE, HeaderValue::from_str(&temp.unwrap().to_string()).unwrap());
    }
}