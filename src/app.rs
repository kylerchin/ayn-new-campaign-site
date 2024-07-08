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
                    <Route path="meet-ayn" view=WhyAynPage/>
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
        <div class="w-full px-32 py-8 md:py-16">
                <p class="font-bold text-octaorange text-lg md:text-xl">"Endorsements"</p>

                <div class="flex flex-col md:flex-row gap-x-4 lg:gap-x-7 w-full align-middle items-center">
                    <img src="/assets/dpoc-logo.png" class="h-16 md:h-32"
                    alt="Democratic Party of Orange County logo"
                    />
                    <img src="/assets/momsaction.png" class="h-16 md:h-32"
                    alt="Moms Demand Action for Gun Sense in America logo"
                    />
                </div>
                <br/>

                <div class="flex flex-col md:flex-row gap-x-4 lg:gap-x-7 w-full align-top items-start">
                <OrgEndorse
                img={String::from("/assets/oc-action.png")}
                name={String::from("OC Action")}
                />
                   
                    <OrgEndorse
                    img={String::from("/assets/rise.png")}
                    name={String::from("Remake Irvine Streets for All")}
                    />

                    <OrgEndorse
                    img={String::from("/assets/sunriseoc.png")}
                    name={String::from("Sunrise Movement OC")}
                    />

                    <PersonEndorse
                    img={String::from("/assets/kathleen.png")}
                    name={String::from("Kathleen Treseder, PhD")}
                    position={String::from("UC Irvine Ecology Professor\n")}
                    position2={String::from("Irvine Councillor")}
                    />

                    <PersonEndorse
                    img={String::from("/assets/katie.png")}
                    name={String::from("Katie Porter")}
                    position={String::from("US Representative")}
                    position2={String::from(" 47th District")}
                    />

                </div>
        </div>
    }
}

#[component]
fn WhyAynHome() -> impl IntoView {
    view! {
        <div class="w-full flex flex-col md:flex-row">
            <img class="w-full aspect-video md:aspect-auto md:w-2/5 why-ayn-img bg-center bg-cover"/>
            <div class="py-16 px-32 md:w-3/5">
                <p class="text-lg font-bold">"Why Ayn"</p>
                <p>
                "Ayn Craciun is a proven community leader, environmental health advocate, Democrat and working mom. Ayn is running for Irvine City Council to create a safer and more sustainable Irvine for everyone, providing a check on corporate special interests in our city."</p>
                <br/>
                <p>"Ayn has a well-established track record of speaking truth to power and winning change that brings lasting benefits to our community -- especially families. Ayn will focus city resources on actions that make our streets safer, our air cleaner, reduce traffic and increase affordable, climate-friendly housing and energy."</p>
           
                <br/>
               <a class="hover:underline" href="/meet-ayn"><button class="bg-octablue text-white hover:bg-blue-500 font-bold py-2 px-2 rounded-lg">"Learn more about Ayn"</button> </a>
            </div>
        </div>
    }
}

#[component]
fn WhyAynPage() -> impl IntoView {
    view!{

        <div class="text-2xl">"About Ayn"</div>
        <div class="flex flex-col md:flex-row w-full sm:w-auto">
        <div>
        <img src={"/assets/AynFull@KorinnePhoto-9.jpg"}
        />
        </div>
       <div class="px-16">
       
       <p>"Ayn Craciun is a proven community leader, environmental health advocate,  successful business woman, and working mom. Ayn has a well-established track record of winning changes that protect our health and bring lasting community benefits."
        </p>
        <br/>
        <p>"Ayn is a Democrat who believes our government should reflect and serve the needs of the people and small businesses, not special interests or corporate donors. Ayn has spoken out against corruption in local public agencies, and in 2023 the Orange County Register named Ayn among the most influential people in Orange County."</p>
        <br/>  <p>
       "Ayn chairs of the City of Irvine Sustainability Commission, and is the OC Policy Director for a nonprofit climate policy watchdog that provides transparency and accountability on climate action by local governments. Ayn played a pivotal role in creating the Orange County Power Authority, bringing clean energy choice to OC for the first time ever. Ayn also helped win Irvine's policy to end the use of harmful fossil gas in new buildings -- a first in OC, and the recently approved ban on noisy, polluting gas-powered leaf blowers in the city."
       </p>
       <br/>
        <p>
        "Ayn is also an advocate for safe, climate-friendly transportation. In 2023, she helped University High School parents win safe bus transportation for southeast Irvine kids to University High School for the first time ever."
        </p>
        <br/>
        <p>
        "In 2016, after learning that three Quail Hill children had been diagnosed with brain tumors, Ayn partnered with other parents and UCI scientists to found Non Toxic Irvine and win an organic landscaping policy for the City of Irvine, which was recognized for excellence by the California Environmental Protection Agency. Ayn wrote a how-to guide and helped people from all over the U.S., other OC cities and several Irvine HOAs including Hidden Canyon and Laguna Altura win organic landscaping policies for their communities. "
        </p>
        <br/>
        <p>
        "Ayn works in coalition with dozens of environmental, small businesses, religious, and community organizations in support of critical local climate policy decisions. She believes everyone deserves safety and equality, and will fight to protect the freedom of people of all races, ethnicities, religions, sexual orientations and gender identities."</p>
        <br/>
        <p>"Ayn served as president of her HOA for four years, ensuring a balanced budget and well-managed community services throughout her leadership. Ayn is also a past president of the Southeast Irvine MOMS club, former PTA committee chair, and frequent room mom at her kids’ elementary school."</p>
        <br/>
        <p>"Ayn and her husband Joe, an attorney and small business owner, have lived in Quail Hill for 20 years. Their two adorable school-aged children attend Irvine public schools."</p>
       </div>

       </div>

       <Footer/>

    }
}

#[component]
fn OrgEndorse(img: String, name: String) -> impl IntoView {
    view! {
        <div class="flex flex-col align-middle items-center"><img src={img} class="flex-grow-0 h-16 md:h-32" alt={format!("{} logo", name)}/>
        <p class="font-bold">{name.clone()}</p>
        </div>
    }
}

#[component]
fn PersonEndorse(img: String, name: String, position: String, position2: String) -> impl IntoView {
    view! {
        <div class="flex flex-col align-middle items-center"><img src={img} class="flex-grow-0 h-16 md:h-32  rounded-lg" alt={format!("{} portrait", name)}/>
        <p class="font-bold">{name.clone()}</p>
        <p>{position}</p>
        <p>{position2}</p>
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
        <a href="https://docs.google.com/forms/d/e/1FAIpQLSdZ1mzVYlWSv3M_wOI3q5YlQZeN22NKNVAZ9Hf47ueARc-cig/viewform">
        <div class="font-bold bg-octaorange text-white px-4 py-2 rounded-md">"Volunteer"</div></a>
            <a href="https://www.efundraisingconnections.com/c/AynCraciun">
            
            <div class="font-bold text-octaorange bg-white px-4 py-2 rounded-md">"Donate"</div>
            </a>
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
