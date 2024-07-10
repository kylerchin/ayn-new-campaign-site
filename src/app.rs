use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use std::ops::Not;

#[component]
pub fn Navbar() -> impl IntoView {
    let (navbar_open, set_navbar_open) = create_signal(false);

    view! {
            <div class="flex flex-row h-16 py-2 px-4">
               <a href="/" class=""> <img class="h-14" src="/assets/ayn-logo.jpeg" alt="Ayn for Irvine City Council Logo"/></a>

                <div class="md:hidden my-auto  ml-auto" on:click={move |_| set_navbar_open.update(|value| {
                    *value = value.not();
                })}>
                <span class="material-symbols-outlined text-gray-900">
    menu
    </span>
                </div>

                <div class="ml-auto flex-row gap-x-3 align-middle items-center hidden md:flex">
                    <a href="/meet-ayn">"About Ayn"</a>
                    <a href="/endorsements">"Endorsements"</a>

                    <a href="/events">"Events"</a>
                    <a href="https://docs.google.com/forms/d/e/1FAIpQLSdZ1mzVYlWSv3M_wOI3q5YlQZeN22NKNVAZ9Hf47ueARc-cig/viewform">"Volunteer"</a>
                    <a href="https://www.efundraisingconnections.com/c/AynCraciun">
                <div class="font-bold bg-octaorange text-white px-3 py-1.5 rounded-md">"Donate"</div>
                </a>
                </div>

            </div>

            {
                move || if navbar_open.get() {
                    view!{
                        <div class="flex flex-col gap-y-2 underline px-4">
                        <a href="/why-ayn" class="hover:text-blue-800">"About Ayn"</a>
                    <a href="/endorsements"  class="hover:text-blue-800">"Endorsements"</a>
                    <a href="/events"  class="hover:text-blue-800">"Events"</a>
                    <a  class="hover:text-blue-800" href="https://docs.google.com/forms/d/e/1FAIpQLSdZ1mzVYlWSv3M_wOI3q5YlQZeN22NKNVAZ9Hf47ueARc-cig/viewform">"Volunteer"</a>
                    <a  class="hover:text-blue-800" href="https://www.efundraisingconnections.com/c/AynCraciun">"Donate"</a></div>

                }
            } else {
                view!{<div></div>}
                    }
                }
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
            <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@24,400,0,0" />
            <link rel="preconnect" href="https://fonts.googleapis.com"/>
    <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin/>
    <link href="https://fonts.googleapis.com/css2?family=Lato:ital,wght@0,100;0,300;0,400;0,700;0,900;1,100;1,300;1,400;1,700;1,900&display=swap" rel="stylesheet"/>

            // sets the document title
            <Title text="Ayn Craciun, Democrat & Environmental Health Advocate for Irvine City Council"/>

            // content for this welcome page
            <Router>
                <main class="h-full">
                    <Routes>
                        <Route path="" view=HomePage/>
                        <Route path="meet-ayn" view=WhyAynPage/>
                        <Route path="/*any" view=NotFound/>
                        <Route path="/endorsements" view=EndorsementPage/>
                        <Route path="/events" view=EventsPage/>
                    </Routes>
                </main>
            </Router>
        }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
        <div class="mt-auto w-full bg-octablue text-white text-center py-8 flex flex-col items-center px-8">
            <div class="py-2 lg:py-6 px-2 border-white border-2 mx-auto lg:w-1/2">"Paid for by Ayn Craciun for Council 2024 FPPC #1464914"</div>

            <p>"For campaign inquires please contact: ayn@ayn4irvine.com"</p>
        </div>
    }
}

