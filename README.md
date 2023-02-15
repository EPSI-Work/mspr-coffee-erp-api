

    
get firebase token : 
API_KEY=
curl -X POST \  'https://www.googleapis.com/identitytoolkit/v3/relyingparty/verifyPassword?key=$API_KEY' \
  -H 'content-type: application/json' \
  -d '{ "email":"test@test.com", "password":"testeuh", "returnSecureToken":true }'

curl -X GET \
  https://token-details-gw-dd05oxhr.uc.gateway.dev/validatetoken \
  -H 'authorization: Bearer AUTH_TOKEN' 



TODO add secret to Github Repo : 

GCP_SERVICE_ACCOUNT_KEY and GCP_PROJECT_ID

cargo tarpaulin --out Xml --output-dir coverage