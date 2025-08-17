let data = http_get("https://httpbin.org/get");
if type_of(data) == "map" {
    print(data.url);
    data.url
} else {
    print(data);
    data
}
