#!/usr/bin/env python3
from http.server import HTTPServer, BaseHTTPRequestHandler
import getopt
from os.path import isdir, isfile
import sys
import time
import os


# global defaults
global index
index = "index.html"

global directory
directory = "."


def usage():
    print("Usage: server.py -p <port> -a <address> -i <index> -d <directory> -h")
    print("Options:")
    print("  -p <port>        Port number")
    print("  -a <address>     Address")
    print("  -i <index>       Index file")
    print("  -d <directory>   Directory")
    print("  -h               Help")
    sys.exit(0)


class CustomServer(BaseHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        with open(os.path.join(directory, index), "rb") as f:
            self.index = f.read()
        super().__init__(*args, **kwargs)

    def do_GET(self):
        print("GET request: %s" % self.path)
        path = os.path.join(directory, self.path[1:])
        if os.path.isdir(path):
            # TODO: list directory
            html = "<html><head><title>Directory listing</title></head><body><h1>Directory listing</h1><ul>"
            for file in os.listdir(path):
                last_modified = os.path.getmtime(os.path.join(path, file))
                last_modified = time.strftime("%Y-%m-%d %H:%M:%S", time.gmtime(last_modified))
                size = os.path.getsize(os.path.join(path, file))
                html += "<li><a href='%s'>%s</a> (%s bytes, %s)</li>" % (file, file, size, last_modified)
            html += "</ul></body></html>"
            self.send_response(200)
            self.send_header("Content-type", "text/html")
            self.end_headers()
            self.wfile.write(html.encode("utf-8"))
            return
        elif os.path.isfile(path):
            content_type = determine_content_type(path)
            with open(path, "rb") as f:
                self.send_response(200)
                self.send_header("Content-type", content_type)
                self.end_headers()
                self.wfile.write(f.read())
                return
        else:
            html = "<html><head><title>404 Not Found</title></head><body><h1><a href=\"https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/404\">404 Not Found</a></h1><p>The requested URL was not found on this server.</p></body></html>"
            self.send_response(404)
            self.send_header("Content-type", "text/html")
            self.end_headers()
            self.wfile.write(html.encode("utf-8"))
            return


def determine_content_type(filename: str):
    if filename.endswith(".html"):
        return "text/html"
    elif filename.endswith(".css"):
        return "text/css"
    elif filename.endswith(".js"):
        return "application/javascript"
    elif filename.endswith(".jpg") or filename.endswith(".jpeg"):
        return "image/jpeg"
    elif filename.endswith(".png"):
        return "image/png"
    elif filename.endswith(".pdf"):
        return "application/pdf"
    elif filename.endswith(".json"):
        return "application/json"
    elif filename.endswith(".txt"):
        return "text/plain"
    else:
        return "application/octet-stream"


if __name__ == "__main__":
    optlist, args = getopt.getopt(sys.argv[1:], 'p:a:i:d:h', [])

    # defaults:
    port = 9000
    address = "0"

    for opt, arg in optlist:
        if opt == "-p":
            port = int(arg)
        elif opt == "-a":
            address = arg
        elif opt == "-i":
            index = arg
        elif opt == "-d":
            directory = arg
        elif opt == "-h":
            sys.exit(0)

    print(os.listdir(directory))
    server = HTTPServer((address, port), CustomServer)
    print("Server started on http://%s:%s" % (address, port))
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        print("Exiting gracefully...")
        server.shutdown()
        server.server_close()

