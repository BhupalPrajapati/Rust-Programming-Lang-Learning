use std::thread;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use ureq::{Agent,AgentBuilder};
fn main()-> Result<(), ureq::Error>{
 let webpages= vec![
    "https://github.com/BhupalPrajapati",
    "https://github.com/BhupalPrajapati/1231231",
    "https://github.com/Bhupalkadkskja/1231231",
    "https://github.com/jjhewqs/1231231",
    "https://github.com/bhuiaid/ahd2324"
 ];
 // to convert the web pages infromation to the textual form we need to create the agent builder
 let agent = ureq::AgentBuilder::new().build();

 // we need to functiom which give as to time module(current time modulw)

 let now = Instant::now();

 // we need to read the textual information from the web page

 for web_page in &webpages{
   // this is send out the request to the access the reading the web url

   let web_body = agent.get(web_page).call()?.into_string()?;

 }
 print!("{:.2?}",now.elapsed()); // the elapsed time function returns the time taken in that thread. the dot part is a fraction which take couple of digits


 // call the function now to keep track of the time 
 let now = Instant::now();

 // for callinf each thread we are creating the Arc
 let agent = Arc::new(agent);

 let mut handles = Vec<thread::JoinHandle<Result(),ureq::Error>>> = Vec::new();

 for web_page in webpages{
    let agent_thread = agent.clone();
    let t= thread::spawn(move ||{
        // code for read the contents of the web page
        let web_body = agent_thread.get(web_page).call()?.into_string()?;
        Ok(())
 });
    

    handles.push(t);
}
    for  handle in handles{
        handle.join().unwrap();
    }
    print!("{:.2?}",now.elapsed());
    
 }



 