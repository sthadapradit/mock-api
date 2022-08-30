## Create docker image
```
cd api
docker build . -t rust_api  
docker run -p 3030:3030 rust_api
```

Then browse to localhost:3030/properties
