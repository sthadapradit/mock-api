## Nginx and Rust Warp (web) Brenchmark
This repository contains rust projects which is used to compare response rate between nginx and warp server (rust).

The client side just send concurrent request to hit webserver and get response stata. We can set number of concurrent through environment variable to experiment which number is fit for us.

There are 3 compoonents in the project as bellow
1. Nginx
2. Rust warp
3. Client 


### To create and up nginx on port 8000 run this command
```
cd ./nginx
docker build . -t nginx_img
docker run -it --rm -p 8000:8000 nginx_img nginx
```


### To create and up Rust warp on port 3030 run this command
```
cd ./api
docker build . -t warp_img
docker run -it --rm -p 3030:3030 warp_img
```


### To create and up Client use this command, we can change port on the environment variable to choose which destination we want
```
cd ./client
docker build . -t client_img

//for rust warp
docker run -e url=http://localhost:3030/properties -e concurrent=100 client_img ./client


//for nginx
docker run -e url=http://localhost:8000/properties -e concurrent=100 client_img ./client
```


## Or run with cargo 

Run with rust webserver
```
url=http://localhost:3030/properties.json concurrent=100 cargo run
```


Run with nginx
```
url=http://localhost:8000/properties.json concurrent=100 cargo run
````