# slowhttp
A very slow HTTP service designed to play a harmless prank on script kiddies scanning your server.
This service is developed in Rust, but don't worry if you're unfamiliar with Rust. It's ready to use out of the box—just download the release corresponding to your CPU architecture, unzip it, and execute it.

```
Parameters:
--port: The port number to bind the service (default is 8080).
--message: The message to send to the other party (default: international friendly message).
--times: The number of repetitions.

For example:
./slowhttp --port 8000 --message "Don't mess with me!" --times 60
```

# First, configure an upstream in your Nginx configuration.
```
upstream slow {
server 127.0.0.1:8080;
}
```

# Then, set up a location block to proxy requests to the upstream.

```
location /phpinfo.php {
proxy_pass http://slow;
}
```

With this configuration, when script kiddies scan your server, they’ll receive your friendly greeting! 😄🚀
