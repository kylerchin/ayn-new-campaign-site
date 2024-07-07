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
        <div class="w-full bg-octablue text-white text-center py-8 flex flex-col items-center px-8">
            <div class="py-2 lg:py-6 px-2 border-white border-2 mx-auto lg:w-1/2">"Paid for by Ayn Craciun for Council 2024 FPPC #1464914"</div>

            <p>"For campaign inquires please contact: ayn@ayn4irvine.com"</p>
        </div>
    }
}

#[component]
fn EndorsementHome() -> impl IntoView {
    view!{
        <div class="w-full">
            
        </div>
    }
}

#[component]
fn WhyAynHome() -> impl IntoView {
    view! {
        <div class="w-full flex flex-col md:flex-row">
            <img src="/assets/ayn-speech.jpg" class="w-full md:w-1/3 aspect-[4/3]"/>
            <div class="py-16">
                <p>"Why Ayn"</p>
            </div>
        </div>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button

    view! {
        <div
       class="mainhero w-full bg-center text-white bg-cover py-64"
        >
        <div class="w-1/2 ml-8 md:ml-16 lg:ml-32 2xl:ml-64 md:w-96">
        <h1 class="text-xl md:text-2xl font-bold text-white">"Vote for Ayn Craciun for
        Irvine City Council"</h1>
        <p class="text-white">"Ayn is an environmental health advocate, working mom, and Democrat ready to deliver results for Irvine families."</p>
        
        <div class="flex flex-row mt-2 gap-x-2">
            <div class="font-bold bg-octaorange text-white px-4 py-2 rounded-md">"Volunteer"</div>
            <div class="font-bold text-octaorange bg-white px-4 py-2 rounded-md">"Donate"</div>
        </div>

        </div>
        </div>

        <WhyAynHome/>
        <EndorsementHome/>
        
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
        <h1>"Ooops! We couldn't find this page!"</h1>
        <p>"Perhaps it is wherever Mike Carroll is as well?"</p>
    }
}