#[component]
fn EndorsementPage() -> impl IntoView {
    view! {
            <Navbar/>

            <div class="mx-4 md:mx-12">
            <p class="text-octaorange text-2xl">"Endorsements"</p>
            <p class="text-octaorange text-lg">"Elected Officials"</p>
            <div>
       <ul>

       <li>
       "Katie Porter, U.S. Congresswoman"
       </li>
    <li>
       "Anthony Rendon, California Assembly Speaker Emeritus"
       </li>
    <li>
       "Cottie Petrie-Norris, California Assembly Member "
       </li>
    <li>


           "Kathleen Treseder, Irvine City Council Member & UC Irvine Professor "
           </li>
    <li>
           "Tammy Kim, Irvine City Council Member "
           </li>
    <li>
       "Sue Kempf, Laguna Beach Mayor "
       </li>
    <li>
       "Alex Rounaghi, Laguna Beach Council Member "
       </li>
    <li>
       "John Stephens, Costa Mesa Mayor"
       </li>
    <li>
       "Arlis Reynolds, Costa Mesa Council Member "
       </li>
    <li>
       "Manuel Chavez, Costa Mesa Council Member "
       </li>
    <li>
       "Natalie Moser, Huntington Beach Council Member "
       </li>
    <li>
       "Dr. Shana Charles, Fullerton Council Member & CSUF Professor "
       </li>
    <li>
       "Connor Traut, Buena Park Council Member"
       </li>
    <li>
       "Jose Trinidad Castaneda, Buena Park Councilmember "
       </li>
    <li>
       "Ahmad Zahra, Fullerton Council Member "
       </li>
    <li>
       "Stephanie Oddo, Laguna Niguel Council Member "
       </li>
    <li>
       "Allyson Damikolas, Tustin Unified School District Trustee"
       </li>
    <li>
       "Kris Erickson, Orange Unified School District Trustee"
       </li>
    <li>
       "Ryan Dack, South Orange County Community College District Board Member "
       </li>
    <li>
    "Carolyn Inmon, South Orange County Community College District Board Member"</li>
       </ul>
            </div>


            <p class="text-octaorange text-lg">"Elected Officials"</p>

            <div>

            <li>"Democratic Party of Orange County"</li>

            <li>"Moms Demand Action"</li>

            <li>"OC Action"</li>

            <li>"Sunrise Movement, Orange County"</li>

            <li>"RISE (Remake Irvine Streets for Everyone)"</li>

            <li>"Branda Lin, City of Irvine Planning Commissioner and Irvine Watchdog Co-founder"</li>

            <li>"Dr. Jessica Pratt, UCI professor and Irvine resident"</li>

            <li>"Libby Wong Sawtell, attorney and Irvine resident"</li>

            <li>"Dr. Vasanthi Narayan, physician and Quail Hill resident"</li>

            <li>"Dr. Jessica Walden, owner/operator Amis de la Terre Zero Waste Market"</li>

            <li>"Susan Sayre, City of Irvine Wall of Recognition Honoree and governance watchdog"</li>

            <li>"Matthew Chang, Climate Garden"</li>

            <li>"Melisa Masri, Science Teacher/dept head and Irvine resident"</li>

            <li>"Youssef Kaddeche, Irvine Transportation Commissioner"</li>

            <li>"Bob Dougherty, former Treasurer Quail Hill Community Association"</li>

            <li>"Christine Byrd, HOA and school board member and Quail Hill resident"</li>

            <li>"Dr. Julie K. Hirota, physician and Quail Hill resident"</li>

            <li>"Naz Hamid, Chair, City of Irvine Community Services Commission"
            </li>
    <li>"Vi Thuy Nguyen MD, Pediatrician and Climate and Health Advocate
    "</li>
    <li>"Dr. Marie-Helene Luebbers "</li>
    <li>"Jeremy Ficarola, President, Cypress Village HOA"</li>
    <li>"Meredith Marquis, Chief of Staff"</li>
    <li>"Matthew Kramer, Quail Hill Resident"</li>
    <li>"Dr. Amy King-Henry, Lecturer in Ecology and Evolutionary Biology, UC Irvine"</li>
    <li>"Doug Elliott, Community Services Commissioner and Westpark II resident"</li>
    <li>"Judie Mancuso, Founder/CEO/President, Social Compassion in Legislation"</li>
    <li>"Lamar Kirchhevel, Medical Device R&D Engineer & Climate Activist"</li>
    <li>"Dr. Glenton Jelbert, PhD Cantab"</li>
    <li>"Gary M. Stewart, MD, Internal Medicine, and Citizens' Climate Lobby media manager, Laguna Chapter"</li>
    <li>"Harry Field, Retired Attorney"</li>
    <li>"Jake Comer, Civil Engineer and OC Climate Voter Guide Member"</li>
    <li>"Ken Montgomery, Chair Irvine Transportation Commission "</li>
    <li>"Dr. Ying Song, Engineer and Irvine resident"</li>
    <li>"Deeti Shah, Environmental Science and Policy student, Fridays for Future Orange County Co-organizer"</li>
    <li>"Taryn Williams, PhD candidate and Irvine DEI committee member"</li>

            </div>

            </div>

            <Footer/>
        }
}

#[component]
fn EndorsementHome() -> impl IntoView {
    view! {
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

                <div class="flex flex-col md:flex-row gap-x-4 lg:gap-x-7 w-full align-top md:items-start">
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
                <a class="hover:underline" href="/endorsements"><button class="bg-octablue text-white hover:bg-blue-500 font-bold py-2 px-2 rounded-lg">"All Endorsements"</button> </a>
        </div>
    }
}

#[component]
fn WhyAynHome() -> impl IntoView {
    view! {
        <div class="w-full flex flex-col md:flex-row ">
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
    view! {
    <div class="flex flex-col h-full">

    <Navbar/>

    <div class="mx-4 md:mx-12">
    <div class="text-2xl">"About Ayn"</div>
    <div class="flex flex-col md:flex-row w-full sm:w-auto mx-auto ">
    <div class="">
    <img class="md:max-w-md" src={"/assets/AynFull@KorinnePhoto-9.jpg"}
    />
    </div>
    <div class="px-16 ">

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
    </div>

     <Footer/></div>

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

#[component]
fn EventsPage() -> impl IntoView {
    view! {
        <div class="h-full flex flex-col">
            <Navbar/>
            <div class="w-full">
                <div class="mx-4 md:mx-auto md:max-w-2xl">

                <h1 class="text-lg text-octaorange font-bold">"Events"</h1>

                <p class="text-octablue">"Sunday, July 21"</p>

                <img src="/assets/eventturtlerock.jpg" alt="Join us to meet Ayn Craciun, an excellent candidate to represent Turtle Rock. Refreshments provided." class="max-w-sm"/>
        <br/>
        <p class="text-bold">"Turtle Rock Meet and Greet with Ayn Craciun"</p>

                <a href="https://forms.gle/ZMq8fyFcpfwCxZw3A">

            <button class="font-bold bg-octaorange text-white px-4 py-2 rounded-md">"RSVP"</button>
            </a>

                </div>
            </div>
            <br/>
            <Footer/>
        </div>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button

    view! {
        <Navbar/>
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
        <Navbar/>
        <h1>"Ooops! We couldn't find this page!"</h1>
        <Footer/>
    }
}
