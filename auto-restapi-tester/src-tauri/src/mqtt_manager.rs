// extern crate paho_mqtt as mqtt;
// use std::{
//     env,
//     str,
//     process,
//     thread,
//     error::Error,
//     time::Duration
// };
// use paho_mqtt::{Client, ConnectOptions};
//
// const DFLT_BROKER:&str = "tcp://192.168.11.100:61614";
// const DFLT_CLIENT:&str = "rust_subscribe";
// const DFLT_TOPICS:&[&str] = &["rust/mqtt", "rust/test"];
// const DFLT_QOS:&[i32] = &[0, 1];
//
// struct Mqtt_manager {
//     client: Client,
//     connect_option: ConnectOptions,
//
// }
//
// impl Mqtt_manager {
//
//     pub fn init(&mut self){
//
//     }
//
//     pub fn connect(&mut self, url: String) {
//         let host = env::args().nth(1).unwrap_or_else(||
//             url.to_string()
//         );
//
//         // Define the set of options for the create.
//         // Use an ID for a persistent session.
//         let create_opts = mqtt::CreateOptionsBuilder::new()
//             .server_uri(host)
//             .client_id(DFLT_CLIENT.to_string())
//             .finalize();
//
//         // Create a client.
//         self.client = Client::new(create_opts).unwrap_or_else(|err| {
//             println!("Error creating the client: {:?}", err);
//             process::exit(1);
//         });
//
//
//         // Initialize the consumer before connecting.
//         let rx = self.client.start_consuming();
//
//         // Define the set of options for the connection.
//         let lwt = mqtt::MessageBuilder::new()
//             .topic("test")
//             .payload("Consumer lost connection")
//             .finalize();
//         self.connect_option = mqtt::ConnectOptionsBuilder::new()
//             .keep_alive_interval(Duration::from_secs(20))
//             .clean_session(false)
//             .will_message(lwt)
//             .finalize();
//         match self.client.connect(self.connect_option.clone()){
//             Ok(_) => {}
//             Err(e) => {println!("Unable to connect:\n\t{:?}", e);process::exit(1);}
//         }
//
//     }
//
//     pub fn subscribe_topics(&mut self,topic: &[&str]) {
//         if let Err(e) = self.client.subscribe_many(topic, DFLT_QOS) {
//             println!("Error subscribes topics: {:?}", e);
//             process::exit(1);
//         }
//     }
//     pub fn unsubscribe_topics(&mut self,topic: &[&str]) {
//         if let Err(e) = self.client.unsubscribe_many(DFLT_TOPICS) {}
//     }
//     pub fn start_receive(){
//
//     }
//     pub fn stop_receive(){
//
//     }
// }