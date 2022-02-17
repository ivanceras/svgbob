# Creates REST API for converting ASCII text to Svgbob diagrams
#
# QUICK START:
#   docker build --tag svgbob-rest-api .
#   docker container run --rm -p80:80 svgbob-rest-api
#
# SAMPLE API REQUEST using curl:
#   curl -X POST -F 'ascii=o------>' http://localhost > output.svg


FROM rust:latest

################## INSTALL SVGBOB SERVER ##################

RUN cargo install svgbob_server

ENV PORT=80 

EXPOSE 80

CMD "svgbob_server"
