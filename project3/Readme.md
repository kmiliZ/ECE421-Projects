# Connect4
This application includes the computerized versions of the games "Connect4" and "Toot & Otto".
The detailed instructions on set-up and execution of the project is discussed below. <br><br>
## Geting Started
### Get front end working
Install trunk, which is a WASM web application bundler for Rust

```
cargo install trunk
```

Add wasm32-unknown-unknown target to the setup instructions

```
rustup target add wasm32-unknown-known
```

Start up the front end using trunk server inside **project3/web**

```
trunk serve
``` 
### Get server side running
If you don't have docker install, [install](https://www.docker.com/products/docker-desktop/) it first then run the following commands inside **project3/server**
``` 
docker-compose up -d

cargo install cargo-watch 

cargo watch -q -c -w src/ -x run
``` 

### See the result
trunk serve by default will run the application on port 8080. To view the application window, open browser and type: http://localhost:8080
Enjooy!


## Run CLI
go to **project3/cli**, then run
```
cargo run
```


roup member
Qi Zhou
Tianyuan Fang
Jakob Lau
