BASEDIR=$(dirname "$0")
DOMAIN="calculator.localhost"

FILE_KEY="${BASEDIR}/ssl.key"
FILE_CRT="${BASEDIR}/ssl.crt"
FILE_CSR="${BASEDIR}/ssl.csr"
SUBJECT="/C=FR/ST=France/L=Lyon/O=JochLAin/CN=${DOMAIN}"

rm -f $FILE_KEY $FILE_CRT $FILE_CSR

#openssl genrsa -des3 -out $FILE_KEY -passout $PASSWORD 2048
openssl req -newkey rsa:2048 -new -nodes -keyout $FILE_KEY -out $FILE_CSR -subj $SUBJECT
openssl x509 -req -days 365 -in $FILE_CSR -signkey $FILE_KEY -out $FILE_CRT