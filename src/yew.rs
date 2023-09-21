use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::format::Json;

pub struct MyComponent {
    fetch_service: FetchService,
    fetch_task: Option<FetchTask>,
    data: Option<Vec<User>>, // User structure should match your backend.
}

impl MyComponent {
    fn fetch_data(&mut self) {
        let request = Request::get("/users") // Replace with the actual endpoint
            .body(Nothing)
            .expect("Failed to build request");

        let callback = self.fetch_service.fetch(
            request,
            self.link().callback(|response: Response<Json<Result<Vec<User>, _>>>| {
                let Json(data) = response.into_body();
                match data {
                    Ok(users) => {
                        // Handle successful data retrieval here.
                        // You can set it to your component's data property.
                    }
                    Err(_) => {
                        // Handle errors here.
                    }
                }
            }),
        );

        self.fetch_task = Some(callback);
    }
}
