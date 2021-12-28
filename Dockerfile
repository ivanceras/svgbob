# Creates REST API for converting ASCII text to Svgbob diagrams
#
# QUICK START:
#   docker build --tag svgbob-rest-api .
#   docker container run --rm -p80:80 svgbob-rest-api
#
# SAMPLE API REQUEST using curl:
#   curl -X POST -F 'ascii=o------>' http://localhost > output.svg


FROM rust:latest

################## INSTALL SVGBOB ##################

RUN cargo install svgbob_cli

################## INSTALL APACHE & PHP ##################

RUN apt-get update
RUN apt-get -y install apache2 php

ENV APACHECONF="/etc/apache2/apache2.conf"
RUN echo "ServerName localhost" >> $APACHECONF

RUN rm /var/www/html/index.html

################## CREATE REST API ##################

RUN touch /var/www/html/input.txt
RUN chmod 666 /var/www/html/input.txt

ENV SERVICEFILE="/var/www/html/index.php"
RUN echo "<?php" > $SERVICEFILE
RUN echo "file_put_contents('input.txt', \$_POST['ascii']);" >> $SERVICEFILE
RUN echo "passthru('svgbob < input.txt');" >> $SERVICEFILE

EXPOSE 80
ENTRYPOINT ["/usr/sbin/apache2ctl" ,"-D", "FOREGROUND"]
