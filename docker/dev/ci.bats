@test "Node help flag works" {
  run themelio-node --help
  [ $status -eq 0 ]
}

@test "Node is running (via nmap)" {
  output="$(nmap 127.0.0.1 -p 11814 | tail -3 | head -1 | awk '{print $2}')"
  [ "$output" = "open" ]
}

@test "Node metrics webserver is running" {
  output="$(nmap 127.0.0.1 -p 8080 | tail -3 | head -1 | awk '{print $2}')"
  [ "$output" = "open" ]
}

@test "Node metrics webserver returns 200" {
  output="$(curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:8080/metrics)"
  [ "$output" = "200" ]
}

@test "Help flag works" {
  run melscan-backend --help
  [ $status -eq 0 ]
}

@test "Backend is running (via nmap)" {
  output="$(nmap 127.0.0.1 -p 13000 | tail -3 | head -1 | awk '{print $2}')"
  [ "$output" = "open" ]
}

@test "Frontend is running" {
  output="$(nmap 127.0.0.1 -p 3000 | tail -3 | head -1 | awk '{print $2}')"
  [ "$output" = "open" ]
}

@test "Frontend returns 200" {
  skip "Skipping until fastsync is implemented."
  output="$(curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:3000/)"
  [ "$output" = "200" ]
}

@test "Frontend ERG/MEL returns 200" {
  output="$(curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:3000/pools/ERG/MEL)"
  [ "$output" = "200" ]
}

@test "Frontend MEL/SYM returns 200" {
  output="$(curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:3000/pools/MEL/SYM)"
  [ "$output" = "200" ]
}

@test "Frontend hash search returns 200" {
  skip "Skipping until fastsync is implemented."
  output="$(curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:3000/blocks/1627701/774965c08b5aabe8e019384a73d5515c993ada00e8210098bb2bc96959c93bd2)"
  [ "$output" = "200" ]
}

@test "Frontend incorrect block search returns 500" {
  output="$(curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:3000/blocks/7777777777777777777777777777777777777777777777777777777777777777777777777777777777777777)"
  [ "$output" = "500" ]
}