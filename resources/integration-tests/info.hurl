# Info endpoint integration test
GET http://app:8080/_/info

HTTP 200
[Asserts]
jsonpath "$.environments" exists
jsonpath "$.sql" exists
jsonpath "$.nosql" exists
jsonpath "$.http" exists

# Verify SQL check results
jsonpath "$.sql.TESTDB.success" == true
jsonpath "$.sql.TESTDB.driver" == "postgres"
jsonpath "$.sql.TESTDB.database" == "testdb"

# Verify NoSQL check results
jsonpath "$.nosql.TESTREDIS.success" == true
jsonpath "$.nosql.TESTREDIS.driver" == "redis"

# Verify HTTP check results
jsonpath "$.http.TESTAPI.success" == true
jsonpath "$.http.TESTAPI.status_code" == 200
