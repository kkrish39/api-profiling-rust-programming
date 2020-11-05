# About the project
 - The project is built on rust with me having no prior rust programming experience. 
 - Only the structopt library is used to handle command-line parsing.
 - The HTTP request call is established through a TCP connection with the host.

# Steps to run
 USAGE:
 cargo run -- <url> <profile>
 ARGS:
 <url> An url endpoint to which the requests must be made
 <profile> An interger value indicating the number times <url> needs to be hit

# Attached Images (/Images)
 - 'Zoom in' and 'Zoom out' version of https://my-worker.kkrish39.workers.dev/links
 - A partial snapshot of https://jsonplaceholder.typicode.com/todos

# Shortened output - (https://my-worker.kkrish39.workers.dev/links)
keerthivasankrishnamurthy@Keerthivasans-MacBook-Pro grrs % cargo run -- "https://my-worker.kkrish39.workers.dev/links" 30
 Finished dev [unoptimized + debuginfo] target(s) in 0.09s
 Running `target/debug/cloudflare_systems_assignment 'https://my-worker.kkrish39.workers.dev/links' 5`
buf = HTTP/1.1 200 OK
Date: Wed, 28 Oct 2020 03:49:57 GMT
Content-Type: application/json;charset=UTF-8
Content-Length: 289
Connection: close
Set-Cookie: __cfduid=d6c25d5a20762e5c49d902a480fb2e1da1603856997; expires=Fri, 27-Nov-20 03:49:57 GMT; path=/; domain=.kkrish39.workers.dev; HttpOnly; SameSite=Lax
cf-request-id: 060eecb2a200001593789e7000000001
Report-To: {"endpoints":[{"url":"https:\/\/a.nel.cloudflare.com\/report?s=u4FvILpCm5yGZDihPdJee5y7XLJzjp0uMDMOoGTetTqX%2BvAbn%2FwUeLXV9P8nDoHCnga77xgzbaMOuVUs%2FkviZ0iJlD6EiFgfdqg6ZmvKuYqEgMxWc1um3PHpAvT%2FNTYvVOjAk4r5Iwo56vk%3D"}],"group":"cf-nel","max_age":604800}
NEL: {"report_to":"cf-nel","max_age":604800}
Server: cloudflare
CF-RAY: 5e91b0976f961593-EWR

[
 {
 "name": "Cloudflare",
 "url": "https://api.cloudflare.com"
 },
 {
 "name": "LinkedIn",
 "url": "https://www.linkedin.com"
 },
 {
 "name": "Google",
 "url": "https://www.google.com"
 },
 {
 "name": "Glassdoor",
 "url": "https://www.glassdoor.com/"
 }
]
.
.
. #shortened
.
.
buf = HTTP/1.1 200 OK
Date: Wed, 28 Oct 2020 03:49:57 GMT
Content-Type: application/json;charset=UTF-8
Content-Length: 289
Connection: close
Set-Cookie: __cfduid=d130ec3952b22a18b58008ea583b278d51603856997; expires=Fri, 27-Nov-20 03:49:57 GMT; path=/; domain=.kkrish39.workers.dev; HttpOnly; SameSite=Lax
cf-request-id: 060eecb4150000e6b818979000000001
Report-To: {"endpoints":[{"url":"https:\/\/a.nel.cloudflare.com\/report?lkg-colo=11&lkg-time=1603856997"}],"group":"cf-nel","max_age":604800}
NEL: {"report_to":"cf-nel","max_age":604800}
Server: cloudflare
CF-RAY: 5e91b099bdf4e6b8-EWR

[
 {
 "name": "Cloudflare",
 "url": "https://api.cloudflare.com"
 },
 {
 "name": "LinkedIn",
 "url": "https://www.linkedin.com"
 },
 {
 "name": "Google",
 "url": "https://www.google.com"
 },
 {
 "name": "Glassdoor",
 "url": "https://www.glassdoor.com/"
 }
]
Number of Requests --> 30
The Fastest response time --> 54ms
The Slowest response time--> 195ms
The Size in bytes of the smallest Response --> 884
The Size in bytes of the largest Response --> 1021
The Mean Time --> 70.8ms
The Median Time --> 59.5ms
The Percentage of Requests succeded --> 100%

# Shortened output - (https://jsonplaceholder.typicode.com/todos)

keerthivasankrishnamurthy@Keerthivasans-MacBook-Pro grrs % cargo run -- "https://jsonplaceholder.typicode.com/todos" 30 
 Finished dev [unoptimized + debuginfo] target(s) in 0.08s
 Running `target/debug/cloudflare_systems_assignment 'https://jsonplaceholder.typicode.com/todos' 2`
buf = HTTP/1.1 200 OK
Date: Wed, 28 Oct 2020 03:51:09 GMT
Content-Type: application/json; charset=utf-8
Connection: close
Set-Cookie: __cfduid=dbd35f6f651b5b9a777cb1c20accdfa821603857069; expires=Fri, 27-Nov-20 03:51:09 GMT; path=/; domain=.typicode.com; HttpOnly; SameSite=Lax
X-Powered-By: Express
X-Ratelimit-Limit: 1000
X-Ratelimit-Remaining: 999
X-Ratelimit-Reset: 1602124042
Vary: Origin, Accept-Encoding
Access-Control-Allow-Credentials: true
Cache-Control: max-age=43200
Pragma: no-cache
Expires: -1
X-Content-Type-Options: nosniff
Etag: W/"5ef7-4Ad6/n39KWY9q6Ykm/ULNQ2F5IM"
Via: 1.1 vegur
CF-Cache-Status: HIT
Age: 768
cf-request-id: 060eedcd380000efe065b09000000001
Report-To: {"endpoints":[{"url":"https:\/\/a.nel.cloudflare.com\/report?lkg-colo=11&lkg-time=1603857069"}],"group":"cf-nel","max_age":604800}
NEL: {"report_to":"cf-nel","max_age":604800}
Server: cloudflare
CF-RAY: 5e91b25b8bd4efe0-EWR

[
 {
 "userId": 1,
 "id": 1,
 "title": "delectus aut autem",
 "completed": false
 },
 {
 "userId": 1,
 "id": 2,
 "title": "quis ut nam facilis et officia qui",
 "completed": false
 },

.
.
. #shortened
.
.


{
 "userId": 10,
 "id": 198,
 "title": "quis eius est sint explicabo",
 "completed": true
 },
 {
 "userId": 10,
 "id": 199,
 "title": "numquam repellendus a magnam",
 "completed": true
 },
 {
 "userId": 10,
 "id": 200,
 "title": "ipsam aperiam voluptates qui",
 "completed": false
 }
]

Number of Requests --> 30
The Fastest response time --> 56ms
The Slowest response time--> 178ms
The Size in bytes of the smallest Response --> 25243
The Size in bytes of the largest Response --> 25372
The Mean Time --> 87.53333333333333ms
The Median Time --> 75ms
The Percentage of Requests succeded --> 100%

# Comparison:

For comparison, I took a dummy JSON endpoint that returns a JSON body. Here, both the requests are hit 5 times. We can compare and infer that the fastest response time was recorded by the 'jsonplaceholder' endpoint.

We can see that the average of both the endpoints is also less hence showing us both the endpoints are performing more or less at the same level.

Median usually helps in getting the histogram of the requests. The higher median in the 'my-worker.kkrish39' endpoint shows the high level of variation in the response time over the 30 requests. Rather in the case of 'jsonplaceholder' endpoint the median of less variant in comparison with the mean.


# limitations:

Time being the code is structured in such a way that, it will run only if there is a single level of specified endpoint.