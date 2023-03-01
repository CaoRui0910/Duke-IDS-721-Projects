# Duke-IDS-721-Projects
## 1.1 Week6-Project2-exchange-time-service
### Description
- My project 2 is used to parse the currency name entered by the user, and then return to the user the real-time exchange rate between that currency and Chinese Yuan (RMB). Also, I deploy this application on AWS AppRunner for Rust-based microservice.

### Usage
- First, user can download my code, then `cd my-microservice` in command line. Then, type type `cargo run`. 
  - Then, open the website http://vcm-30906.vm.duke.edu:8080/. User will get this page (home page):
<img width="983" alt="Screen Shot 2023-03-01 at 13 43 15" src="https://user-images.githubusercontent.com/93239143/222235157-2a68065a-09f5-414e-a3fc-997d6b49cb7d.png">
  - Then, open the website http://vcm-30906.vm.duke.edu:8080/exchange_rate. It will describe how to use the microservice.
<img width="807" alt="Screen Shot 2023-03-01 at 13 45 12" src="https://user-images.githubusercontent.com/93239143/222235505-8ecac747-622d-4305-8547-3fc9c2965b8d.png">
  - Then, open the website using '/exchange_rate/<currency name>'. For example, users can see the exchange rate between USD and RMB by open this website http://vcm-30906.vm.duke.edu:8080/exchange_rate/USD.
<img width="700" alt="Screen Shot 2023-03-01 at 13 48 16" src="https://user-images.githubusercontent.com/93239143/222235958-fd1c3c87-1574-45c1-bc4f-37d32d2a75d4.png">

- Second, I push it to AWS ECR. According to the introduction of this video (https://www.youtube.com/watch?v=I3cEQ_7aD1A), I implemented this step.
<img width="1318" alt="Screen Shot 2023-03-01 at 13 58 15" src="https://user-images.githubusercontent.com/93239143/222237974-ea7ae817-0fc7-4566-921f-583036a8d041.png">
  - So, now user can use my microservice by opening this website https://3mzm5dnk7d.us-east-1.awsapprunner.com/. For example, open this website: https://3mzm5dnk7d.us-east-1.awsapprunner.com/exchange_rate/JPY.
<img width="712" alt="Screen Shot 2023-03-01 at 13 59 49" src="https://user-images.githubusercontent.com/93239143/222238263-5fee92af-5c1e-45aa-97f5-78c6cd5dc548.png">
