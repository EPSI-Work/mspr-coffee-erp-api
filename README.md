
Setup Firebase : 
- create a project
- Authentification / Add email/password auth

- create Firestore database / test mode / europe
- rules / delete : 
    : if
  request.time < timestamp.date(2023, 3, 17);
    
Get Firebase Token : 
curl -X POST \  'https://www.googleapis.com/identitytoolkit/v3/relyingparty/verifyPassword?key=$API_KEY' \
  -H 'content-type: application/json' \
  -d '{ "email":"test@test.com", "password":"testeuh", "returnSecureToken":true }'

Add secret to Github Repo : 
GCP_SERVICE_ACCOUNT_KEY, GCP_PROJECT_ID and FIREBASE_ADMIN_KEY





Test firestore : 
curl -sL https://firebase.tools | bash
(firebase login:ci)

FIREBASE_TOKEN=1//03sbkviJIEhE2CgYIARAAGAMSNwF-L9Ir0ZlSRWKGfzc8AuXKcttOOPkoL_EaOz-nAAe43E5pVDHA2iOy7iU0vb9LRtPgO7d_Y5E
firebase init

firebase init emulators 


emulators:start --only firestore


cargo run --bin erp-import -- --file-path import/import-products.json --firebase-id mspr-epsi-coffee --firebase-token firebase-token/firebase-adminsdk-sa.json

docker run -d \
  --name firestore-emulator \
  --env "FIRESTORE_PROJECT_ID=mspr-epsi-coffee" \
  --env "PORT=8080" \
  --publish 8080:8080 \
  mtlynch/firestore-emulator-docker

export FIRESTORE_EMULATOR_HOST=localhost:8080
<!-- export FIRESTORE_PROJECT_ID="mspr-epsi-coffee
export PROJECT_ID="mspr-epsi-coffee
export GOOGLE_APPLICATION_CREDENTIALS=firebase-token/firebase-adminsdk-sa.json -->


firebase init
firebase emulators:start

1//03ZkpALpe1DWyCgYIARAAGAMSNwF-L9IrbcUavCVOeNjorBhlmCEr3Axsk6JwHukGUTmLtT9PSMN3ugK-loRznmIiB_4_1119Dlg