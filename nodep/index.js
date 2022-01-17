const http = require('http')

const server = http.createServer((req, res) => {
    res.statusCode = 200
    res.setHeader('Content-type', 'text/plain')
    res.end('Hello world!')
})

server.listen(8080, '0.0.0.0', () => {
    console.log("Nodejs server started on 0.0.0.0 port 8080");
})