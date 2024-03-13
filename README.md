### Install cargo documentation
```
cargo doc
cargo doc --open
```

### Install cargo watch
```
cargo install cargo-watch
cargo watch -x run ##execute with watch
```


### run
sudo su
git clone https://github.com/wlopezob/casino-v1 -b feature/v1
cd casino-v1
vim .env
cargo build --release
./target/release/casino-v1
nohup ./target/release/casino-v1 > output.log 2>&1 &


## install nginx
```
sudo su
sudo vim /etc/nginx/conf.d/myapp.conf

# content myapp.conf
server {
    listen 80;
    server_name "";

    location / {
        proxy_pass http://localhost:8080; # El puerto donde corre tu app Axum
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
}

sudo nginx -t
sudo systemctl restart nginx
```