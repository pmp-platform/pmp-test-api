# Dashboard UI integration test
GET http://app:8080/

HTTP 200
[Asserts]
header "Content-Type" contains "text/html"
body contains "PMP Test API"
body contains "System Information"
body contains "Dashboard"
