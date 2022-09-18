wget https://repo1.maven.org/maven2/org/openapitools/openapi-generator-cli/6.1.0/openapi-generator-cli-6.1.0.jar -O openapi-generator-cli.jar

wget --no-check-certificate https://api.ntropy.network/openapi.json

PACKAGE_NAME="ntropy-api-client"
rm -rf ${PACKAGE_NAME}

java -jar openapi-generator-cli.jar generate \
   -i openapi.json \
   -g rust \
   -o ${PACKAGE_NAME} \
   -p packageName=${PACKAGE_NAME} \
   -p packageVersion="0.0.1"

rm -rf openapi-generator-cli.jar
rm -rf openapi.json
rm -rf ${PACKAGE_NAME}/docs
rm -rf ${PACKAGE_NAME}/.travis.yml
rm -rf ${PACKAGE_NAME}/git_push.sh
