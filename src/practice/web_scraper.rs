use std::{ sync::Arc, thread };
use ureq::AgentBuilder;
use std::time::Instant;

pub fn scraper() -> Result<(), ureq::Error> {
    let webpages = vec![
        "https://github.com/danslobodan/ticketing/blob/main/.gitignore",
        "https://github.com/danslobodan/ticketing/blob/main/.prettierrc",
        "https://github.com/danslobodan/ticketing/blob/main/client/pages/_app.js",
        "https://github.com/danslobodan/ticketing/blob/main/client/pages/index.js",
        "https://github.com/danslobodan/ticketing/blob/main/client/pages/tickets/%5BticketId%5D.js",
        "https://github.com/danslobodan/ticketing/blob/main/client/pages/tickets/new.js",
        "https://github.com/danslobodan/ticketing/blob/main/client/pages/auth/signin.js",
        "https://github.com/danslobodan/ticketing/blob/main/client/pages/auth/signout.js",
        "https://github.com/danslobodan/ticketing/blob/main/client/pages/auth/signup.js"
    ];

    let agent = AgentBuilder::new().build();
    let now = Instant::now();

    for web_page in &webpages {
        let web_body = agent.get(web_page).call().unwrap().into_string().unwrap();
    }
    println!("Time taken without threads: {:.2?}", now.elapsed());

    let now = Instant::now();

    let agent = Arc::new(agent);
    let mut handles: Vec<thread::JoinHandle<Result<(), ureq::Error>>> = Vec::new();

    for web_page in webpages {
        let agent_thread = agent.clone();
        let t = thread::spawn(move || {
            let web_body = agent_thread.get(web_page).call().unwrap().into_string().unwrap();
            Ok(())
        });
        handles.push(t);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Time taken with threads: {:.2?}", now.elapsed());
    Ok(())
}
