use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view!{
        <div>

        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/ayn-new-campaign-site.css"/>
        <link rel="icon" href="/assets/ayn-favicon.png"/>
        <link rel="preconnect" href="https://fonts.googleapis.com"/>
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
<link href="https://fonts.googleapis.com/css2?family=Lato:ital,wght@0,100;0,300;0,400;0,700;0,900;1,100;1,300;1,400;1,700;1,900&display=swap" rel="stylesheet"/>

        // sets the document title
        <Title text="Ayn Craciun, Democrat & Environmental Health Advocate for Irvine City Council"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view!{
        <div class="w-full bg-octablue text-white text-center py-8">
            <div class="py-2 py-8 border-white border-2 mx-auto">"Paid for by Ayn Craciun for Council 2024 FPPC #1464914"</div>

            <p>"For campaign inquires please contact: ayn@ayn4irvine.com"</p>
        </div>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button

    view! {
        <div
       class="mainhero w-full bg-center bg-cover py-64"
        >
        <h1 class="text-lg text-white md:ml-1/4">"Vote for Ayn Craciun for
        Irvine City Council"</h1>
        </div>
        <Footer/>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
