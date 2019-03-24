FROM alpine

COPY target/debug/producer /usr/local/bin/
COPY docker-entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/producer
RUN chmod +x /usr/local/bin/docker-entrypoint.sh
ENTRYPOINT ["producer"]
CMD []
#EXPOSE 21
